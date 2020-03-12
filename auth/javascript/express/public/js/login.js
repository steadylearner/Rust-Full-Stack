// https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API/Using_Fetch
// https://github.com/axios/axios
// https://stackoverflow.com/questions/49601795/making-redirects-after-an-axios-post-request-with-express

document.getElementById("loginForm").addEventListener("submit", login);
async function login(event) {
	try {
		event.preventDefault();
		const { value: email } = document.getElementById("email");
		const { value: password } = document.getElementById("password");

		// It is in login.ejs
		// eslint-disable-next-line no-undef
		const response = await axios.post("/login", {
			email,
			password,
		});
		console.log(response);
		window.location = response.data.redirect;
	} catch (error) {
		console.error(error);
		window.location = "/login";
	}
}