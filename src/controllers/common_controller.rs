use actix_web::{
    Error
};
use actix_web::http::StatusCode;
use actix_files as fs;

pub fn p404() -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}