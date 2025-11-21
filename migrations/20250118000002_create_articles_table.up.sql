-- Create articles table
CREATE TABLE IF NOT EXISTS articles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    slug VARCHAR(255) NOT NULL UNIQUE,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    body TEXT NOT NULL,
    author_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_articles_author FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create index on slug for faster lookups
CREATE INDEX idx_articles_slug ON articles(slug);

-- Create index on author_id for faster lookups of articles by author
CREATE INDEX idx_articles_author_id ON articles(author_id);

-- Create index on created_at for sorting by date
CREATE INDEX idx_articles_created_at ON articles(created_at DESC);
