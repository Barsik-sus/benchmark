use postgres::{Client, NoTls};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let mut client = Client::connect(&std::env::var("DATABASE_URL")?, NoTls)?;

    for i in 0..1_000 {
        let _result = client.query(&format!("SELECT * FROM table_{i}"), &[])?;
    }

    Ok(())
}

// 3s - build time
