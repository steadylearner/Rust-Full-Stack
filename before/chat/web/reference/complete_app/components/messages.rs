use yew::prelude::*;

pub struct Messages {
    title: String,
    messages: Vec<String>,
}

pub enum Msg {}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub messages: Vec<String>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            messages: Vec::new(),
        }
    }
}

impl Component for Messages {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Messages {
            title: "Messages".into(),
            messages: props.messages,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.messages = props.messages;
        true
    }
}

impl Renderable<Messages> for Messages {
    fn view(&self) -> Html<Self> {
        let view_message = |message: &String| {
            html! {
                <li>{message}</li>
            }
        };
        html! {
            <div class=("container", "container-messages"),>
                <div class="title",>{&self.title}</div>
                <div class="scroller",>
                    <ul>{ for self.messages.iter().rev().map(view_message) }</ul>
                </div>
            </div>
        }
    }
}
