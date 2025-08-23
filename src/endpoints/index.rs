use actix_web::{get, HttpResponse};
use askama::Template;
use tracing::{debug, error, info, instrument};

use crate::templates::{Chores, Index};

#[get("/")]
#[instrument(name = "Main page", level = "debug")]
pub async fn index() -> HttpResponse {
    info!("Rendering the index page");
    let template = Index {
        title: "Chore Tracker".to_string(),
        chores: vec![
            Chores::new(
                String::from("Sink and Animal Care"),
                String::from("Achai"),
                String::from("Today"),
                String::from("Not Completed"),
            ),
            Chores::new(
                String::from("Kitchen & Grocery put away"),
                String::from("Aluel"),
                String::from("Tomorrow"),
                String::from("Not Completed"),
            ),
            Chores::new(
                String::from("Vehicle Repair"),
                String::from("Baba"),
                String::from("Tomorrow"),
                String::from("Not Completed"),
            ),
        ],
    };

    debug!("rendering the main page");
    let body = match template.render() {
        Ok(body) => body,
        Err(err) => {
            error!("Error rendering template: {err:#?}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .append_header(("Authorization", "Bearer token"))
        .body(body)
}
