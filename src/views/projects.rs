use dioxus::prelude::*;

use crate::components::project_card::ProjectCard;

const PROJECTS_CSS: Asset = asset!("assets/styling/projects.css");

const MODIFI_IMAGE: Asset = asset!("assets/project_images/modifi_images/modifi.jpg");
const MODEM_IMAGE: Asset = asset!("assets/project_images/modem_images/modem.png");
const HOST_IMAGE: Asset = asset!("assets/project_images/host_images/host.png");
const CUBIC16_IMAGE: Asset = asset!("assets/project_images/cubic-16_images/cubic16.png");
const DRIP_IMAGE: Asset = asset!("assets/project_images/drip_images/drip.png");
const EAGLESAT_IMAGE: Asset = asset!("assets/project_images/eaglesat_images/eaglesat.jpg");

#[component]
pub fn Projects() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: PROJECTS_CSS }

        section { class: "projects-section", id: "projects",
            div { class: "projects-header",
                h1 { class: "projects-title", "Projects" }
                p { class: "projects-subtitle", "Here are a few projects I've worked on recently." }
            }

            div { class: "projects-container",
                ProjectCard {
                    title: "ModiFi",
                    date: "2023-2025",
                    description: "A ruggedized wireless replacement for cable harnesses in aerospace vehicles, featuring fault tolerance and 
                    WiFi HaLoW support.",
                    image: MODIFI_IMAGE,
                    github_url: None,
                    external_url: Some("https://modularitywireless.com/products/modifi".to_string()),
                }
                ProjectCard {
                    title: "MODEM",
                    date: "2023-2025",
                    description: "A Rust (Axum + Dioxus)-based desktop application for real-time data visualization, remote and local network management 
                    of IoT devices, and more.",
                    image: MODEM_IMAGE,
                    github_url: None,
                    external_url: Some("https://modularitywireless.com/products/modem".to_string()),
                }
                ProjectCard {
                    title: "HOST",
                    date: "2021-2022",
                    description: "An ESPA-class smallsat designed for operation in SSO-LEO. Modularized structures, subsystems and payload interfaces to 
                    enable a \"Real Estate\" - like lease based approach to customer procurement for scientific, DoD and commercial payloads.",
                    image: HOST_IMAGE,
                    github_url: None,
                    external_url: None,
                }
                ProjectCard {
                    title: "CUBIC-16",
                    date: "2021-2022",
                    description: "A 16U cubesat to test core systems in HOST in a rapid and low(ish)-cost manner.",
                    image: CUBIC16_IMAGE,
                    github_url: None,
                    external_url: None,
                }
                ProjectCard {
                    title: "DRIP",
                    date: "2022",
                    description: "Researching, designing and developing a prototype to determine the feasibility of space-based debris detection and 
                    classification with stereo-optic camera systems.",
                    image: DRIP_IMAGE,
                    github_url: None,
                    external_url: Some("https://arc.aiaa.org/doi/10.2514/6.2022-4276".to_string()),
                }
                ProjectCard {
                    title: "EagleSat-II",
                    date: "2022",
                    description: "A 3U cubesat planned to launch in mid to late 2025. Includes an in-house developed payload to monitor the degradation
                    of multiple types of commercial computer memory when exposed to radiation in Low Earth Orbit. Development included the bring-up of 
                    an ISO-6 clean room and a 433MHz ground station.",
                    image: EAGLESAT_IMAGE,
                    github_url: Some("https://github.com/ERAU-EagleSat-PR/es2-estts-cpp".to_string()),
                    external_url: Some("https://eaglelife.erau.edu/eaglesat/home/".to_string()),
                }
            }
        }
    }
}