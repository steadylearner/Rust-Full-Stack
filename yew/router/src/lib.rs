// #[macro_use]
#![recursion_limit="128"]
extern crate stdweb;
// #[macro_use]
extern crate yew;

use stdweb::web::Location;
use stdweb::web::window;

mod router;
mod routing; // For use crate::routing::RouteService; in router.rs to work

use log::info;
use router::Route;
use yew::{html, Bridge, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::agent::Bridged;
use yew::services::{
    ConsoleService,
};

// https://github.com/DenisKolodin/yew/blob/master/examples/routing/src/lib.rs

pub enum Child {
    Home,
    With,
    PathNotFound(String)
}

pub struct Model {
    child: Child,
    router: Box<dyn Bridge<router::Router<()>>>, // Can't use this easily because it is dynamically allocated

    // Yew
    console: ConsoleService,
    // Temporary 
    location: Location,
}

pub enum Msg {
    NavigateTo(Child),
    HandleRoute(Route<()>)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {

        let callback = link.send_back(|route: Route<()>| Msg::HandleRoute(route));
        let mut router = router::Router::bridge(callback);

        router.send(router::Request::GetCurrentRoute);

        let location = window().location().expect("browser does not support location API");

        Model {
            child: Child::Home, // This should be quickly overwritten by the actual route.
            router,
            console: ConsoleService::new(),
            location,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // If you click various times, it will save every instance
        // should wait for a little time to app work after it shows
        match msg {
            Msg::NavigateTo(child) => {
                // use pathname here for compatibility with self.location.pathname().unwrap(); below
                let pathname: Vec<String> = match child {
                    Child::Home => vec!["".into()],
                    Child::With => vec!["with".into()],
                    Child::PathNotFound(_) => vec!["path_not_found".into()]
                };

                // How to get current path?
                // instead of self.router.route_service.get_path();

                // https://docs.rs/stdweb/0.4.18/stdweb/web/struct.Location.html
                let current_route = self.location.pathname().unwrap();

                let target_route = format!("/{}", &pathname[0]).to_string();

                self.console.log(&format!("Current Route: {}", &current_route).to_string());
                self.console.log(&format!("Target Route: {}", &target_route).to_string());

                if current_route == target_route {
                    self.console.warn("It is not allowed to use route for the same page.");
                    false
                } else {
                    let route = router::Route {
                        pathname,
                        query: None,
                        fragment: None,
                        state: (),
                    };

                    self.router.send(router::Request::ChangeRoute(route));
                    false
                }

                // let current_route = self.location.pathname().unwrap(); // shows with
                // self.console.log(&format!("Current Route: {}", &current_route).to_string());
                // self.console.log(&format!("Target Route: {}", pathname[0]).to_string());

                // // You can see that there difference is only /
                // // Current Route: /
                // // Target Route:
                // // Current Route: /about
                // // Target Route: about

                // let route = router::Route {
                //     pathname,
                //     query: None,
                //     fragment: None,
                //     state: (),
                // };

                // self.router.send(router::Request::ChangeRoute(route));
                // false
            }
            Msg::HandleRoute(route) => {
                info!("Routing: {}", route.to_route_string());
                self.child = if let Some(first_segment) = route.pathname.get(0) {
                   match first_segment.as_str() {
                       "" => Child::Home,
                       "with" => Child::With,
                        other => Child::PathNotFound(other.into())
                   }
                } else {
                    Child::PathNotFound("path_not_found".into())
                };

                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {

        let nav_class = "nav nav-height flex center border-white white font-two".to_string();
        let list_class = "flex no-text-decoration hover cursor-pointer transition-half margin-left-one".to_string();
        
        let signup_in_list = "transition-half no-text-decoration flex cursor-pointer font-one-and-eight hover margin-right-one".to_string();

        let subnav_class = "fixed sub nav-height width-vw theme-black border-white center".to_string();

        let rust_image_class = "width-two-and-a-half theme-white border-round margin-right-half";

        html! {
            <section>
                <nav id="nav", class=nav_class, >
                    // <ul> // Let CSS later for Rust is our foucs
                        <li
                            class=&list_class,
                        >
                            <img
                                src="https://www.steadylearner.com/static/images/code/Rust.svg",
                                class=rust_image_class,
                                onclick=|_| Msg::NavigateTo(Child::Home),
                            />
                        </li>
                        <li
                            class={format!("{} font-one-and-eight right-auto", &list_class)},
                            onclick=|_| Msg::NavigateTo(Child::With),
                        >
                            <span>
                                { "With" }
                            </span>
                        </li>
                        <li
                            class=signup_in_list,
                            // onclick=|_| Msg::NavigateTo(Child::SignUp),
                        >
                            <span class="sign-up", title="Use your own code.", >
                                // or build modal with
                                // https://www.steadylearner.com/blog/read/How-to-use-a-modal-in-Rust
                                { "Sign Up" }
                            </span>
                        </li>
                    // </ul>
                </nav>
                <main>
                    <section>
                        {self.child.view()}
                    </section>
                </main>
                <footer class=subnav_class, >
                    <a
                        class=("flex", "no-text-decoration", "hover", "cursor-pointer", "transition-half", "auto", "margin-left-one"),
                        href="https://www.steadylearner.com/blog/search/Rust",
                        title="Click it to learn how to code this.",
                        target="_blank",
                        rel="noopener noreferrer",
                    >
                        <span 
                            class=("white", "bold", "flex", "center"), 
                        >
                            { "© Steadylearner" }
                        </span>
                    </a>
                </footer>
            <section/>
        }
    }
}



impl Renderable<Model> for Child {
    fn view(&self) -> Html<Model> {
        match *self {
            Child::Home => html! {
                <img 
                    class=("width-vw", "height-vh"),
                    src="https://www.steadylearner.com/static/images/brand/code_b.png",
                />
            },
            Child::With => html! {
                <img 
                    class=("width-vw", "height-vh"),
                    src="https://www.steadylearner.com/static/images/brand/rust_you.png",
                />
            },
            Child::PathNotFound(ref path) => html! {
                <section class=("center-percent-absolute", "text-center"), >
                    <span class=("bold", "font-two-and-a-half", "red-white"), >
                        {format!("Invalid path: '{}'", path)}
                    </span>
                </section>
            }
        }
    }
}

