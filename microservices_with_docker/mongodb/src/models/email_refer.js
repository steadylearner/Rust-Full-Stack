const mongoose = require('mongoose');
require('mongoose-type-email');
const Schema = mongoose.Schema;

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
  response: {
    type: Boolean,
    default: false
  },
  comment: {
    type: String,
    required: false,
  },
});

const Item = mongoose.model('email', EmailSchema);

module.exports = Item;

