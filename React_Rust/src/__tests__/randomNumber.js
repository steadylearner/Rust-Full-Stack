/* eslint-disable */
import { randomDigit } from "../components/reusable/randomNumber";

describe('Test randomDigit function from /reusable that use bitwise operation for faster result.', () => {
    test("should be in range between [0, 5] inclusive", () => {

        const value = randomDigit();
        expect(value).toBeGreaterThanOrEqual(0);
        expect(value).toBeLessThanOrEqual(5);
    });
});

