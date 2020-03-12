/* eslint-disable */
import React from "react";
// Read more documenations here if necessary 
// https://testing-library.com/docs/guide-events
import { render, fireEvent } from "@testing-library/react";

import SignUp from "../components/Home";

describe('Test SignUp wtih Jest and React Testing Library.', () => {
	test("Should have validation error given input field is touched and error exists on form", async (done) => {
		const { getByLabelText, findByLabelText } = render(
			<SignUp />
		);

		const input = getByLabelText("Nome Completo");
		expect(input.className).toBe("text-input");

		// Call blur without inputting anything which should trigger a validation error
		fireEvent.blur(input);

		const inputWithErrors = await findByLabelText("Nome Completo");

		console.log(inputWithErrors.className);
		expect(inputWithErrors.className).toBe("text-input error");

		done();
	});

	test("Should pass with valid name.", async (done) => {
		const { getByLabelText, findByLabelText } = render(
			<SignUp />
		);

		const input = getByLabelText("Nome Completo");
		expect(input.className).toBe("text-input");

		fireEvent.change(input, { target: { value: "John Doe" } });
		expect(input.value).toBe("John Doe");

		const withValidName = await findByLabelText("Nome Completo");

		expect(withValidName.className).toBe("text-input");

		done()
	});
});

