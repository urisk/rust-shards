CREATE TABLE users
(
    id         SERIAL PRIMARY KEY,
    username   varchar(64)             NOT NULL UNIQUE,
    first_name varchar(64),
    last_name  varchar(64),
    email      varchar(128),
    phone      char(15),
    bio        text,
    password   varchar(128)            NOT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL
)
