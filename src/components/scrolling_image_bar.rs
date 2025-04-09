use dioxus::prelude::*;

#[component]
pub fn ImageScrollingBar(images: Vec<Asset>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("assets/styling/scrolling_image.css") }
        
        div { 
            class: "scrolling-container",
            div { 
                class: "scrolling-images",
                for image in images.iter() {
                    img { src: "{image}", alt: "Image" }
                }
                // Duplicate images for smooth infinite scrolling effect
                for image in images.iter() {
                    img { src: "{image}", alt: "Image" }
                }
            }
        }
    }
}