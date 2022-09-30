use yew::functional::*;
use yew::prelude::*;
use yew::{function_component, html, Properties};

use crate::pages::home::{ControlsStruct, ImageControlsEnum};

#[derive(Properties, PartialEq)]
pub struct ImageProps {
    pub controls_callback: Callback<ControlsStruct>,
    pub src: String,
    pub uuid: String,
}
pub enum Msg {
    MoveFileLeft,
    MoveFileRight,
    Delete,
}

pub struct Image;

impl Component for Image {
    type Message = Msg;
    type Properties = ImageProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let id = ctx.props().uuid.clone();
        match msg {
            Msg::MoveFileLeft => {
                let controls_struct = ControlsStruct {
                    action: ImageControlsEnum::Left,
                    uuid: id,
                };
                ctx.props().controls_callback.emit(controls_struct);
                true
            }
            Msg::MoveFileRight => {
                let controls_struct = ControlsStruct {
                    action: ImageControlsEnum::Right,
                    uuid: id,
                };
                ctx.props().controls_callback.emit(controls_struct);
                true
            }
            Msg::Delete => {
                let controls_struct = ControlsStruct {
                    action: ImageControlsEnum::Delete,
                    uuid: id,
                };
                ctx.props().controls_callback.emit(controls_struct);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
              <div class="block relative max-w-sm bg-white rounded-lg shadow-md dark:bg-gray-800 dark:border-gray-700">
                <img class="block rounded-lg h-72" src={ctx.props().src.clone()} alt="Test image"/>
                <div class="p-5 absolute bottom-0 flex items-center">
                <button
                onclick={ctx.link().callback(|_event: MouseEvent| {Msg::MoveFileLeft})}
                class="inline-flex items-center py-2 px-3 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
              >
              <svg ariaHidden="true" class="hover:text-white rotate-180 w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fillRule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clipRule="evenodd"></path></svg>
              </button>

              <button
                onclick={ctx.link().callback(|_event: MouseEvent| {Msg::MoveFileRight})}
                class="inline-flex items-center justify-center py-2 px-3 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
              >
              <svg ariaHidden="true" class="hover:text-white w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fillRule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clipRule="evenodd"></path></svg>
              </button>

              <button
                  onclick={ctx.link().callback(|_event: MouseEvent| {Msg::Delete})}
                  class="inline-flex items-center py-2 px-3 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                >
                  {"Delete"}
                </button>

                </div>
              </div>
        }
    }
}
