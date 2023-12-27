use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod router;

use router::{Route, switch};

#[function_component(App)]
fn app() -> Html {
  html! {
      <BrowserRouter>
          <Switch<Route> render={switch} />
      </BrowserRouter>
  }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
