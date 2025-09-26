use std::fmt::Display;

use actix_web::{get, HttpResponse};
use askama::Template;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument};

use crate::endpoints::templates::{Chores, Index};

#[derive(Serialize, Deserialize)]
pub enum ChoreAssignee {
    Alieyet,
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
            ChoreAssignee::Alieyet => "Sudania",
            ChoreAssignee::Ajathyij => "Sonara",
            ChoreAssignee::Abeyi => "Sahara",
            ChoreAssignee::Achyi => "Samira",
            ChoreAssignee::Acobayi => "Safara",
            ChoreAssignee::Anwan => "Simeon",
            ChoreAssignee::Alual => "Somara",
            ChoreAssignee::Aluel => "Samaia",
            ChoreAssignee::Aping => "Simadaly",
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
    SinkCleanAndAnimalCare,
    KitchenAndGroceries,
    BreakfastAndBathroom,
    VacuumAndDiningTable,
    LunchAndGrassCutting,
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
            ChoresList::SinkCleanAndAnimalCare => "Sink & Animal Care",
            ChoresList::KitchenAndGroceries => "Kitchen & Grocery put away",
            ChoresList::BreakfastAndBathroom => "Breakfast & Bathroom",
            ChoresList::VacuumAndDiningTable => "Vacuum & Dining Table",
            ChoresList::LunchAndGrassCutting => "Lunch & Grass Cutting",
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

#[get("/")]
#[instrument(name = "Index", level = "info")]
pub async fn index() -> HttpResponse {
    info!("Rendering the index page");
    let template = Index {
        title: "Chore Tracker".to_string(),
        chores: vec![
            Chores::new(ChoresList::SinkCleanAndAnimalCare, ChoreAssignee::Achyi),
            Chores::new(ChoresList::KitchenAndGroceries, ChoreAssignee::Alual),
            Chores::new(ChoresList::BreakfastAndBathroom, ChoreAssignee::Akol),
            Chores::new(ChoresList::VacuumAndDiningTable, ChoreAssignee::Anwan),
            Chores::new(ChoresList::LunchAndGrassCutting, ChoreAssignee::Alieyet),
            Chores::new(ChoresList::DinnerAndBottleCleaning, ChoreAssignee::Aluel),
            Chores::new(ChoresList::ShoeCleanUp, ChoreAssignee::Aping),
            Chores::new(ChoresList::TrashPickup, ChoreAssignee::Abeyi),
            Chores::new(ChoresList::ClothesPickup, ChoreAssignee::Ajathyij),
            Chores::new(ChoresList::ToyCleanUp, ChoreAssignee::Acobayi),
            Chores::new(
                ChoresList::TrashRemovalAndWaterAndMail,
                ChoreAssignee::DeAnna,
            ),
            Chores::new(ChoresList::Dishes, ChoreAssignee::DeAnna),
            Chores::new(ChoresList::VehicleRepair, ChoreAssignee::Kaman),
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
