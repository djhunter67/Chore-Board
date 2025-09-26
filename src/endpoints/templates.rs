use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{get, HttpResponse, Responder};
use askama::Template;
use serde::{Deserialize, Serialize};
use tracing::{error, info, instrument};

use super::index::{ChoreAssignee, ChoresList};

#[derive(Serialize, Deserialize)]
pub struct Chores {
    pub name: ChoresList,
    pub assigned_to: ChoreAssignee,
}

impl Chores {
    pub fn new(name: ChoresList, assigned_to: ChoreAssignee) -> Self {
        Self { name, assigned_to }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub title: String,
    pub chores: Vec<Chores>,
}

#[derive(Template)]
#[template(path = "errors.html")]
pub struct ErrorPage<'a> {
    pub title: &'a str,
    pub code: u16,
    pub error: &'a str,
    pub message: &'a str,
}

#[derive(Template)]
#[template(path = "email.html")]
pub struct EmailPage {
    pub title: String,
    pub confirmation_link: String,
    pub domain: String,
    pub expiration_time: String,
    pub exact_time: String,
}

#[derive(Template)]
#[template(path = "school.html")]
pub struct School<'a> {
    pub title: &'a str,
    pub name: &'a str,
}

#[get("/favicon")]
#[instrument(name = "Favicon", level = "info", target = "kid_data")]
async fn favicon() -> impl Responder {
    info!("Serving favicon");
    let file = include_str!("../../static/imgs/education.svg");
    HttpResponse::Ok().content_type("icon").body(file)
}

#[get("/stylesheet")]
#[instrument(name = "Stylesheet", level = "info", target = "kid_data")]
async fn stylesheet() -> impl Responder {
    info!("Serving stylesheet");
    let file = include_str!("../../static/css/style.css");
    HttpResponse::Ok().content_type("text/css").body(file)
}

#[get("/style.css.map")]
#[instrument(name = "Source map", level = "info", target = "kid_data")]
async fn source_map() -> impl Responder {
    info!("Serving source map");
    let file = include_str!("../../static/css/style.css.map");
    HttpResponse::Ok()
        .content_type("application/json")
        .body(file)
}

#[get("/htmx")]
#[instrument(name = "Htmx", level = "info", target = "kid_data")]
async fn htmx() -> Result<NamedFile, actix_web::Error> {
    info!("Serving htmx.min.js");
    let path: PathBuf = ["static", "assets", "htmx", "htmx.min.js"].iter().collect();
    match NamedFile::open(path) {
        Ok(file) => Ok(file),
        Err(err) => {
            error!("Error opening file: {err:#?}");
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

#[get("/response-targets")]
#[instrument(name = "Response targets", level = "info", target = "kid_data")]
async fn response_targets() -> Result<NamedFile, actix_web::Error> {
    info!("Serving response-targets.js");
    let pash: PathBuf = ["static", "assets", "htmx", "response-targets.js"]
        .iter()
        .collect();
    match NamedFile::open(pash) {
        Ok(file) => Ok(file),
        Err(err) => {
            error!("Error opening file: {err:#?}");
            Err(actix_web::error::ErrorInternalServerError(err))
        }
    }
}

#[get("/health_check")]
#[instrument(name = "Health check", level = "info")]
pub async fn health_check() -> impl Responder {
    info!("Health check endpoint called.");
    HttpResponse::Ok().json("I'm alive!")
}
