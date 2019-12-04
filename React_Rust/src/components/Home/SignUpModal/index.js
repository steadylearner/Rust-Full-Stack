import React from "react";
import PropTypes from "prop-types";

import Modal from "./CSS";

import Success from "./success";
// import Error from "./error";

const SignUpModal = ({
	modalIsOpen,
	nome,
	// temporary solution for {email.length < 25 ? email : `${email.slice(0, 25)}...}`} to work
	email = "",
	cpf,
	telefone,
	setModal,
}) => (<Modal style={{ display: `${modalIsOpen ? "block" : "none"}` }} >
	<Success
		nome={nome}
		email={email}
		cpf={cpf}
		telefone={telefone}
		setModal={setModal}
	/>
</Modal>);

SignUpModal.propTypes = {
	modalIsOpen: PropTypes.bool.isRequired,
	nome: PropTypes.string,
	email: PropTypes.string,
	cpf: PropTypes.string,
	telefone: PropTypes.string,
	setModal: PropTypes.func.isRequired,
};

export default SignUpModal;
