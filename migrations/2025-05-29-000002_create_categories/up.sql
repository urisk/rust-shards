CREATE TABLE categories
(
    id         SERIAL PRIMARY KEY,
    name       varchar(128)            NOT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL
);

INSERT INTO categories(name) VALUES('shard');
INSERT INTO categories(name) VALUES('dream');
INSERT INTO categories(name) VALUES('book');
INSERT INTO categories(name) VALUES('meditation');