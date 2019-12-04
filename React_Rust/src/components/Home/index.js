import React, { useState } from "react";
import { useHotkeys } from "react-hotkeys-hook";

import SignUpModal from "./SignUpModal";
import SignUpCSS from "./CSS";

import SignUpImage from "./SignUpImage";
import SignUpForm from "./SignUpForm";

export default function SignUp() {
	const [modalIsOpen, setModal] = useState(false);
	const [data, setData] = useState({});
	const {
		nome,
		email,
		cpf,
		telefone,
	} = data;

	// Either click shadow or use esc when the main component is in focus to close the modal
	// Follow the example of https://johannesklauss.github.io/react-hotkeys-hook/docs-use-hotkeys#example
	// eslint-disable-next-line consistent-return
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
				<SignUpForm
					setModal={setModal}
					setData={setData}
				/>
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
}