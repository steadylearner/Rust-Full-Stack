const bcrypt = require("bcrypt");

// password="password" node serialize.js
const { password = "password" } = process.env;
const saltRounds = 10; // Because of this, it return different result each time.

bcrypt.hash(password, saltRounds, function (err, hash) {
	if (err) {
		console.log(err);
	} else {
		console.log(hash);
	}
});

// bcrypt.compareSync(password, user.password))
