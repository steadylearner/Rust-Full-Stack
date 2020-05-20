import styled, { keyframes } from "styled-components";

const modalAnimation = keyframes`
    0%{
        transform: scale(0);
        opacity: 0;
    }
    1% {
        display: flex;
        opacity: 0;
    }
    100% {
        transform: scale(1);
        opacity: 1;
    }
`;

const Modal = styled.section`

    display: none;
    position: fixed;
    z-index: 1;
    left: 0;
    top: 0;
    width: 100vw;
    height: 100vh;

    background-color: rgba(0, 0, 0, 0.7);
    animation: 0.6s ${modalAnimation};

    /* Contents in it  */

    .content {
        padding: 2rem;
    }

    @media all and (max-width: 38.4rem) {
		.content {
            font-size: 1.2rem;
        }
	}

    @media all and (max-height: 38.4rem) {
		.content {
            font-size: 1.2rem;
        }
	}

`;

export default Modal;
