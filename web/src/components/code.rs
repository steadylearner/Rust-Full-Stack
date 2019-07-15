// https://github.com/markedjs/marked
// marked('# Marked in the browser\n\nRendered by **marked**.'); or test with pulldown-cmark in Rust
// test it with https://cdn.jsdelivr.net/npm

use stdweb::web::Node;
use stdweb::unstable::TryFrom;
use yew::{html, Html};
use yew::virtual_dom::VNode;

use crate::Model;

pub fn view_code(value: &str) -> Html<Model> {
    //  or use code from pulldown-cmark here
    // and pass it with @{} ? 
    let markdown = js! {
        var div = document.createElement("div");
        var code = @{&value};
        div.innerHTML = marked(code);
        console.log(div);
        return div;
    };
    eprintln!("markdown: {:?}", markdown);
    let node = Node::try_from(markdown).expect("convert markdown");
    let vnode = VNode::VRef(node);
    eprintln!("div: {:?}", vnode);

    html! {
       { vnode }
    }
}

