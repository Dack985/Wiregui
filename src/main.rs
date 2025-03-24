use dioxus::prelude::*;

use crate::components::navbar::Navbar;


use views::home::Home;
use views::tunnels::Tunnels;

pub mod components;
pub mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/tunnels")]
    Tunnels {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

//#[component]
//pub fn Hero() -> Element {
//    rsx! {
//        div {
//            id: "hero",
//            img { src: HEADER_SVG, id: "header" }
//            div { id: "links",
//                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
//                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
//                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
//                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
//                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
//                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
//            }
//        }
//    }
//}

// Home page
//#[component]
//fn Home() -> Element {
//    rsx! {
//        Hero {}
//      //  Echo {}
//    }
//}

// Blog page
//#[component]
//pub fn Blog(id: i32) -> Element {
//    rsx! {
//        div {
//            id: "blog",
//
//            // Content
//            h1 { "This is blog #{id}!" }
//            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }
//
//            // Navigation links
//            Link {
//                to: Route::Blog { id: id - 1 },
//                "Previous"
//            }
//            span { " <---> " }
//            Link {
//                to: Route::Blog { id: id + 1 },
//                "Next"
//            }
//        }
//    }
//}


// Blog page
//#[component]
//pub fn Tunnels() -> Element {
//    rsx! {
//        div {
//            class: "flex flex-col grow ",
//                div {
//                    div {
//                        p { "hello world!!" }
//                        }
//                    div {
//                        p {"test 1"}
//                    }
//                    div {
//                        p {"test 2"}
//                    }                        
//                }
//            }
//    }
//}


// Shared navbar component.
//#[component]
//fn Navbar() -> Element {
//    rsx! {
//        div {
//            id: "navbar",
//            Link {
//                to: Route::Home {},
//                "Home"
//            }
//         //   Link {
//         //       to: Route::Blog { id: 1 },
//         //       "Blog"
//         //   }
//            Link {
//                to: Route::Tunnels {},
//                "Tunnels"
//            }
//        }
//        Outlet::<Route> {}
//    }
//}

// Echo component that demonstrates fullstack server functions.
//#[component]
//fn Echo() -> Element {
//    let mut response = use_signal(|| String::new());
//
//    rsx! {
//        div {
//            id: "echo",
//            h4 { "ServerFn Echo" }
//            input {
//                placeholder: "Type here to echo...",
//                oninput:  move |event| async move {
//                    let data = echo_server(event.value()).await.unwrap();
//                    response.set(data);
//                },
//            }
//
//            if !response().is_empty() {
//                p {
//                    "Server echoed: "
//                    i { "{response}" }
//                }
//            }
//        }
//    }
//}
//
// Echo the user input on the server.

//#[server(EchoServer)]
//async fn echo_server(input: String) -> Result<String, ServerFnError> {
//    Ok(input)
//}
