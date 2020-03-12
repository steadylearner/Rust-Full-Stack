import React, { useState } from 'react';

const YupError = ({ children }) => {
    // Uso CSS além de uns em SteadylearnerCSS.js e coloca isso no fim de string 
    // e deixo espaço vazio para usar concat mais fácil
    const error = "flex red margin-bottom-one-and-a-half margin-left-two-point font-normal hover cursor-pointer errors ";

    const [visible, setVisible] = useState(true);

    return (
        <span
            className={visible ? error : error.concat("x-display")}
            onClick={() => { setVisible(false) }}
            title="Pode remover isso com clique caso incomda você."
        >
            {children}
        </span>
    );
}

export {
    YupError,
}