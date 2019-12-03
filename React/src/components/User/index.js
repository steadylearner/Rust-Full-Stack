import React from "react";
import { key } from "prop-passer";

import UserCSS from "./CSS";
import {
	loadStateFromLocalStorage,
} from "../../store/localStorage";

import UserList from "./UserList";

export default function User() {
	const userList = loadStateFromLocalStorage("users");
	// console.log(userList);

	if (userList !== undefined && userList.payload && userList.payload.length > 0) {
		const { payload } = userList;

		// const response = [1, 2, 3, 4, 5].map(x => <h1 key={key()} >{x}</h1>);
		const response = payload.map(({
			nome,
			email,
			cpf,
			telefone,
		}) => (
			<UserList
				nome={nome}
				email={email}
				cpf={cpf}
				telefone={telefone}
				key={key()}
			/>
		));

		return (
			// Use <section className="theme-black-white" >
			// for only to make layout when you are in development
			// <section className="theme-black-white" >
			<UserCSS>
				<ul className={`
					x-ul
					flex
					flex-flow-column
					center-auto-margin
					width-percent
					animated fadeInDownBig
				`} >
					{response}
				</ul>
			</UserCSS>
			// </section>
		);
	}

	return null;
}
