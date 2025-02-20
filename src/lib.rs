pub use routes::health_check;
pub use routes::subscribe;
pub use startup::run;

pub mod configuration;
pub mod routes;
pub mod startup;
pub mod domain;
pub mod telemetry;
pub mod email_client;