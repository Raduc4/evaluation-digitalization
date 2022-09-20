use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;

#[function_component(Input)]
pub fn input() -> Html {
    let name = use_state(|| String::new());

    let oninput = {
        let curr_name = name.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            curr_name.set(input.value());
            log::debug!("{}", *curr_name);
        })
    };
    html! {
      <div class="mb-6">
        <label class="block mb-2 text-md font-medium text-green-900 dark:text-white">{"Numele elevului"}</label>
        <input type="text" id="success" {oninput}
          class="bg-green-50 border border-white text-white dark:text-white placeholder-white dark:placeholder-white text-sm rounded-lg focus:ring-green-500 focus:border-green-500 block w-full p-2.5 dark:bg-gray-700 dark:border-white" placeholder="" />
    </div>
    }
}
