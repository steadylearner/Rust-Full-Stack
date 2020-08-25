/* eslint-disable jest/no-test-callback */
// Its documentation is not helpful at all.

// Refer to this when you want to test it only.
// $yarn test __tests__/routes/register.js --force-exit
const { app, closeRedis } = require("../../server");

const supertest = require("supertest");
const request = supertest(app);

afterAll(() => closeRedis());

// https://chartio.com/resources/tutorials/how-to-get-all-keys-in-redis/
// $redis-cli FLUSHALL to delete all dats in redis
// $del sess:d398cc57-92d3-4560-9efb-36641f9e3319 to remove a data.
describe("Test /logout", () => {
	test("[GET] to remove sessionID from the user and redirect to home /", async done => {
		try {
			const response = await request
				.get("/logout");

			// Use this to inspect response isntead of console.log(response);
			// It is removed from the session and Redis with session.destroy(). Refer to the /login test.
			expect(response.header["set-cookie"]).toBe(undefined);
			expect(response.header.location).toBe("/");

			done();
		} catch (e) {
			done(e);
		}
	});
});
