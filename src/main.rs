#![allow(non_snake_case)]

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let texto = "Texto prestado";
    cx.render(rsx! {
        div {
            h1 { "Hello, world!" }
            About {}
            Demo { texto: "Texto de prueba".to_string() }
            DemoBorrowed { texto: texto }
            button { onclick: move |event| println!("Clicked! Event: {event:?}"), "click me!" }
            button { onclick: move |event| println!("Clicked! Event: {event:?}"), "click me!" }
        }
    })
}

#[derive(PartialEq, Props)]
struct DemoProps {
    texto: String,
}

fn Demo(cx: Scope<DemoProps>) -> Element {
    cx.render(rsx! {
        div {
            p {
                "Probando las props propias"
                b { cx.props.texto.clone() }
                " Prueba"
            }
        }
    })
}

#[derive(PartialEq, Props)]
struct DemoBorrowedProps<'a> {
    texto: &'a str,
}

fn DemoBorrowed<'a>(cx: Scope<'a, DemoBorrowedProps<'a>>) -> Element {
    cx.render(rsx! {
        h1 { "Probando texto prestado ", cx.props.texto }
    })
}

pub fn About(cx: Scope) -> Element {
    cx.render(rsx!(
        p {
            b { "Dioxus Labs" }
            " An Open Source project dedicated to making Rust UI wonderful."
        }
    ))
}
