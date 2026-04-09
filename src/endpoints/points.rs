use actix_web::{get, web::Data, HttpResponse};
use deadpool_redis::{redis::AsyncCommands, Connection};
use tracing::{error, info, instrument, warn};

#[get("/points")]
#[instrument(name = "Points", level = "info", skip(conn))]
pub async fn points(conn: Data<deadpool_redis::Pool>) -> HttpResponse {
    info!("Adding 1 point to the database");

    let mut conn: Connection = match conn.get().await {
        Ok(conn) => conn,
        Err(err) => {
            error!("Error getting the Redis connection {err:#?}");
            return HttpResponse::InternalServerError().json("Error connecting to Redis");
        }
    };

    // Get the 'points' array and add 1 to the value
    let points: Vec<String> = match conn.lrange("points", 0, -1).await {
        Ok(points) => {
            warn!("Points retrieved from Redis: {points:#?}");
            points
        }
        Err(err) => {
            error!("Error retrieving points from Redis: {err:#?}");
            return HttpResponse::InternalServerError().json("Error retrieving points from Redis");
        }
    };

    // Convert "\0\0\0\0\0\0\0\0\0\0\0\0\0" to individual strings and add 1 to each value.
    let points_vec: Vec<&str> = points
        .first()
        .expect("unable to get first element")
        .split('\0')
        .fold(Vec::new(), |mut acc, point| {
            if !point.is_empty() {
                acc.push(point);
            }
            acc
        });

    // Convert "\0\0\0\0\0\0\0\0\0\0\0\0\0" to an array of integers and add 1 to each value.
    warn!("The points that are converted to string: {points_vec:#?}");

    // let converted_points: Vec<u8> = points_vec
    //     .iter()
    //     .filter_map(|point| point.parse::<u8>().ok())
    //     .map(|num| num + 1)
    //     .collect::<Vec<u8>>();

    // warn!("The points that are converted to integer and added 1: {converted_points:#?}");

    HttpResponse::Ok().finish()
}
