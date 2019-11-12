import React, { useState } from "react";
import { useHotkeys } from "react-hotkeys-hook";
import {
	Formik, Form, Field,
	// ErrorMessage,
} from "formik";
import MaskedInput from "react-text-mask";

import SignUpModal from "./SignUpModal";
import SignUpCSS from "./CSS";

import {
	SignUpSchema,
	// YupError, // Use it to debug

	Nome,
	Cpf,
	Email,
	Telefone,
} from "./yup";

import { phoneNumberMask, cpfMask } from "../reusable/mask";

import { saveStateToLocalStorage, loadStateFromLocalStorage } from "../../store/localStorage";

import SignUpImage from "./SignUpImage";

const SignUp = () => {
	const [modalIsOpen, setModal] = useState(false);
	const [data, setData] = useState({});
	const {
		nome,
		email,
		cpf,
		telefone,
	} = data;

	const formButtonClass = "x-outline hover white cursor-pointer x-border-style";

	// Either click shadow or use esc when the main component is in focus to close the modal
	// Follow the example of https://johannesklauss.github.io/react-hotkeys-hook/docs-use-hotkeys#example
	useHotkeys("esc", () => setModal((prevModalIsOpen) => {
		if (prevModalIsOpen) {
			return false;
		}
	}));

	return (
		<SignUpCSS>
			<section className="sign-up__left">
				<SignUpImage />
			</section>
			<section className="sign-up__main">
				<Formik
					initialValues={{
						nome: "",
						email: "",
						cpf: "",
						telefone: "",
					}}
					validationSchema={SignUpSchema}
					onSubmit={(values, { setSubmitting }) => { // destrcuture here
						setTimeout(() => {
							setModal(true);
							setData(values);
							setSubmitting(false);

							const users = loadStateFromLocalStorage("users");

							if (users === false || users === undefined) {
								saveStateToLocalStorage("users", {
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
								<h1 className="sign-up__title" >
									Lean Cadastro
								</h1>
							</section>
							<Nome />
							<Field
								id="nome" name="nome"
								type="text"
								minLength="2"
								maxLength="50"
								placeholder="Ex.: John Doe"
							/>
							{/* <ErrorMessage name="nome" component={YupError} /> */}
							<Email />
							<Field
								id="email" name="email"
								type="text" minLength="5" maxLength="255" placeholder="Ex.: nome@exemplo.com"
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
				{/* Use redirect etc later */}
				<SignUpModal
					modalIsOpen={modalIsOpen}
					nome={nome}
					email={email}
					cpf={cpf}
					telefone={telefone}
					setModal={setModal}
				/>
			</section>
		</SignUpCSS>
	);
};

export default SignUp;
