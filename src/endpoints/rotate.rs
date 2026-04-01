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
    let mut points: Vec<u8> = [0; 10].to_vec();

    'redis_tracking: {
        if let Ok(exists) = conn.exists::<&str, bool>("chores_order").await && let Ok (exists_points) = conn.exists::<&str, bool>("points").await {
            debug!("Redis key 'chores_order' and points exists: {exists} and {exists_points}");
            // Actions upon the existence of the keys in Redis
            if exists && exists_points {
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

                points = match conn.lrange("points", 0, -1).await {
                    Ok(points) => {
                        warn!("Getting the points for each child");
                        points
                    }
                    Err(err) => {
                        error!("Points retrieval from Redis is invalid: {err:#?}");
                        return HttpResponse::InternalServerError()
                            .json("Error retrieving the Points from Redis");
                    }
                };

		info!("Chore order retrieved from Redis: {name_order:#?}");
                // Fill in the chores vector with the chores in the order of the names retrieved from Redis, if the name matches the assigned_to field of the database values vector
                chores.extend(name_order.clone().iter().filter_map(|name| {
                    default_chores
                        .iter()
                        .find(|chore| chore.assigned_to.to_string() == *name)
                        .cloned()
                }));
                
                // If the assigned_to of the chores vector is identical to the db chores assigned_to values, rotate the order of the chores for the next week
                if chores
                    .iter()
                    .map(|chore| chore.assigned_to.to_string())
                    .collect::<Vec<String>>()
                    == name_order.into_iter().collect::<Vec<String>>()
                {
                    info!("Chore order is the same as the default order, rotating the chore order for the next week");
                    let alt_chores = chores
                        .iter()
                        .map(|chore| Chores::new(chore.name, chore.assigned_to.clone()))
                        .collect::<Vec<Chores>>();

                    chores = rotate_assigned_to(&alt_chores);
                    debug!(
                        "Rotated the chore order for the next week: {:#?}",
                        chores
                            .iter()
                            .map(|chore| chore.assigned_to.to_string())
                            .collect::<Vec<String>>()
                    );

                    // Update the database in Redis with the new order of the chores, by deleting the existing key and setting it to the new order of the chores

                    conn.del("chores_order").await.unwrap_or_else(|err| {
                        error!("Error deleting Redis key: {err:#?}");
                    });
		    conn.del("points").await.unwrap_or_else(|err| {
			error!("Error deleting the points key: {err:#?}");
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

		    conn.rpush(
			"points",
			points.iter().copied().filter(u8::is_ascii_digit).collect::<Vec<u8>>()
		    ).await.unwrap_or_else(|err| {
			error!("Error setting Redis key for the Assignee's points: {err:#?}");
		    });


                } else {
                    warn!(
                    "Chore order is different from the default order, not rotating the chore order"
                );
                }
            } else {
                error!("Redis key 'chores_order' and 'points' does not exist, setting it to the default order of chores");
                let zero_vec: Vec<u8> = vec![0; default_chores.len()];
                // let request = VectorSetAddRequest::Member("points", zero_vector);

                conn.rpush("points", zero_vec).await.unwrap_or_else(|err| {
                    error!("Error setting Redis key for the Assignee's points: {err:#?}");
                });

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
                points = vec![0; default_chores.len()];

                break 'redis_tracking;
            }
        }
    }

    let template = RotateAssignee {
        chores: default_chores
            .iter()
            .map(|chore| chore.name)
            .collect::<Vec<ChoresList>>(),
        assignees: chores
            .iter()
            .map(|chore| chore.assigned_to.clone())
            .collect::<Vec<ChoreAssignee>>(),
        points,
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
