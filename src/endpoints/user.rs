use actix_web::{
    delete, get, put,
    web::{Data, Json},
    HttpResponse, Responder,
};
use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};
use rusqlite::{Connection, ErrorCode};
use serde::{Deserialize, Serialize};
use tracing::{error, info, instrument};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

/// Endpoint to create a new user.
/// Returns 201 Created if the user is successfully created, otherwise returns an appropriate error code.
#[put("/user")]
#[instrument(name = "create_user", level = "info", target = "chore_tracker")]
pub async fn create_user() -> impl Responder {
    HttpResponse::Created().finish()
}

#[get("/user/{email}")]
#[instrument(
    name = "get_user",
    level = "info",
    target = "chore_tracker",
    skip(user)
)]
pub async fn get_user(user: Json<User>) -> impl Responder {
    if verify_email(&user).is_err() {
        return HttpResponse::Forbidden().finish(); // Example error code
    }

    HttpResponse::Ok().json(user) // Return user details as JSON
}

#[put("/user/{email}")]
#[instrument(
    name = "update_user",
    level = "info",
    target = "chore_tracker",
    skip(user)
)]
pub async fn update_user(user: Json<User>) -> impl Responder {
    if verify_email(&user).is_err() {
        return HttpResponse::Forbidden().finish(); // Example error code
    }

    // Logic to update user details
    HttpResponse::Ok().json(user) // Return updated user details as JSON
}

#[delete("/user/{email}")]
#[instrument(
    name = "delete_user",
    level = "info",
    target = "chore_tracker",
    skip(user)
)]
pub async fn delete_user(conn: Data<Connection>, user: Json<User>) -> impl Responder {
    if verify_email(&user).is_err() {
        return HttpResponse::Forbidden().finish(); // Example error code
    }

    if authenticate_user(&user, &user.password).is_err() {
        return HttpResponse::Forbidden().finish(); // Example error code
    }

    // Logic to delete user
    info!("User {} deleted successfully", user.email);
    HttpResponse::NoContent().finish() // Return 204 No Content
}

/// Endpoint to list all users.
#[get("/users")]
#[instrument(name = "list_users", level = "info", target = "chore_tracker")]
pub async fn list_users() -> impl Responder {
    // Logic to list all users
    let users: Vec<User> = vec![]; // Placeholder for user list

    if users.is_empty() {
        return HttpResponse::NotFound().finish(); // Example error code
    }

    HttpResponse::Ok().json(users) // Return user list as JSON
}

#[instrument(
    name = "authenticate_user",
    level = "info",
    target = "chore_tracker",
    skip(user, password)
)]
fn authenticate_user(user: &User, password: &str) -> Result<User, ErrorCode> {
    if verify_email(user).is_err() {
        Err(ErrorCode::PermissionDenied) // Example error code
    } else {
        // Logic to authenticate user
        Ok(User {
            email: user.email.to_owned(),
            password: password.to_owned(),
            first_name: user.first_name.to_owned(),
            last_name: user.last_name.to_owned(),
        })
    }
}

/// Endpoint to change a user's password.
/// Returns 200 OK if the password is successfully changed, otherwise returns an appropriate error code.
#[put("/user/{email}/change_password")]
#[instrument(
    name = "change_password",
    level = "info",
    target = "chore_tracker",
    skip(user, old_password, new_password)
)]
pub async fn change_password(
    conn: Data<Connection>,
    user: Json<User>,
    old_password: Json<String>,
    new_password: Json<String>,
) -> impl Responder {
    if old_password.0.is_empty() || new_password.0.is_empty() {
        return HttpResponse::BadRequest().finish();
    } else if old_password.0 == new_password.0 {
        return HttpResponse::ExpectationFailed().finish();
    } else if new_password.0.len() < 6 {
        HttpResponse::Forbidden().finish();
    }

    // Logic to change the password
    let user = authenticate_user(&user, &old_password.0).expect("User not authenticated");

    // Update the user's password in the database
    conn.execute(
        "UPDATE users SET password = ? WHERE email = ?",
        rusqlite::params![new_password.0, user.email],
    )
    .expect("Failed to update password");

    info!("Password for user {} changed successfully", user.email);

    HttpResponse::Ok().finish()
}

/// Endpoint to verify a user's email address.
/// Returns 200 OK if the email is successfully verified, otherwise returns an appropriate error code.
#[put("/user/{email}/reset_password")]
#[instrument(
    name = "reset_password",
    level = "info",
    target = "chore_tracker",
    skip(user)
)]
pub async fn reset_password(user: Json<User>) -> impl Responder {
    if send_verification_email(user).is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

/// Verify the email address.
/// Returns 200 OK if the email is successfully verified, otherwise returns an appropriate error code.
#[instrument(
    name = "verify_email",
    level = "info",
    target = "chore_tracker",
    skip(user)
)]
fn verify_email(user: &User) -> Result<(), ErrorCode> {
    // parse the email address
    if user.email.is_empty() {
        return Err(ErrorCode::InternalMalfunction); // Example error code
    }
    // Logic to verify the email address
    let letters: Vec<char> = user.email.chars().collect();

    if letters.is_empty() || !letters.iter().any(|&c| c.is_alphanumeric()) {
        return Err(ErrorCode::NotFound); // Example error code
    }

    if !letters.iter().any(|&c| c == '@') {
        return Err(ErrorCode::NotFound); // Example error code
    }

    // Ensure there are at least two letters after the last '.' character
    if let Some(last_dot) = user.email.rfind('.') {
        if last_dot + 2 >= user.email.len()
            || !user.email[last_dot + 1..]
                .chars()
                .all(|c| c.is_alphanumeric())
        {
            return Err(ErrorCode::NotFound); // Example error code
        }
    } else {
        error!("Email does not contain a '.' character");
        return Err(ErrorCode::NotFound); // Example error code
    }

    Ok(())
}

/// Send a verification email to the user.
/// Returns 200 OK if the email is successfully sent, otherwise returns an appropriate error code.
fn send_verification_email(user: Json<User>) -> Result<(), ErrorCode> {
    // verify the passed in email
    if verify_email(&user).is_err() {
        return Err(ErrorCode::PermissionDenied); // Example error code
    }

    let email = Message::builder()
        .from(Mailbox::new(Some("Chore Tracker application".to_owned()), "nobody@chore_tracker.com".parse().expect("Failed to parse email address")))
        .reply_to(Mailbox::new(Some("Hunter".to_owned()), "chore_tracking@chore_tracker.com".parse().expect("Failed to parse email address")))
        .to(Mailbox::new(Some(user.first_name.clone()), user.email.parse().expect("Failed to parse email address")))
        .subject("Chore Tracker - Verify your email")
        .header(ContentType::TEXT_PLAIN)
	.body(format!(
	    "Hello {},\n\nPlease verify your email address by clicking the link below:\n\nhttp://chore_tracker.tld/verify?email={}\n\nThank you!",
	    user.first_name, user.email
	))
	.expect("Failed to build email message");

    // Logic to send the email using a mailer service
    let credentials = Credentials::new("username".to_string(), "password".to_string());

    // Open a remote connection to the gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .expect("Failed to create SMTP transport")
        .credentials(credentials)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => info!("Email sent successfully!"),
        Err(e) => {
            error!("Failed to send email: {}", e);
            return Err(ErrorCode::AuthorizationForStatementDenied); // Example error code
        }
    }

    // If sending fails, return an error
    Ok(())
}
