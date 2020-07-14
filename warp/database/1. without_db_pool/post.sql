-- It should already be used with diesel CLI

-- CREATE TABLE posts (
--   id SERIAL PRIMARY KEY,
--   title VARCHAR NOT NULL,
--   body TEXT NOT NULL
-- )

-- Login to postgresql with postgresql comands
-- sudo -i -u postgres psql
-- sudo -u postgres createuser --interactive
-- \c warp;

-- Then, copy and paste these commands.

INSERT INTO
posts(title, body)
VALUES('How to be a Rust full stack developer?', 'Love Rust');

INSERT INTO
posts(title, body)
VALUES('Where is a developer job?', 'Wait for your time to come while you help others. Hire Steadylearner if you liked his work.');

INSERT INTO
posts(title, body)
VALUES('What can you work with?', 'Rust, Node, React, Python, SQL, Docker, AWS, Linux etc. Do you believe that I never had an opportunity to work with it?');

INSERT INTO
posts(title, body)
VALUES('Give me a chance please?', 'I will do my best.');

-- Verify it worked with $SELECT * FROM posts;