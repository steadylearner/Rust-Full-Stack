#[derive(Debug)]
pub struct Modal {
  pub used: bool
}

impl Modal {
    pub fn set(&mut self, intention: bool) { // or separate or with
        self.used = intention
    }
}

impl Default for Modal {
  fn default() -> Self {
    let modal = Self {
      used: false,
    };
    modal
  }
}