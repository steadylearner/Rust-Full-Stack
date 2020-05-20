/* eslint-disable */
import React from "react";
import PropTypes from "prop-types";
import { Formik, Field } from "formik";
// https://testing-library.com/docs/react-testing-library/intro
// Not compatible with Enzyme
import { render, fireEvent } from "@testing-library/react";

const InputField = props => (
	<Field
		name={props.fieldName}
		render={({ field, form }) => (
			<div>
				<label htmlFor={props.fieldName}>{props.labelName}</label>
				<input {...field} id={props.fieldName} type="text" />
				{form.errors[props.fieldName] && form.touched[props.fieldName] ? (
					<div data-testid={`errors-${props.fieldName}`}>
						{form.errors[props.fieldName]}
					</div>
				) : null}
			</div>
		)}
	/>
);

InputField.propTypes = {
	fieldName: PropTypes.string.isRequired,
	labelName: PropTypes.string.isRequired,
};

test("should have validation error given input field is touched and error exists on form", async () => {
	const fieldName = "firstName";
	const labelName = "First Name";
	const { getByLabelText, findByTestId } = render(
		<Formik
			validate={(values) => {
				const errors = {};

				if (!values.firstName) {
					errors.firstName = "Required.";
				}

				return errors;
			}}
		>
			<InputField fieldName={fieldName} labelName={labelName} />
		</Formik>,
	);

	const input = getByLabelText(labelName);

	// Call blur without inputting anything which should trigger a validation error
	fireEvent.blur(input);

	const validationErrors = await findByTestId(`errors-${fieldName}`);

	expect(validationErrors.innerHTML).toBe("Required.");
});
