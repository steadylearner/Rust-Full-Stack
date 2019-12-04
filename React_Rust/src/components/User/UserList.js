import React, { useState } from "react";
import PropTypes from "prop-types";

// https://www.react-spring.io/docs/hooks/use-transition
import { useTransition, animated } from "react-spring";

import { randomDigit } from "../reusable/randomNumber";
import { saveStateToLocalStorage, loadStateFromLocalStorage } from "../../store/localStorage";

function UserList({
	nome,
	email,
	cpf,
	telefone,
}) {
	const [show, setShow] = useState(true);
	const transitions = useTransition(show, null, {
		from: { opacity: 0 },
		enter: { opacity: 1 },
		leave: { opacity: 0 },
	});

	const slGitHub = "https://avatars0.githubusercontent.com/u/32325099?s=460&v=4";
	// This is to use less images only wiwhout any dependencies or real datas.
	// I tested it with Cypress and could see the new image when click remove button.
	// But, there will be any problem in production for users will use real datas.
	const randomImage = [
		"animal-example.jpg",
		"code_b.png",
		"docker.png",
		"rust-chat-app.png",
		"flamengo_animation_by_Steadylearner.gif",
		"grêmio_heart_animation_made_by_steadylearner.gif",
	];
	const imageIndex = randomDigit();

	return (transitions.map(({ item, key, props }) => item && <animated.li
		className="user-list"
		key={key} style={props}
	>
		<section className="user-list__main width-percent" >
			<h1 className="user-list__name" >{nome}</h1>
			{/* <h1 className="user-list__name animated heartBeat" >{nome}</h1> */}
			<h1 className="user-list__email" >{email}</h1>
			<section className="user-list__util">
				<a
					className="margin-right-half center"
					href="https://www.steadylearner.com"
					target="_blank"
					rel="noopener noreferrer"
					title="Link to the website of the author www.steadylearner.com"
				>
					<img
						className="
							user-image
							border-round
							hover cursor-pointer
							disappear-at-mobile--slowly
						"
						src={slGitHub}
						alt={slGitHub}
					/>
				</a>
				<section className="user-list__cpf column-center-start right-auto">
					<span className="font-normal" >
						🇧🇷
						{" "}
						{cpf}
					</span>
					<span
						className="user-list__telefone font-normal"
					>
						<i className="fab fa-whatsapp green animated pulse" />
						{" "}
						<span className="more-opacity" >{telefone}</span>
					</span>
				</section>
				<section className="center" >
					<span
						className="font-two hover hover-red cursor-pointer animated tada"
						onClick={() => {
							const users = loadStateFromLocalStorage("users");

							if (users !== undefined) {
								// What you use here
								// should be not duplicate in the production database
								const payload = users.payload
									.filter(user => user.cpf !== cpf);

								const saved = saveStateToLocalStorage("users", {
									payload,
								});

								if (saved) {
									setShow(false);
								}
							}
						}}
					>
						<i
							className={`
								user-list__remove
								fas fa-times-circle
								margin-right-half
							`}
						/>
					</span>
				</section>
			</section>
		</section>
		<section className="user-list__image margin-left-one x-overflow center" >
			<section
				className={`
					width-percent height-percent
					thumbnail scale-five-point--hover
					background-percent border max-width
				`}
				style={{ backgroundImage: `url(/src/images/${randomImage[imageIndex]})` }}
				title={randomImage[imageIndex]} >
			</section>
		</section>
	</animated.li>)
	);
}

UserList.propTypes = {
	nome: PropTypes.string.isRequired,
	email: PropTypes.string.isRequired,
	cpf: PropTypes.string.isRequired,
	telefone: PropTypes.string.isRequired,
};

export default UserList;
