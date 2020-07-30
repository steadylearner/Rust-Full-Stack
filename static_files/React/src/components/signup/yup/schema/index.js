import * as Yup from "yup"; // Yup(https://github.com/jquense/yup)
import { nameRegExp, phoneRegExp } from "./regex"

const schema = {
  // should write better message and more code
  nome: Yup.string()
    .matches(nameRegExp, "Nome Inválido")
    .min(2, "Nome deve ser mais que uma letra.")
    .max(50, "Pode usar nome mais curto.")
    .required("Este campo é OBRIGATÓRIO"), // Required or OBRIGATÓRIO
  email: Yup.string()
    .min(5, "Email muito curto.") // Very short email!
    .max(255, "Email deve ter menos 255 letras.")
    .email("Email Inválido") // errors.email, Invalid Email
    .required("Este campo é OBRIGATÓRIO"), // errors.email, This field is required
  idade: Yup.number()
    .min(16, "Você ainda não é adulto.") // You are not adult yet. 
    // .max(80) // message de erro para isso com HTML padrão porque Yup usa inglês para esse.
    .positive("Número tem que mais que um.")
    .integer("Número Inválido para ser idade")
    .required("Este campo é OBRIGATÓRIO"),
  telefone: Yup.string()
    .matches(phoneRegExp, "Numero de telefone não é valido.") // Phone number is not valid
    .required("Este campo é OBRIGATÓRIO.")
};

const SignUpSchema = Yup.object().shape(schema);

export {
  SignUpSchema,
};

// use this here or separate it in another file if necessary
// async function schemaTest() {
//     const {
//       nome,
//       email,
//       idade,
//       telefone,
//     } = schema;

//     // valid<variable> shows true, and vice versa with invalid

//     // const validNome = await nome.isValid("John Doe");
//     // const invalidEmail = await nome.isValid("11111111111111111111111111111111");

//     // console.log(validNome, invalidEmail);

//     // const vaildEmail = await email.isValid("valid@email.com");
//     // const invalidEmail = await email.isValid("itisnotwithemailsyntax");

//     // console.log(vaildEmail, invalidEmail);

//     // const validIdade = await idade.isValid(50);
//     // const invalidIdade = await idade.isValid(10000);

//     // console.log(validIdade, invalidIdade);

//     const validTelefone = await telefone.isValid("11 11111-1111"); // => true
//     const invalidTelefone = await telefone.isValid("Não tenho celular para usar"); // => false

//     console.log(validTelefone, invalidTelefone);
// }

// schemaTest()