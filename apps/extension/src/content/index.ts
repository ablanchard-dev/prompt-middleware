import { getPlatformAdapter } from "../platforms/registry";
import { optimizePrompt } from "../shared/api-client";
import { createOptimizeButton } from "../ui/optimize-button";
import {
  showPreviewModal,
  showRestoreOriginal,
  showStatusMessage
} from "../ui/preview-modal";

const adapter = getPlatformAdapter();

if (adapter) {
  const mount = (): void => {
    const input = adapter.detectInput();
    const anchor = adapter.findAnchorElement();

    if (!input || !anchor || document.querySelector(".pmw-optimize-button")) {
      return;
    }

    const button = createOptimizeButton(async () => {
      const currentInput = adapter.detectInput();
      if (!currentInput) {
        showStatusMessage("Champ de saisie introuvable.", "error");
        return;
      }

      const original = adapter.readInput(currentInput).trim();
      if (!original) {
        showStatusMessage("Le prompt est vide.", "error");
        return;
      }

      button.disabled = true;
      button.textContent = "Optimisation...";

      try {
        const response = await optimizePrompt({
          raw_user_input: original,
          target_platform: adapter.id,
          language: "auto",
          mode: "preview",
          user_preferences: {
            tone: null,
            detail_level: "normal"
          }
        });

        let modal: HTMLElement | null = null;
        let restoreToast: HTMLElement | null = null;
        modal = showPreviewModal(original, response.optimized_prompt, {
          onAccept(editedPrompt) {
            adapter.replaceInput(currentInput, editedPrompt);
            modal?.remove();
            restoreToast?.remove();
            restoreToast = showRestoreOriginal(
              () => adapter.replaceInput(currentInput, original),
              () => {
                restoreToast = null;
              }
            );
          },
          onCancel() {
            modal?.remove();
          }
        });
      } catch {
        showStatusMessage(
          "Le moteur local n'est pas disponible. Verifiez que le serveur Rust tourne sur 127.0.0.1:43187.",
          "error"
        );
      } finally {
        button.disabled = false;
        button.textContent = "Optimiser";
      }
    });

    anchor.parentElement?.append(button);
  };

  mount();

  const observer = new MutationObserver(() => mount());
  observer.observe(document.body, { childList: true, subtree: true });
}
