import React from "react";
import styled from "styled-components";
// styled-components doesn't work well with Fragment

const SignUpCSS = styled.section.attrs({
    className: "font-two max-width"
})`
	padding: 5.6rem 2rem 0 1rem;
	background: white;
	width: 64rem;
	margin: 0 auto;

	input {
		padding: 0.5rem;
		display: block;
		border-radius: 4px;
		border: 1px solid #ccc;
		margin-bottom: 2rem;
		width: 100%;
	}

	input:focus {
		border-color: #08c;
		box-shadow: inset 0 1px 1px rgba(0, 0, 0, 0.075), 0 0 0 2px rgba(0, 126, 255, 0.1);
		outline: none;
	}

	input.error {
		border-color: red;
	}

	.input-feedback {
		color: red;
		margin-top: 0.25rem;
	}

	label {
		font-weight: bold;
		display: block;
		text-align: start;
		margin-bottom: 0.5rem;
		font-size: 1.6rem;
	}

	.form-button {
		max-width: 15rem;
		margin: 2rem 0 2rem 0.5rem;
		padding: 1.2rem 1.8rem;
		border-radius: 0.5rem;
		box-shadow: 0 0.2rem 0.2rem rgba(0, 0, 0, 0.15);
		font-size: 1.7rem;
		font-weight: 500;
		-webkit-appearance: none;
	}

	.form-button:disabled {
		opacity: 0.5;
		cursor: not-allowed !important;
	}

	.form-button.outline {
		background-color: #eee;
		border: 1px solid #aaa;
		color: #555;
	}

	@media all and (max-width: 66rem) {
		width: 48rem;
	}

	@media all and (max-width: 48rem) {
		width: 25.6rem;
		font-size: 2rem;

		.sign-up {
			text-align: center;
			width: 100%;
		}

		/* 
			Uso !important aqui porque isso tem menons importância para CSS renderer que SteadylearnerCSS.js 
			Não terá problema porque não vai usar .social depois de ele fica disaparecido.
		*/
	
		.social {
			display: none !important; 
		}

		.errors {
			font-size: 1.2rem !important;
		}
	}

	@media all and (max-height: 38.4rem) {
		.sign-up {
			text-align: center;
			width: 100%;
		}

		.social {
			display: none !important;
		}
	}
`;

export default props => <SignUpCSS {...props} />;
