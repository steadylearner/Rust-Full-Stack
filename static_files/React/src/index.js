import React from "react";
import ReactDOM from "react-dom";

import SignUp from "./components/signup";
import SteadylearenerCSS from "./CSS/SteadylearnerCSS";

import "./normalize.css";

function App() {
  return (
    <section className="App">
      {/* Tem que usar outra estratura se incluir react-router-dom e redux etc */}
      <SteadylearenerCSS>
        <SignUp />
      </SteadylearenerCSS>
    </section>
  );
}

const rootElement = document.getElementById("root");
ReactDOM.render(<App />, rootElement);

// - Desenvolva um projeto em React que contenha apenas uma página com um formulário para coleta de leads.
//   A página deve ser responsiva e utilizar as libs Formik e Yup para exibição e
//   validações pertinentes a cada campo. No submit do form, a página deve exibir uma modal de sucesso.
//   Caso exista algum erro de validação no formulário, a modal deve exibir a mensagem específica do campo com erro. 
//   O formulário deve ter os seguintes campos:

//     - Nome;
//     - Email;
//     - Idade;
//     - Telefone;

// achei que é melhor ajudar usuario mandar informação sem erro antes de usar submit botão do que deixar eles mandam informçaõ com erro e mostrar modal

// e isso é mais fácil também e seguindo caminho de escritor de formik e yup