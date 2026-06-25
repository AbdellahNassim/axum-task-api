CREATE TABLE tasks (
    id UUID PRIMARY KEY,

    title TEXT NOT NULL,

    description TEXT,

    completed BOOLEAN NOT NULL DEFAULT FALSE,

    user_id UUID NOT NULL
        REFERENCES users(id)
        ON DELETE CASCADE,

    created_at TIMESTAMPTZ NOT NULL
);