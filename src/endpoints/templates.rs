use std::{fmt::Display, path::PathBuf};

use actix_files::NamedFile;
use actix_web::{HttpResponse, Responder, get};
use askama::Template;
use serde::{Deserialize, Serialize};
use tracing::{error, info, instrument};

#[derive(Serialize, Deserialize, Clone)]
pub struct Chores {
    pub name: ChoresList,
    pub assigned_to: (ChoreAssignee, u8),
}

impl Iterator for Chores {
    type Item = Vec<ChoreAssignee>;

    fn next(&mut self) -> Option<Vec<ChoreAssignee>> {
        let chorees = vec![
            ChoreAssignee::Aleyet,
            ChoreAssignee::Ajathyij,
            ChoreAssignee::Abeyi,
            ChoreAssignee::Achyi,
            ChoreAssignee::Acobayi,
            ChoreAssignee::Anwan,
            ChoreAssignee::Alual,
            ChoreAssignee::Aluel,
            ChoreAssignee::Aping,
            ChoreAssignee::Akol,
            ChoreAssignee::Kaman,
            ChoreAssignee::DeAnna,
            ChoreAssignee::Christerpher,
        ];

        Some(chorees)
    }
}

impl Chores {
    #[must_use]
    pub fn default_vec() -> Vec<Self> {
        vec![
            Self::new(ChoresList::KitchenAndGroceries, (ChoreAssignee::Alual, 0)),
            Self::new(ChoresList::BreakfastAndBathroom, (ChoreAssignee::Akol, 0)),
            Self::new(ChoresList::VacuumAndDiningTable, (ChoreAssignee::Anwan, 0)),
            Self::new(ChoresList::LunchAndGrassCutting, (ChoreAssignee::Aleyet, 0)),
            Self::new(
                ChoresList::SinkCleanAndAnimalCare,
                (ChoreAssignee::Achyi, 0),
            ),
            Self::new(
                ChoresList::DinnerAndBottleCleaning,
                (ChoreAssignee::Aluel, 0),
            ),
            Self::new(ChoresList::ShoeCleanUp, (ChoreAssignee::Aping, 0)),
            Self::new(ChoresList::TrashPickup, (ChoreAssignee::Abeyi, 0)),
            Self::new(ChoresList::ClothesPickup, (ChoreAssignee::Ajathyij, 0)),
            Self::new(ChoresList::ToyCleanUp, (ChoreAssignee::Acobayi, 0)),
            Self::new(
                ChoresList::TrashRemovalAndWaterAndMail,
                (ChoreAssignee::Christerpher, 0),
            ),
            Self::new(ChoresList::Dishes, (ChoreAssignee::DeAnna, 0)),
            Self::new(ChoresList::VehicleRepair, (ChoreAssignee::Kaman, 0)),
        ]
    }

    #[must_use]
    pub const fn new(name: ChoresList, assigned_to: (ChoreAssignee, u8)) -> Self {
        Self { name, assigned_to }
    }

    #[must_use]
    pub fn to_display(self) -> String {
        format!(
            "{} - {}: {}",
            self.name, self.assigned_to.0, self.assigned_to.1
        )
    }
}

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
            Self::Ajathyij => "Sanara",
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

// Derive the ChoreAssignee from a string, for example "Aleyet" should return ChoreAssignee::Aleyet
impl ChoreAssignee {
    #[must_use]
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "Sudania" => Some(Self::Aleyet),
            "Sanara" => Some(Self::Ajathyij),
            "Sahara" => Some(Self::Abeyi),
            "Samira" => Some(Self::Achyi),
            "Safara" => Some(Self::Acobayi),
            "Semian" => Some(Self::Anwan),
            "Somara" => Some(Self::Alual),
            "Samaia" => Some(Self::Aluel),
            "Simidale" => Some(Self::Aping),
            "Sakeem" => Some(Self::Akol),
            "Baba" => Some(Self::Kaman),
            "Yuma" => Some(Self::DeAnna),
            "Sayfon" => Some(Self::Christerpher),
            _ => None,
        }
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

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub title: String,
    pub chores: Vec<ChoresList>,
    pub assignees: Vec<ChoreAssignee>,
    pub points: Vec<u8>,
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
#[template(path = "rotate.html")]
pub struct RotateAssignee {
    pub chores: Vec<ChoresList>,
    pub assignees: Vec<ChoreAssignee>,
    pub points: Vec<u8>,
}

#[get("/favicon")]
#[instrument(name = "Favicon", level = "info", target = "chore_tracker")]
async fn favicon() -> impl Responder {
    info!("Serving favicon");
    let file = include_str!("../../static/imgs/education.svg");
    HttpResponse::Ok().content_type("icon").body(file)
}

#[get("/stylesheet")]
#[instrument(name = "Stylesheet", level = "info", target = "chore_tracker")]
async fn stylesheet() -> impl Responder {
    info!("Serving stylesheet");
    let file = include_str!("../../static/css/style.css");
    HttpResponse::Ok().content_type("text/css").body(file)
}

#[get("/style.css.map")]
#[instrument(name = "Source map", level = "info", target = "chore_tracker")]
async fn source_map() -> impl Responder {
    info!("Serving source map");
    let file = include_str!("../../static/css/style.css.map");
    HttpResponse::Ok()
        .content_type("application/json")
        .body(file)
}

#[get("/htmx")]
#[instrument(name = "Htmx", level = "info", target = "chore_tracker")]
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
#[instrument(name = "Response targets", level = "info", target = "chore_tracker")]
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
