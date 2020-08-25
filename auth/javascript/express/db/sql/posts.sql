-- Make comments similar to (user, post), (post, comments)
-- post_id uuid UNIQUE NOT NULL REFERENCES posts(id),

-- Should implement it later
-- https://stackoverflow.com/questions/9789736/how-to-implement-a-many-to-many-relationship-in-postgresql
-- categories = models.ManyToManyField('Category', related_name='posts')
-- posts to categories, many to many relationships
CREATE TABLE posts (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
  -- forein key to JOIN a user and post to show at /username/posts.
  user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  title VARCHAR(60) NOT NULL,
  --   subtitle VARCHAR(80) NOT NULL,
  body TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NULL,
  -- ALTER TABLE posts ADD COLUMN tags VARCHAR(15)[] DEFAULT '{}';
  -- ALTER TABLE posts DROP COLUMN tags';
  tags VARCHAR(15)[] DEFAULT '{}'
);

-- Test with postgresql console first.

INSERT INTO
posts(id, user_id, title, body, tags)
VALUES(uuid_generate_v4(), '6ad63f1b-fc5b-47dc-a05b-042474837664', 'The title of the test blog post', 'Test', '{"python"}');

INSERT INTO
posts
  (id, user_id, title, body, tags)
VALUES(uuid_generate_v4(), '6ad63f1b-fc5b-47dc-a05b-042474837664', 'The title of another test blog post', 'More test', '{"rust"}');

INSERT INTO
posts
  (id, user_id, title, body, tags)
VALUES(uuid_generate_v4(), '6ad63f1b-fc5b-47dc-a05b-042474837664', 'The title of another test blog post', 'More test', '{"javascript"}');

INSERT INTO
posts
  (id, user_id, title, body, tags)
VALUES(uuid_generate_v4(), '6ad63f1b-fc5b-47dc-a05b-042474837664', 'The title of another test blog post', 'More test', '{"javascript", "python"}');
