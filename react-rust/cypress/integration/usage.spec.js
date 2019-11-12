/* eslint-disable no-undef */
describe("Test SignUp page at /", () => {
	it("Type all the correct datas at / and verify user and remove it at /user.", () => {
		cy.clearLocalStorage();

		const validName = "John Doe";
		const validEmail = "nome@exemplo.com";
		const validCpf = "111.111.111-11";
		const validTelefone = "88 98888-8888";

		// const maskTelefone = (telefone) => {
		// 	const payload = `(${telefone.split(" ")[0]}) ${telefone.split(" ")[1]}`;
		// 	return payload;
		// };

		const expectedUser = {
			nome: "John Doe",
			email: "nome@exemplo.com",
			cpf: "111.111.111-11",
			telefone: "(88) 98888-8888",
		};

		cy.visit("/");
		// mock submit with this if necessary .type("{enter}");;
		cy.get("input[id*=\"nome\"]").click().type(validName).should("have.value", validName);
		cy.get("input[id*=\"email\"]").click().type(validEmail).should("have.value", validEmail);
		cy.get("input[id*=\"cpf\"]").click().type(validCpf).should("have.value", validCpf);
		cy.get("input[id*=\"telefone\"]").click().type(validTelefone).should("have.value", "(88) 98888-8888");
		// https://docs.cypress.io/api/commands/screenshot.html
		cy.screenshot();
		cy.get(".button--cadastrar").click().wait(2000);
		cy.screenshot();
		cy.get(".sign-up__main").trigger("keydown", { keyCode: 27, which: 27 });

		cy.get(".sign-up__main").should(() => {
			const users = JSON.parse(localStorage.getItem("users"));
			const { payload } = users;
			const user = payload[0];

			// Should use deep.equal for object comparision.
			// https://docs.cypress.io/guides/references/assertions.html
			expect(user).to.deep.equal(expectedUser);
			expect(payload.length).to.eq(1);
		});
		cy.visit("/user");
		cy.get(".user-list").click().wait(2000);
        cy.screenshot();
		cy.get(".user-list__remove").click().should(() => {
			const users = JSON.parse(localStorage.getItem("users"));
			const { payload } = users;
			const expectedUsers = [];

			expect(payload.length).to.eq(0);
			expect(payload).to.deep.equal(expectedUsers);

		});
	});
});
