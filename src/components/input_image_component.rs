extern crate base64;

use std::collections::HashMap;

use crate::components::image::Image;
use base64::encode;
use gloo::file::callbacks::FileReader;
use gloo::file::File;
use web_sys::{DragEvent, Event, FileList, HtmlInputElement};
use yew::html::TargetCast;
use yew::virtual_dom::AttrValue;
use yew::{html, use_context, Callback, Component, Context, Html, Properties};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileDetails {
    pub file_type: String,
    pub data: Vec<u8>,
}

pub enum Msg {
    Loaded(String, Vec<u8>),
    Files(Vec<File>),
}

pub struct FileInput {
    readers: HashMap<String, FileReader>,
    files_local: Vec<FileDetails>,
}

// #[derive(Clone, PartialEq, Properties)]
// pub struct ChildProps {
//     #[prop_or_default]
//     pub on_clicked: Callback<FileDetails>,
// }

impl Component for FileInput {
    type Message = Msg;
    type Properties = ();
    // type Properties = ChildProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            files_local: Vec::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file_type, data) => {
                // let file_type = file_type.clone();
                self.files_local.push(FileDetails { data, file_type });
                // self.readers.remove(&file_name);

                // let data = data.clone();

                // self.files_local.push(FileDetails { data, file_type });

                // let file_details_struct = FileDetails { file_type, data };

                // ctx.props().on_clicked.emit(file_details_struct);

                true
            }
            Msg::Files(files) => {
                for file in files.into_iter() {
                    let file_type = file.raw_mime_type();

                    let task = {
                        let link = ctx.link().clone();

                        gloo::file::callbacks::read_as_bytes(&file, move |res| {
                            link.send_message(Msg::Loaded(
                                file_type,
                                res.expect("failed to read file"),
                            ))
                        })
                    };
                    // self.readers.insert(file_name, task);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
                       <div id="wrapper">
                           <label for="file-upload">
                               <div
                               class="bg-gray-700"
                                   id="drop-container"
                                   ondrop={ctx.link().callback(|event: DragEvent| {
                                       event.prevent_default();
                                       let files = event.data_transfer().unwrap().files();
                                       Self::upload_files(files)

                                   })}
                                   ondragover={Callback::from(|event: DragEvent| {
                                       event.prevent_default();
                                   })}
                                   ondragenter={Callback::from(|event: DragEvent| {
                                       event.prevent_default();
                                   })}
                               >
        <svg
                ariaHidden="true"
                class="mb-3 w-10 h-10 text-gray-400"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth="10"
                  d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
                ></path>
              </svg>
            <p class="mb-2 text-sm text-white dark:text-white">
                       <span class="font-semibold">{"Click pentru a încărca imagini"}</span> {" sau trage imaginile în această zonă"}
                     </p>
                               </div>
                           </label>
                           <input
                               id="file-upload"
                               type="file"
                               accept="image/*"
                               multiple={true}
                               onchange={ctx.link().callback(move |e: Event| {
                                   let input: HtmlInputElement = e.target_unchecked_into();
                                   Self::upload_files(input.files())
                               })}
                           />
                           <div id="preview-area" class="flex flex-wrap relative right-72">
                               { for self.files_local.iter().map(Self::view_file) }
                           </div>
                       </div>
                   }
    }
}

impl FileInput {
    fn view_file(file: &FileDetails) -> Html {
        html! {

                <div class="preview-media">
                    if file.file_type.contains("image") {
                      <img src={format!("data:{};base64,{}", file.file_type, encode(&file.data))} />
                        // <Image  src={format!("data:{};base64,{}", file.file_type, encode(&file.data))} />
                    }
                </div>

        }
    }

    fn upload_files(files: Option<FileList>) -> Msg {
        let mut result = Vec::new();

        if let Some(files) = files {
            let files = js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from);
            result.extend(files);
        }
        Msg::Files(result)
    }
}
