use serde::Serialize;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;
use gloo_utils::document;

use crate::pages::TinyData;

#[derive(Serialize, Debug)]
#[serde(rename_all="snake_case")]
struct OriginUrl{
    pub origin_url: String,
}

#[function_component(Create)]
pub fn create() -> Html {
  const DOMAIN_URL: &str = "http://127.0.0.1:8080/";
  const HTTP_PRE: &str = "http://";
  let tiny_data: Box<UseStateHandle<Option<_>>> = Box::new(use_state(||None));
  let onclick = {
      let tiny_data = tiny_data.clone();
      Callback::from(move |_| {
          let tiny_data = tiny_data.clone();
          wasm_bindgen_futures::spawn_local(async move{
              let tiny_data = tiny_data.clone();
              let tiny_data_endpoint = format!("/api/create", );
              let dom_ou = document().get_element_by_id("origin_url").unwrap().dyn_into::<HtmlInputElement>().unwrap();
              let o = OriginUrl{
                  origin_url: format!("{}{}", HTTP_PRE, String::from(dom_ou.value())),
              };
              let f = TinyData {
                  data: "1111",
                  ok: true,
                  err: None,
              };
              tiny_data.set(Some(f));
              // let r = Request::post(&tiny_data_endpoint).json(&o).expect("post is wrong");
              // let fetched_tiny_data = r.send().await;
              // if let Ok(response) = fetched_tiny_data{
              //     let json: Result<TinyData<String>, _> = response.json().await;
              //     if let Ok(f) = json {
              //         tiny_data.set(Some(f));
              //     }
              // }
          })
      })
  };
  html! {
      <div class="mx-auto max-w-7xl py-12 sm:px-6 lg:px-8">
          <div class="md:grid md:grid-cols-3 md:gap-6">
              <div class="md:col-span-1">
              <div class="px-4 sm:px-0">
                  <h3 class="text-lg font-medium leading-6 text-gray-900">{"Hi"}</h3>
                  <p class="mt-1 text-sm text-gray-600">{"Make a tiny Url"}</p>
              </div>
              </div>
              <div class="mt-5 md:col-span-2 md:mt-0">
                  <div class="shadow sm:overflow-hidden sm:rounded-md">
                  <div class="space-y-6 bg-white px-4 py-5 sm:p-6">
                  <div class="grid grid-cols-3 gap-6">
                      <div class="col-span-2 sm:col-span-4">
                      <label for="origin_url" class="block text-sm font-medium text-gray-700">{"Your Original URL"}</label>
                      <div class="mt-1 flex rounded-md shadow-sm">
                          <span class="inline-flex items-center rounded-l-md border border-r-0 border-gray-300 bg-gray-50 px-3 text-sm text-gray-500">{HTTP_PRE}</span>
                          <input type="text" name="origin_url" id="origin_url" class="block w-full flex-1 rounded-none rounded-r-md border-gray-300 focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm" placeholder={"www.example.com"} />
                      </div>
                      </div>
                      <div class="col-span-6 mt-1 sm:col-span-4">
                      <label for="email-address" class="block text-sm font-medium text-gray-700">{"Tiny URL:"}</label>
                      <input type="text" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm" value={
                          match (*tiny_data).as_ref(){
                              Some(f) => {
                                  log::info!("=====");
                                  format!("{}{}", String::from(DOMAIN_URL), &f.data.clone())
                                  // format!("{}{}", String::from(DOMAIN_URL), "111")
                              },
                              None => String::from("")
                          }
                      }/>
                      </div>
                  </div>
                  </div>
                  <div class="bg-gray-50 px-4 py-3 text-right sm:px-6">
                      <button type="button" {onclick} class="inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">{"Make Tiny URL"}</button>
                  </div>
                  </div>
              </div>
          </div>
      </div>
  }
}