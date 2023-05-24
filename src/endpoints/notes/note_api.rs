use serde::{Deserialize, Serialize};

use crate::db;

#[derive(Serialize)]
pub struct NoteResponse {
    pub note_id: i32,
    pub note_title: String,
    pub note_content: String,
    pub note_public: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NewNote {
    pub account_id: i32,
    pub note_title: String,
    pub note_content: String,
    pub note_public: bool,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateNote {
    pub note_id: i32,
    pub note_title: String,
    pub note_content: String,
    pub note_public: bool,
}

impl From<&db::Note> for NoteResponse {
    fn from(value: &db::Note) -> Self {
        NoteResponse {
            note_id: value.note_id,
            note_title: value.title.clone(),
            note_content: value.content.clone(),
            note_public: value.public.clone(),
        }
    }
}
