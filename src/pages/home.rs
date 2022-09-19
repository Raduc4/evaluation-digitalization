use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{
    acordeon::AcordeonCard, add_btn::Add, input_image::FileInput, input_name::Input,
};

#[function_component(Home)]
pub fn home() -> Html {
    // let username = use_state(|| String::new());
    // let user = use_context::<User>().expect("No context found.");

    // let oninput = {
    //     let current_username = username.clone();

    //     Callback::from(move |e: InputEvent| {
    //         let input: HtmlInputElement = e.target_unchecked_into();
    //         current_username.set(input.value());
    //     })
    // };

    // let onclick = {
    //     let username = username.clone();
    //     let user = user.clone();
    //     Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    // };

    html! {
      <div class="h-screen py-10">
        <div class="mx-auto flex justify-between items-center max-w-5xl mb-10">
          <div class="flex flex-col justify-between items-center">
            <Input />
            <Add />
          </div>
            <FileInput />
        </div>

        // Local images component

        <div class="block mx-auto max-w-5xl mt-16">
          <AcordeonCard />
          <AcordeonCard />
        </div>
      </div>
    }
}
