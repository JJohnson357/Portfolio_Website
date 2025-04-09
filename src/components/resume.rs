use dioxus::prelude::*;

const RESUME_CSS: Asset = asset!("assets/styling/resume.css");



#[component]
pub fn ResumeDownload(resume: Asset) -> Element {
    // Create a function to handle the download when the label is clicked
    let handle_download = move |_| {
        // Create a programmatic download via JavaScript
        let window = web_sys::window().expect("No window found");
        let document = window.document().expect("No document found");
        
        // Create a temporary anchor element
        let temp_link = document.create_element("a").expect("Failed to create element");
        let temp_anchor = web_sys::wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlAnchorElement>(temp_link).expect("Failed to convert");
        
        // Set its attributes for downloading
        temp_anchor.set_href(&resume.to_string());
        temp_anchor.set_download("JJohnson_Resume.pdf");
        
        // Append to document, click it, then remove it
        document.body().unwrap().append_child(&temp_anchor).expect("Failed to append");
        temp_anchor.click();
        document.body().unwrap().remove_child(&temp_anchor).expect("Failed to remove");
    };

    rsx! {
        document::Link { rel: "stylesheet", href: RESUME_CSS }
        div {
            div { class: "container noselect",
                label {
                    id: "clickHandler",
                    r#for: "checkbox",
                    onclick: handle_download,
                }
                input { id: "checkbox", r#type: "checkbox" }
                button { id: "button",
                    p { id: "to-launch", "Download Here" }
                    p { id: "tag", "Launch Success!" }
                    div { id: "platform" }
                    div { class: "caution",
                        div { id: "caution-left", "LAUNCH ZONE" }
                        div { id: "caution-right", "LAUNCH ZONE" }
                    }
                }
                div { id: "shuttle-wrapper",
                    div { id: "shadow" }
                    svg {
                        id: "b",
                        view_box: "0 0 230.24 542.46",
                        "xlink": "http://www.w3.org/1999/xlink",
                        xmlns: "http://www.w3.org/2000/svg",
                        defs {
                            linearGradient {
                                gradient_transform: "translate(160.35 -471.85) rotate(-7.06) scale(1.56 1.68)",
                                gradient_units: "userSpaceOnUse",
                                id: "d",
                                x1: "-130.63",
                                x2: "-104.11",
                                y1: "545.09",
                                y2: "549.77",
                                stop { offset: "0", stop_color: "#c6c6c6" }
                                stop { offset: "0", stop_color: "#b1b1b1" }
                                stop { offset: ".02", stop_color: "#858585" }
                                stop { offset: ".04", stop_color: "#606060" }
                                stop { offset: ".05", stop_color: "#424242" }
                                stop { offset: ".07", stop_color: "#2a2a2a" }
                                stop { offset: ".09", stop_color: "#191919" }
                                stop { offset: ".1", stop_color: "#101010" }
                                stop { offset: ".13", stop_color: "#0d0d0d" }
                                stop { offset: ".31", stop_color: "#454545" }
                                stop { offset: ".33", stop_color: "#4c4c4c" }
                                stop { offset: ".35", stop_color: "#484848" }
                                stop { offset: ".52", stop_color: "#2b2c2b" }
                                stop { offset: ".68", stop_color: "#171817" }
                                stop { offset: ".84", stop_color: "#0b0c0b" }
                                stop { offset: "1", stop_color: "#070807" }
                            }
                            linearGradient {
                                gradient_units: "userSpaceOnUse",
                                id: "e",
                                x1: "70.56",
                                x2: "113.37",
                                y1: "475.4",
                                y2: "479.08",
                                stop {
                                    offset: "0",
                                    stop_color: "#fff",
                                    stop_opacity: "0",
                                }
                                stop {
                                    offset: ".32",
                                    stop_color: "#fff",
                                    stop_opacity: "0",
                                }
                                stop {
                                    offset: ".38",
                                    stop_color: "#fff",
                                    stop_opacity: "0",
                                }
                                stop {
                                    offset: "1",
                                    stop_color: "#b3511b",
                                    stop_opacity: "0",
                                }
                            }
                            linearGradient {
                                gradient_units: "userSpaceOnUse",
                                id: "f",
                                x1: "65.74",
                                x2: "123.89",
                                y1: "269.31",
                                y2: "270.33",
                                stop { offset: "0", stop_color: "#e5e5e5" }
                                stop { offset: ".63", stop_color: "#e3e3e3" }
                                stop { offset: ".85", stop_color: "#dcdcdc" }
                                stop { offset: "1", stop_color: "#d3d2d3" }
                            }
                            linearGradient {
                                gradient_units: "userSpaceOnUse",
                                id: "g",
                                x1: "70.84",
                                x2: "112.45",
                                y1: "114.99",
                                y2: "114.99",
                                stop { offset: "0", stop_color: "#1a1a1a" }
                                stop { offset: ".05", stop_color: "#2a2a2a" }
                                stop { offset: ".15", stop_color: "#434343" }
                                stop { offset: ".2", stop_color: "#4d4d4d" }
                                stop { offset: ".3", stop_color: "#474747" }
                                stop { offset: ".69", stop_color: "#383838" }
                                stop { offset: "1", stop_color: "#333" }
                            }
                            linearGradient {
                                href: "#g",
                                id: "h",
                                x1: "70.84",
                                x2: "112.45",
                                y1: "142.07",
                                y2: "142.07",
                            }
                            linearGradient {
                                href: "#g",
                                id: "i",
                                x1: "70.84",
                                x2: "112.45",
                                y1: "178.73",
                                y2: "178.73",
                            }
                            linearGradient {
                                href: "#g",
                                id: "j",
                                x1: "70.84",
                                x2: "112.45",
                                y1: "254.03",
                                y2: "254.03",
                            }
                            linearGradient {
                                href: "#g",
                                id: "k",
                                x1: "70.84",
                                x2: "112.45",
                                y1: "329.25",
                                y2: "329.25",
                            }
                            linearGradient {
                                href: "#g",
                                id: "l",
                                x1: "70.84",
                                x2: "112.45",
                                y1: "386.88",
                                y2: "386.88",
                            }
                            linearGradient {
                                href: "#g",
                                id: "m",
                                x1: "70.84",
                                x2: "112.45",
                                y1: "407.2",
                                y2: "407.2",
                            }
                            linearGradient {
                                href: "#g",
                                id: "n",
                                x1: "70.84",
                                x2: "112.45",
                                y1: "414.79",
                                y2: "414.79",
                            }
                            linearGradient {
                                href: "#g",
                                id: "o",
                                x1: "70.84",
                                x2: "112.45",
                                y1: "423.88",
                                y2: "423.88",
                            }
                            linearGradient {
                                href: "#g",
                                id: "p",
                                x1: "70.84",
                                x2: "112.45",
                                y1: "435.68",
                                y2: "435.68",
                            }
                            linearGradient {
                                href: "#g",
                                id: "q",
                                x1: "70.84",
                                x2: "112.45",
                                y1: "446.33",
                                y2: "446.33",
                            }
                            linearGradient {
                                gradient_units: "userSpaceOnUse",
                                id: "r",
                                x1: "70.67",
                                x2: "112.45",
                                y1: "280.59",
                                y2: "280.59",
                                stop {
                                    offset: "0",
                                    stop_color: "#fff",
                                    stop_opacity: "0",
                                }
                                stop {
                                    offset: ".14",
                                    stop_color: "#fff",
                                    stop_opacity: "0",
                                }
                                stop {
                                    offset: ".19",
                                    stop_color: "#fff",
                                    stop_opacity: ".03",
                                }
                                stop {
                                    offset: ".23",
                                    stop_color: "#fff",
                                    stop_opacity: ".08",
                                }
                                stop {
                                    offset: ".25",
                                    stop_color: "#fff",
                                    stop_opacity: ".15",
                                }
                                stop {
                                    offset: ".28",
                                    stop_color: "#fff",
                                    stop_opacity: ".23",
                                }
                                stop {
                                    offset: ".3",
                                    stop_color: "#fff",
                                    stop_opacity: ".34",
                                }
                                stop {
                                    offset: ".32",
                                    stop_color: "#fff",
                                    stop_opacity: ".47",
                                }
                                stop {
                                    offset: ".34",
                                    stop_color: "#fff",
                                    stop_opacity: ".61",
                                }
                                stop {
                                    offset: ".36",
                                    stop_color: "#fff",
                                    stop_opacity: ".78",
                                }
                                stop {
                                    offset: ".36",
                                    stop_color: "#fff",
                                    stop_opacity: ".8",
                                }
                                stop {
                                    offset: ".38",
                                    stop_color: "#fff",
                                    stop_opacity: "0",
                                }
                                stop {
                                    offset: "1",
                                    stop_color: "#b3511b",
                                    stop_opacity: "0",
                                }
                            }
                            linearGradient {
                                gradient_units: "userSpaceOnUse",
                                id: "s",
                                x1: "63.17",
                                x2: "120.43",
                                y1: "455.38",
                                y2: "464.45",
                                stop {
                                    offset: "0",
                                    stop_color: "#fff",
                                    stop_opacity: "0",
                                }
                                stop {
                                    offset: ".15",
                                    stop_color: "#fff",
                                    stop_opacity: "0",
                                }
                                stop {
                                    offset: ".2",
                                    stop_color: "#fff",
                                    stop_opacity: ".03",
                                }
                                stop {
                                    offset: ".24",
                                    stop_color: "#fff",
                                    stop_opacity: ".08",
                                }
                                stop {
                                    offset: ".27",
                                    stop_color: "#fff",
                                    stop_opacity: ".15",
                                }
                                stop {
                                    offset: ".29",
                                    stop_color: "#fff",
                                    stop_opacity: ".23",
                                }
                                stop {
                                    offset: ".32",
                                    stop_color: "#fff",
                                    stop_opacity: ".34",
                                }
                                stop {
                                    offset: ".34",
                                    stop_color: "#fff",
                                    stop_opacity: ".47",
                                }
                                stop {
                                    offset: ".36",
                                    stop_color: "#fff",
                                    stop_opacity: ".61",
                                }
                                stop {
                                    offset: ".38",
                                    stop_color: "#fff",
                                    stop_opacity: ".78",
                                }
                                stop {
                                    offset: ".38",
                                    stop_color: "#fff",
                                    stop_opacity: ".8",
                                }
                                stop {
                                    offset: ".39",
                                    stop_color: "#fff",
                                    stop_opacity: "0",
                                }
                                stop { offset: ".72", stop_color: "#d3d3d3" }
                                stop { offset: "1", stop_color: "#d2d1d2" }
                            }
                            linearGradient {
                                href: "#r",
                                id: "t",
                                x1: "72.87",
                                x2: "115.35",
                                y1: "87.78",
                                y2: "91.43",
                            }
                            linearGradient {
                                gradient_units: "userSpaceOnUse",
                                id: "v",
                                x1: "102.87",
                                x2: "194.16",
                                y1: "217.5",
                                y2: "217.5",
                                stop { offset: "0", stop_color: "#d68029" }
                                stop { offset: "1", stop_color: "#b3511b" }
                            }
                            linearGradient {
                                gradient_units: "userSpaceOnUse",
                                id: "w",
                                x1: "134.44",
                                x2: "158.77",
                                y1: "23.53",
                                y2: "404.51",
                                stop {
                                    offset: ".02",
                                    stop_color: "#dd884e",
                                    stop_opacity: ".4",
                                }
                                stop {
                                    offset: ".42",
                                    stop_color: "#b76031",
                                    stop_opacity: ".67",
                                }
                                stop { offset: "1", stop_color: "#882e0d" }
                            }
                            linearGradient {
                                gradient_units: "userSpaceOnUse",
                                id: "x",
                                x1: "102.66",
                                x2: "194.16",
                                y1: "162",
                                y2: "162",
                                stop { offset: "0", stop_color: "#d8823a" }
                                stop { offset: "1", stop_color: "#bf611e" }
                            }
                            radialGradient {
                                cx: "122.52",
                                cy: "64.81",
                                fx: "60.36",
                                fy: "71.95",
                                gradient_transform: "translate(119.12 192.29) rotate(-104.58) scale(1 .55)",
                                gradient_units: "userSpaceOnUse",
                                id: "y",
                                r: "134.05",
                                stop {
                                    offset: ".72",
                                    stop_color: "#dd884e",
                                    stop_opacity: "0",
                                }
                                stop { offset: "1", stop_color: "#ed9654" }
                            }
                            radialGradient {
                                cx: "130.39",
                                cy: "65.62",
                                fx: "68.57",
                                fy: "72.72",
                                gradient_transform: "translate(176.37 200.83) rotate(-75.42) scale(1 -.55)",
                                href: "#y",
                                id: "z",
                                r: "133.29",
                            }
                            linearGradient {
                                gradient_units: "userSpaceOnUse",
                                id: "aa",
                                x1: "161.35",
                                x2: "142.83",
                                y1: "438.65",
                                y2: "217.56",
                                stop { offset: "0", stop_color: "#a52c14" }
                                stop {
                                    offset: ".19",
                                    stop_color: "#a53014",
                                    stop_opacity: ".84",
                                }
                                stop {
                                    offset: ".59",
                                    stop_color: "#a73b14",
                                    stop_opacity: ".44",
                                }
                                stop {
                                    offset: "1",
                                    stop_color: "#aa4814",
                                    stop_opacity: "0",
                                }
                            }
                            linearGradient {
                                gradient_transform: "translate(189.08 .72) rotate(.18) scale(1.34 1)",
                                gradient_units: "userSpaceOnUse",
                                id: "ab",
                                x1: "-43.8",
                                x2: "-7.1",
                                y1: "284.22",
                                y2: "285.84",
                                stop {
                                    offset: "0",
                                    stop_color: "#e65900",
                                    stop_opacity: "0",
                                }
                                stop {
                                    offset: ".13",
                                    stop_color: "#e45800",
                                    stop_opacity: ".03",
                                }
                                stop {
                                    offset: ".29",
                                    stop_color: "#de5502",
                                    stop_opacity: ".12",
                                }
                                stop {
                                    offset: ".46",
                                    stop_color: "#d55104",
                                    stop_opacity: ".27",
                                }
                                stop {
                                    offset: ".65",
                                    stop_color: "#c84b08",
                                    stop_opacity: ".48",
                                }
                                stop {
                                    offset: ".84",
                                    stop_color: "#b7430c",
                                    stop_opacity: ".75",
                                }
                                stop { offset: "1", stop_color: "#a83c11" }
                            }
                            linearGradient {
                                href: "#d",
                                id: "ac",
                                x1: "-65.46",
                                x2: "-38.94",
                                y1: "586.55",
                                y2: "591.23",
                            }
                            linearGradient {
                                href: "#e",
                                id: "ad",
                                x1: "180.14",
                                x2: "222.95",
                                y1: "532.12",
                                y2: "535.79",
                            }
                            linearGradient {
                                gradient_units: "userSpaceOnUse",
                                id: "ae",
                                x1: "175.32",
                                x2: "233.47",
                                y1: "326.03",
                                y2: "327.04",
                                stop { offset: "0", stop_color: "#e5e5e5" }
                                stop { offset: ".04", stop_color: "#e5e5e5" }
                                stop { offset: ".05", stop_color: "#e5e5e5" }
                                stop { offset: ".63", stop_color: "#e3e3e3" }
                                stop { offset: ".86", stop_color: "#dcdcdc" }
                                stop { offset: "1", stop_color: "#d3d2d3" }
                            }
                            linearGradient {
                                href: "#g",
                                id: "af",
                                x1: "180.42",
                                x2: "222.03",
                                y1: "463.91",
                                y2: "463.91",
                            }
                            linearGradient {
                                href: "#g",
                                id: "ag",
                                x1: "180.42",
                                x2: "222.03",
                                y1: "471.51",
                                y2: "471.51",
                            }
                            linearGradient {
                                href: "#g",
                                id: "ah",
                                x1: "180.42",
                                x2: "222.03",
                                y1: "480.59",
                                y2: "480.59",
                            }
                            linearGradient {
                                href: "#g",
                                id: "ai",
                                x1: "180.42",
                                x2: "222.03",
                                y1: "492.4",
                                y2: "492.4",
                            }
                            linearGradient {
                                gradient_units: "userSpaceOnUse",
                                id: "aj",
                                x1: "180.43",
                                x2: "222.04",
                                y1: "385.97",
                                y2: "385.97",
                                stop { offset: "0", stop_color: "#d45300" }
                                stop { offset: "0", stop_color: "#d35200" }
                                stop { offset: ".07", stop_color: "#995021" }
                                stop { offset: ".13", stop_color: "#704e38" }
                                stop { offset: ".18", stop_color: "#564d47" }
                                stop { offset: ".2", stop_color: "#4d4d4d" }
                                stop { offset: ".3", stop_color: "#474747" }
                                stop { offset: ".69", stop_color: "#383838" }
                                stop { offset: "1", stop_color: "#333" }
                            }
                            linearGradient {
                                href: "#aj",
                                id: "ak",
                                y1: "443.6",
                                y2: "443.6",
                            }
                            linearGradient {
                                href: "#aj",
                                id: "al",
                                y1: "310.75",
                                y2: "310.75",
                            }
                            linearGradient {
                                href: "#aj",
                                id: "am",
                                y1: "235.44",
                                y2: "235.44",
                            }
                            linearGradient {
                                href: "#aj",
                                id: "an",
                                y1: "198.78",
                                y2: "198.78",
                            }
                            linearGradient {
                                href: "#aj",
                                id: "ao",
                                y1: "171.7",
                                y2: "171.7",
                            }
                            linearGradient {
                                href: "#g",
                                id: "ap",
                                x1: "180.42",
                                x2: "222.03",
                                y1: "503.1",
                                y2: "503.1",
                            }
                            linearGradient {
                                href: "#r",
                                id: "aq",
                                x1: "180.25",
                                x2: "222.03",
                                y1: "337.31",
                                y2: "337.31",
                            }
                            linearGradient {
                                href: "#s",
                                id: "ar",
                                x1: "172.75",
                                x2: "230.01",
                                y1: "512.1",
                                y2: "521.17",
                            }
                            linearGradient {
                                href: "#r",
                                id: "as",
                                x1: "182.45",
                                x2: "224.93",
                                y1: "144.49",
                                y2: "148.14",
                            }
                            linearGradient {
                                gradient_units: "userSpaceOnUse",
                                id: "at",
                                x1: "33.58",
                                x2: "68.43",
                                y1: "414.65",
                                y2: "424",
                                stop { offset: "0", stop_color: "#e8e8e8" }
                                stop { offset: ".02", stop_color: "#f2f2f2" }
                                stop { offset: ".04", stop_color: "#fafafa" }
                                stop { offset: ".09", stop_color: "#fdfdfd" }
                                stop { offset: ".18", stop_color: "#ededed" }
                                stop { offset: ".38", stop_color: "#d3d3d3" }
                                stop { offset: ".58", stop_color: "silver" }
                                stop { offset: ".78", stop_color: "#b5b5b5" }
                                stop { offset: "1", stop_color: "#b2b2b2" }
                            }
                            linearGradient {
                                gradient_transform: "matrix(1, 0, 0, 1, 0, 0)",
                                href: "#d",
                                id: "au",
                                x1: "84.78",
                                x2: "111.3",
                                y1: "461.9",
                                y2: "466.58",
                            }
                            linearGradient {
                                gradient_transform: "translate(108.68 -1.95) rotate(13.28)",
                                href: "#d",
                                id: "av",
                                x1: "49.55",
                                x2: "78.06",
                                y1: "459.96",
                                y2: "464.99",
                            }
                            linearGradient {
                                gradient_transform: "translate(37.21 -2.14) rotate(4.64)",
                                href: "#d",
                                id: "aw",
                                x1: "70.1",
                                x2: "84.28",
                                y1: "465.78",
                                y2: "468.28",
                            }
                            linearGradient {
                                gradient_units: "userSpaceOnUse",
                                id: "ax",
                                x1: "17.4",
                                x2: "185.62",
                                y1: "317.69",
                                y2: "317.69",
                                stop { offset: "0", stop_color: "#d5d5d5" }
                                stop { offset: "1", stop_color: "#d1d1d1" }
                            }
                            filter { filterUnits: "userSpaceOnUse", id: "ay",
                                feOffset { dx: "13", dy: "-6" }
                                feGaussianBlur { result: "az", std_deviation: "6" }
                                feFlood { flood_color: "#333", flood_opacity: ".3" }
                                feComposite { in2: "az", operator: "in" }
                                feComposite { "in": "SourceGraphic" }
                            }
                            linearGradient {
                                gradient_units: "userSpaceOnUse",
                                id: "ba",
                                x1: "52.2",
                                x2: "131.64",
                                y1: "302.93",
                                y2: "307.1",
                                stop { offset: "0", stop_color: "#fff" }
                                stop { offset: ".06", stop_color: "#f9f9f9" }
                                stop { offset: ".18", stop_color: "#f5f5f5" }
                                stop { offset: ".37", stop_color: "#c7c7c7" }
                                stop { offset: ".55", stop_color: "#c3c3c3" }
                                stop { offset: ".7", stop_color: "#b9b9b9" }
                                stop { offset: ".75", stop_color: "#b4b4b4" }
                                stop { offset: ".94", stop_color: "#d3d3d3" }
                            }
                            linearGradient {
                                gradient_units: "userSpaceOnUse",
                                id: "bb",
                                x1: "89.33",
                                x2: "88.01",
                                y1: "180.46",
                                y2: "427.46",
                                stop { offset: "0", stop_color: "#fff" }
                                stop {
                                    offset: ".07",
                                    stop_color: "#f7f8f8",
                                    stop_opacity: ".85",
                                }
                                stop {
                                    offset: ".16",
                                    stop_color: "#eeefef",
                                    stop_opacity: ".65",
                                }
                                stop {
                                    offset: ".26",
                                    stop_color: "#e5e7e7",
                                    stop_opacity: ".48",
                                }
                                stop {
                                    offset: ".37",
                                    stop_color: "#dee0e1",
                                    stop_opacity: ".33",
                                }
                                stop {
                                    offset: ".47",
                                    stop_color: "#d9dbdc",
                                    stop_opacity: ".21",
                                }
                                stop {
                                    offset: ".58",
                                    stop_color: "#d4d7d8",
                                    stop_opacity: ".12",
                                }
                                stop {
                                    offset: ".7",
                                    stop_color: "#d1d4d5",
                                    stop_opacity: ".05",
                                }
                                stop {
                                    offset: ".83",
                                    stop_color: "#cfd2d3",
                                    stop_opacity: ".01",
                                }
                                stop {
                                    offset: "1",
                                    stop_color: "#cfd2d3",
                                    stop_opacity: "0",
                                }
                            }
                            linearGradient {
                                gradient_units: "userSpaceOnUse",
                                id: "bc",
                                x1: "-.05",
                                x2: "69.93",
                                y1: "523.99",
                                y2: "413.58",
                                stop { offset: "0", stop_color: "#b8b7b7" }
                                stop { offset: ".4", stop_color: "#d1d0d0" }
                                stop { offset: "1", stop_color: "#fff" }
                            }
                            linearGradient {
                                href: "#bc",
                                id: "bd",
                                x1: "19.81",
                                x2: "44.69",
                                y1: "505.35",
                                y2: "466.09",
                            }
                            radialGradient {
                                cx: "66.91",
                                cy: "217.89",
                                fx: "66.91",
                                fy: "217.89",
                                gradient_transform: "translate(261.63 273.14) rotate(125.14) scale(1 .88)",
                                gradient_units: "userSpaceOnUse",
                                id: "be",
                                r: "39.33",
                                stop { offset: "0", stop_color: "#565656" }
                                stop { offset: ".1", stop_color: "#484848" }
                                stop { offset: ".3", stop_color: "#323232" }
                                stop { offset: ".42", stop_color: "#2b2b2b" }
                                stop { offset: ".51", stop_color: "#2b2b2b" }
                                stop { offset: "1", stop_color: "#2b2b2b" }
                            }
                            linearGradient {
                                gradient_units: "userSpaceOnUse",
                                id: "bf",
                                x1: "129.28",
                                x2: "126.25",
                                y1: "374.95",
                                y2: "380.41",
                                stop { offset: "0", stop_color: "#fff" }
                                stop { offset: ".27", stop_color: "#fff" }
                                stop { offset: ".32", stop_color: "#fff" }
                                stop { offset: ".33", stop_color: "#fdfdfe" }
                                stop { offset: ".33", stop_color: "#f6f8fd" }
                                stop { offset: ".33", stop_color: "#eaeffc" }
                                stop { offset: ".33", stop_color: "#d9e3f9" }
                                stop { offset: ".34", stop_color: "#c2d2f6" }
                                stop { offset: ".34", stop_color: "#a7bef3" }
                                stop { offset: ".34", stop_color: "#86a6ee" }
                                stop { offset: ".34", stop_color: "#628ae9" }
                                stop { offset: ".34", stop_color: "#386ce4" }
                                stop { offset: ".34", stop_color: "#376be4" }
                                stop { offset: ".68", stop_color: "#3d6ade" }
                                stop { offset: ".69", stop_color: "#dd4f38" }
                                stop { offset: "1", stop_color: "#ff4915" }
                            }
                            linearGradient {
                                gradient_units: "userSpaceOnUse",
                                id: "bg",
                                x1: "65.52",
                                x2: "97.61",
                                y1: "437.49",
                                y2: "440.89",
                                stop { offset: "0", stop_color: "#e8e8e8" }
                                stop { offset: ".05", stop_color: "#f2f2f2" }
                                stop { offset: ".13", stop_color: "#fafafa" }
                                stop { offset: ".27", stop_color: "#fdfdfd" }
                                stop { offset: ".36", stop_color: "#f2f2f2" }
                                stop { offset: ".78", stop_color: "#c3c3c3" }
                                stop { offset: "1", stop_color: "#b2b2b2" }
                            }
                            linearGradient {
                                gradient_units: "userSpaceOnUse",
                                id: "bh",
                                x1: "77.37",
                                x2: "136.86",
                                y1: "176.68",
                                y2: "198.33",
                                stop { offset: "0", stop_color: "#0e0a09" }
                                stop { offset: "0", stop_color: "#0e0a09" }
                                stop { offset: ".08", stop_color: "#2a2828" }
                                stop { offset: ".14", stop_color: "#3b3b3b" }
                                stop { offset: ".19", stop_color: "#424242" }
                                stop { offset: ".26", stop_color: "#303030" }
                                stop { offset: ".37", stop_color: "#090909" }
                                stop { offset: ".53", stop_color: "#0c0b09" }
                                stop { offset: ".66", stop_color: "#16110c" }
                                stop { offset: ".78", stop_color: "#261c11" }
                                stop { offset: ".89", stop_color: "#3e2b18" }
                                stop { offset: ".99", stop_color: "#5c3f21" }
                                stop { offset: "1", stop_color: "#604122" }
                            }
                            linearGradient {
                                gradient_units: "userSpaceOnUse",
                                id: "bi",
                                x1: "79.7",
                                x2: "81.11",
                                y1: "411.38",
                                y2: "443.62",
                                stop {
                                    offset: "0",
                                    stop_color: "#fff",
                                    stop_opacity: ".5",
                                }
                                stop {
                                    offset: ".07",
                                    stop_color: "#fefefe",
                                    stop_opacity: ".38",
                                }
                                stop {
                                    offset: ".17",
                                    stop_color: "#fefefe",
                                    stop_opacity: ".27",
                                }
                                stop {
                                    offset: ".27",
                                    stop_color: "#fdfdfd",
                                    stop_opacity: ".17",
                                }
                                stop {
                                    offset: ".38",
                                    stop_color: "#fdfdfd",
                                    stop_opacity: ".09",
                                }
                                stop {
                                    offset: ".52",
                                    stop_color: "#fdfdfd",
                                    stop_opacity: ".04",
                                }
                                stop {
                                    offset: ".68",
                                    stop_color: "#fdfdfd",
                                    stop_opacity: "0",
                                }
                                stop {
                                    offset: "1",
                                    stop_color: "#fdfdfd",
                                    stop_opacity: "0",
                                }
                            }
                            linearGradient {
                                gradient_transform: "translate(-8.09 4.61) rotate(-2.79)",
                                gradient_units: "userSpaceOnUse",
                                id: "bj",
                                x1: "94.5",
                                x2: "91.3",
                                y1: "162.16",
                                y2: "170.77",
                                stop { offset: ".13", stop_color: "#3f3f3f" }
                                stop { offset: "1", stop_color: "#090909" }
                            }
                            linearGradient {
                                gradient_transform: "translate(31.76 -5.44) rotate(5.03) scale(.81 1)",
                                href: "#bj",
                                id: "bk",
                                x1: "89.54",
                                x2: "86.96",
                                y1: "164.53",
                                y2: "171.47",
                            }
                            linearGradient {
                                gradient_transform: "translate(-16.29 4.64) rotate(-2.49) scale(1.1 1)",
                                href: "#bj",
                                id: "bl",
                                x1: "96.87",
                                x2: "94.31",
                                y1: "166.02",
                                y2: "172.9",
                            }
                        }
                        g { id: "c",
                            g {
                                path {
                                    d: "M103.28,444.18l8.98,31.12s-3.3,10.51-20.97,10.46c-17.68-.05-21.18-10.93-21.18-10.93l9.38-29.9,23.8-.74Z",
                                    style: "fill:url(#d);",
                                }
                                path {
                                    d: "M91.32,474.9c16.6,0,20.97-4.48,20.97-4.48v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,4.48,20.64,4.48Z",
                                    style: "fill:url(#e);",
                                }
                                path {
                                    d: "M70.84,127.87s-.02-17.57,0-20.15,14.01-38.61,16.18-44.26,6.7-5.39,8.75,0,16.68,41.99,16.68,43.93-.04,330.65,0,333.41,8.09,17.79,8.09,19.32,1.69,20.06-29.06,20.06-29.06-18.5-29.06-20.06,8.13-16.64,8.26-19.32,.17-312.94,.17-312.94Z",
                                    style: "fill:url(#f);",
                                }
                                g {
                                    path {
                                        d: "M91.48,119.36c16.6,0,20.97-11.97,20.97-11.97v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,11.97,20.64,11.97Z",
                                        style: "fill:url(#g);",
                                    }
                                    path {
                                        d: "M91.48,146.44c16.6,0,20.97-11.97,20.97-11.97v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,11.97,20.64,11.97Z",
                                        style: "fill:url(#h);",
                                    }
                                    path {
                                        d: "M91.48,183.1c16.6,0,20.97-11.97,20.97-11.97v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,11.97,20.64,11.97Z",
                                        style: "fill:url(#i);",
                                    }
                                    path {
                                        d: "M91.48,258.41c16.6,0,20.97-11.97,20.97-11.97v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,11.97,20.64,11.97Z",
                                        style: "fill:url(#j);",
                                    }
                                    path {
                                        d: "M91.48,333.63c16.6,0,20.97-11.97,20.97-11.97v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,11.97,20.64,11.97Z",
                                        style: "fill:url(#k);",
                                    }
                                    path {
                                        d: "M91.48,391.26c16.6,0,20.97-11.97,20.97-11.97v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,11.97,20.64,11.97Z",
                                        style: "fill:url(#l);",
                                    }
                                    path {
                                        d: "M91.48,411.57c16.6,0,20.97-11.97,20.97-11.97v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,11.97,20.64,11.97Z",
                                        style: "fill:url(#m);",
                                    }
                                    path {
                                        d: "M91.48,419.17c16.6,0,20.97-11.97,20.97-11.97v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,11.97,20.64,11.97Z",
                                        style: "fill:url(#n);",
                                    }
                                    path {
                                        d: "M91.48,428.25c16.6,0,20.97-11.97,20.97-11.97v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,11.97,20.64,11.97Z",
                                        style: "fill:url(#o);",
                                    }
                                    path {
                                        d: "M91.48,440.06c16.6,0,20.97-11.97,20.97-11.97v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,11.97,20.64,11.97Z",
                                        style: "fill:url(#p);",
                                    }
                                    path {
                                        d: "M91.48,450.71c16.6,0,20.97-11.97,20.97-11.97v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,11.97,20.64,11.97Z",
                                        style: "fill:url(#q);",
                                    }
                                }
                                path {
                                    d: "M70.84,127.87s-.02-17.57,0-20.15c.01-1.29,2.64,11.56,20.81,11.56s20.81-12.86,20.81-11.89c0,1.94-.04,330.65,0,333.41,.02,1.36-3.06,13.05-20.97,13.05s-20.87-11.69-20.81-13.05c.12-2.68,.17-312.94,.17-312.94Z",
                                    style: "fill:url(#r);",
                                }
                                path {
                                    d: "M91.48,480.19c32.61,0,29.06-20.06,29.06-20.06l-8.1-18.08s-2.39,11.81-20.97,11.81-20.81-13.05-20.81-13.05l-8.26,19.32s-3.55,20.06,29.06,20.06Z",
                                    style: "fill:url(#s);",
                                }
                                path {
                                    d: "M70.84,107.72s4.18,11.56,20.81,11.56,20.81-11.89,20.81-11.89c0,0-14.86-40.33-17.12-44.93s-6.38-3.6-7.87,0-16.63,45.27-16.63,45.27Z",
                                    style: "fill:url(#t);",
                                }
                            }
                            g { id: "u",
                                path {
                                    d: "M131.16,17c2.99-4.19,17-17,18-17s12.08,8.58,17.78,17c14.22,21,27.22,38,27.22,62V411c0,33-92,31-91,0s-1-299,0-329,23-58,28-65Z",
                                    style: "fill:url(#v);",
                                }
                                path {
                                    d: "M131.05,17c2.99-4.19,17-17,18-17s12.08,8.58,17.78,17c14.22,21,27.22,38,27.22,62V411c0,33-92,31-91,0s-1-299,0-329,23-58,28-65Z",
                                    style: "fill:url(#w); opacity:.22;",
                                }
                                path {
                                    d: "M102.66,117.5s6,24,45.5,24,46-24,46-24l-.5,74s-12.5,15-45,15-46-15-46-15V117.5Z",
                                    style: "fill:url(#x);",
                                }
                                path {
                                    d: "M149.05,0s9.4,5.19,17.78,17,27.22,35.73,27.22,62,.11,38.5,.11,38.5c0,0-1.15,6.16-9.28,12.79-6.39,5.2-16.71,11.21-36.72,11.21-45.5,0,.89-141.5,.89-141.5Z",
                                    style: "fill:url(#y);",
                                }
                                path {
                                    d: "M147.97,1.5s-9.4,5.19-17.78,17c-8.38,11.81-27.22,35.73-27.22,62,0,26.27-.11,38.5-.11,38.5,0,0,1.84,8.63,15.38,15.74,6.82,3.58,15.4,6.75,30.63,6.75,45.5,0-.9-140-.9-140Z",
                                    style: "fill:url(#z);",
                                }
                                path {
                                    d: "M105.66,209.83l-2.46,202.5s.23,32.96,46.76,33.93c43.35,.91,46.66-21.97,46.66-21.97l.11-231.42-91.07,16.96Z",
                                    style: "fill:url(#aa);",
                                }
                                path {
                                    d: "M193.63,138.78s-2.38,275.86-2.31,278.63c.03,1.35-9.7,16.7-33.71,16.62s-33.8-23.73-33.71-25.09c.14-2.12-14.71-164.59,11.97-234.48,7.01-18.36,41.9,7.85,57.76-35.68Z",
                                    style: "fill:url(#ab);",
                                }
                            }
                            g {
                                path {
                                    d: "M212.86,500.89l8.98,31.12s-3.3,10.51-20.97,10.46c-17.68-.05-21.18-10.93-21.18-10.93l9.38-29.9,23.8-.74Z",
                                    style: "fill:url(#ac);",
                                }
                                path {
                                    d: "M200.9,531.62c16.6,0,20.97-4.48,20.97-4.48v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,4.48,20.64,4.48Z",
                                    style: "fill:url(#ad);",
                                }
                                path {
                                    d: "M180.42,184.58s-.02-17.57,0-20.15,14.01-38.61,16.18-44.26,6.7-5.39,8.75,0c2.05,5.39,16.68,41.99,16.68,43.93s-.04,330.65,0,333.41,8.09,17.79,8.09,19.32,1.69,20.06-29.06,20.06-29.06-18.5-29.06-20.06,8.13-16.64,8.26-19.32,.17-312.94,.17-312.94Z",
                                    style: "fill:url(#ae);",
                                }
                                g {
                                    path {
                                        d: "M201.06,468.29c16.6,0,20.97-11.97,20.97-11.97v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,11.97,20.64,11.97Z",
                                        style: "fill:url(#af);",
                                    }
                                    path {
                                        d: "M201.06,475.89c16.6,0,20.97-11.97,20.97-11.97v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,11.97,20.64,11.97Z",
                                        style: "fill:url(#ag);",
                                    }
                                    path {
                                        d: "M201.06,484.97c16.6,0,20.97-11.97,20.97-11.97v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,11.97,20.64,11.97Z",
                                        style: "fill:url(#ah);",
                                    }
                                    path {
                                        d: "M201.06,496.78c16.6,0,20.97-11.97,20.97-11.97v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,11.97,20.64,11.97Z",
                                        style: "fill:url(#ai);",
                                    }
                                    path {
                                        d: "M201.07,390.34c16.6,0,20.97-11.97,20.97-11.97v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,11.97,20.64,11.97Z",
                                        style: "fill:url(#aj);",
                                    }
                                    path {
                                        d: "M201.07,447.98c16.6,0,20.97-11.97,20.97-11.97v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,11.97,20.64,11.97Z",
                                        style: "fill:url(#ak);",
                                    }
                                    path {
                                        d: "M201.07,315.12c16.6,0,20.97-11.97,20.97-11.97v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,11.97,20.64,11.97Z",
                                        style: "fill:url(#al);",
                                    }
                                    path {
                                        d: "M201.07,239.82c16.6,0,20.97-11.97,20.97-11.97v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,11.97,20.64,11.97Z",
                                        style: "fill:url(#am);",
                                    }
                                    path {
                                        d: "M201.07,203.16c16.6,0,20.97-11.97,20.97-11.97v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,11.97,20.64,11.97Z",
                                        style: "fill:url(#an);",
                                    }
                                    path {
                                        d: "M201.07,176.08c16.6,0,20.97-11.97,20.97-11.97v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,11.97,20.64,11.97Z",
                                        style: "fill:url(#ao);",
                                    }
                                    path {
                                        d: "M201.06,507.47c16.6,0,20.97-11.97,20.97-11.97v3.39s-3.88,11.81-20.97,11.81-20.64-11.23-20.64-11.23v-3.96s4.05,11.97,20.64,11.97Z",
                                        style: "fill:url(#ap);",
                                    }
                                }
                                path {
                                    d: "M180.42,184.58s-.02-17.57,0-20.15c.01-1.29,2.64,11.56,20.81,11.56s20.81-12.86,20.81-11.89c0,1.94-.04,330.65,0,333.41,.02,1.36-3.06,13.05-20.97,13.05s-20.87-11.69-20.81-13.05c.12-2.68,.17-312.94,.17-312.94Z",
                                    style: "fill:url(#aq);",
                                }
                                path {
                                    d: "M201.06,536.9c32.61,0,29.06-20.06,29.06-20.06l-8.1-18.08s-2.39,11.81-20.97,11.81-20.81-13.05-20.81-13.05l-8.26,19.32s-3.55,20.06,29.06,20.06Z",
                                    style: "fill:url(#ar);",
                                }
                                path {
                                    d: "M180.42,164.44s4.18,11.56,20.81,11.56,20.81-11.89,20.81-11.89c0,0-14.86-40.33-17.12-44.93s-6.38-3.6-7.87,0-16.63,45.27-16.63,45.27Z",
                                    style: "fill:url(#as);",
                                }
                            }
                            g {
                                path {
                                    d: "M32.69,439.07c-.14,2.86-2.97,10.79,15.3,12.66,17.94,1.43,19.6-8.7,19.6-8.7,0,0,3.28-21.39,3.85-27.96,.39-4.44-.33-10.13-1.07-16.32-.71-2.13-18.48-13.36-19.51-13.29-7.93,.55-10.9,6.5-12.66,10.9-1.45,3.62-5.38,39.85-5.51,42.72Z",
                                    style: "fill:url(#at);",
                                }
                                path {
                                    d: "M106.84,454.41l3.26,19.01s-2.53,3.14-13.76,1.82-12.99-5.17-12.99-5.17l8.31-16.95,15.18,1.3Z",
                                    style: "fill:url(#au);",
                                }
                                path {
                                    d: "M76.16,454.7l-1.28,20.7s-3.42,2.66-14.85-1.5-12.31-8.62-12.31-8.62l12.88-15.68,15.56,5.11Z",
                                    style: "fill:url(#av);",
                                }
                                path {
                                    d: "M81.49,464.75l.92,10.27s-1.48,1.56-7.41,.37c-5.93-1.19-6.7-3.32-6.7-3.32l5.16-8.67,8.03,1.35Z",
                                    style: "fill:url(#aw);",
                                }
                                path {
                                    d: "M185.62,458.89l-94.24-40.73L18.5,351.66l-1.1-9.25,5.94-20.7,39.85-24,17.39-81.03s3.55-39.08,22.06-40.18,23.1,49.89,23.1,49.89l14.51,118.44,40.51,77.5,4.84,24.44v12.11Z",
                                    style: "fill:url(#ax); filter:url(#ay); stroke:#2b2b2b; stroke-miterlimit:10; stroke-width:2px;",
                                }
                                line {
                                    style: "fill:none; stroke:#2b2b2b; stroke-miterlimit:10; stroke-width:.75px;",
                                    x1: "183.2",
                                    x2: "126.39",
                                    y1: "448.76",
                                    y2: "426.74",
                                }
                                line {
                                    style: "fill:none; stroke:#2b2b2b; stroke-miterlimit:10; stroke-width:.75px;",
                                    x1: "51.09",
                                    x2: "17.4",
                                    y1: "368.13",
                                    y2: "342.41",
                                }
                                text {
                                    style: "fill:#333; font-size:12.27px;",
                                    transform: "translate(27.32 341.67) rotate(34.86) scale(1.05 .83) skewX(27.74)",
                                    tspan {
                                        style: "font-family:Bahnschrift-Bold, Bahnschrift; font-variation-settings:'wght' 700, 'wdth' 100; font-weight:700; letter-spacing:0em;",
                                        x: "0",
                                        y: "0",
                                        "U"
                                    }
                                    tspan {
                                        style: "font-family:Bahnschrift-Bold, Bahnschrift; font-variation-settings:'wght' 700, 'wdth' 100; font-weight:700;",
                                        x: "7.92",
                                        y: "0",
                                        "I"
                                    }
                                    tspan {
                                        style: "font-family:Bahnschrift, Bahnschrift; font-variation-settings:'wght' 400, 'wdth' 100; letter-spacing:0em;",
                                        x: "11.3",
                                        y: "0",
                                        "V"
                                    }
                                    tspan {
                                        style: "font-family:Bahnschrift, Bahnschrift; font-variation-settings:'wght' 400, 'wdth' 100;",
                                        x: "18.56",
                                        y: "0",
                                        "E"
                                    }
                                    tspan {
                                        style: "font-family:Bahnschrift, Bahnschrift; font-variation-settings:'wght' 400, 'wdth' 100; letter-spacing:-.02em;",
                                        x: "25.89",
                                        y: "0",
                                        "R"
                                    }
                                    tspan {
                                        style: "font-family:Bahnschrift, Bahnschrift; font-variation-settings:'wght' 400, 'wdth' 100; letter-spacing:0em;",
                                        x: "33.65",
                                        y: "0",
                                        "S"
                                    }
                                    tspan {
                                        style: "font-family:Bahnschrift, Bahnschrift; font-variation-settings:'wght' 400, 'wdth' 100;",
                                        x: "41.08",
                                        y: "0",
                                        "E"
                                    }
                                }
                                path {
                                    d: "M123.31,450.69s-12.22,5.01-20.31,5.45-27.69-1.87-36.99-3.63-18.17-7.27-18.17-7.27l-1.49-30.72,2.31-34.68s6.39-114.92,6.88-121.36,.41-18.29,1.21-25.12,8.31-31.4,9.06-32.72,3.66-3.44,3.66-3.44l9.08-22.79s1.32-1.93,7.27-9.41c3.07-3.87,8.37-10.11,13.21-10.4,4.52-.27,8.82,5.09,10.4,7.6,2.52,4,3.28,5.35,4.36,7.99s11.06,32.96,11.99,43.03,4.46,120.55,4.46,120.55l-6.94,116.92Z",
                                    style: "fill:url(#ba);",
                                }
                                path {
                                    d: "M123.31,450.69s-12.22,5.01-20.31,5.45c-8.09,.44-27.69-1.87-36.99-3.63s-18.17-7.27-18.17-7.27l-1.49-30.72,2.31-34.68s6.39-114.92,6.88-121.36,.41-18.29,1.21-25.12,8.31-31.4,9.06-32.72,3.66-3.44,3.66-3.44l9.08-22.79s1.32-1.93,7.27-9.41c3.07-3.87,8.37-10.11,13.21-10.4,4.52-.27,8.82,5.09,10.4,7.6,2.52,4,3.28,5.35,4.36,7.99s11.06,32.96,11.99,43.03,4.46,120.55,4.46,120.55l-6.94,116.92Z",
                                    style: "fill:url(#bb);",
                                }
                                polygon {
                                    points: "56.81 407.75 49.88 415.51 1.82 501.05 .5 504.69 .5 524.34 51.53 472.32 58.96 448.54 61.77 408.41 56.81 407.75",
                                    style: "fill:url(#bc); stroke:#4c4c4c; stroke-miterlimit:10;",
                                }
                                polygon {
                                    points: "48.05 475.95 51.28 458.33 52.01 454.37 51.64 456.37 14.76 500.65 9.73 506.09 8.49 516.23 9.9 515.12 48.05 475.95",
                                    style: "fill:url(#bd); stroke:#5b5b5b; stroke-miterlimit:10;",
                                }
                                path {
                                    d: "M78.06,232.21c1.17,1.37-.73,7.04,.83,8.64,3.18,3.27,9.84,2.28,9.98,1.98,3.28-6.94,1.14-17.53-1.04-22.1s-7.68-9.41-7.95-9.55-3.12-3.98-3.3-4.07c-.5-.25-3.22,.25-3.22,.25,0,0-7.14,3.92-8.02,4.71-4.79,4.29-6.89,12.07-6.32,11.13,.51-.84,6.87-3.31,10.71-2.96,.76,.07,6.91,10.31,8.34,11.97Z",
                                    style: "fill:url(#be);",
                                }
                                path {
                                    d: "M78.27,248.23c.82-.21,3.44,1.21,6.65,1.57,4.12,.46,9.01-.06,9.99,.21,3.56,.95,3.3,8.34-.91,10.07-.87,.36-5.26,1-9.29,.7-3.3-.25-6.42-1.71-7.27-2.39-2.57-2.09-1.76-9.49,.83-10.16Z",
                                    style: "fill:none; stroke:#777574; stroke-miterlimit:10; stroke-width:.75px;",
                                }
                                text {
                                    style: "fill:#666; font-family:Bahnschrift, Bahnschrift; font-size:5.69px; font-variation-settings:'wght' 400, 'wdth' 100;",
                                    transform: "translate(121.16 383.28) rotate(28.29) scale(1.26 .89) skewX(19.94)",
                                    tspan { x: "0", y: "0", "@ken" }
                                    tspan {
                                        style: "letter-spacing:-.01em;",
                                        x: "14.3",
                                        y: "0",
                                        "n"
                                    }
                                    tspan {
                                        style: "letter-spacing:0em;",
                                        x: "17.4",
                                        y: "0",
                                        "y"
                                    }
                                    tspan {
                                        style: "letter-spacing:0em;",
                                        x: "20.09",
                                        y: "0",
                                        "otsu"
                                    }
                                }
                                text {
                                    style: "fill:#333; font-size:12.31px;",
                                    transform: "translate(120.21 399.04) rotate(28.29) scale(.98 .89) skewX(24.96)",
                                    tspan {
                                        style: "font-family:Bahnschrift-Bold, Bahnschrift; font-variation-settings:'wght' 700, 'wdth' 100; font-weight:700; letter-spacing:0em;",
                                        x: "0",
                                        y: "0",
                                        "U"
                                    }
                                    tspan {
                                        style: "font-family:Bahnschrift-Bold, Bahnschrift; font-variation-settings:'wght' 700, 'wdth' 100; font-weight:700;",
                                        x: "7.95",
                                        y: "0",
                                        "I"
                                    }
                                    tspan {
                                        style: "font-family:Bahnschrift, Bahnschrift; font-variation-settings:'wght' 400, 'wdth' 100; letter-spacing:0em;",
                                        x: "11.33",
                                        y: "0",
                                        "V"
                                    }
                                    tspan {
                                        style: "font-family:Bahnschrift, Bahnschrift; font-variation-settings:'wght' 400, 'wdth' 100;",
                                        x: "18.62",
                                        y: "0",
                                        "E"
                                    }
                                    tspan {
                                        style: "font-family:Bahnschrift, Bahnschrift; font-variation-settings:'wght' 400, 'wdth' 100; letter-spacing:-.02em;",
                                        x: "25.98",
                                        y: "0",
                                        "R"
                                    }
                                    tspan {
                                        style: "font-family:Bahnschrift, Bahnschrift; font-variation-settings:'wght' 400, 'wdth' 100; letter-spacing:0em;",
                                        x: "33.76",
                                        y: "0",
                                        "S"
                                    }
                                    tspan {
                                        style: "font-family:Bahnschrift, Bahnschrift; font-variation-settings:'wght' 400, 'wdth' 100;",
                                        x: "41.22",
                                        y: "0",
                                        "E"
                                    }
                                }
                                polygon {
                                    points: "122.02 377.86 133.5 384.43 133.77 377.45 122.02 370.93 122.02 377.86",
                                    style: "fill:url(#bf); opacity:.82;",
                                }
                                path {
                                    d: "M61.77,455.26c-.14,2.86-2.97,10.79,15.3,12.66,17.94,1.43,19.6-8.7,19.6-8.7,0,0,3.28-21.39,3.85-27.96,.39-4.44-.33-10.13-1.07-16.32-.71-2.13-3.14-3.64-4.16-3.45-3.11,.57-17.2,2.93-20.86,4.28s-9.03,9.77-9.03,9.77c0,0-3.5,26.86-3.63,29.72Z",
                                    style: "fill:url(#bg);",
                                }
                                path {
                                    d: "M99.03,154.6c15.8,0,23.06,42.39,25.97,53.78,.59,3.25,2.05,16.9,2.05,16.9,0,0-16.95-10.91-19.38-28.19s-12.66-23.72-18.17-24.39-13.29,7.56-13.29,7.56c0,0,.61-2.93,2.33-5.85,4.4-7.46,13.63-19.82,20.48-19.82Z",
                                    style: "fill:url(#bh);",
                                }
                                text {
                                    style: "fill:#333; font-size:12.31px;",
                                    transform: "translate(112.43 320.11) rotate(-88.85) scale(.98) skewX(13.92)",
                                    tspan {
                                        style: "font-family:Bahnschrift-Bold, Bahnschrift; font-variation-settings:'wght' 700, 'wdth' 100; font-weight:700; letter-spacing:0em;",
                                        x: "0",
                                        y: "0",
                                        "U"
                                    }
                                    tspan {
                                        style: "font-family:Bahnschrift-Bold, Bahnschrift; font-variation-settings:'wght' 700, 'wdth' 100; font-weight:700;",
                                        x: "7.95",
                                        y: "0",
                                        "I"
                                    }
                                    tspan {
                                        style: "font-family:Bahnschrift, Bahnschrift; font-variation-settings:'wght' 400, 'wdth' 100; letter-spacing:0em;",
                                        x: "11.33",
                                        y: "0",
                                        "V"
                                    }
                                    tspan {
                                        style: "font-family:Bahnschrift, Bahnschrift; font-variation-settings:'wght' 400, 'wdth' 100;",
                                        x: "18.62",
                                        y: "0",
                                        "E"
                                    }
                                    tspan {
                                        style: "font-family:Bahnschrift, Bahnschrift; font-variation-settings:'wght' 400, 'wdth' 100; letter-spacing:-.02em;",
                                        x: "25.98",
                                        y: "0",
                                        "R"
                                    }
                                    tspan {
                                        style: "font-family:Bahnschrift, Bahnschrift; font-variation-settings:'wght' 400, 'wdth' 100; letter-spacing:0em;",
                                        x: "33.76",
                                        y: "0",
                                        "S"
                                    }
                                    tspan {
                                        style: "font-family:Bahnschrift, Bahnschrift; font-variation-settings:'wght' 400, 'wdth' 100;",
                                        x: "41.22",
                                        y: "0",
                                        "E"
                                    }
                                }
                                path {
                                    d: "M61.77,455.26c-.14,2.86-2.97,10.79,15.3,12.66,17.94,1.43,19.6-8.7,19.6-8.7,0,0,3.28-21.39,3.85-27.96,.39-4.44-.33-10.13-1.07-16.32-.71-2.13-3.14-3.64-4.16-3.45-3.11,.57-17.2,2.93-20.86,4.28s-9.03,9.77-9.03,9.77c0,0-3.5,26.86-3.63,29.72Z",
                                    style: "fill:url(#bi);",
                                }
                                line {
                                    style: "fill:#fff; stroke:#2b2b2b; stroke-miterlimit:10; stroke-width:.5px;",
                                    x1: "124.42",
                                    x2: "126.39",
                                    y1: "432.01",
                                    y2: "426.74",
                                }
                                path {
                                    d: "M91.12,170.73c2.47,.54,5.71-8.54,3.35-8.72s-5.82,8.18-3.35,8.72Z",
                                    style: "fill:url(#bj);",
                                }
                                path {
                                    d: "M86.98,171.58c1.41,.66,4.25-6.41,2.87-6.77s-4.28,6.1-2.87,6.77Z",
                                    style: "fill:url(#bk);",
                                }
                                path {
                                    d: "M94.44,172.88c2.08,.44,4.88-6.87,2.88-7.01s-4.96,6.57-2.88,7.01Z",
                                    style: "fill:url(#bl);",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn EmbeddedResume(resume: Asset) -> Element {
    let pdf_url = format!("{}#navpanes=0", resume); // Defaults page navigation pane to closed

    rsx! {
        document::Link { rel: "stylesheet", href: RESUME_CSS }

        div { class: "resume-container",
            iframe {
                class: "resume-frame",
                src: pdf_url,
                width: "100%",
                height: "1200px",
            }
        }
    }
}