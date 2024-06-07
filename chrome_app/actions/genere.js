(async () => {
    try {
        let res = await pageFunction()
        let genere = res[0].genre.reduce((acc, item) => item + (acc != "" ? ", "+acc : acc), "")
        await copyToTheClipboard(genere) 
    } catch(e) {

    }
})()
