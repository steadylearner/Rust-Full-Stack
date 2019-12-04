/* eslint-disable */
import { schema } from "../components/Home/yup/schema";

describe('Test yupSchema for this project with its built in test isValid feature.', () => {
    test("should work with custom prop. It doesn't need complicated test.", async (done) => {
        const {
            nome,
            email,
            cpf,
            telefone,
        } = schema;

        const validNome = await nome.isValid("Johne Doe");
        const invalidNome = await nome.isValid("111111111111111");
        expect(validNome)
            .toBe(true);
        expect(invalidNome)
            .toBe(false);

        const validEmail = await email.isValid("valid@email.com");
        const invalidEmail = await email.isValid("invalidemail");
        expect(validEmail)
            .toBe(true);
        expect(invalidEmail)
            .toBe(false);

        const validCpf = await cpf.isValid("111.111.111-11");
        const invalidCpf = await cpf.isValid("111111111111111");
        expect(validCpf)
            .toBe(true);
        expect(invalidCpf)
            .toBe(false);

        const validTelefone = await telefone.isValid("(11) 98888-1111");
        // const validTelefone = await telefone.isValid("11 98888-1111"); // without ()
         const invalidTelefone = await telefone.isValid("111111111111111");

        expect(validTelefone)
            .toBe(true);
        expect(invalidTelefone)
            .toBe(false);

        done();
    });
});

