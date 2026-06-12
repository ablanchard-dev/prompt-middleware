export interface PreviewModalActions {
  onAccept(editedPrompt: string): void;
  onCancel(): void;
}

export function showPreviewModal(
  originalPrompt: string,
  optimizedPrompt: string,
  actions: PreviewModalActions
): HTMLElement {
  const root = document.createElement("div");
  root.className = "pmw-preview";

  const title = document.createElement("h2");
  title.textContent = "Preview du prompt optimise";

  const original = document.createElement("pre");
  original.textContent = originalPrompt;

  const optimized = document.createElement("textarea");
  optimized.value = optimizedPrompt;

  const accept = document.createElement("button");
  accept.type = "button";
  accept.textContent = "Remplacer";
  accept.addEventListener("click", () => actions.onAccept(optimized.value));

  const cancel = document.createElement("button");
  cancel.type = "button";
  cancel.textContent = "Annuler";
  cancel.addEventListener("click", actions.onCancel);

  root.append(title, original, optimized, accept, cancel);
  document.body.append(root);

  return root;
}

export function showStatusMessage(message: string, kind: "error" | "success"): HTMLElement {
  const root = document.createElement("div");
  root.className = `pmw-status pmw-status-${kind}`;
  root.textContent = message;

  document.body.append(root);
  window.setTimeout(() => root.remove(), 6_000);

  return root;
}

export function showRestoreOriginal(
  onRestore: () => void,
  onDismiss: () => void
): HTMLElement {
  const root = document.createElement("div");
  root.className = "pmw-status pmw-status-success";

  const message = document.createElement("span");
  message.textContent = "Prompt remplace.";

  const restore = document.createElement("button");
  restore.type = "button";
  restore.textContent = "Restaurer l'original";
  restore.addEventListener("click", () => {
    onRestore();
    root.remove();
  });

  const dismiss = document.createElement("button");
  dismiss.type = "button";
  dismiss.textContent = "Fermer";
  dismiss.addEventListener("click", () => {
    onDismiss();
    root.remove();
  });

  root.append(message, restore, dismiss);
  document.body.append(root);

  return root;
}
