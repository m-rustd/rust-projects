use dioxus_app::app;

fn main() {
    start();
}

#[cfg(feature = "web")]
fn start() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    dioxus_web::launch(app);
}

#[cfg(feature = "desktop")]
fn start() {
    tracing_subscriber::fmt::init();
    dioxus_desktop::launch(app);
}