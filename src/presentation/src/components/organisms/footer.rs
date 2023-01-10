use dioxus::prelude::*;

pub fn Footer(cx: Scope) -> Element {
    cx.render(rsx! {
        div {class: "py-16 flex h-64 w-full absolute bottom-0 bg-gray-300",
            div{ class: "m-auto", "フッターです" }
        }
    })
}
