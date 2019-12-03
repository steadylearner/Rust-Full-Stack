-- Make comments similar to (user, post), (post, comments)
-- post_id uuid UNIQUE NOT NULL REFERENCES posts(id),

-- https://realpython.com/get-started-with-django-1/
-- Comments to a post(Many to one).
-- Comment to a user.(One to one)
CREATE TABLE comments (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
  -- forein key to JOIN a user and post.
  -- Use ON DELETE CASCADE to make them deleted when the post is removed.
  -- When a user is deleted, the posts will be deleted also.
  -- So we don't need ON DELETE CASCADE for user_id here.
  user_id uuid NOT NULL REFERENCES users(id),
  post_id uuid NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
  body TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NULL
);

INSERT INTO
comments (id, user_id, post_id, body)
VALUES(uuid_generate_v4(), '6ad63f1b-fc5b-47dc-a05b-042474837664', '9b3fc489-e85e-4395-ab95-f19d8d6c4fd8', 'Test a comment');

INSERT INTO
comments (id, user_id, post_id, body)
VALUES(uuid_generate_v4(), '6ad63f1b-fc5b-47dc-a05b-042474837664', '9b3fc489-e85e-4395-ab95-f19d8d6c4fd8', 'Test another comment');

INSERT INTO
comments (id, user_id, post_id, body)
VALUES(uuid_generate_v4(), '68eba618-032b-43c4-a1fb-45a477774be8', '9b3fc489-e85e-4395-ab95-f19d8d6c4fd8', 'Test a comment from another user.');

INSERT INTO
comments (id, user_id, post_id, body)
VALUES(uuid_generate_v4(), '68eba618-032b-43c4-a1fb-45a477774be8', '9b3fc489-e85e-4395-ab95-f19d8d6c4fd8', 'Test another comment from another user.');
