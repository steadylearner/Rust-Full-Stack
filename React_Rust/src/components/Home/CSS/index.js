// eslint-disable-next-line no-unused-vars
import React from "react";
import styled from "styled-components";
// styled-components doesn't work well with Fragment

const SignUpCSS = styled.section.attrs({
	className: "font-two max-width flex animated fadeInLeftBig",
})`
	padding-top: 5.6rem;
	height: calc(100vh - 5.6rem);
	background: white;
	margin: 0 auto;
	overflow-y: hidden;

	/* Measuring the result file, assume that flex will be 9:3 */
	/* Left will be disply none in responsive mode */
	.sign-up__left {
		flex: 9;
		max-height: 100%;
		/* https://developer.mozilla.org/en-US/docs/Web/CSS/linear-gradient */
		background: linear-gradient(#8f2f61, #fff);
		/* background: linear-gradient(#40c8f4, #2179b5); */
	}

	/* Should give an opacity to the image and background color for the wrapper element */
	.sign-up__image {
		background-image: url(/src/images/animal-example.jpg);
		opacity: 0.5;
	}

	.sign-up__main {
		flex: 3;
		max-height: 100%;
		padding: 2rem 5rem;
	}

	.sign-up__title {
		font-size: 3rem;
		margin-bottom: 3rem;
		color: #121212;
		font-weight: normal;
	}

	input {
		padding: 0.5rem;
		display: block;
		border-radius: 0.1rem;
		margin-bottom: 2rem;
		width: 100%;
		color: #555555;
		/* Remove default bolder CSS */
		border: none;
		border-bottom: 0.2rem solid #efefef;
	}

	/* https://stackoverflow.com/questions/2943548/how-to-reset-remove-chromes-input-highlighting-focus-border */
	/* 
		outline-style: none;
		box-shadow: none; 
	*/
	input:focus {
		border-color: #555555;
		outline: none;
		outline-style: none;
		box-shadow: none;
	}

	input.error {
		border-color: red;
		outline-style: none;
		box-shadow: none;
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

		color: #999999;
	}

	.button--cadastrar {
		margin: 1rem 0.5rem 2rem 0.5rem;
		padding: 1rem 5rem;
		border-radius: 2rem;
		box-shadow: 0 0.2rem 0.2rem rgba(0, 0, 0, 0.15);
		font-size: 1.7rem;
		font-weight: 500;
		-webkit-appearance: none;

		color: #ffffff;
		background: #40c8f4;

		:hover {
			opacity: 0.7;
		}

		:disabled {
			color: #dddcdc;
			background: #f6f6f6;
			cursor: not-allowed !important;
		}
	}

	.button--login {
		margin: 1rem 0.5rem 2rem 0.5rem;
		padding: 1rem 1rem;
		border-radius: 2rem;
		font-size: 1.7rem;
		font-weight: 500;
		-webkit-appearance: none;

		color: #999999;
		background: white;

		:hover {
			opacity: 0.7;
		}

		/* :disabled {
			color: #dddcdc;
			background: #f6f6f6;
			cursor: not-allowed !important;
		} */
	}

	@media all and (max-width: 76.8rem) {
		.sign-up__title {
			font-size: 2.5rem;
		}

		label {
			font-size: 1.6rem;
		}

		input {
			font-size: 1.2rem;
		}

		.button--cadastrar {
			font-size: 1.6rem;
			padding: 1rem 2rem;
		}

		.button--login {
			font-size: 1.6rem;
			padding: 0;
		}

		/* .sing-up__input--last {
			margin-bottom: 0.5rem;
		} */

	}

        /* Similar CSS with parameter 'display' of the image' */

	@media all and (max-width: 48rem) {
		.sign-up__left {
			display: none;
		}

		.sign-up__main {
			padding: 0.5rem 5rem;
		}

		.sign-up__title {
			font-size: 4rem;
			text-align: center;
			width: 100%;
                        margin-top: 2.5rem;
		}

                label {
			font-size: 2rem;
		}

		input {
			font-size: 1.6rem;
		}

		.button--cadastrar {
			padding: 1rem 2rem;
			font-size: 2rem;
		}

		.button--login {
			padding: 1rem 0rem;
			font-size: 2rem;
		}

                .sing-up__input--last {
			margin-bottom: 2rem;
		}
	}

	@media all and (max-width: 38.4rem) {

		.sign-up__title {
			font-size: 3rem;
		}

		.label {
			font-size: 2rem;
		}

		input {
			font-size: 1.6rem;
		}

		.button--cadastrar {
			font-size: 2rem;
			padding: 1rem 2em;
		}

		.button--login {
			font-size: 2rem;
			padding: 1rem 0;
		}

                .sing-up__input--last {
			margin-bottom: 1rem;
		}
	}

    @media all and (max-width: 34.8rem) {
		.sign-up__title {
			font-size: 2.5rem;
		}

		label {
			font-size: 1.6rem;
		}

		input {
			font-size: 1.2rem;
		}

		.button--cadastrar {
			font-size: 1.6rem;
			padding: 1rem 2rem;
		}

		.button--login {
			font-size: 1.6rem;
			padding: 0;
		}

		/* .sing-up__input--last {
			margin-bottom: 0.5rem;
		} */
	}

        /* Similar CSS with parameter Height */

	@media all and (max-height: 38.4rem) {
		height: 100%;
                
                /* 
                   Help user to scroll in small devices.
                   Comment these from responsive CSS because it doesn't work well with SignUp Modal. 
                */
		/* padding-bottom: 5rem; /*
		
		.sign-up__title {
			font-size: 2.5rem;
		}

		label {
			font-size: 1.6rem;
		}

		input {
			font-size: 1.2rem;
		}

		.button--cadastrar {
			font-size: 1.6rem;
			padding: 1rem 1rem;
		}

		.button--login {
			font-size: 1.6rem;
			padding: 0;
		}
	}

	@media all and (max-height: 34.8rem) {


		/* .sing-up__input--last {
			margin-bottom: 0.5rem;
		} */

                /* 
                   Help user to scroll in small devices.
                   Comment these from responsive CSS because it doesn't work well with SignUp Modal. 
                */
		/* padding-bottom: 10rem; /*

		.sign-up__title {
			font-size: 2rem;
		}

		label {
			font-size: 1.2rem;
		}

		input {
			font-size: 1rem;
		}

		.button--cadastrar {
			font-size: 1.2rem;
			padding: 1rem 1rem;
		}

		.button--login {
			font-size: 1.2rem;
			padding: 0;
		}

		.sing-up__input--last {
			margin-bottom: 0.5rem;
		}
	}

	@media all and (max-height: 28.4rem) {
                /* 
                   Help user to scroll in small devices.
                   Comment these from responsive CSS because it doesn't work well with SignUp Modal. 
                */
		/* padding-bottom: 15rem; /*
	}
`;

export default SignUpCSS;
