#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Router,Route};
use home::Home;
use not_found::NotFound;

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