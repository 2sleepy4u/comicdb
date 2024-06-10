CREATE TABLE Comics (
    isbn VARCHAR(255) PRIMARY KEY,
    title VARCHAR(255),
    genre VARCHAR(255),
    author VARCHAR(255),
    image_url VARCHAR(255),
    price FLOAT DEFAULT 0
);

CREATE TABLE MagMov (
    id_mov INT PRIMARY KEY AUTO_INCREMENT,
    isbn VARCHAR(255),
    quantity_s INT NOT NULL,
    quantity_c INT NOT NULL,
    mov_date DATETIME,
    note VARCHAR(255),
    FOREIGN KEY (isbn) REFERENCES Comics(isbn)
);
