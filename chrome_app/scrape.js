
document.getElementById('title').addEventListener('click', () => {
    chrome.tabs.query({active: true, currentWindow: true}, (tabs) => {
        chrome.scripting.executeScript({ 
            target: {tabId: tabs[0].id},
            files: ["jquery.js", 'content.js', "actions/title.js"]
        }) 
    });
});
document.getElementById('genere').addEventListener('click', () => {
    chrome.tabs.query({active: true, currentWindow: true}, (tabs) => {
        chrome.scripting.executeScript({ 
            target: {tabId: tabs[0].id},
            files: ["jquery.js", 'content.js', "actions/genere.js"]
        }) 
    });
});
document.getElementById('autore').addEventListener('click', () => {
    chrome.tabs.query({active: true, currentWindow: true}, (tabs) => {
        chrome.scripting.executeScript({ 
            target: {tabId: tabs[0].id},
            files: ["jquery.js", 'content.js', "actions/author.js"]
        }) 
    });
});
document.getElementById('thumb').addEventListener('click', () => {
    chrome.tabs.query({active: true, currentWindow: true}, (tabs) => {
        chrome.scripting.executeScript({ 
            target: {tabId: tabs[0].id},
            files: ["jquery.js", 'content.js', "actions/thumb.js"]
        }) 
    });
});

