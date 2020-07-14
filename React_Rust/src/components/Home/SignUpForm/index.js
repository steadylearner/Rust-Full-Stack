import React from "react";
import PropTypes from "prop-types";
import {
	Formik, Form, Field,
} from "formik";
import MaskedInput from "react-text-mask";

import {
	SignUpSchema,

	Nome,
	Cpf,
	Email,
	Telefone,
} from "../yup";

import { phoneNumberMask, cpfMask } from "../../reusable/mask";
import { saveStateToLocalStorage, loadStateFromLocalStorage } from "../../../store/localStorage";

function SignUpForm({
	setModal,
	setData,
}) {
	const formButtonClass = "x-outline hover cursor-pointer x-border-style";

	return (
		<Formik
			initialValues={{
				nome: "",
				email: "",
				cpf: "",
				telefone: "",
			}}
			validationSchema={SignUpSchema}
			onSubmit={(values, { setSubmitting }) => {
				setTimeout(() => {
					setModal(true);
					setData(values);
					setSubmitting(false);

					const users = loadStateFromLocalStorage("users");

					if (users === false || users === undefined) {
						saveStateToLocalStorage("users", {
							// Use uuid with it or database if necessary later to use id
							// Then, use id instead of cpf in UserList
							payload: [values],
						});
					} else {
						const payload = [...users.payload];
						payload.push(values);

						saveStateToLocalStorage("users", {
							payload,
						});
					}
				}, 500);
			}}
		>
			{({
				dirty,
				isSubmitting,
				// handleReset,
				handleChange,
				handleBlur,
				errors,
				touched,
			}) => (
				<Form>
					{/* tem que ser parecido com o schema file em pasta de yup */}
					{/* https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/text */}
					<section className="flex" >
						<h1 className="sign-up__title bold" >
                            ©ode
						</h1>
					</section>
					<Nome />
					<Field
						name="nome"
						id="nome"
						type="text"
						minLength="2"
						maxLength="50"
						placeholder="Ex.: John Doe"
						// Should include this because of react-text-mask?
						className={
							errors.nome && touched.nome
								? "text-input error"
								: "text-input"
						}
					/>
					{/* <ErrorMessage name="nome" component={YupError} /> */}
					<Email />
					<Field
						name="email"
						id="email"
						type="email"
						minLength="5" maxLength="255"
						placeholder="Ex.: nome@exemplo.com"
						className={
							errors.email && touched.email
								? "text-input error"
								: "text-input"
						}
					/>
					{/* <ErrorMessage name="email" component={YupError} /> */}
					<Cpf />
					<Field
						name="cpf"
						render={({ field }) => (
							<MaskedInput
								{...field}
								mask={cpfMask}
								id="cpf"
								placeholder="Ex.: 000.000.000-00"
								size="14"
								type="text"
								onChange={handleChange}
								onBlur={handleBlur}
								className={
									errors.cpf && touched.cpf
										? "text-input error"
										: "text-input"
								}
							/>
						)}
					/>
					{/* <ErrorMessage name="cpf" component={YupError} /> */}
					<Telefone />
					{/* <Field
								id="telefone" name="telefone"
								type="tel" placeholder="(00) 00000-0000"
							/> */}
					<Field
						name="telefone"
						class="sing-up__input--last"
						render={({ field }) => (
							<MaskedInput
								{...field}
								mask={phoneNumberMask}
								id="telefone"
								placeholder="Ex.: (00) 00000-0000"
								type="tel"
								onChange={handleChange}
								onBlur={handleBlur}
								className={
									errors.telefone && touched.telefone
										// eslint-disable-next-line max-len
										? "sing-up__input--last text-input error"
										: "sing-up__input--last text-input"
								}
							/>
						)}
					/>
					{/* <ErrorMessage name="telefone" component={YupError} /> */}
					<section className="center margin-bottom-half" >
						<button
							className={"button--cadastrar ".concat(formButtonClass)}
							type="submit"
							disabled={!dirty || isSubmitting}
							title="Usa isso para cadastar."
						>
                            Cadastrar
						</button>
						<button
							className={"button--login ".concat(formButtonClass)}
							type="button"
							disabled={true}
							title="Usa isso para fazer login."
						>
                            Login ➡
						</button>
					</section>
				</Form>
			)}
		</Formik>
	);
}

SignUpForm.propTypes = {
	setModal: PropTypes.func.isRequired,
	setData: PropTypes.func.isRequired,
};

export default SignUpForm;
