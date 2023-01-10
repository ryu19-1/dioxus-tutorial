use crate::hooks::use_recruits::use_recruits;
use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    let result = use_future(&cx, (), |_| async move { use_recruits().await });

    cx.render(rsx! {
        div{ class: "flex flex-col mt-32",
            h1 { "トップ画面" }
            h2 { "募集一覧" }
            match result.value() {
                Some(Ok(d)) => cx.render(rsx! {
                    d.iter().map(|v| rsx!{ div{v.to_string()}})
                }),
                _ => cx.render(rsx! {
                    div{"No recruit found."}
                }),
            }
        }
    })
}
