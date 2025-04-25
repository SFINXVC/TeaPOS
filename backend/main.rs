use teapos::app::App;

#[ntex::main]
async fn main() {
    let app = App::new().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(-1);
    });

    if let Err(e) = app.run().await {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}
