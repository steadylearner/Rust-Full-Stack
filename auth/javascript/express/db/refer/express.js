/* eslint-disable jest/no-test-callback */
// Its documentation is not helpful at all.
const app = require("../../server");

const supertest = require("supertest");
const request = supertest(app);

afterEach(() => {
	client.end(true);
});

describe("Test end points with jest and supertest", () => {
	test("[GET] /", async done => {
		try {
			const response = await request
				.get("/");

			expect(response.status).toBe(200);
			expect(response.text).toBe("User visit / and verify the console to see sessionID");

			done();
		} catch (e) {
			done(e);
		}
	});

	test("[POST] /login without credentials", async done => {
		try {
			const response = await request
				.post("/login");

			const expected = {"errors":[{"email":"Invalid value"},{"password":"Invalid value"}]};
			expect(response.status).toBe(422);
			expect(JSON.parse(response.text)).toStrictEqual(expected);

			done();
		} catch (e) {
			done(e);
		}
	});

	test("[POST] /login with totally invalid credentials", async done => {
		try {
			const response = await request
				.post("/login")
				.send({
					email: "invalid@email.com",
					password: "badpasword",
				})
				.set("Content-Type", "application/json");

			const expected = {
				message: "Invalid credentials.",
			};

			expect(response.status).toBe(200);
			expect(JSON.parse(response.text)).toStrictEqual(expected);

			done();
		} catch (e) {
			done(e);
		}
	});

	// it('should POST JSON', function (done) {
	// 	supertest(app)
	// 		.post('/postjson')
	// 		.send({
	// 			'id': 1,
	// 			'name': 'Mike'
	// 		})
	// 		.set('Content-Type', 'application/json')
	// 		.set('Accept', 'application/json')
	// 		.expect(200)
	// 		.end(function (err, res) {
	// 			if (err) throw err;
	// 			console.log(res.body);
	// 			done();
	// 		});
	// });

	// curl - X POST  http://localhost:8000/login -H 'Content-Type: application/json' -d '{"email":"steady@learner.com", "password":"password"}'
	// test("[POST] /login with incorrect credentials", async done => {
	// 	try {
	// 		const response = await request
	// 			.get("/");

	// 		expect(response.status).toBe(200);
	// 		expect(response.text).toBe("User visit / and verify the console to see sessionID");

	// 		done();
	// 	} catch (e) {
	// 		done(e);
	// 	}
	// });

	// test("[POST] /login with the right credentials", async done => {
	// 	try {
	// 		const response = await request
	// 			.get("/");

	// 		expect(response.status).toBe(200);
	// 		expect(response.text).toBe("User visit / and verify the console to see sessionID");

	// 		done();
	// 	} catch (e) {
	// 		done(e);
	// 	}
	// });
});
