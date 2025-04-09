use crate::components::scrolling_image_bar::ImageScrollingBar;
use dioxus::prelude::*;

const HOME_CSS: Asset = asset!("assets/styling/home.css");

const IMAGE1: Asset = asset!("assets/20220224_Eagle_Sat_Lab_PC-06492.jpg");
const IMAGE2: Asset = asset!("assets/20220224_Eagle_Sat_Lab_PC-06356.jpg");
const IMAGE3: Asset = asset!("assets/20220224_Eagle_Sat_Lab_PC-06309.jpg");
const IMAGE4: Asset = asset!("assets/20220224_Eagle_Sat_Lab_PC-06036.jpg");
const IMAGE5: Asset = asset!("assets/20220223_Prescott_Eagle_Sat-05098.jpg");
const IMAGE6: Asset = asset!("assets/20220224_Eagle_Sat_Lab_PC-06492.jpg");
const IMAGE7: Asset = asset!("assets/20220224_Eagle_Sat_Lab_PC-06356.jpg");
const IMAGE8: Asset = asset!("assets/20220224_Eagle_Sat_Lab_PC-06309.jpg");
const IMAGE9: Asset = asset!("assets/20220224_Eagle_Sat_Lab_PC-06036.jpg");
const IMAGE10: Asset = asset!("assets/20220223_Prescott_Eagle_Sat-05098.jpg");

#[component]
pub fn Home() -> Element {
    let images = vec![
        IMAGE1, IMAGE2, IMAGE3, IMAGE4, IMAGE5, IMAGE6, IMAGE7, IMAGE8, IMAGE9, IMAGE10,
    ];
    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }
        h2 { "Engineering Portfolio" }
        h1 { "Joe Johnson" }
        div { style: "text-align: center;",
            "Hello there. 👋 Hi, I'm Joe Johnson — aerospace engineer, systems wrangler, and lifelong tinkerer. I approach every project like it’s
            a quest to Mordor (but with fewer orcs and more code). With a wand in one hand (okay, maybe it's a soldering iron) and a lightsaber 
            in the other, I build systems that bridge earth and space — seamlessly, securely, and sometimes spectacularly. This portfolio is my 
            Master Sword — forged through late nights, epic team missions, and a relentless drive to level up."
        }
        div { style: "text-align: center; margin-top: 10px; font-size: 1.2rem; font-weight: bold;", "Welcome to my workshop."}
        ImageScrollingBar { images }
    }
}
