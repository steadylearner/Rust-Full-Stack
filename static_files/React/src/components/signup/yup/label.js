import React from 'react';

const Nome = () => {
    return (<label
        htmlFor="nome"
    >
        <span>
            Nome
        </span>
        <strong>
            <abbr
                className="red-white"
                title="This is required."
            >
                *
            </abbr>
        </strong>
    </label>);
};

const Idade = () => {
    return (<label
        htmlFor="idade"
    >
        Idade
        <strong>
            <abbr
                className="red-white"
                title="
                    This is required.
                "
            >
                *
            </abbr>
        </strong>
    </label>);
};

const Email = () => {
    return (<label
        htmlFor="email"
    >
        <i className="fas fa-envelope blue hover" />
        {" "} Email
        <strong>
            <abbr
                    className="red-white"
                title="
                    This is required.
                "
            >
                *
            </abbr>
        </strong>
    </label>);
};

const Telefone = () => {
    return (<label
        htmlFor="telefone"
    >
        <i className="fas fa-phone-square green hover" />
        {" "} Telefone
        <strong>
            <abbr
                className="red-white"
                title="
                    This is required.
                "
            >
                *
            </abbr>
        </strong>
    </label>);
}

export {
    Nome,
    Idade,
    Email,
    Telefone,
}