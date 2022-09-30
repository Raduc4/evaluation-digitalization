use yew::{html, Callback, Component, Context, Html};

use base64::encode;

use crate::components::{
    acordeon::AcordeonCard,
    add_btn::Add,
    image::Image,
    input_image_component::{FileDetails, FileInput},
    input_name::Input,
    submit_btn::SubmitBtn,
};

pub(crate) struct Home {
    files: Vec<FileDetails>,
}

pub enum Msg {
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
    action: ImageControlsEnum,
    uuid: String,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            files: Vec::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add(file) => {
                self.files.push(file);
                true
            }
            Msg::ImageControlsEnum(controls_struct) => {
                let image_to_move_idx = self
                    .files
                    .iter()
                    .position(|item| item.uuid == controls_struct.uuid)
                    .unwrap();

                match controls_struct.action {
                    ImageControlsEnum::Right => {
                        if self.files.len() != image_to_move_idx {
                            self.files.swap(image_to_move_idx, image_to_move_idx + 1)
                        }
                    }
                    ImageControlsEnum::Left => {
                        if image_to_move_idx != 0 {
                            self.files.swap(image_to_move_idx, image_to_move_idx - 1)
                        }
                    }
                    ImageControlsEnum::Delete => {
                        self.files.remove(image_to_move_idx);
                        ()
                    }
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_upload_imgs = ctx.link().callback(Msg::Add);
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
         { for self.files.iter().map(|item| Self::view_file(item, on_click_controls_img.clone())) }
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
    fn view_file(file: &FileDetails, controls_callback: Callback<ControlsStruct>) -> Html {
        html! {

                <div class="preview-media">
                    if file.file_type.contains("image") {
                        <Image {controls_callback} src={format!("data:{};base64,{}", file.file_type, encode(&file.data))} />
                    }
                </div>

        }
    }
}
