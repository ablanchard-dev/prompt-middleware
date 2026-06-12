const portInput = document.querySelector<HTMLInputElement>("#local-server-port");

if (portInput) {
  chrome.storage.local.get(["localServerPort"], (values) => {
    portInput.value = String(values.localServerPort ?? 43187);
  });

  portInput.addEventListener("change", () => {
    chrome.storage.local.set({ localServerPort: Number(portInput.value) });
  });
}

