// < !--https://www.google.com/search?q=what+happen+when+onsubmit+value+is+false -->
// It will be used at editProfile.ejs
// eslint-disable-next-line no-unused-vars

document.getElementsByClassName("button--delete-profile-image")[0].onclick = (e) => deleteProfileImage(e);
async function deleteProfileImage(event) {
	try {
		event.preventDefault();

		// It is in login.ejs
		// eslint-disable-next-line no-undef
		const response = await axios.post("/profile/image/delete");
		window.location = response.data.redirect;
	} catch (error) {
		console.error(error);
		// window.location = "/login";
	}
}