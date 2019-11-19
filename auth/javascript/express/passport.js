const passport = require('passport');
const LocalStrategy = require('passport-local').Strategy;

// Use Postgresql(pg) later.
const users = [
    // use uuid or from database
    { id: 'steadylearner', email: 'steady@learner.com', password: 'password' },
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
        if (email === user.email && password === user.password) {
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

module.exports = passport;