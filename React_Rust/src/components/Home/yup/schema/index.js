import * as Yup from "yup"; // Yup(https://github.com/jquense/yup)
import { nameRegExp, cpfRegExp, phoneRegExp } from "../../../reusable/regex";

const schema = {
	nome: Yup.string()
		.matches(nameRegExp, "Nome Inválido")
		.min(2, "Nome deve ser mais que uma letra.")
		.max(50, "Usa nome mais curto.") // Mensagem de erro e limite temporário
		.required("Este campo é OBRIGATÓRIO"),
	email: Yup.string()
		.min(5, "Email muito curto.") // Very short email!
		.max(255, "Email deve ter menos 255 letras.")
		.email("Email Inválido") // errors.email, Invalid Email
		.required("Este campo é OBRIGATÓRIO"), // errors.email, This field is required
	cpf: Yup.string()
		.matches(cpfRegExp, "CPF Inválido. Tem que ser xxx.xxx.xxx-xx.")
		.required("Este campo é OBRIGATÓRIO"),
	telefone: Yup.string()
		.matches(phoneRegExp, "Numero de telefone não é valido. Tem que ser (xx) xxxxx-xxxx.")
		.required("Este campo é OBRIGATÓRIO."),
};

const SignUpSchema = Yup.object().shape(schema);

export {
	SignUpSchema,
	schema,
};
