use dioxus::prelude::*;
use crate::components::resume::{EmbeddedResume, ResumeDownload};

const RESUME: Asset = asset!("assets/JJohnson_Resume_Website.pdf"); // TODO: Link to LinkedIN and Portfolio?

#[component]
pub fn Resume() -> Element {
    let resume = RESUME;
    rsx! {
        ResumeDownload { resume },
        EmbeddedResume { resume },
    }
}
