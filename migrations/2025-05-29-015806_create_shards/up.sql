CREATE TABLE shards
(
    id             SERIAL PRIMARY KEY,
    shard_category integer                 NOT NULL REFERENCES categories (id),
    title          varchar(128)            NOT NULL,
    owner_id       integer                 NOT NULL REFERENCES users (id),
    visibility     integer                 NOT NULL,
    parent_shard   integer REFERENCES shards (id),
    genre          varchar(128),
    shard          text,
    created_at     TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at     TIMESTAMP DEFAULT NOW() NOT NULL
)
