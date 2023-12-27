use yew::prelude::*;

use crate::pages::TinyData;

#[derive(Debug, Clone, PartialEq)]
struct TinyUrl {
    pub tiny_code: String,
    pub origin_url: String,
}

#[derive(Debug, Clone, PartialEq)]
struct TinyUrls {
    pub tus: Vec<TinyUrl>
}

impl TinyUrls {
    pub fn new() -> Self {
        TinyUrls { tus: Vec::new() }
    }

    pub fn init(tus: Vec<TinyUrl>) -> Self {
        TinyUrls { tus }
    }
}

#[function_component(TinyUrlProps)]
fn tiny_url_props() -> Html {
    const DOMAIN_URL: &str = "http://127.0.0.1:8080/";
    let tiny_urls = use_context::<TinyUrls>().expect("no uct");
    tiny_urls.tus.iter().map(|t| {
        let tiny_url = t.clone();
        html!{
            <>
            <div class="bg-white px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                <dt class="text-sm font-medium text-gray-500">{format!("{}{}", DOMAIN_URL, tiny_url.tiny_code)}</dt>
                <dd class="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0">{tiny_url.origin_url}</dd>
            </div>
            </>
        }
    }).collect()
}

#[function_component(Links)]
pub fn links() -> Html {
  log::info!("this is links page");
  let tiny_data = Box::new(use_state(||None));
    let onclick = {
        let tiny_data = tiny_data.clone();
        Callback::from(move |_|{
            let mut v = Vec::new();
            v.push(TinyUrl{tiny_code: String::from("aaa"), origin_url: String::from("http://baidu.com")});
            v.push(TinyUrl{tiny_code: String::from("bbb"), origin_url: String::from("http://google.com")});
            let td = TinyData{ok: true, data: v, err: None};
            tiny_data.set(Some(td));

            // let tiny_data = tiny_data.clone();
            // wasm_bindgen_futures::spawn_local(async move{
            //     let tiny_data_endpoint = format!("/api/links", );
            //     let fetched_tiny_data = Request::get(&tiny_data_endpoint).send().await;
            //     if let Ok(response) = fetched_tiny_data{
            //         let json: Result<TinyData<Vec<TinyUrl>>, _> = response.json().await;
            //         if let Ok(f) = json{
            //             tiny_data.set(Some(f));
            //         }
            //     }
            // });
        })
    };
    let mut v: TinyUrls = TinyUrls::new();
    match (*tiny_data).as_ref() {
        Some(t) => {
            v.tus = t.data.clone();
        },
        None => {},
    }
    html!{
        <div class="mx-auto max-w-7xl py-12 sm:px-6 lg:px-8">
            <div class="mx-auto max-w-4xl">
                <div class="overflow-hidden bg-white shadow sm:rounded-lg ">
                    <div class="px-4 py-5 sm:px-6">
                        <h3 class="text-lg font-medium cleading-6 text-gray-900">{"All Tiny Urls"}</h3>
                    </div>
                    <div class="border-t border-gray-200">
                        <div class="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                            <dt class="text-sm font-medium text-gray-500">{"Tiny URL"}</dt>
                            <dd class="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0">{"Origin URL"}</dd>
                        </div>
                        <ContextProvider<TinyUrls> context={v.clone()} >
                            <TinyUrlProps/>
                        </ContextProvider<TinyUrls>>
                    </div>
                </div>
                <div class="bg-gray-50 px-4 py-3 text-right sm:px-6">
                    <button type="button" {onclick} class="inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">{"Query"}</button>
                </div>
            </div>
        </div>
    }
}