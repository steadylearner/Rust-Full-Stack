/*global chrome*/
export function getCurrentTab(callback) {
    chrome.tabs.query({
        active: true,
        currentWindow: true
    },
    (tabs) => {
        callback(tabs[0]);
    });
}