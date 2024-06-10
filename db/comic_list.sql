SELECT 
    C.id_comic,
    C.isbn,
    C.title,
    C.author,
    CAST(SUM(IFNULL(MM.quantity_c, 0) - IFNULL(MM.quantity_s, 0)) AS INT) as quantity,
    C.image,
    C.volume,
    C.price,
    C.genre,
    C.active,
    C.external_link
FROM
    Comics C LEFT JOIN
    MagMov MM ON C.id_comic = MM.id_comic
WHERE
    C.isbn = IFNULL(?, C.isbn)
AND C.title LIKE CONCAT('%', ?, '%')
AND C.genre LIKE CONCAT('%', ?, '%')
AND C.author LIKE CONCAT('%', ?, '%')
AND (? = false OR C.active = ?)
GROUP By
    C.isbn, C.title, C.author, C.image, C.price, C.genre
