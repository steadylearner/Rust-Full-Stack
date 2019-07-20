use serde_derive::{Deserialize, Serialize};

// should separate value and use it in Input components or use localStorage and integrate them all?
#[derive(Serialize, Deserialize, Debug)]
pub struct State {
  pub user_id: Option<String>,
  pub user_inputs: Vec<String>,
  pub ws_responses: Vec<Option<String>>,
  pub value: String,
  // pub use_code: bool // use this to enable markdown or not
}

// impl Default for State {
//   fn default() -> Self {
//     let state = Self {
//       user_id: None,
//       user_inputs: Vec::new(),
//       ws_responses: Vec::new(), // is this right approach?
//     };
//     // I can use some methods to handle state with let mut state
//     state
//   }
// }
