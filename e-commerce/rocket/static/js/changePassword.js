// < !--https://www.google.com/search?q=what+happen+when+onsubmit+value+is+false -->
function validate() {
    var pass = document.getElementById("newpassword").value;
    var cpass = document.getElementById("cpassword").value;
    if (pass == cpass) {
        return true;
    } else {
        alert("Passwords do not match!");
        return false;
    }
}

