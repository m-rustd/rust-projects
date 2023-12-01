use dioxus::{prelude::*, html::button};
use tracing::info;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    render! {
        p {
            b {"Dioxus Labs"}
            " An Open Source project dedicated to making Rust UI wonderful."
        }
        div {
            p {"Dioxus is a React-like Rust framework, but with a different API"}
        }
        likes_component { 
            score: 100,
            title: Some("hello"),
        }
        clickable_component { 
            href: "https://github.com/athifr/dioxus", 
            div {
              p {
                "Dioxus is a React-like Rust framework, but with a different API"
              }
              button {
                "Click me"
              }
            }
        },
        click_component {
            on_click: move |data| {
                info!("======== div clicked== {data:?}");
            },
        }
    }
}

#[derive(Debug)]
struct CustomData(i32);

#[derive(Props)]
struct ClickComponentProps<'a> {
    on_click: EventHandler<'a, CustomData>,
}

fn click_component<'a>(cx: Scope<'a, ClickComponentProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            onclick: move |_| {
                info!("div clicked");
            },
            "This is a clickable component"
            button {
                onclick: move |event| {
                    info!("button clicked");
                    event.stop_propagation();
                    cx.props.on_click.call(CustomData(10));
                },
                "Click me"
            }
        }
    })
}

#[derive(Debug, Props)]
struct ClickableProps<'a> {
    href: &'a str,
    children: Element<'a>,
}

fn clickable_component<'a>(cx: Scope<'a, ClickableProps<'a>>) -> Element {
    cx.render(rsx! {
        a {
            href: "{cx.props.href}",
            &cx.props.children
        }
    })
}

// #[derive(Props, PartialEq)]
// struct LikesProps<'a> {
//     score: i32,
//     title: Option<&'a str>,
// }

#[derive(Props)]
struct LikesProps<'a> {
    score: i32,
    #[props(!optional)]
    title: Option<&'a str>,
}

fn likes_component<'a>(cx: Scope<'a, LikesProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            "This post has ",
            b { "{cx.props.score}" },
            " likes"
        }
    })
}

// fn likes_component<'a>(cx: Scope, props: LikesProps) -> Element {
//     render! {
//       div {
//           "This post has ",
//           b { "{props.score}" },
//           " likes"
//       }
//     }  
// }