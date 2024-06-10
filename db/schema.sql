CREATE TABLE Comics (
    id_comic INT PRIMARY KEY,
    isbn VARCHAR(255),
    title VARCHAR(255),
    genre VARCHAR(255),
    author VARCHAR(255),
    image VARCHAR(255),
    volume INT,
    price FLOAT DEFAULT 0,
    active BOOL DEFAULT TRUE,
    external_link VARCHAR(255)
);

CREATE TABLE MagMov (
    id_mov INT PRIMARY KEY,
    id_comic VARCHAR(255),
    quantity_s INT NOT NULL,
    quantity_c INT NOT NULL,
    mov_date DATETIME,
    note VARCHAR(255),
    FOREIGN KEY (id_comic) REFERENCES Comics(id_comic)
);
