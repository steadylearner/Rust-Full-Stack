const chalk = require("chalk");
const app = require("./server");

const {
    home,
    // register,
    login,
    authrequired,
    // logout,
} = require("./routes");

// Use this to deploy it with Docker and AWS
const { PORT = 8000 } = process.env;

// /views route will serve index.ejs, edit this later with auth relevant codes.
// app.get('/views', function (req, res) {
//   res.render('index', { title: 'Edit this later', })
// })

app.use("/", home);
// app.use("/register", register);
app.use("/login", login);
app.use("/authrequired", authrequired);
// app.use("logout", logout)

app.listen(PORT, () => {
    const blue = chalk.blue;
    // 0.0.0.0 to use it easily with Docker
    const target = blue(`http://0.0.0.0:${PORT}`);
    console.log(`🚀 Express Server ready at ${target}`);
});

// @app.route("/register", methods = ['GET', 'POST'])
// def register():
//     if request.method == 'POST':
//         #Parse form data
//         password = request.form['password']
//         email = request.form['email']
//         firstName = request.form['firstName']
//         lastName = request.form['lastName']
//         address1 = request.form['address1']
//         address2 = request.form['address2']
//         zipcode = request.form['zipcode']
//         city = request.form['city']
//         state = request.form['state']
//         country = request.form['country']
//         phone = request.form['phone']

//         with sqlite3.connect('database.db') as con:
//             try:
//                 cur = con.cursor()
//                 cur.execute('INSERT INTO users (password, email, firstName, lastName, address1, address2, zipcode, city, state, country, phone) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)', (hashlib.md5(password.encode()).hexdigest(), email, firstName, lastName, address1, address2, zipcode, city, state, country, phone))

//                 con.commit()

//                 msg = "Registered Successfully"
//             except:
//                 con.rollback()
//                 msg = "Error occured"
//         con.close()
//         return render_template("login.html", error=msg)

// const bcrypt = require("bcrypt");

// const password = "123123";
// const saltRounds = 10;

// bcrypt.hash(password, saltRounds, function (err, hash) {
//   if (err) {
//     console.log(err)
//   } else {
//     console.log(hash);
//   }
// });

// Convert this to JavaScript code
// Redirect, Remove credentials with redis, server, user etc
// Find API for them
// @app.route("/logout")
// def logout():
//    session.pop('email', None)
//    return redirect(url_for('root'))
