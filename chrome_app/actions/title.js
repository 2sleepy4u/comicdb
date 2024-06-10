(async () => {
    try {
        let res = await pageFunction()
        let title = res[0].name[0]
        await copyToTheClipboard(title) 
    } catch(e) {

    }
})()
