(async () => {
    try {
        let img = window.location.origin + $("#coverImg").attr("src")
        await copyToTheClipboard(img) 
    } catch(e) {

    }
})()
