use actix_web::{HttpResponse, get};
use askama::Template;
use tracing::{debug, error, info, instrument};

use crate::endpoints::templates::{ChoreAssignee, Chores, ChoresList, Index};

/// Display the tasks and assigned workers to each task
fn _display_tasks(workers: &Chores, chores: ChoresList) {
    println!("{} is assigned to {}", workers.name, chores);
}

#[get("/")]
#[instrument(name = "Index", level = "info")]
pub async fn index() -> HttpResponse {
    info!("Rendering the index page");

    let default_chores: Vec<Chores> = Chores::default_vec();

    let template = Index {
        title: "Chore Tracker".to_string(),
        chores: default_chores
            .iter()
            .map(|chore| chore.name)
            .collect::<Vec<ChoresList>>(),
        assignees: default_chores
            .iter()
            .map(|chore| chore.assigned_to.0.clone())
            .collect::<Vec<ChoreAssignee>>(),
        points: [6; 20].to_vec(),
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
