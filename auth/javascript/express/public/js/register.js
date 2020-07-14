// <!-- https://www.google.com/search?q=what+happen+when+onsubmit+value+is+false -->
// It is used at register.ejs
// eslint-disable-next-line no-unused-vars
function validateRegister() {
	var pass = document.getElementById("password").value;
	var cpass = document.getElementById("cpassword").value;
	if (pass == cpass) {
		return true;
	} else {
		alert("Passwords do not match");
		return false;
	}
}


