// use crate::components::input_image_component::FileDetails;
use rexie::Rexie;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use yewdux::store::Store;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FileDetails {
    pub uuid: String,
    pub name: String,
    pub file_type: String,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PupilDetails {
    pub id: i32,
    pub name: String,
    pub photos: Vec<u8>,
}

impl PupilDetails {
    pub fn new(id: i32, name: String, photos: Vec<u8>) -> Self {
        PupilDetails { id, name, photos }
    }
}

#[derive(Default, Clone, PartialEq, Deserialize, Serialize, Store)]
#[store(storage = "session", storage_tab_sync)]
pub struct State {
    pub files: Vec<FileDetails>,
    pub pupils: Vec<PupilDetails>,
}
