extern crate yew;

extern crate sl_lib;

use sl_lib::Model;
// use yew::prelude::App;

fn main() {
  // yew::initialize();
  // let app: App<Model> = App::new(); // Not yew::start_app::<routing::Model>();
  // app.mount_to_body();
  // yew::run_loop();

  yew::start_app::<Model>();
}

// pub fn start_app<COMP>()
// where
//     COMP: Component<Properties = ()> + Renderable<COMP>,
// {
//     initialize();
//     App::<COMP>::new().mount_to_body();
//     run_loop();
// }
