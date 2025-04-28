use diesel_async::AsyncPgConnection;
use crate::errors::Result;

// Import individual seeders
mod user_seeder;

pub async fn run_all_seeds(conn: &mut AsyncPgConnection) -> Result<()> {
    // Run all seeders in sequence
    user_seeder::seed(conn).await?;
    
    // Add more seeders here as needed
    
    println!("All seeds completed successfully!");
    Ok(())
}

// Function to run a specific seeder by name
pub async fn run_seeder(seeder_name: &str, conn: &mut AsyncPgConnection) -> Result<()> {
    match seeder_name {
        "users" => user_seeder::seed(conn).await?,
        // Add more seeders here as needed
        _ => println!("Unknown seeder: {}", seeder_name),
    }
    
    Ok(())
}
