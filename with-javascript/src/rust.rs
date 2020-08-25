// Number in JavaScript is all float(f64)?
// (It shows you can not send or receive i64 value. So use f64 instead.)
#[no_mangle]
pub fn divide_by_number_of_participants(number_of_participants: f64) -> f64 {
  // Normally you need to convert i64 to f64
  // let result = 1f64 / number_of_participants as f64;
  let result = 1f64 / number_of_participants
  result
}
