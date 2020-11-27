use lapin::{
    options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Connection,
    ConnectionProperties, Result,
};
use log::info;

fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://localhost:5672/%2f".into());

    async_global_executor::block_on(async {
        let conn = Connection::connect(
            &addr,
            ConnectionProperties::default().with_default_executor(8),
        ).await?;

        info!("CONNECTED");

        let channel = conn.create_channel().await?;
        let queue = channel.queue_declare(
            "hello",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        ).await?;

        info!("Declared queue {:?}", queue);

        let confirm = channel.basic_publish(
            "",
            "hello",
            BasicPublishOptions::default(),
            b"Hello World".to_vec(),
            BasicProperties::default(),
        )
        .await?
        .await?;
        info!("Confirmation: {:?}", confirm);
        assert_eq!(confirm, Confirmation::NotRequested);

        Ok(())
    })
}
