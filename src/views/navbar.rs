use crate::Route;
use dioxus::prelude::*;
use crate::components::footer::Footer;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");
const JJ: Asset = asset!("/assets/JJ.svg");

#[component]
pub fn Navbar() -> Element {
    // Get current route to determine which link is active
    let route = use_route::<Route>();

    // Function to determine if a link should have the "active" class
    let is_active = move |check_route: Route| -> String {
        if std::mem::discriminant(&route) == std::mem::discriminant(&check_route) {
            "active".to_string()
        } else {
            "".to_string()
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div { id: "navbar",
            div { style: "display: flex; flex-direction: row; align-items: center; justify-items: center;",
                img { src: JJ, style: "width: 70px" }
                div { style: "font-size: 34px; color: #ccc;", "Joe Johnson" }
            }
            div { class: "nav-links",
                Link { to: Route::Home {}, class: is_active(Route::Home {}), "Home" }
                Link { to: Route::Resume {}, class: is_active(Route::Resume {}), "Resume" }
                Link {
                    to: Route::Projects {},
                    class: is_active(Route::Projects {}),
                    "Projects"
                }
                Link {
                    to: Route::AboutMe {},
                    class: is_active(Route::AboutMe {}),
                    "About Me"
                }
                Link {
                    to: Route::Contact {},
                    class: is_active(Route::Contact {}),
                    "Contact"
                }
            }
        }
        main { Outlet::<Route> {} }

        Footer {}
    }
}
