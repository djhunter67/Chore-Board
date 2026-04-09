// //! This file is where the database helper functions are defined.

// use serde::{Deserialize, Serialize};
// use tracing::error;
// use tracing::instrument;

// #[derive(Deserialize, Serialize, Debug, Clone)]
// pub struct CreateNewUser {
//     pub email: String,
//     pub password: String,
//     pub password_2: String,
//     pub first_name: String,
//     pub last_name: String,
// }

// /// # Results
// ///   - Returns an `Ok` if the document is successfully inserted into the table
// /// # Errors
// ///   - Returns an `Error` if the document fails to insert into the table
// /// # Panics
// ///   - If the document fails to insert into the table, it will panic with the error message
// #[instrument(
//     name = "Create user",
//     level = "debug",
//     target = "kid_data",
//     skip(new_user)
// )]
// pub async fn create_user(
//     conn: rusqlite::Connection,
//     new_user: CreateNewUser,
// ) -> Result<(), rusqlite::Error> {
//     // Check if passwords match and meet length requirements
//     check_pw(&new_user.password, &new_user.password_2).map_err(|e| {
//         error!("Password validation failed: {}", e);
//         rusqlite::Error::InvalidParameterName(e)
//     })?;

//     let mut stmt = conn.prepare(
//         "INSERT INTO users (email, password, first_name, last_name) VALUES (?1, ?2, ?3, ?4)",
//     )?;

//     let result = stmt.execute((
//         new_user.email,
//         new_user.password,
//         new_user.first_name,
//         new_user.last_name,
//     ));

//     match result {
//         Ok(_) => Ok(()),
//         Err(e) => {
//             error!("Failed to insert user: {}", e);
//             Err(rusqlite::Error::ExecuteReturnedResults)
//         }
//     }
// }

// fn check_pw(password: &str, password_2: &str) -> Result<(), String> {
//     if password != password_2 {
//         return Err("Passwords do not match".to_string());
//     }
//     if password.len() < 8 {
//         return Err("Password must be at least 8 characters long".to_string());
//     }
//     Ok(())
// }
