CREATE TABLE circle_members
(
    id        SERIAL PRIMARY KEY,
    circle_id integer NOT NULL REFERENCES circles (id),
    user_id   integer NOT NULL REFERENCES users (id),
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL
)
