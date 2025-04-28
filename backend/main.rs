use std::env;
use teapos::app::App;

#[ntex::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    let app = App::new().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(-1);
    });
    
    // Check for --seed argument
    if args.len() > 1 && args[1] == "--seed" {
        if let Err(e) = app.run_seeds(args.get(2).map(|s| s.as_str())).await {
            eprintln!("Error running seeds: {}", e);
            std::process::exit(-1);
        }
        println!("Seeding completed successfully!");
        return;
    }

    if let Err(e) = app.run().await {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}