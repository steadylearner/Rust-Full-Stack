import React from 'react'
import Modal from "./CSS";

import Success from "./success";
// import Error from "./error";

const SignUpModal = ({
    modalIsOpen,
    nome,
    email = "", // temporary solution for {email.length < 25 ? email : `${email.slice(0, 25)}...}`} to work
    idade,
    telefone,
    setModal,
}) => {
    return (<Modal style={{ display: `${modalIsOpen ? "block" : "none"}`, }} >
        <Success
            nome={nome}
            email={email}
            idade={idade}
            telefone={telefone}
            setModal={setModal}
        />
    </Modal>);
}

export default SignUpModal;

