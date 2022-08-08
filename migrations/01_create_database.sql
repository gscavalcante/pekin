CREATE TABLE book
(
    id          varchar(255) NOT NULL,
    description varchar(255) NOT NULL,

    CONSTRAINT id_pk PRIMARY KEY (id)
);

CREATE TABLE account
(
    id          varchar(255) NOT NULL,
    name        varchar(255) NOT NULL,
    description varchar(255),
    category    varchar(12) NOT NULL,

    CONSTRAINT id_pk PRIMARY KEY (id)
);
