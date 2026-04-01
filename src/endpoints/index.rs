use actix_web::{get, HttpResponse};
use askama::Template;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tracing::{debug, error, info, instrument};

use crate::endpoints::templates::{Chores, Index};

#[derive(Serialize, Deserialize, Clone)]
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
            Self::Aleyet => "Sudania",
            Self::Ajathyij => "Sonara",
            Self::Abeyi => "Sahara",
            Self::Achyi => "Samira",
            Self::Acobayi => "Safara",
            Self::Anwan => "Semian",
            Self::Alual => "Somara",
            Self::Aluel => "Samaia",
            Self::Aping => "Simidale",
            Self::Akol => "Sakeem",
            Self::Kaman => "Baba",
            Self::DeAnna => "Yuma",
            Self::Christerpher => "Sayfon",
        };
        write!(f, "{assignee}")
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
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
            Self::KitchenAndGroceries => "Kitchen & Grocery put away",
            Self::BreakfastAndBathroom => "Breakfast & Bathroom",
            Self::VacuumAndDiningTable => "Vacuum & Dining Table",
            Self::LunchAndGrassCutting => "Lunch & Grass Cutting",
            Self::SinkCleanAndAnimalCare => "Sink & Animal Care",
            Self::DinnerAndBottleCleaning => "Dinner & Bottle Cleaning",
            Self::ShoeCleanUp => "Shoe Clean Up",
            Self::TrashPickup => "Trash Pickup",
            Self::ClothesPickup => "Clothes Pickup",
            Self::ToyCleanUp => "Toy Clean Up",
            Self::TrashRemovalAndWaterAndMail => "Trash Removal, Water, & Mail",
            Self::Dishes => "Dishes",
            Self::VehicleRepair => "Vehicle Repair",
        };
        write!(f, "{chore}")
    }
}

/// Display the tasks and assigned workers to each task
fn _display_tasks(workers: &Chores, chores: ChoresList) {
    println!("{} is assigned to {}", workers.name, chores);
}

#[get("/")]
#[instrument(name = "Index", level = "info")]
pub async fn index() -> HttpResponse {
    info!("Rendering the index page");

    // Get the Redis connection from the deadredis pool passed into the function
    info!("Establishing the connection to Redis");

    info!("Redis connection established");

    let default_chores: Vec<Chores> = vec![
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

    let template = Index {
        title: "Chore Tracker".to_string(),
        chores: default_chores
            .iter()
            .map(|chore| chore.name)
            .collect::<Vec<ChoresList>>(),
        assignees: default_chores
            .iter()
            .map(|chore| chore.assigned_to.clone())
            .collect::<Vec<ChoreAssignee>>(),
        points: [0; 20].to_vec(),
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

#[must_use]
pub fn rotate_assigned_to(chores: &[Chores]) -> Vec<Chores> {
    let mut rotated_chores = chores.to_vec();
    rotated_chores.rotate_right(1);
    rotated_chores
}
