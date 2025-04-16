use dioxus::prelude::*;

const FOOTER_CSS: Asset = asset!("assets/styling/footer.css");

#[component]
pub fn Footer() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: FOOTER_CSS }

        footer { class: "footer",
            div { class: "footer-content",
                // Left side (copyright)
                div { class: "footer-left",
                    p { "© 2025 Joe Johnson. All rights reserved." }
                }
                // Right side (social links)
                div { class: "footer-right",
                    div { class: "footer-links",
                        a {
                            href: "https://github.com/JJohnson357",
                            target: "_blank",
                            class: "footer-link-item",
                            img {
                                src: asset!("assets/github.svg"),
                                alt: "GitHub",
                                class: "footer-icon",
                            }
                            span { "GitHub" }
                        }
                        a {
                            href: "https://www.linkedin.com/in/joe-johnson357/",
                            target: "_blank",
                            class: "footer-link-item",
                            img {
                                src: asset!("assets/linkedin.svg"),
                                alt: "LinkedIn",
                                class: "footer-icon",
                            }
                            span { "LinkedIn" }
                        }
                    }
                }
            }
        }
    }
    
}