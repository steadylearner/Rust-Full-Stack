const moment = require("moment");
const Router = require("express-promise-router");
const router = Router();
const pg = require("../db/postgresql");
// const {
//     index,
// } = require("../controllers/blog");

// Make it work. Then, build email authorization example.

router.get("/", async (req, res) => {
	console.log("Inside the GET /blog callback function");

	const sql = `
		SELECT posts.id, posts.title, posts.body, posts.created_at, users.username
		FROM posts INNER JOIN users ON(posts.user_id = users.id)
		ORDER BY posts.created_at DESC
	`;

	const query = {
		text: sql,
	};

	try {
		const { rows: blogs, rowLength } = await pg.query(query);
		console.log(blogs);
		// Refer to ./profileController
		if (rowLength === 0) { // There is no post with the id.
			res.json({});
		}
		res.json(blogs);
	} catch (error) {
		console.log("Couldn't execute the query.");
		res.json({ error });
	}
});

router.get("/:tag", async (req, res) => {
	const { tag } = req.params;
	console.log(`Inside the GET /blog/${tag} callback function`);

	// http://www.postgresqltutorial.com/postgresql-array/
	// Use posts_tags.sql with many to many relations if necessary
	const sql = `
		SELECT * FROM posts WHERE $1 = ANY (tags)
	`;

	const query = {
		text: sql,
		values: [tag],
	};

	try {
		const { rows: blogs, rowLength } = await pg.query(query);
		console.log(blogs);
		if (rowLength === 0) { // There is no post with the id.
			res.json({});
		}
		res.json(blogs);
	} catch (error) {
		console.log("Couldn't execute the query.");
		res.json({ error });
	}
});

// localhost:8000/blog/view?title=test&id=4e9f311c-8719-4667-955c-ec04cddeff9e
router.get("/view", async (req, res) => {
	const { title, id } = req.query;
	console.log(req.query);
	console.log(`Inside the GET /blog/view/?title=${title}&id=${id} callback function`);

	// SELECT posts.title, posts.content, posts.created_at, users.username, users.id FROM posts INNER JOIN users ON(posts.user_id = users.id) WHERE posts.id='4e9f311c-8719-4667-955c-ec04cddeff9e';
	const sqlBlogPost = `
		SELECT posts.id, posts.title, posts.body, posts.created_at, users.username
		FROM posts INNER JOIN users ON(posts.user_id = users.id)
		WHERE posts.id=$1
	`;
	// Use various INNER JOIN to include comments later otherwise separate sqlBlogPost commands for comments.
	// Then, send thoses datas separately.

	const blogQuery = {
		text: sqlBlogPost,
		values: [id],
	};

	try {
		const { rows: blogs, rowLength } = await pg.query(blogQuery);
		console.log(blogs);
		// Refer to ./profileController
		if (rowLength === 0) { // There is no post with the id.
			res.json({});
		}

		const blog = blogs[0];
		const {
			id: post_id,
			title,
			body,
			created_at,
			username,
		} = blog;

		// SELECT comments.id, comments.body, comments.created_at, users.username
		// FROM comments INNER JOIN users ON(comments.user_id = users.id)
		// WHERE comments.post_id = 9b3fc489-e85e-4395-ab95-f19d8d6c4fd8
		const sqlBlogComments = `
			SELECT comments.id, comments.body, comments.created_at, users.username
			FROM comments INNER JOIN users ON(comments.user_id = users.id)
			WHERE comments.post_id=$1
		`;

		const blogCommentsQuery = {
			text: sqlBlogComments,
			values: [post_id],
		};

		try {
			const { rows: comments, } = await pg.query(blogCommentsQuery);

			res.json({
				blog: {
					id: post_id,
					title,
					body,
					created_at: moment(created_at).format("YYYY-MM-DD"),
					username,
				},
				comments,
			});

		} catch(error) {
			console.log("Couldn't execute the query to view the comments.");
			res.json({ error });
		}
	} catch (error) {
		console.log("Couldn't execute the query to view the blog post.");
		res.json({ error });
	}
});

module.exports = router;

// // localhost:8000/blog/view?title=test&id=4e9f311c-8719-4667-955c-ec04cddeff9e
// router.get("/view", async (req, res) => {
// 	const { title, id } = req.query;
// 	console.log(req.query);
// 	console.log(`Inside the GET /blog/view/?title=${title}&id=${id} callback function`);

// 	// SELECT posts.title, posts.id, posts.content, posts.created_at, users.username FROM posts INNER JOIN users ON(posts.user_id = users.id) WHERE posts.id='4e9f311c-8719-4667-955c-ec04cddeff9e';
// 	const sql = `
// 		SELECT posts.title, posts.id, posts.content, posts.created_at, users.username
// 		FROM posts INNER JOIN users ON(posts.user_id = users.id)
// 		WHERE posts.id=$1
// 	`;
// 	// Use various INNER JOIN to include comments later otherwise separate sql commands for comments.
// 	// Then, send thoses datas separately.

// 	const query = {
// 		text: sql,
// 		values: [id],
// 	};

// 	console.log("\n[GET] Post \n");
// 	try {
// 		const { rows: blogs, rowLength } = await pg.query(query);
// 		console.log(blogs);
// 		// Refer to ./profileController
// 		if (rowLength === 0) { // There is no post with the id.
// 			res.json({});
// 		}
// 		res.json(blogs[0]);
// 	} catch (error) {
// 		console.log("Couldn't execute the query.");
// 		res.json({ error });
// 	}
// });
