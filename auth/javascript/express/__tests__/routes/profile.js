/* eslint-disable jest/no-test-callback */
// Its documentation is not helpful at all.

// Refer to this when you want to test it only.
// $yarn test __tests__/routes/register.js --force-exit
const { app, closeRedis } = require("../../server");

const supertest = require("supertest");
const request = supertest(app);

afterAll(() => closeRedis());

describe("Test /profile", () => {
	test("[GET] to view undefined profile and should redirect to /", async done => {
		try {
			const response = await request
				.get("/profile/undefined");

			expect(response.header.location).toBe("/");

			done();
		} catch (e) {
			done(e);
		}
	});
	test("[GET] to edit undefined profile and should redirect to /", async done => {
		try {
			const response = await request
				.get("/profile/edit/undefined");

			expect(response.header.location).toBe("/");

			done();
		} catch (e) {
			done(e);
		}
	});
});
