// < !--https://www.google.com/search?q=what+happen+when+onsubmit+value+is+false -->
// It will be used at editProfile.ejs
// eslint-disable-next-line no-unused-vars
function validateEditPassword() {
	const pass = document.getElementById("newpassword").value;
	const cpass = document.getElementById("cpassword").value;
	if (pass == cpass) {
		return true;
	} else {
		alert("Passwords should be equal.");
		return false;
	}
}
