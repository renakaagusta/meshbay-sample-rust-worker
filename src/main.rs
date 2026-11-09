use mongodb::bson::{doc, DateTime};
use mongodb::Client;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let mongo_url = std::env::var("MONGO_URL").expect("MONGO_URL is required");
    let client = Client::with_uri_str(&mongo_url).await?;

    let (database, database_name) = match client.default_database() {
        Some(default_database) => {
            let resolved_name = default_database.name().to_string();
            (default_database, resolved_name)
        }
        None => (client.database("meshbay"), "meshbay".to_string()),
    };

    let items = database.collection::<mongodb::bson::Document>("items");
    let insert_result = items
        .insert_one(doc! {
            "source": "cron",
            "at": DateTime::now(),
        })
        .await?;

    println!(
        "inserted {} into {}.items from cron",
        insert_result.inserted_id, database_name
    );

    Ok(())
}
