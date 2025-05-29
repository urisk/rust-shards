CREATE TABLE roles
(
    id         SERIAL PRIMARY KEY,
    code       varchar(64)             NOT NULL UNIQUE,
    name       varchar(128)            NOT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL
);

INSERT INTO roles(code,name)
VALUES ('user','user');
INSERT INTO roles(code,name)
VALUES ('admin','admin');
