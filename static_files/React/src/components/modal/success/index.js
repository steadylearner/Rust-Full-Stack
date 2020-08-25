import React from 'react'

const Success = ({
    modalIsOpen,
    nome,
    email = "", // temporary solution for {email.length < 25 ? email : `${email.slice(0, 25)}...}`} to work
    idade,
    telefone,
    setModal,
}) => {
    return (
        <>
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
                    <span class="text-center blue bold margin-bottom-one" >Bem Recebido!</span>
                    <span>
                        <span className="disappear-at-mobile" >Seu nome é </span>
                        <span className="bold" >{nome}</span>
                    </span>
                    <span>
                        <span>Você tem </span>
                        <span className="bold">{idade}</span> {" "} anos
                    </span>
                    <span>
                        <i className="fas fa-envelope blue disappear-at-mobile--slowly" />
                        <span title={email} > {" "} {email.length < 25 ? email : `${email.slice(0, 25)}...`}</span>
                    </span>
                    <span>
                        <i className="fas fa-phone-square green disappear-at-mobile--slowly" />
                        <span> {" "} {telefone}</span>
                    </span>
                </section>
            </section>
        </>
    );
}

export default Success;

