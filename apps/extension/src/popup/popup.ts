const enabledInput = document.querySelector<HTMLInputElement>("#enabled");

if (enabledInput) {
  chrome.storage.local.get(["enabled"], (values) => {
    enabledInput.checked = values.enabled !== false;
  });

  enabledInput.addEventListener("change", () => {
    chrome.storage.local.set({ enabled: enabledInput.checked });
  });
}

