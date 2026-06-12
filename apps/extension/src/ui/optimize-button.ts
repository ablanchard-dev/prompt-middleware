export function createOptimizeButton(onClick: () => void): HTMLButtonElement {
  const button = document.createElement("button");
  button.type = "button";
  button.className = "pmw-optimize-button";
  button.textContent = "Optimiser";
  button.title = "Optimiser localement ce prompt";
  button.addEventListener("click", onClick);
  return button;
}

