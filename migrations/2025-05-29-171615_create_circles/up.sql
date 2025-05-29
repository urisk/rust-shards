CREATE TABLE circles
(
    id         SERIAL PRIMARY KEY,
    name       varchar(128)            NOT NULL,
    owner_id   integer                 NOT NULL REFERENCES users (id),
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL
);
