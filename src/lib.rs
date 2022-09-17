#![recursion_limit = "512"]

// mod components;
// mod services;

use wasm_bindgen::prelude::*;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

pub type Images = Rc<ImagesInner>;

#[derive(PartialEq, Debug)]
pub struct ImagesInner {
    pub images: RefCell<String>,
}
#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Main,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(selected_route: &Route) -> Html {
    match selected_route {
        Route::Main => {
            html! {<button class="text-black bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 mr-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800">{"Default"}</button>}
        }
        Route::NotFound => html! {<h1>{"404 baby"}</h1>},
    }
}

#[function_component(Main)]
fn main() -> Html {
    let ctx = use_state(|| {
        Rc::new(ImagesInner {
            images: RefCell::new("initial".into()),
        })
    });

    html! {
            <ContextProvider<Images> context={(*ctx).clone()}>
        <BrowserRouter>
            <div class="w-screen h-screen">
                <Switch<Route> render={Switch::render(switch)} />
            </div>
        </BrowserRouter>
                </ContextProvider<Images>>
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
    Ok(())
}
// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
