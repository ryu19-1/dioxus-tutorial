use dioxus::prelude::*;

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "py-16 flex h-32 w-full absolute top-0 border",
            div{ class: "m-auto","ヘッダーです" }
        }
    })
}
