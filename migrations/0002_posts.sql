CREATE TABLE posts
(
    id      BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    title   TEXT   NOT NULL,
    body    TEXT   NOT NULL
);

CREATE INDEX idx_posts_user_id ON posts (user_id);