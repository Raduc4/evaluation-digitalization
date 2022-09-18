use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Input)]
pub fn input() -> Html {
    html! {
     <div class="mb-6">
        <label class="block mb-2 text-sm font-medium text-green-900 dark:text-white">{"Numele elevului"}</label>
        <input type="text" id="success" 
					class="bg-green-50 border border-white text-white dark:text-white placeholder-white dark:placeholder-white text-sm rounded-lg focus:ring-green-500 focus:border-green-500 block w-full p-2.5 dark:bg-gray-700 dark:border-white" placeholder="" />
    </div>
    }
}
