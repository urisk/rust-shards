CREATE TABLE templates
(
    id         SERIAL PRIMARY KEY,
    title      varchar(128)            NOT NULL,
    owner_id   integer                 NOT NULL REFERENCES users (id),
    version    varchar(16)             NOT NULL DEFAULT '',
    visibility integer                 NOT NULL,
    template   text,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL
)
