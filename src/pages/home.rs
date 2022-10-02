use crate::components::{
    acordeon::AcordeonCard, add_btn::Add, image::Image, input_image_component::FileInput,
    input_name::Input, submit_btn::SubmitBtn,
};
use crate::services::{indexdb::example, rexie::build_database};
use crate::state::state::FileDetails;
use crate::state::state::PupilDetails;
use crate::state::state::State;
use base64::encode;
use gloo::console::log;
use rexie::*;
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::{html, Callback, Component, Context, Html};
use yewdux::prelude::*;

#[wasm_bindgen(module = "/src/pages/addPupil.js")]
extern "C" {
    fn addPupil(id: i32, name: String, photos_bytes_array: Vec<u8>);
}
pub struct Home {
    pub state: Rc<State>,
    pub dispatch: Dispatch<State>,
    pub rexie: Option<Rexie>,
}

pub enum Msg {
    State(Rc<State>),
    Add(FileDetails),
    ImageControlsEnum(ControlsStruct),
}

#[derive(Debug)]
pub enum ImageControlsEnum {
    Right,
    Left,
    Delete,
}

#[derive(Debug)]
pub struct ControlsStruct {
    pub action: ImageControlsEnum,
    pub uuid: String,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::<State>::subscribe(ctx.link().callback(Msg::State));
        Self {
            state: dispatch.get(),
            dispatch,
            rexie: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::State(state) => {
                // let rexie = wasm_bindgen_futures::spawn_local(example().await.unwrap());

                self.state = state;
                true
            }
            Msg::Add(file) => {
                self.dispatch.reduce_mut(|state| state.files.push(file));
                true
            }
            Msg::ImageControlsEnum(controls_struct) => {
                let image_to_move_idx = self
                    .state
                    .files
                    .iter()
                    .position(|item| item.uuid == controls_struct.uuid)
                    .unwrap();

                match controls_struct.action {
                    ImageControlsEnum::Right => {
                        if self.state.files.len() != image_to_move_idx {
                            self.dispatch.reduce_mut(|state| {
                                state.files.swap(image_to_move_idx, image_to_move_idx + 1)
                            })
                        }
                    }
                    ImageControlsEnum::Left => {
                        if image_to_move_idx != 0 {
                            self.dispatch.reduce_mut(|state| {
                                state.files.swap(image_to_move_idx, image_to_move_idx - 1)
                            })
                        }
                    }
                    ImageControlsEnum::Delete => {
                        self.dispatch
                            .reduce_mut(|state| state.files.remove(image_to_move_idx));
                        ()
                    }
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let pup = PupilDetails::new(31, "Name".to_string(), vec![12, 13]);
        // refreshform("body");
        addPupil(pup.id, pup.name, pup.photos);
        let state_local = self.state.files.clone();
        // let (files, dispatch) = use_store::<ImagesPersistentState>();
        let on_upload_imgs = ctx.link().callback(|item| Msg::Add(item));
        let on_click_controls_img = ctx.link().callback(Msg::ImageControlsEnum);
        html! {
          <div class="h-screen py-10 pb-20 relative">
          <div class="mx-auto flex justify-between items-start max-w-5xl mb-10">
          <div class="flex flex-col justify-between items-start">
                     <Input />
                     <Add />
                   </div>
                     <FileInput {on_upload_imgs}/>
                 </div>


        <div class="mx-auto max-w-5xl grid grid-cols-4 gap-8">
         { for state_local.iter().map(|item| Self::view_file(item, on_click_controls_img.clone(), item.uuid.clone())) }
        </div>

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
}

impl Home {
    fn view_file(
        file: &FileDetails,
        controls_callback: Callback<ControlsStruct>,
        uuid: String,
    ) -> Html {
        html! {

                <div class="preview-media">
                    if file.file_type.contains("image") {
                        <Image {uuid} {controls_callback} src={format!("data:{};base64,{}", file.file_type, encode(&file.data))} />
                    }
                </div>

        }
    }
}
