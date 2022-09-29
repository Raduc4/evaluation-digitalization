use yew::{html, Callback, Component, Context, Html, Properties};

use web_sys::HtmlInputElement;
// use yew::functional::*;
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
use crate::Images;

pub(crate) struct Home {
    files: Vec<FileDetails>,
}

pub enum Msg {
    Add(FileDetails),
    // Loaded(String, Vec<u8>),
    // Files(Vec<File>),
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_clicked = ctx.link().callback(Msg::Add);
        html! {
               <div class="h-screen py-10 pb-20 relative">
                 <div class="mx-auto flex justify-between items-start max-w-5xl mb-10">
                   <div class="flex flex-col justify-between items-start">
                     <Input />
                     <Add />
                   </div>
                     <FileInput {on_clicked}/>
                 </div>

                 // <button {onclick}>{"Click"}</button>
                   // <div id="preview-area" class="flex flex-wrap relative right-72">
                   //                      { files.images.borrow().iter().map(create_image) }
                   //                  </div>
        <div class="mx-auto max-w-5xl grid grid-cols-4 gap-8">
                                    { for self.files.iter().map(Self::view_file) }
                                </div>

                 // <div>
                 //   {files.}
                 //   </div>
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
}

impl Home {
    fn view_file(file: &FileDetails) -> Html {
        html! {

                <div class="preview-media">
                    if file.file_type.contains("image") {
                        <Image  src={format!("data:{};base64,{}", file.file_type, encode(&file.data))} />
                    }
                </div>

        }
    }
}
