
use dioxus::prelude::*;
use crate::Route;


#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
         //   Link {
         //       to: Route::Blog { id: 1 },
         //       "Blog"
         //   }
            Link {
                to: Route::Tunnels {},
                "Tunnels"
            }
        }
        Outlet::<Route> {}
    }
}