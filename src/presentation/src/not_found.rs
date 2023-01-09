use dioxus::prelude::*;

pub fn NotFound(cx: Scope)->Element{
    cx.render(rsx!{
        div{ "404" }
    })
}