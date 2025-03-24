use dioxus::prelude::*;
use crate::HEADER_SVG;


#[component]
pub fn Home() -> Element {
    rsx! {
        img { src: HEADER_SVG, id: "header" }
        div {
            id: "home",
            class: "flex flex-col items-center justify-center w-screen h-screen",
            
            div { 
                id: "wg-status",
                class: "flex bg-stone-400 flex-row w-full h-1/2 grow shrink gap-2 justify-center items-center",
                p { "status" }
            }
            div { 
                id: "wg-status",
                class: "flex bg-stone-400 flex-row w-full h-1/2 grow shrink gap-2 justify-center items-center",
                p { "status" }
            }
        }
    }
}