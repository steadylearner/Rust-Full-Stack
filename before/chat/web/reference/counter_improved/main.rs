#[macro_use]
extern crate yew;
use yew::prelude::*;

struct Model {
    value: i64,
}

enum Msg {
    Plus,
    Minus,
    Zero,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Plus => self.value = self.value + 1,
            Msg::Minus => self.value = self.value - 1,
            Msg::Zero => self.value = 0, 
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <section class=("flex", "center", "height-vh"), >
               <section>
                    <button
                        class=("hover-blue", "cursor-pointer"),
                        onclick=|_| Msg::Plus,
                        title="Click this to plus one",
                    >
                        { "+1" }
                    </button>
                    <button
                        class=("hover-red", "cursor-pointer"),
                        onclick=|_| Msg::Minus,
                        title="Click this to minus one",
                    >
                        { "-1" }
                    </button>
                   <p
                        class=("flex", "center", "cursor-pointer"),
                        onclick=|_| Msg::Zero,
                        title="Click this back to zero",
                    >
                        { self.value }
                    </p>
               </section>
            </section>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
