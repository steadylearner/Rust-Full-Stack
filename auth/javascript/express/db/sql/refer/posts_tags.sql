-- Many to many
-- Each post can have many tags and each tag can have many
-- https://www.bthlabs.pl/post/fun_with_postgresql_tagging_blog_posts.html
-- Use array and substitue it with ORM or framework.

CREATE TABLE posts_tags (
  post_id uuid NOT NULL,
  tag_id uuid NOT NULL,
  FOREIGN KEY (post_id) REFERENCES posts(id) ON UPDATE CASCADE,
  FOREIGN KEY (tag_id) REFERENCES tags(id) ON UPDATE CASCADE,
  PRIMARY KEY (post_id, tag_id)
);

-- 1. How to insert data to it? Use sql commands?
-- 2. How to get datas from it? All the codes and queries here should be modified?

-- INSERT INTO posts
-- VALUES
--   (post_id, 'user1'),
--   (post_id, 'user2'),
--   (post_id, 'user3');

-- INSERT INTO tags
-- VALUES
--   (tag_id, 'group1'),
--   (6, 'group2'),
--   (7, 'group3');

-- INSERT INTO user_group
-- VALUES
--   (1, 6),
--   (1, 7),
--   (2, 5),
--   (2, 7),
--   (3, 5),
--   (3, 6),
--   (3, 7);