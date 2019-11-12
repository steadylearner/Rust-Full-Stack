import React from "react";

// const Warning = () => (<strong><abbr
//     className="red-white"
//     title="
//         This is required.
//     "
// >
//     *
// </abbr></strong>);

const Nome = () => (<label
	htmlFor="nome"
>
	<span>
            Nome Completo
	</span>
	{/* <Warning /> */}
</label>);

const Email = () => (<label
	htmlFor="email"
>
	E-mail
	{/* <Warning /> */}
</label>);

const Cpf = () => (<label
	htmlFor="cpf"
>
        CPF
	{/* <Warning /> */}
</label>);

const Telefone = () => (<label
	htmlFor="telefone"
>
        Telefone
	{/* <Warning /> */}
</label>);

export {
	Nome,
	Email,
	Cpf,
	Telefone,
};
