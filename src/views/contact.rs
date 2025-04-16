use dioxus::prelude::*;
use gloo_console::{error, info};
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use async_std::task::sleep;

const CONTACT_CSS: Asset = asset!("assets/styling/contact.css");

// Define the payload structure for serialization
#[derive(Serialize, Deserialize)]
struct ContactFormData {
    name: String,
    email: String,
    message: String,
}

// Define a submission state enum
#[derive(PartialEq, Clone)]
enum SubmissionState {
    Ready,
    Sending,
    Success,
    Error,
}

#[component]
pub fn Contact() -> Element {
    // Use a signal to track submission state instead of just status text
    let submission_state = use_signal(|| SubmissionState::Ready);
    
    // Function to get button text based on state
    let button_text = move || {
        match *submission_state.read() {
            SubmissionState::Ready => "Send Message",
            SubmissionState::Sending => "Sending...",
            SubmissionState::Success => "Message Sent!",
            SubmissionState::Error => "Failed! Try Again",
        }
    };
    
    // Function to get button class based on state
    let button_class = move || {
        match *submission_state.read() {
            SubmissionState::Ready => "",
            SubmissionState::Sending => "sending",
            SubmissionState::Success => "success",
            SubmissionState::Error => "error",
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: CONTACT_CSS }

        section { class: "contact-page",
            div { class: "contact-container",
                h1 { "Get in Touch" }
                p {
                    "Feel free to reach out using the form below, email me directly at jjohnson2018tina@gmail.com,
                    or connect with me through social links in the footer."
                }

                form {
                    class: "contact-form",
                    onsubmit: move |evt: Event<FormData>| {
                        let data = evt.values();
                        let name = data
                            .get("name")
                            .and_then(|v| v.first())
                            .map(|v| v.clone())
                            .unwrap_or_default();
                        let email = data
                            .get("email")
                            .and_then(|v| v.first())
                            .map(|v| v.clone())
                            .unwrap_or_default();
                        let message = data
                            .get("message")
                            .and_then(|v| v.first())
                            .map(|v| v.clone())
                            .unwrap_or_default();
                        
                        // Only proceed if not already sending
                        if *submission_state.read() == SubmissionState::Sending {
                            return;
                        }
                        
                        let mut state = submission_state.to_owned();
                        state.set(SubmissionState::Sending);
                        
                        // Create the form data
                        let form_data = ContactFormData {
                            name,
                            email,
                            message,
                        };
                        
                        spawn(async move {
                            info!("Sending form data...");
                            
                            // Create a reqwest client
                            match reqwest::Client::new()
                                .post("https://formspree.io/f/xyzeogqy")
                                .header(CONTENT_TYPE, "application/json")
                                .json(&form_data)
                                .send()
                                .await
                            {
                                Ok(response) => {
                                    info!("Response status: {}", response.status().as_u16());
                                    
                                    if response.status().is_success() {
                                        // Clear the form fields
                                        document::eval(
                                            r#"
                                            document.getElementById('name').value = '';
                                            document.getElementById('email').value = '';
                                            document.getElementById('message').value = '';
                                            "#,
                                        )
                                        .await
                                        .ok();
                                        
                                        state.set(SubmissionState::Success);
                                        
                                        // Reset to Ready state after 3 seconds
                                        spawn(async move {
                                            sleep(std::time::Duration::from_secs(3)).await;
                                            state.set(SubmissionState::Ready);
                                        });
                                    } else {
                                        error!("Request failed with status: {}", response.status().to_string());
                                        state.set(SubmissionState::Error);
                                        
                                        // Reset to Ready state after 3 seconds
                                        spawn(async move {
                                            sleep(std::time::Duration::from_secs(3)).await;
                                            state.set(SubmissionState::Ready);
                                        });
                                    }
                                },
                                Err(e) => {
                                    error!("Request error: {}", e.to_string());
                                    state.set(SubmissionState::Error);
                                    
                                    // Reset to Ready state after 3 seconds
                                    spawn(async move {
                                        sleep(std::time::Duration::from_secs(3)).await;
                                        state.set(SubmissionState::Ready);
                                    });
                                }
                            }
                        });
                    },

                    label { r#for: "name", "Name" }
                    input {
                        r#type: "text",
                        id: "name",
                        name: "name",
                        placeholder: "Your Name",
                        required: true,
                    }

                    label { r#for: "email", "Email" }
                    input {
                        r#type: "email",
                        id: "email",
                        name: "email",
                        placeholder: "you@example.com",
                        required: true,
                    }

                    label { r#for: "message", "Message" }
                    textarea {
                        id: "message",
                        name: "message",
                        placeholder: "Your message...",
                        rows: "5",
                        required: true,
                    }

                    button {
                        r#type: "submit",
                        class: button_class(),
                        disabled: *submission_state.read() == SubmissionState::Sending,
                        {button_text()}
                    }
                    
                    // If you want to keep the separate status message as well, you can add:
                    if *submission_state.read() == SubmissionState::Error {
                        p { 
                            class: "form-status error", 
                            "Failed to send form! Please directly email jjohnson2018tina@gmail.com" 
                        }
                    }
                }
            }
        }
    }
}