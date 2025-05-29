CREATE TABLE user_roles
(
    id         SERIAL PRIMARY KEY,
    user_id    integer                 NOT NULL REFERENCES users (id),
    role_id    integer                 NOT NULL REFERENCES roles (id),
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL
)
