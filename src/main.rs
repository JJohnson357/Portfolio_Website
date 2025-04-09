use dioxus::prelude::*;

use views::{Home, Navbar, Resume, Projects, AboutMe, Contact};

mod components;

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/resume")]
        Resume {},
        #[route("/projects")]
        Projects {},
        #[route("/about_me")]
        AboutMe {},
        #[route("/contact")]
        Contact {},
}

const JJ: Asset = asset!("/assets/JJ.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: JJ }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}
