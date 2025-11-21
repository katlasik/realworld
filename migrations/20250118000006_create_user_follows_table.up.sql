-- Create user_follows join table for following relationships
CREATE TABLE IF NOT EXISTS user_follows (
    follower_id UUID NOT NULL,
    followee_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (follower_id, followee_id),
    CONSTRAINT fk_user_follows_follower FOREIGN KEY (follower_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_user_follows_followee FOREIGN KEY (followee_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT chk_no_self_follow CHECK (follower_id != followee_id)
);

-- Create index on followee_id for faster lookups of followers
CREATE INDEX idx_user_follows_followee_id ON user_follows(followee_id);

-- Create index on follower_id for faster lookups of following
CREATE INDEX idx_user_follows_follower_id ON user_follows(follower_id);
