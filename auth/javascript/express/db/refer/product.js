const grpc = require("grpc");

const {
	GetProduct,
	// UpdateProduct
} = require("./classes/Product");
const db = require("../db");

const getProduct = async ({ request }, callback) => {
	const { id } = request;

	const sql = "SELECT * FROM products WHERE id = $1";
	const query = {
		text: sql,
		values: [id],
	};

	try {
		const { rows } = await db.query(query);

		if (rows.length !== 0) {
			console.log("\n[GET] Product\n");
			const product = rows[0];
			console.log(product);
			const payload = new GetProduct(id, product);

			callback(null, payload);
		}
		else {
			callback({
				code: grpc.status.NOT_FOUND,
				details: "Not Found"
			});
		}
	} catch (e) {
		console.log(e);
		callback(e);
	}
};

const createProduct = async ({ request }, callback) => {
	const id = require("crypto").randomBytes(10).toString("hex");
	const sql = "INSERT INTO products(id, price_in_cents, title, description, discount.pct, discount.value_in_cents) VALUES($1, $2, $3, $4, $5, $6)";

	const { price_in_cents, title, description, pct } = request;
	const value_in_cents = Math.ceil(price_in_cents * pct); // Type match for int
	const query = {
		text: sql,
		values: [id, price_in_cents, title, description, pct, value_in_cents],
	};

	console.log(request);

	try {
		const { rowCount } = await db.query(query);

		if (rowCount === 1) {
			console.log(`Create ${rowCount} product with id(${id}).`);
			callback(null, {
				id,
				price_in_cents,
				title,
				description,
				discount: {
					pct,
					value_in_cents,
				}
			});
		} else {
			callback({
				code: grpc.status.CANCELLED,
				details: "CANCELLED",
			});

			// This goes to catch part
			// callback({
			//     code: grpc.status.ALREADY_EXISTS,
			//     details: "ALREADY EXISTS",
			// })
		}
	} catch (e) {
		console.log(e);
		callback(e);
	}
};

module.exports = {
	getProduct,
	createProduct,
};

