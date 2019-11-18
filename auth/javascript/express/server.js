// https://medium.com/@evangow/server-authentication-basics-express-sessions-passport-and-curl-359b7456003d

// Modulize this with prexisting project with ESLINT etc.

const express = require("express");
const app = express();
const chalk = require("chalk");
const path = require("path");

const uuid = require('uuid/v4');
// https://github.com/expressjs/session#example
// https://www.npmjs.com/package/session-file-store
// Use these instead of session-file-store if necessary.
// https://www.npmjs.com/package/connect-pg-simple
// https://www.npmjs.com/package/connect-redis
const session = require('express-session');
const FileStore = require('session-file-store')(session);

const bodyParser = require('body-parser');

// Include GitHub etc later with OAUTH2 if necessary.
const passport = require('passport');
const LocalStrategy = require('passport-local').Strategy;

// Use this to deploy it with Docker and AWS
let { PORT } = process.env;
if (PORT === undefined) {
  PORT = 8000;
}

// You can serve static files in /public and tempaltes in /views with this.
// Always be the start of your app.
app.use(express.static("public"));
app.set('views', path.join(__dirname, 'views'));
app.set('view engine', 'ejs');

app.use(bodyParser.urlencoded({ extended: false })); // -H application/x-www-form-urlencoded
app.use(bodyParser.json());

// Use Postgresql(pg) later.
const users = [
  // use uuid or from database
  {id: 'steadylearner', email: 'steady@learner.com', password: 'password' },
];

// configure passport.js to use the local strategy
passport.use(new LocalStrategy(
  { usernameField: 'email' },
  (email, password, done) => {
    console.log('Inside local strategy callback')
    // here is where you make a call to the database
    // to find the user based on their username or email address
    // for now, we'll just pretend we found that it was users[0]
    const user = users[0]
    // [Payload] and use database instead
    if(email === user.email && password === user.password) {
      console.log('Local strategy returned true')
      return done(null, user)
    }
    // axios.get(${email})
    // .then(res => {
    //   const user = res.data[0]
    //   if (!user) {
    //     return done(null, false, { message: 'Invalid credentials.\n' });
    //   }
    //   if (!bcrypt.compareSync(password, user.password)) {
    //     return done(null, false, { message: 'Invalid credentials.\n' });
    //   }
    //   return done(null, user);
    // })
    // .catch(error => done(error));

    // bcrypt.compare(password, db.numenu.password, function (err, result) {
    //   if (err) {
    //     console.log(err);
    //     res.json({
    //       sucess: false,
    //       err,
    //       token: null,
    //     })
    //   } else {
    //     if (result === true) {
    //       console.log("Valid!");
    //       let token = jwt.sign({ username: db.numenu.username }, "steadylearner", { expiresIn: 129600 }); // Signing the token, use the same data("steadylearner") in jwtMW
    //       // compare isso com Payload de Cliente em test/index.js pasta
    //       console.log("Payload de Servidor: ", token);
    //       res.json({
    //         sucess: true,
    //         err: null,
    //         token
    //       });
    //     }
    //     else {
    //       console.log("Senha recebido e hash gerado não são iguais!");
    //       res.status(401).json({
    //         sucess: false,
    //         token: null,
    //         err: "Senha recebido e hash gerado não são iguais!"
    //       });
    //     }
    //   }
    // }
  }
));

// tell passport how to serialize the user
passport.serializeUser((user, done) => {
  console.log('Inside serializeUser callback. User id is save to the session file store here')
  done(null, user.id);
});

passport.deserializeUser((id, done) => {
  console.log('Inside deserializeUser callback')
  console.log(`The user id passport saved in the session file store is: ${id}`)
  // Use database instead
  const user = users[0].id === id ? users[0] : false;
  done(null, user);
  // axios.get(`${id}`)
  //   .then(res => done(null, res.data))
  //   .catch(error => done(error, false))
});

app.use(session({
  genid: (req) => {
    console.log('Inside the session middleware')
    console.log(req.sessionID) // undefined yet
    return uuid() // use UUIDs for session IDs
  },
  store: new FileStore(),
  secret: 'steady learner',
  resave: false,
  saveUninitialized: true
}));

app.use(passport.initialize());
app.use(passport.session());

// /views route will serve index.ejs, edit this later with auth relevant codes.
app.get('/views', function (req, res) {
  res.render('index', { title: 'Edit this later', })
})

// $curl -X GET http://localhost:8000 -c cookie-file.txt -v
// $curl -X GET http://localhost:8000 -b cookie-file.txt -v
app.get('/', (req, res) => {
  console.log('Inside the homepage callback function');
  console.log(req.sessionID);
  res.send(`User visit / and verify the console to see sessionID`);
})

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

// create the login get and post routes
app.get('/login', (req, res) => {
  console.log('Inside GET /login callback function')
  console.log(req.sessionID)
  // Use res.render later
  res.send(`You are at the login page! Type your id and password.\n`)
})

// curl -X POST http://localhost:8000/login -b cookie-file.txt -H 'Content-Type: application/json' -d '{"email":"steady@learner.com", "password":"password"}'
// https://www.npmjs.com/package/passport
// app.post('/login', (req, res) => {
//  console.log('Inside POST /login callback function')
//  console.log(req.body)
//  res.send(`You attempted to login with POST HTTP method and the credential datas!\n`)
// })

// Should write logic for error(fail) and redirect also.
// https://www.npmjs.com/package/passport#authenticate-requests
// $curl -X POST  http://localhost:8000/login -c cookie-file.txt -H 'Content-Type: application/json' -d '{"email":"steady@learner.com", "password":"password"}'
app.post('/login', (req, res, next) => {
  console.log('Inside POST /login callback')
  passport.authenticate('local', (err, user, info) => {
    console.log('Inside passport.authenticate() callback');
    console.log(`req.session.passport: ${JSON.stringify(req.session.passport)}`)
    console.log(`req.user: ${JSON.stringify(req.user)}`)
    req.login(user, (err) => {
      console.log('Inside req.login() callback')
      console.log(`req.session.passport: ${JSON.stringify(req.session.passport)}`)
      console.log(`req.user: ${JSON.stringify(req.user)}`)
      // Use res.render("profile");
      return res.send('You were authenticated & logged in!\n');
    })
  })(req, res, next);
})

// $curl -X GET http://localhost:8000 -c cookie-file.txt
// $curl -X GET http://localhost:8000/authrequired -b cookie-file.txt -L
// [SERVER] User authenticated? false
// [CLIENT] Redirect to / and shows User visit / and verify the console to see sessionID

// curl -X POST  http://localhost:8000/login -c cookie-file.txt -H 'Content-Type: application/json' -d '{"email":"steady@learner.com", "password":"password"}'
// curl -X GET http://localhost:8000/authrequired -b cookie-file.txt -L
// [SERVER] User authenticated? true
// [CLIENT] you hit the authentication endpoint
app.get('/authrequired', (req, res) => {
  console.log('Inside GET /authrequired callback')
  console.log(`User authenticated? ${req.isAuthenticated()}`)
  if(req.isAuthenticated()) {
    res.send('you hit the authentication endpoint\n')
  } else {
    res.redirect('/')
  }
})

// Convert this to JavaScript code
// Redirect, Remove credentials from session/, server, user etc
// Find API for them
// @app.route("/logout")
// def logout():
//    session.pop('email', None)
//    return redirect(url_for('root'))

app.listen(PORT, () => {
  const blue = chalk.blue;
  // 0.0.0.0 to use it easily with Docker
  const target = blue(`http://0.0.0.0:${PORT}`);
  console.log(`🚀 Server ready at ${target}`);
});

