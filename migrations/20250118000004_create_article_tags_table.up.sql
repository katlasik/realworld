-- Create article_tags join table
CREATE TABLE IF NOT EXISTS article_tags (
    article_id UUID NOT NULL,
    tag_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (article_id, tag_id),
    CONSTRAINT fk_article_tags_article FOREIGN KEY (article_id) REFERENCES articles(id) ON DELETE CASCADE,
    CONSTRAINT fk_article_tags_tag FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- Create index on tag_id for faster lookups of articles by tag
CREATE INDEX idx_article_tags_tag_id ON article_tags(tag_id);

-- Create index on article_id for faster lookups of tags by article
CREATE INDEX idx_article_tags_article_id ON article_tags(article_id);
