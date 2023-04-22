use serde::{Serialize, Deserialize};

use crate::db;

#[derive(Serialize)]
pub struct NoteFile {
    pub note_id: i32,
    pub note_url: String,
    pub note_title: String,
}

impl From<&db::Note> for NoteFile {
    fn from(value: &db::Note) -> Self {
        NoteFile {
            note_id: value.note_id,
            note_title: value.title.clone(),
            note_url: value.url.clone()
        }
    }
}
