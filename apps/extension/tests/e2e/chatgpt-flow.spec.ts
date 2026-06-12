import { chromium, expect, test } from "@playwright/test";
import path from "node:path";

const chromePath = "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe";
const profilePath = path.resolve(".playwright-profile");

async function openChatGptMock() {
  const context = await chromium.launchPersistentContext(profilePath, {
    executablePath: chromePath,
    headless: false
  });
  const page = await context.newPage();

  await page.route("https://chatgpt.com/**", async (route) => {
    await route.fulfill({
      contentType: "text/html",
      body: `
        <!doctype html>
        <html>
          <body>
            <main>
              <textarea aria-label="Message ChatGPT"></textarea>
              <button type="button" aria-label="Send prompt" onclick="window.__sent = true">Send</button>
            </main>
          </body>
        </html>
      `
    });
  });

  await page.goto("https://chatgpt.com/");
  await page.addScriptTag({ path: path.resolve("dist/content/index.js") });

  return { context, page };
}

test("optimizes and restores a prompt on a ChatGPT-like page", async () => {
  const { context, page } = await openChatGptMock();

  try {
    await page.route("http://127.0.0.1:43187/optimize", async (route) => {
      await route.fulfill({
        contentType: "application/json",
        body: JSON.stringify({
          optimized_prompt:
            "Tu es un pedagogue expert.\n\nDemande utilisateur :\nFais moi un plan pour apprendre Rust",
          detected_language: "fr",
          detected_domain: "apprentissage",
          detected_intent: "planifier",
          confidence: 0.78,
          quality_score: {
            clarity: 0.82,
            context: 0.55,
            constraints: 0.8,
            format: 0.9,
            overall: 0.74
          },
          warnings: ["La demande initiale contient peu de contexte."],
          needs_clarification: true,
          clarification_questions: [
            "Quel contexte ou objectif final doit etre pris en compte ?"
          ]
        })
      });
    });

    await page.locator("textarea").fill("Fais moi un plan pour apprendre Rust");

    await expect(page.locator(".pmw-optimize-button")).toBeVisible();
    await page.locator(".pmw-optimize-button").click();

    await expect(page.locator(".pmw-preview")).toBeVisible();
    await expect(page.locator(".pmw-preview textarea")).toHaveValue(/pedagogue expert/);

    await page.getByRole("button", { name: "Remplacer" }).click();
    await expect(page.locator("textarea")).toHaveValue(/pedagogue expert/);
    await expect(page.getByRole("button", { name: "Send prompt" })).toBeVisible();
    await expect.poll(() => page.evaluate(() => Boolean(window.__sent))).toBe(false);

    await page.getByRole("button", { name: "Restaurer l'original" }).click();
    await expect(page.locator("textarea")).toHaveValue("Fais moi un plan pour apprendre Rust");
  } finally {
    await context.close();
  }
});

test("shows a clean error when local optimizer is unavailable", async () => {
  const { context, page } = await openChatGptMock();

  try {
    await page.route("http://127.0.0.1:43187/optimize", async (route) => {
      await route.abort("connectionrefused");
    });

    await page.locator("textarea").fill("Fais moi un plan pour apprendre Rust");
    await page.locator(".pmw-optimize-button").click();

    await expect(page.locator(".pmw-status-error")).toContainText(
      "Le moteur local n'est pas disponible"
    );
    await expect(page.locator("textarea")).toHaveValue(
      "Fais moi un plan pour apprendre Rust"
    );
    await expect.poll(() => page.evaluate(() => Boolean(window.__sent))).toBe(false);
  } finally {
    await context.close();
  }
});
