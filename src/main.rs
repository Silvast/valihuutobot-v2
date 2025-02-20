use std::error::Error;
mod memos;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    match memos::get_feed().await {
        Ok(channel) => {
            memos::edit_memo(channel.items().get(3).unwrap()).await?;
        }
        Err(e) => {
            println!("Error: {}", e);
        }

    }

    Ok(())
}
