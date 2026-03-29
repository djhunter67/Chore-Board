use actix_web::{post, web::Data, HttpResponse};
use askama::Template;
use deadpool_redis::{self, redis::AsyncCommands};
use tracing::{debug, error, info, instrument, warn};

use crate::endpoints::{
    index::{rotate_assigned_to, ChoreAssignee, ChoresList},
    templates::{Chores, RotateAssignee},
};

#[post("/rotate_choree")]
#[instrument(name = "Rotate", level = "info", skip(conn))]
pub async fn rotate(conn: Data<deadpool_redis::Pool>) -> HttpResponse {
    info!("Rotating the assignees");

    let mut conn = match conn.get().await {
        Ok(conn) => conn,
        Err(err) => {
            error!("Error getting Redis connection: {err:#?}");
            return HttpResponse::InternalServerError().json("Error connecting to Redis");
        }
    };

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

    let mut chores: Vec<Chores> = Vec::new();

    'redis_tracking: {
        if let Ok(exists) = conn.exists::<&str, bool>("chores_order").await {
            debug!("Redis key 'chores_order' exists: {exists}");
            // Get the chore order
            if exists {
                debug!("Retrieving chore order from Redis");
                let name_order: Vec<String> = match conn.lrange("chores_order", 0, -1).await {
                    Ok(orders) => {
                        debug!("Chore order retrieved from Redis: {orders:#?}");
                        orders
                    }
                    Err(err) => {
                        error!("Error retrieving chore order from Redis: {err:#?}");
                        return HttpResponse::InternalServerError()
                            .json("Error retrieving chore order from Redis");
                    }
                };

                // Fill in the chores vector with the chores in the order of the names retrieved from Redis, if the name matches the assigned_to field of the database values vector
                chores.extend(name_order.clone().iter().filter_map(|name| {
                    default_chores
                        .iter()
                        .find(|chore| chore.assigned_to.to_string() == *name)
                        .cloned()
                }));

                info!("Chore order retrieved from Redis: {name_order:#?}");
                // If the assigned_to of the chores vector is identical to the db chores assigned_to values, rotate the order of the chores for the next week
                if chores
                    .iter()
                    .map(|chore| chore.assigned_to.to_string())
                    .collect::<Vec<String>>()
                    == name_order.into_iter().collect::<Vec<String>>()
                {
                    info!("Chore order is the same as the default order, rotating the chore order for the next week");
                    chores = rotate_assigned_to(&chores);
                    debug!(
                        "Rotated the chore order for the next week: {:#?}",
                        chores
                            .iter()
                            .map(|chore| chore.assigned_to.to_string())
                            .collect::<Vec<String>>()
                    );

                    // Save name order to the database
                    conn.del("chores_order").await.unwrap_or_else(|err| {
                        error!("Error deleting Redis key: {err:#?}");
                    });
                    conn.rpush(
                        "chores_order",
                        chores
                            .iter()
                            .map(|chore| chore.assigned_to.to_string())
                            .collect::<Vec<String>>(),
                    )
                    .await
                    .unwrap_or_else(|err| {
                        error!("Error setting Redis key: {err:#?}");
                    });
                } else {
                    warn!(
                    "Chore order is different from the default order, not rotating the chore order"
                );
                }
            } else {
                error!("Redis key 'chores_order' does not exist, setting it to the default order of chores");
                conn.rpush(
                    "chores_order",
                    default_chores
                        .iter()
                        .map(|chore| chore.assigned_to.to_string())
                        .collect::<Vec<String>>(),
                )
                .await
                .unwrap_or_else(|err| {
                    error!("Error setting Redis key: {err:#?}");
                });

                chores = default_chores.clone();

                break 'redis_tracking;
            }
        }
    }

    let template = RotateAssignee {
        chores: default_chores
            .iter()
            .map(|chore| chore.name.clone())
            .collect::<Vec<ChoresList>>(),
        assignees: chores
            .iter()
            .map(|chore| chore.assigned_to.clone())
            .collect::<Vec<ChoreAssignee>>(),
    };

    let body = match template.render() {
        Ok(body) => body,
        Err(err) => {
            info!("Error rendering template: {err:#?}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().content_type("text/html").body(body)
}
