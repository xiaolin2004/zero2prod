use zero2prod::configuration::get_configuration;
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "debug".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Fail to read configurations");
    let application = Application::build(configuration).await?;
    application.run_until_stop().await?;
    Ok(())
}
