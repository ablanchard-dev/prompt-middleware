chrome.runtime.onInstalled.addListener(() => {
  chrome.storage.local.set({ enabled: true, localServerPort: 43187 });
});

