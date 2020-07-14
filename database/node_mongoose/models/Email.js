const mongoose = require('mongoose');
require('mongoose-type-email');
const Schema = mongoose.Schema;

// Use Recruiter or Receiver instead when you make a separate GitHub.
// Refer to it.
// https://github.com/Automattic/mongoose/blob/master/examples/schema/schema.js

// Include timestamps and use with dateOfEntry?
// https://mongoosejs.com/docs/guide.html#timestamps
const EmailSchema = new Schema({
  email: {
    type: mongoose.SchemaTypes.Email,
    required: true,
    unique: true,
    dropDups: true,
  },
  dateOfEntry: {
    type: Date,
    default: Date.now()
  },
  // 'true' When receiver responded.
  response: {
    type: Boolean,
    default: false
  },
  // Write a personal comment how you thought about the reply email.
  comment: {
    type: String,
    required: false,
  },
});

// Refer to this.
// NOTE: Should be before compiling it with mongoose.model()
// kittySchema.methods.speak = function () {
//  var greeting = this.name
//    ? "Meow name is " + this.name
//    : "I don't have a name";
//  console.log(greeting);
// }

// You can use this instead also. https://mongoosejs.com/docs/guide.html#es6-classes

const Email = mongoose.model('email', EmailSchema);

module.exports = Email;


