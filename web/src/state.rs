#[derive(Debug)]
pub struct State {
  pub responses: Vec<String>, // should be Vec<String> to save messages
  pub message_type: String, // use Enum later?
  pub connection: bool,
}
