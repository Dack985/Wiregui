use dioxus::prelude::*;
use crate::HEADER_SVG;


#[component]
pub fn Tunnels() -> Element {
    rsx! {
        div {
            id: "tunnels",
            class: "flex flex-col items-center justify-center w-screen h-screen",
            
            div { 
                id: "flexbox",
                class: "flex bg-blue-600 flex-row w-full h-1/2 grow shrink gap-2 justify-center items-center",
                p { "poop" }
            }
            div { 
                id: "flexbox",
                class: "flex bg-red-600 flex-row w-full h-1/2 grow shrink gap-2 justify-center items-center",
                p { "butt" }
            }
        }
    }
}