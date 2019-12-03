import React, { useState } from 'react';
import { Formik, Form, Field, ErrorMessage } from 'formik';

import SignUpModal from "../modal";
import SignUpCSS from "./CSS";

import {
    SignUpSchema,
    YupError,

    Nome,
    Email,
    Idade,
    Telefone,
} from "./yup";

import Nav from "../nav";
import Subnav from "../subnav";

import { YouTube, Twitter } from "../reuse"

const SignUp = () => {
    const [modalIsOpen, setModal] = useState(false);
    const [data, setData] = useState({})

    const { nome, email, idade, telefone } = data;

    const buttonclass = "form-button x-outline hover white cursor-pointer x-border-style ";

    return (
        <>
            <Nav />
                <SignUpCSS>
                    <section className="flex" >
                        <h1 className="sign-up" >
                            <span>©</span>
                            {" "} Sign Up
                        </h1>
                        <section className="left-auto flex center font-two-and-a-half social" >
                            <YouTube />
                            <Twitter />
                        </section>
                    </section>

                    {/* 1. https://jaredpalmer.com/formik/ */}
                    <Formik
                        initialValues={{
                            nome: '',
                            email: '',
                            idade: '',
                            telefone: '',
                        }}
                        validationSchema={SignUpSchema}
                        onSubmit={(values, { setSubmitting }) => { // destrcuture here
                            setTimeout(() => {
                                setModal(true);
                                setData(values)
                                setSubmitting(false);
                            }, 500);
                        }}
                    >
                        {({
                            dirty,
                            handleReset,
                            isSubmitting,
                            errors,
                        }) => (
                            // 2. https://jaredpalmer.com/formik/docs/api/errormessage
                            // https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input
                            <Form>
                                {/* tem que ser parecido com o schema file em pasta de yup*/}
                                {/* https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/text */}
                                <Nome />
                                <Field
                                    id="nome" name="nome"
                                    type="text" minLength="2" maxLength="50" placeholder="Steady Learner"
                                />
                                <ErrorMessage name="nome" component={YupError} />
                                {/* https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/number */}
                                <Idade />
                                <Field
                                    id="idade" name="idade"
                                    type="number" min="16" max="80" placeholder="16" step="1"
                                />
                                <ErrorMessage name="idade" component={YupError} />
                                {/* https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/email */}
                                <Email />
                                {/* Corto isso em view para Modal quando é muito longo. */}
                                <Field
                                    id="email" name="email"
                                    type="email" minLength="5" maxLength="255" placeholder="steady@learner.com"
                                />
                                <ErrorMessage name="email" component={YupError} />
                                {/* https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/tel */}
                                <Telefone />
                                <Field
                                    id="telefone" name="telefone"
                                    type="tel" minLength="10" maxLength="25" placeholder="11 11111-1111"
                                />
                                <ErrorMessage name="telefone" component={YupError} />
                                <section className="center margin-bottom-half" >
                                    <button
                                        className={buttonclass.concat("theme-red")}
                                        type="button"
                                        onClick={handleReset}
                                        disabled={!dirty || isSubmitting}
                                        title="Para recompor essa forma."
                                    >
                                        <i className="fas fa-eraser" />
                                        {" "} Reset
                                    </button>
                                    <button
                                        className={buttonclass.concat("theme-blue")}
                                        type="submit"
                                        disabled={isSubmitting}
                                        title="Para submeter essa forma."
                                    >
                                        <i className="fas fa-user-edit" />
                                        {" "} Submit
                                    </button>
                                </section>
                            </Form>
                        )}
                    </Formik>
                    {/* 3. Include it to use errors otherwise can be anywhere in this file*/}
                    <SignUpModal
                        modalIsOpen={modalIsOpen}
                        nome={nome}
                        email={email}
                        idade={idade}
                        telefone={telefone}
                        setModal={setModal}
                    />
                </SignUpCSS>
            <Subnav />
        </>
    )
}

export default SignUp;