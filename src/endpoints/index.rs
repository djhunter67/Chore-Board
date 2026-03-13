use std::fmt::Display;

use actix_web::{get, HttpResponse};
use askama::Template;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument};

use crate::endpoints::templates::{Chores, Index};

#[derive(Serialize, Deserialize)]
pub enum ChoreAssignee {
    Aleyet,
    Ajathyij,
    Abeyi,
    Achyi,
    Acobayi,
    Anwan,
    Alual,
    Aluel,
    Aping,
    Akol,
    Kaman,
    DeAnna,
    Christerpher,
}

impl Display for ChoreAssignee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let assignee = match self {
            ChoreAssignee::Aleyet => "Sudania",
            ChoreAssignee::Ajathyij => "Sonara",
            ChoreAssignee::Abeyi => "Sahara",
            ChoreAssignee::Achyi => "Samira",
            ChoreAssignee::Acobayi => "Safara",
            ChoreAssignee::Anwan => "Semian",
            ChoreAssignee::Alual => "Somara",
            ChoreAssignee::Aluel => "Samaia",
            ChoreAssignee::Aping => "Simidale",
            ChoreAssignee::Akol => "Sakeem",
            ChoreAssignee::Kaman => "Baba",
            ChoreAssignee::DeAnna => "Yuma",
            ChoreAssignee::Christerpher => "Sayfon",
        };
        write!(f, "{assignee}")
    }
}

#[derive(Serialize, Deserialize)]
pub enum ChoresList {
    KitchenAndGroceries,
    BreakfastAndBathroom,
    VacuumAndDiningTable,
    LunchAndGrassCutting,
    SinkCleanAndAnimalCare,
    DinnerAndBottleCleaning,
    ShoeCleanUp,
    TrashPickup,
    ClothesPickup,
    ToyCleanUp,
    TrashRemovalAndWaterAndMail,
    Dishes,
    VehicleRepair,
}

impl Display for ChoresList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chore = match self {
            ChoresList::KitchenAndGroceries => "Kitchen & Grocery put away",
            ChoresList::BreakfastAndBathroom => "Breakfast & Bathroom",
            ChoresList::VacuumAndDiningTable => "Vacuum & Dining Table",
            ChoresList::LunchAndGrassCutting => "Lunch & Grass Cutting",
            ChoresList::SinkCleanAndAnimalCare => "Sink & Animal Care",
            ChoresList::DinnerAndBottleCleaning => "Dinner & Bottle Cleaning",
            ChoresList::ShoeCleanUp => "Shoe Clean Up",
            ChoresList::TrashPickup => "Trash Pickup",
            ChoresList::ClothesPickup => "Clothes Pickup",
            ChoresList::ToyCleanUp => "Toy Clean Up",
            ChoresList::TrashRemovalAndWaterAndMail => "Trash Removal, Water, & Mail",
            ChoresList::Dishes => "Dishes",
            ChoresList::VehicleRepair => "Vehicle Repair",
        };
        write!(f, "{chore}")
    }
}

/// Display the tasks and assigned workers to each task
fn display_tasks(workers: ChoresList, chores: Chores) {
    println!("{} is assigned to {}", chores.name, workers);
}

#[get("/")]
#[instrument(name = "Index", level = "info")]
pub async fn index() -> HttpResponse {
    info!("Rendering the index page");

    // let preferred_names = preferred_names(name: ChoreAssignee);

    let chores: Vec<Chores> = vec![
        Chores::new(ChoresList::KitchenAndGroceries, ChoreAssignee::Alual),
        Chores::new(ChoresList::BreakfastAndBathroom, ChoreAssignee::Akol),
        Chores::new(ChoresList::VacuumAndDiningTable, ChoreAssignee::Anwan),
        Chores::new(ChoresList::LunchAndGrassCutting, ChoreAssignee::Aleyet),
        Chores::new(ChoresList::SinkCleanAndAnimalCare, ChoreAssignee::Achyi),
        Chores::new(ChoresList::DinnerAndBottleCleaning, ChoreAssignee::Aluel),
        Chores::new(ChoresList::ShoeCleanUp, ChoreAssignee::Aping),
        Chores::new(ChoresList::TrashPickup, ChoreAssignee::Abeyi),
        Chores::new(ChoresList::ClothesPickup, ChoreAssignee::Ajathyij),
        Chores::new(ChoresList::ToyCleanUp, ChoreAssignee::Acobayi),
        Chores::new(
            ChoresList::TrashRemovalAndWaterAndMail,
            ChoreAssignee::Christerpher,
        ),
        Chores::new(ChoresList::Dishes, ChoreAssignee::DeAnna),
        Chores::new(ChoresList::VehicleRepair, ChoreAssignee::Kaman),
    ];

    let chores = rotate_tasks(&chores);

    let template = Index {
        title: "Chore Tracker".to_string(),
        chores,
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

fn rotate_tasks(chores: &[Chores]) -> Vec<Chores> {
    let mut rotated_chores = chores.to_vec();
    rotated_chores.rotate_left(1);
    rotated_chores
}

fn preferred_names(name: ChoreAssignee, chores: Vec<Chores>) -> String {
    format!("{}", name);

    for chore in &chores {
        display_tasks(chore.name, chore.assigned_to);
    }

    info!("Chores and assignments displayed");
    let template = Index {
        title: "Chore Tracker".to_string(),
        chores,
    };
    debug!("Rendering the main page");
    let body = match template.render() {
        Ok(body) => body,
        Err(err) => {
            error!("Error rendering template: {err:#?}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().content_type("text/html")
}
