(async () => {
    try {
        let res = await pageFunction()
        let title = res[0].name[0]
        let author = res[0].creator.filter(x => x._type.includes("Person"))[0].name
        let genre = res[0].genre.reduce((acc, item) => item + (acc != "" ? ", "+acc : acc), "")
        let image = window.location.origin + $("#coverImg").attr("src")
        let external_link = window.location.href
        let comic = JSON.stringify({
            id_comic: 0,
            isbn: "",
            title,
            author,
            genre,
            image,
            price: 0,
            quantity: 0,
            volume: 0,
            active: true,
            external_link
        })
        await copyToTheClipboard(comic) 
    } catch(e) {

    }
})()
