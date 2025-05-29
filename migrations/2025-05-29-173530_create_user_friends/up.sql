CREATE TABLE user_friends
(
    id         SERIAL PRIMARY KEY,
    user_id    integer                 NOT NULL REFERENCES users (id),
    friend_id  integer                 NOT NULL REFERENCES users (id),
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL
)
