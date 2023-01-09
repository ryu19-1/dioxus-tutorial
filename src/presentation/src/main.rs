#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use home::Home;
use not_found::NotFound;

pub mod composition_root;
pub mod home;
pub mod not_found;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        Router {
            Route { to: "/home", Home {} }
            Route { to: "", NotFound {} }
        }
    ))
}
