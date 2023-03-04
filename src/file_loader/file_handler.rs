use crate::document::Document;
use crate::error::HectaError;
use crate::file_loader::file_extensions;
use std::io::Write;
use std::{fs::File, io::Read};

use super::FileExtensions;

pub fn save_to_curr_dir(
    ex: file_extensions::FileExtensions,
    doc: &Document,
) -> Result<(), HectaError> {
    let s = doc.rows.join("\n");
    let ex = match ex {
        FileExtensions::MD => ".md",
        FileExtensions::TXT => ".txt",
    };
    let mut f = File::create(format!("./{}{}", doc.title, ex))?;
    f.write_all(s.as_bytes())?;
    Ok(())
}

pub fn load_file_to_doc(path: &str) -> Result<Document, HectaError> {
    let mut file = File::open(path)?;
    let mut s = String::new();
    file.read_to_string(&mut s);
    Ok(Document::from(s))
}
