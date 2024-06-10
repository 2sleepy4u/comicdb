(async () => {
    try {
        let img = window.location.origin + $("[itemprop=image]").attr("src") 
        await copyToTheClipboard(img) 
    } catch(e) {

    }
})()
