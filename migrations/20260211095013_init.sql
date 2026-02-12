-- Add migration script here-- AcidComics™ initial schema

-- Enable UUID generation (Postgres extension)
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- USERS
CREATE TABLE IF NOT EXISTS users (
  id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  email        TEXT NOT NULL UNIQUE,
  password_hash TEXT NOT NULL,
  created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- COMICS
CREATE TABLE IF NOT EXISTS comics (
  id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  author_id   UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  title       TEXT NOT NULL,
  description TEXT,
  image_url   TEXT, -- later: could be Supabase Storage URL / local path
  created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_comics_created_at ON comics(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_comics_author_id ON comics(author_id);

-- VOTES (one vote per user per comic)
-- value: +1 (upvote) or -1 (downvote) if you want that later; for now we allow int.
CREATE TABLE IF NOT EXISTS votes (
  user_id   UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  comic_id  UUID NOT NULL REFERENCES comics(id) ON DELETE CASCADE,
  value     INT NOT NULL DEFAULT 1,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (user_id, comic_id),
  CONSTRAINT votes_value_chk CHECK (value IN (-1, 1))
);

CREATE INDEX IF NOT EXISTS idx_votes_comic_id ON votes(comic_id);
