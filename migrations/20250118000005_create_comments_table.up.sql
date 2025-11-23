-- Create comments table
CREATE TABLE IF NOT EXISTS comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    body TEXT NOT NULL,
    article_id UUID NOT NULL,
    author_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_comments_article FOREIGN KEY (article_id) REFERENCES articles(id) ON DELETE CASCADE,
    CONSTRAINT fk_comments_author FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create index on article_id for faster lookups of comments by article
CREATE INDEX idx_comments_article_id ON comments(article_id);

-- Create index on author_id for faster lookups of comments by author
CREATE INDEX idx_comments_author_id ON comments(author_id);

-- Create index on created_at for sorting by date
CREATE INDEX idx_comments_created_at ON comments(created_at DESC);
