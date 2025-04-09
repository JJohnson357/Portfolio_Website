use dioxus::prelude::*;

const FOOTER_CSS: Asset = asset!("assets/styling/footer.css");

#[component]
pub fn Footer() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: FOOTER_CSS }

        footer { class: "footer",
            div { class: "footer-content",
                // Copyright
                p { "© 2025 Joe Johnson. All rights reserved." }
                // Social links
                div { class: "footer-links",
                    a {
                        href: "https://github.com/yourusername",
                        target: "_blank",
                        "GitHub"
                    }
                    a {
                        href: "https://linkedin.com/in/yourusername",
                        target: "_blank",
                        "LinkedIn"
                    }
                }
            }
        }
    }
    
}