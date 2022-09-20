use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use base64::encode;

use crate::components::{
    acordeon::AcordeonCard,
    add_btn::Add,
    image::Image,
    input_image_component::{FileDetails, FileInput},
    input_name::Input,
    submit_btn::SubmitBtn,
};

#[function_component(Home)]
pub fn home() -> Html {
    fn create_image(file: &FileDetails) -> Html {
        html! {

                <div class="preview-media">
                    if file.file_type.contains("image") {
                        <Image  src={format!("data:{};base64,{}", file.file_type, encode(&file.data))} />
                    }
                </div>

        }
    }

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
      <div class="h-screen py-10 pb-20 relative">
        <div class="mx-auto flex justify-between items-start max-w-5xl mb-10">
          <div class="flex flex-col justify-between items-start">
            <Input />
            <Add />
          </div>
            <FileInput />
        </div>


        // <div class="mx-auto max-w-5xl grid grid-cols-4 gap-8 ">

          // <Image />
          // <Image />
          // <Image />
          // <Image />
          // <Image />
          // <Image />

      // </div>
        // Local images component

        <div class="block mx-auto max-w-5xl mt-16">
          <AcordeonCard />
          <AcordeonCard />
        </div>

        <div class="fixed right-5 bottom-5">
        <SubmitBtn />
        </div>
      </div>
    }
}
