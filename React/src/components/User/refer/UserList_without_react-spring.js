import React, { useState } from "react";
import PropTypes from "prop-types";

import { randomNumber } from "../../reusable/randomNumber";
import { saveStateToLocalStorage, loadStateFromLocalStorage } from "../../../store/localStorage";

const UserList = ({
	nome,
	email,
	cpf,
	telefone,
}) => {
	// Simple approach instead of filter data and localStorage
	// If you insist, I will make a React hook in ./index.js and pass it as a prop here
	// Then, user filter data of list of users in ./index.js
	const [display, setDisplay] = useState(true);

	const slGitHub = "https://avatars0.githubusercontent.com/u/32325099?s=460&v=4";
	const randomImage = [
		"leanwork.jpg",
		"versao-web.jpg",
		"docker.png",
		"github.png",
		"scrapy.png",
		"jest.png",
	];
	const imageIndex = randomNumber();

	return (
		// ${display && "animated fadeOut fastest"}
		// ${!display && "x-display"}
		<li 
			className={`
				user-list
				${!display && "x-display"}
			`}
		>
			<section className="user-list__main width-percent" >
				<h1 className="user-list__name animated heartBeat" >{nome}</h1>
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
							className="user-image border-round hover cursor-pointer"
							src={slGitHub}
							alt={slGitHub}
						/>
					</a>
					<section className="column-center-start right-auto">
						<span className="font-normal" >
							🇧🇷
							{" "}
							{cpf}
						</span>
						<span
							className="font-normal"
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
									const payload = users.payload.filter(user => user.cpf !== cpf);

									const saved = saveStateToLocalStorage("users", {
										payload,
									});

									if (saved) {
										// setDisplay(false);
									}
								}
							}}
						>
							<i className="fas fa-times-circle margin-right-half" />
						</span>
					</section>
				</section>
			</section>
			<section className="user-list__image margin-left-one x-overflow center" >
				<section
					className="
						width-percent height-percent
						thumbnail scale-five-point--hover
						background-percent border max-width
					"
					style={{ backgroundImage: `url(/src/images/${randomImage[imageIndex]})` }}
					title="Rust JSON Webservice Example">
				</section>
			</section>
		</li>
	);
};

UserList.propTypes = {
	nome: PropTypes.string.isRequired,
	email: PropTypes.string.isRequired,
	cpf: PropTypes.string.isRequired,
	telefone: PropTypes.string.isRequired,
};

export default UserList;
