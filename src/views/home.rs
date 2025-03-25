use dioxus::prelude::*;
use crate::HEADER_SVG;


#[component]
pub fn Home() -> Element {
        rsx! {
        div {
            id: "home",
            class: "flex flex-row w-screen h-screen min-h-screen gap-10 border border-red-600 items-stretch", // Ensure full height

            // Left Column (1/3 of the screen)
            div { 
                id: "wg-status-left",
                class: "flex bg-sky-50 flex-col w-1/3 h-full grow shrink border border-blue-500 p-4",
                p { "Tunnel Config:" }
                
                // Nested box inside left column
                div { 
                    class: "flex bg-stone-200 flex-col w-full grow h-auto border border-gray-500 p-4",
                    p { "Tunnel 1: homelab" }
                    p { "Tunnel 2: club" }
                }
            }
            
            // Right Section (2 Rows)
            div { 
                id: "wg-status-right-boxes",
                class: "flex flex-col w-2/3 h-full gap-4 grow shrink border border-emerald-500",  // 2/3 width, full height

                // Top Row
                div { 
                    id: "wg-status-top",
                    class: "flex bg-stone-400 flex-col w-full grow shrink h-1/2 border border-yellow-500 p-4",
                    p { "Interface:" }
                    // Nested flexbox inside the top row
                    div { 
                        class: "flex bg-stone-200 flex-col w-full grow shrink h-auto border border-gray-500 p-4",
                        p { "Status:         Active" }
                        p { "Public Key:     hLNZ45UXR0127fYeJyi32434236023423XRP4F2765TgJBw=" }
                        p { "Addresses:      192.168.1.0/24" }
                        p { "DNS:         8.8.8.8" }
                        div {
                            class: "flex flex-col",
                            button {
                                style: "background:rgb(218, 211, 211); color: #fff; margin-left: 20%; position: relative; width: 120px; height: 40px; border: none; padding: 0.5em; font-weight: bold; text-transform: uppercase; transition: 0.2s; border-radius: 5px; opacity: 0.8; letter-spacing: 1px; box-shadow:rgb(14, 13, 13) 0px 7px 2px, #000 0px 8px 5px;",
                                onclick: move |_| {
                                    println!("Button clicked!");
                                },
                                "Activate"
                            }
                        }
                    }
                }
                
                // Bottom Row
                div { 
                    id: "wg-status-bottom",
                    class: "flex bg-stone-400 flex-col w-full grow shrink h-1/2 border border-purple-500 p-4",
                    p { "Peer Info:" }
                
                    // Nested flexbox inside the bottom row
                    div { 
                        class: "flex bg-stone-200 flex-col w-full grow shrink h-auto border border-gray-500 p-4",
                        p { "Public Key:     hLNZ45UXR0127fYeJyi32434236023423XRP4F2765TgJBw=" }
                        p { "Allowed IP's:   192.168.1.0/24" }
                        p { "Persistent Keepalive:         25" }
                    }
                }
            }
        }
    }
}
