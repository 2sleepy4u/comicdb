(async () => {
    try {
        let res = await pageFunction()
        let title = res[0].creator.filter(x => x._type.includes("Person"))[0].name
        await copyToTheClipboard(title) 
    } catch(e) {

    }
})()
