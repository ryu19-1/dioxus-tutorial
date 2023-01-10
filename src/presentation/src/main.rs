#![allow(non_snake_case)]

use components::organisms::footer::Footer;
use components::organisms::header::Header;
use containers::*;
use dioxus::prelude::*;
use dioxus_router::{Route, Router};

pub mod components;
pub mod composition_root;
pub mod containers;
pub mod hooks;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        Header(cx),
        Router {
            Route { to: "/home", home::Home {} }
            Route { to: "", not_found::NotFound {} }
        },
        Footer(cx)
    ))
}
