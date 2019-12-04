import React from "react";
import PropTypes from "prop-types";

const Success = ({
	nome,
	cpf,
	// temporary solution for {email.length < 25 ? email : `${email.slice(0, 25)}...}`} to work
	email = "", 
	telefone,
	setModal,
}) => (<>
		<section
			className="width-vw height-vh cursor-pointer absolute"
			title="🖱 clique para encerrar isso."
			onClick={(e) => {
				e.preventDefault();
				setModal(false);
			}}
		>
		</section>
		{/* 2. */}
		<section
			className="center-percent-absolute theme-white max-width border-half content"
		>
			{/* 3. title={email} para mostrar dado interio quando é cortado com `${email.slice(0, 25)}...` */}
			<section className="flex flex-flow-column" >
				<span className="text-center blue bold margin-bottom-one" >Bem Recebido!</span>
				<span>
					<span>
						<span className="disappear-at-mobile--slowly" >Seu nome é</span> <span className="bold" >{nome}</span>
					</span>
				</span>
				<span>
					<span>
						<span className="disappear-at-mobile--slowly" >Seu CPF é</span> <span className="bold">{cpf}</span>
					</span>
				</span>
				<span>
					<span title={email} >
						<i className="fas fa-envelope blue disappear-at-mobile--slowly" /> {email.length < 25 ? email : `${email.slice(0, 25)}...`}
					</span>
				</span>
				<span>
					<span>
						<i className="fas fa-phone-square green disappear-at-mobile--slowly" /> {telefone}
					</span>
				</span>
			</section>
		</section>
	</>);

Success.propTypes = {
	nome: PropTypes.string,
	cpf: PropTypes.string,
	email: PropTypes.string,
	telefone: PropTypes.string,
	setModal: PropTypes.func.isRequired,
};

export default Success;
