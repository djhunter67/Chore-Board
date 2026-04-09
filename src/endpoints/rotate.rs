use std::collections::HashMap;

use actix_web::{HttpResponse, post, web::Data};
use askama::Template;
use deadpool_redis::redis::AsyncCommands;
use tracing::{debug, error, info, instrument, warn};

use crate::endpoints::templates::{ChoreAssignee, Chores, ChoresList, RotateAssignee};

#[post("/rotate_choree")]
#[instrument(name = "Rotate", level = "info", skip(red_conn))]
pub async fn rotate(red_conn: Data<deadpool_redis::Pool>) -> HttpResponse {
    info!("Rotating the assignees");

    let mut red_conn = match red_conn.get().await {
        Ok(red_conn) => red_conn,
        Err(err) => {
            error!("Error getting Redis red_connection: {err:#?}");
            return HttpResponse::InternalServerError().json("Error red_connecting to Redis");
        }
    };

    let default_chores: Vec<Chores> = Chores::default_vec();

    let mut chores: Vec<Chores> = Vec::new();

    'redis_tracking: {
        if let Ok(exists) = red_conn.exists::<&str, bool>("chore_tracker").await {
            debug!("Redis key 'chores_order' and points exists: {exists}");
            // Actions upon the existence of the keys in Redis
            if exists {
                // get the chores order and points from Redis

                chores = rotate_assignees(red_conn, chores, &default_chores).await;
            } else {
                error!(
                    "DB keys 'chores_order' and 'points' are not found, setting it to the default order of chores"
                );

                // Save the default values if no key is in the database
                // let _: () = red_conn
                //     .hset_multiple(
                //         "assignee:points",
                //         &default_chores
                //             .iter()
                //             .map(|chore| {
                //                 (chore.assigned_to.0.clone().to_string(), chore.assigned_to.1)
                //             })
                //             .collect::<Vec<(String, u8)>>(),
                //     )
                //     .await
                //     .expect("Unable to save the hset");

                for data in &default_chores {
                    let _: () = red_conn
                        .zadd(
                            "chore_tracker",
                            data.assigned_to.0.to_string(),
                            data.assigned_to.1,
                        )
                        .await
                        .expect("Failed to ZADD redis data");
                }

                // Use the default list if no list is found in the DB
                chores = default_chores.clone();

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
            .map(|chore| chore.assigned_to.0.clone())
            .collect::<Vec<ChoreAssignee>>(),
        points: chores
            .iter()
            .map(|chore| chore.assigned_to.1)
            .collect::<Vec<u8>>(),
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

fn rotate_assigned_to(chores: &mut [Chores]) -> Vec<Chores> {
    // let mut assignee_names: Vec<(ChoreAssignee, u8)> = chores
    // .iter()
    // .map(|chore| chore.assigned_to.clone())
    // .collect();

    // let mut rotated_names = chore_names;
    // assignee_names.rotate_right(1);
    chores.rotate_right(1);

    // let _ = chores
    //     .iter()
    //     .enumerate()
    //     .map(|(i, chore)| Chores {
    //         name: chore.name,
    //         assigned_to: assignee_names
    //             .get(i)
    //             .expect("Failed to query the vector index")
    //             .clone(),
    //     })
    //     .collect::<Vec<Chores>>();

    chores.to_vec()
}

async fn rotate_assignees(
    mut red_conn: deadpool_redis::Connection,
    mut chores: Vec<Chores>,
    default_chores: &[Chores],
) -> Vec<Chores> {
    debug!("Retrieving chore order from Redis");

    // let red_data: HashMap<String, u8> = red_conn
    //     .hgetall("assignee:points")
    //     .await
    //     .expect("Failure to hgetall");

    let red_data: Vec<(String, u8)> = red_conn
        .zrange_withscores("chore_tracker", 0, -1)
        .await
        .expect("Failed to query redis data");

    warn!("The HGETALL data: {red_data:#?}");

    let (name_order, points): (Vec<String>, Vec<u8>) = red_data.into_iter().fold(
        (Vec::new(), Vec::new()),
        |(mut names, mut points), (name, point)| {
            names.push(name);
            points.push(point);
            (names, points)
        },
    );

    // At this point populate the 'chores' vec with the redis data
    chores.extend(default_chores.iter().map(|chore| {
        let index = name_order
            .iter()
            .position(|name| name == &chore.assigned_to.0.to_string())
            .expect("Failed to find the index of the chore assignee in the name order vector");

        Chores {
            name: chore.name,
            assigned_to: (
                ChoreAssignee::from_string(
                    &name_order
                        .get(index)
                        .expect("Failed to query the name order vector")
                        .clone(),
                )
                .expect("Failed to convert the name string to a ChoreAssignee enum"),
                *points
                    .get(index)
                    .expect("Failed to query the points vector"),
            ),
        }
    }));

    warn!(
        "Chores before rotation: {:#?}",
        chores
            .iter()
            .map(|chores| chores.assigned_to.0.to_string())
            .collect::<Vec<String>>()
    );
    chores = rotate_assigned_to(&mut chores);
    warn!(
        "Rotated the chore order for the next week: {:#?}",
        chores
            .iter()
            .map(|chore| chore.assigned_to.0.to_string())
            .collect::<Vec<String>>()
    );

    // Save the new rotation in  redis
    // let _: () = red_conn
    //     .hset_multiple(
    //         "assignee:points",
    //         &name_order
    //             .iter()
    //             .zip(points.iter())
    //             .map(|(name, point)| (name.clone(), *point))
    //             .collect::<Vec<(String, u8)>>(),
    //     )
    //     .await
    //     .expect("Unable to save the hset");
    // let _: () = red_conn

    for (i, data) in name_order.iter().enumerate() {
        let _: () = red_conn
            .zadd(
                "chore_tracker",
                data,
                points.get(i).expect("no index at value"),
            )
            .await
            .expect("Failed to ZADD redis data");
    }

    chores
}
