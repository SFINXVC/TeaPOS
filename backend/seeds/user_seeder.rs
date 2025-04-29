use diesel_async::AsyncPgConnection;
use crate::error::Result;
use crate::models::user::{NewUser, User, UserRole};

pub async fn seed(conn: &mut AsyncPgConnection) -> Result<()> {
    println!("Seeding users table...");
    
    // Define users to seed
    let users_to_seed = vec![
        NewUser {
            username: "admin".to_string(),
            fullname: "System Administrator".to_string(),
            password: "admin123".to_string(), // Will be hashed automatically
            whatsapp: "123456789".to_string(),
            role: UserRole::SuperAdmin,
        },
        NewUser {
            username: "cashier".to_string(),
            fullname: "Default Cashier".to_string(),
            password: "cashier123".to_string(), // Will be hashed automatically
            whatsapp: "987654321".to_string(),
            role: UserRole::Employee,
        },
        // Add more users here as needed
    ];
    
    // Insert users if they don't already exist
    let mut created_count = 0;
    let mut skipped_count = 0;
    
    for user in users_to_seed {
        // Store the username before moving the user
        let username = user.username.clone();
        
        // Check if user already exists
        let existing_user = User::find_by_username(&username, conn).await;
        
        match existing_user {
            Ok(_) => {
                println!("User '{}' already exists, skipping", username);
                skipped_count += 1;
            },
            Err(_) => {
                // User doesn't exist, create it
                match User::create(user, conn).await {
                    Ok(_) => {
                        println!("Created user '{}'!", username);
                        created_count += 1;
                    },
                    Err(e) => {
                        println!("Error creating user '{}': {}", username, e);
                    }
                }
            }
        }
    }
    
    println!("Users seeding completed: {} created, {} skipped", created_count, skipped_count);
    Ok(())
}
