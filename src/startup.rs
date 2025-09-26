use std::net;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::dev;
use actix_web::http::header;
use actix_web::{
    http::KeepAlive,
    middleware,
    web::{scope, Data},
    App, HttpServer,
};
use tracing::instrument;
use tracing::{debug, error, info, warn};

use crate::endpoints;
use crate::endpoints::templates::{
    favicon, health_check, htmx, response_targets, source_map, stylesheet,
};
use crate::endpoints::user::{create_user, delete_user, get_user, update_user};
use crate::settings::Settings;

#[instrument(
    name = "main runner",
    level = "info",
    target = "kid_data",
    skip(listener, _db_pool, settings)
)]
fn run(
    listener: std::net::TcpListener,
    _db_pool: rusqlite::Connection,
    settings: Settings,
) -> Result<dev::Server, std::io::Error> {
    // For each session
    let _secret_key = actix_web::cookie::Key::from(settings.secret.hmac_secret.as_bytes());
    info!("Obtaining the cookie secret");

    // Connect to the MongoDB database
    info!("Processed DB connection pool for distribution");

    // Redis connection pool
    let cfg = deadpool_redis::Config::from_url(settings.clone().redis.url);
    let redis_pool = match cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1)) {
        Ok(pool) => pool,
        Err(err) => {
            error!("Failed to connect to Redis: {err}\nExiting...");
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Failed to connect to Redis",
            ));
        }
    };
    info!("Established secondary cache db connection pool");

    let redis_pool = Data::new(redis_pool);
    let setters = Data::new(settings);

    let _cors_middleware = Cors::default()
        .allowed_origin("http://localhost:8099")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
        ])
        .max_age(3600);

    let server = HttpServer::new(move || {
        let service = App::new()
            // .wrap(cors_middleware)
            // .wrap(if setters.debug {
            //     warn!("DEBUG MODE");
            //     SessionMiddleware::builder(<dyn SessionStore>::default(), secret_key.clone())
            //         .cookie_http_only(true)
            //         .cookie_same_site(actix_web::cookie::SameSite::Lax)
            //         .cookie_secure(true)
            //         .build()
            // } else {
            //     // TODO: Check if the below implementation is a secure cookie
            //     warn!("PRODUCTION MODE");
            //     SessionMiddleware::new(<dyn SessionStore>::default(), secret_key.clone())
            // })
            .wrap(middleware::Compress::default())
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", env!("CARGO_PKG_VERSION"))))
            .wrap(middleware::Logger::default())
            .app_data(redis_pool.clone())
            .app_data(setters.clone())
            .service(
                Files::new("/static", "./static")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .service(favicon)
            .service(stylesheet)
            .service(source_map)
            .service(htmx)
            .service(response_targets);
        service
            .service(endpoints::index::index)
            .service(
                scope("/v1")
                    .service(create_user)
                    .service(get_user)
                    .service(update_user)
                    .service(delete_user),
            )
            .service(health_check)
    })
    .keep_alive(KeepAlive::Os) // Keep the connection alive; OS handled
    .disable_signals() // Disable the signals to allow the OS to handle the signals
    .workers(1)
    .shutdown_timeout(3)
    .listen(listener)?
    .run();

    Ok(server)
}
pub struct Application {
    port: u16,
    server: actix_web::dev::Server,
}

impl Application {
    /// # Result
    ///  - `Ok(Application)` if the application was successfully built
    /// # Errors
    ///  - `std::io::Error` if the application could not be built
    /// # Panics
    ///  - If the application could not be built
    #[instrument(
        name = "Application builder",
        level = "info",
        target = "kid_data",
        skip(settings, db_pool)
    )]
    pub async fn build(
        settings: Settings,
        db_pool: Option<rusqlite::Connection>,
    ) -> Result<Self, std::io::Error> {
        info!("Buidling the main application");
        let connection_pool = if let Some(pool) = db_pool {
            pool
        } else {
            warn!("No database connection pool provided, using default");
            rusqlite::Connection::open_in_memory().expect("Failed to create in-memory database")
        };

        let address = format!(
            "{}:{}",
            settings.application.host, settings.application.port
        );

        debug!("Binding the TCP port: {address}");
        let listener: net::TcpListener = net::TcpListener::bind(&address)?;
        let port = listener.local_addr()?.port();
        let server = run(listener, connection_pool, settings)?;

        Ok(Self { port, server })
    }

    #[must_use]
    pub const fn port(&self) -> u16 {
        self.port
    }

    /// # Result
    ///  - `Ok(())` if the application was successfully started
    /// # Errors
    ///  - `std::io::Error` if the application could not be started
    /// # Panics
    ///  - If the application could not be started
    #[instrument(
        name = "Application runner",
        level = "info",
        target = "kid_data",
        skip(self)
    )]
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        info!("Running until stopped");
        self.server.await
    }
}
