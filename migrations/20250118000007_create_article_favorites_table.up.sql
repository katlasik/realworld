-- Create article_favorites join table
CREATE TABLE IF NOT EXISTS article_favorites (
    user_id UUID NOT NULL,
    article_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, article_id),
    CONSTRAINT fk_article_favorites_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_article_favorites_article FOREIGN KEY (article_id) REFERENCES articles(id) ON DELETE CASCADE
);

-- Create index on article_id for faster lookups of users who favorited
CREATE INDEX idx_article_favorites_article_id ON article_favorites(article_id);

-- Create index on user_id for faster lookups of favorited articles by user
CREATE INDEX idx_article_favorites_user_id ON article_favorites(user_id);
