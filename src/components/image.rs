use yew::functional::*;
use yew::prelude::*;
use yew::{function_component, html, Properties};

use crate::pages::home::{ControlsStruct, ImageControlsEnum};

#[derive(Properties, PartialEq)]
pub struct ImageProps {
    pub controls_callback: Callback<ControlsStruct>,
    pub src: String,
    // pub uuid: String,
}
pub enum Msg {
    // Add(FileDetails),
    ImageControlsEnum(ControlsStruct),
}

pub struct Image;

impl Component for Image {
    type Message = Msg;
    type Properties = ImageProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let onclick_left = {
        //     let controls = props.controls_callback.clone();

        //     let controls_struct_local = ControlsStruct {
        //         action: ImageControlsEnum::Left,
        //         uuid: "uuid1".to_string(),
        //     };

        //     controls.emit(controls_struct_local)
        // };
        html! {
              <div class="block relative max-w-sm bg-white rounded-lg shadow-md dark:bg-gray-800 dark:border-gray-700">
                <img class="block rounded-lg h-72" src={ctx.props().src.clone()} alt="Test image"/>
                <div class="p-5 absolute bottom-0 flex items-center">
                <button
                // onclick={onclick_left}
                // onClick={() => {
                //   context.removeImgFromState(id);
                // }}
                class="inline-flex items-center py-2 px-3 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
              >
              <svg ariaHidden="true" class="hover:text-white rotate-180 w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fillRule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clipRule="evenodd"></path></svg>
              </button>

              <button
                // onClick={() => {
                //   context.removeImgFromState(id);
                // }}
                class="inline-flex items-center justify-center py-2 px-3 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
              >
              <svg ariaHidden="true" class="hover:text-white w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fillRule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clipRule="evenodd"></path></svg>
              </button>

              <button
                  // onClick={() => {
                  //   context.removeImgFromState(id);
                  // }}
                  class="inline-flex items-center py-2 px-3 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                >
                  {"Delete"}
                </button>

                </div>
              </div>
        }
    }
}
