use dioxus::prelude::*;

const THATCHER_IMG: Asset = asset!("assets/Thatcher.jpg");
const SNEEZY_IMG: Asset = asset!("assets/Sneezy.jpg");
const ABOUT_ME_CSS: Asset = asset!("assets/styling/about_me.css");

#[component]
pub fn AboutMe() -> Element {
    rsx! {
        // Link to the CSS file
        document::Link { rel: "stylesheet", href: ABOUT_ME_CSS }

        section { class: "about-page",
            div { class: "hero-section",
                h1 { class: "hero-title", "Meet Joe" }
                p { class: "hero-tagline", "Designing for the stars — Grounded by curiosity" }
            }

            div { class: "content-section bg-light",
                h2 { class: "section-title", "Where I'm From & Why Space" }
                p { class: "section-text",
                    "I grew up on the outskirts of Tina, Missouri — A town (technically a village) you've probably never heard of. Working on cars and tractors I always 
                    found myself curious about how things worked and how to make them work better. My fascination with space started with stargazing and sci-fi, 
                    and evolved into a love for building real-world systems that make exploration possible."
                }
            }

            div { class: "content-section pets-section",
                h2 { class: "section-title", "Meet My Co-Pilots" }
                div { class: "pet-images",
                    div { class: "pet-container",
                        img { class: "pet-image", src: THATCHER_IMG, alt: "Thatcher the dog"}
                        p { class: "pet-name", "Thatcher" }
                        p { class: "pet-title", "Chief Stress Reliever"}
                        p{ "**Occassionaly Chief Stress Inducer"}
                    }
                    div { class: "pet-container",
                        img { style: "transform: rotate(90deg);", class: "pet-image", src: SNEEZY_IMG, alt: "Sneezy the cat" }
                        p { class: "pet-name", "Sneezy" }
                        p { class: "pet-title", "Feline Flight Director"}
                        p { "**Will steal food off your plate"}
                    }
                }
            }

            div { class: "content-section bg-light favorites-section",
                h2 { class: "section-title", "A Few of My Favorite Things" }
                ul { class: "favorites-list",
                    li { class: "favorite-item", 
                        span { class: "favorite-category", "Books: " } 
                        "The Hobbit, Uglies, The God Equation" 
                    }
                    li { class: "favorite-item", 
                        span { class: "favorite-category", "Games: " } 
                        "Rainbow Six Siege, Dark Souls, Satisfactory" 
                    }
                    li { class: "favorite-item", 
                        span { class: "favorite-category", "Movies: " } 
                        "Interstellar, The Martian, Wall-E" 
                    }
                }
            }

            div { class: "content-section",
                h2 { class: "section-title", "What Drives Me" }
                p { class: "section-text",
                    "I'm driven by systems thinking, cross-domain collaboration, and a deep curiosity for the unknown. 
                    I thrive in small, adaptable teams that care about the mission and enjoy the journey."
                }
            }

            div { class: "cta-section",
                a {
                    class: "resume-button",
                    href: "/resume",
                    "View My Resume"
                }
            }
        }
    }
}