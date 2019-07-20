use yew::prelude::*;

pub struct Controls {
  title: String,
  exits: [u8; 3],
  onsignal: Option<Callback<crate::Msg>>,
}

pub enum Msg {
  ButtonPressed(crate::Msg),
}

#[derive(PartialEq, Clone)]
pub struct Props {
  pub exits: [u8; 3],
  pub onsignal: Option<Callback<crate::Msg>>,
}

impl Default for Props {
  fn default() -> Self {
    Self {
      exits: [0, 0, 0],
      onsignal: None,
    }
  }
}

impl Component for Controls {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Controls {
      title: "Controls".into(),
      exits: props.exits,
      onsignal: props.onsignal,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::ButtonPressed(msg) => {
        if let Some(ref mut callback) = self.onsignal {
          callback.emit(msg);
        }
      }
    }
    false
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.exits = props.exits;
    self.onsignal = props.onsignal;
    true
  }
}

impl Renderable<Controls> for Controls {
  fn view(&self) -> Html<Self> {
    let move_button = |target: &u8| {
      use crate::Msg::*;
      let t = *target;
      html! {
          <div class="control-button",>
              <button onclick=|_| Msg::ButtonPressed(SwitchRoom(t)),>{&format!("Move to {}", target)}</button>
              <button onclick=|_| Msg::ButtonPressed(ShootArrow(t)),>{&format!("Shoot {}", target)}</button>
          </div>
      }
    };
    html! {
        <div class=("container", "container-controls"),>
            <div class="title",>{&self.title}</div>
            <div class="exits",>{ for self.exits.iter().map(move_button) }</div>
        </div>
    }
  }
}
