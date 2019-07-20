#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

mod components;
mod game;
mod util;

use self::{
  components::{controls::Controls, messages::Messages, stats::Stats},
  game::Game,
  util::*,
};
use yew::prelude::*;

pub enum Model {
  Waiting(String),
  Playing(Game),
}

impl Default for Model {
  fn default() -> Self {
    Model::Waiting("New game!".into())
  }
}

#[derive(Debug, Clone)]
pub enum Msg {
  StartGame,
  ShootArrow(u8),
  SwitchRoom(u8),
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Model::default()
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    use self::Msg::*;
    match msg {
      SwitchRoom(target) => match self {
        Model::Playing(game) => {
          game.current_room = target;
          if let Some(msg) = game.move_effects() {
            *self = Model::Waiting(msg);
          };
        }
        _ => unreachable!(),
      },
      StartGame => *self = Model::Playing(Game::default()),
      ShootArrow(target) => match self {
        Model::Playing(game) => {
          if game.wumpus == target {
            *self = Model::Waiting("With a sickening, satisfying thwack, your arrow finds its mark.  Wumpus for dinner tonight!  You win.".into());
          } else {
            game.arrows -= 1;
            game
              .messages
              .push("You arrow whistles aimlessly into the void".into());

            // If we exhausted our arrows, we lose
            if game.arrows == 0 {
              *self =
                Model::Waiting("You fired your very last arrow - you are now wumpus food".into());
            } else {
              // On each shot there's a 75% chance you scare the wumpus into an adjacant cell.
              let rand = js_rand(1, 4);
              if rand == 1 {
                game.messages.push(
                  "You listen quietly for any sign of movement - but the cave remains still."
                    .into(),
                );
              } else {
                game
                  .messages
                  .push("You hear a deafening roar - you've disturbed the wumpus!".into());
                let wumpus_exits = room_exits(game.wumpus).unwrap();
                let rand_idx = js_rand(0, 2);
                game.wumpus = wumpus_exits[rand_idx as usize];
                if game.wumpus == game.current_room {
                  *self = Model::Waiting(
                    "You scared the wumpus right on top of you.  Good going, mincemeat".into(),
                  );
                }
              }
            }
          }
        }
        _ => unreachable!(),
      },
    }
    true
  }
}

impl Renderable<Model> for Model {
  fn view(&self) -> Html<Self> {
    use self::Model::*;

    match self {
      Waiting(s) => html! {
        <div class="hunt",>
          <span class="over-message",>{s}</span>
          <button onclick=|_| Msg::StartGame,>{"Play Again"}</button>
        </div>
      },
      Playing(game) => html! {
          <div class="hunt",>
              <div class="header",>{"Hunt the Wumpus"}</div>
              <div class="window",>
                <Stats: arrows=game.arrows, current_room=game.current_room,/>
                <Controls: exits=room_exits(game.current_room).unwrap(), onsignal=|msg| msg,/>
              </div>
              <Messages: messages=&game.messages,/>
          </div>
      },
    }
  }
}
