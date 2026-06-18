use hotaru::http::*;
use hotaru::prelude::*;

// 1. Set up the Hotaru Server
LServer!(
    APP = Server::new()
        .binding("127.0.0.1:3003")
        .single_protocol(ProtocolBuilder::new(HTTP::server(HttpSafety::default())))
        .build()
);

// 2. Insert the Question 3 Dashboard Endpoint from your assignment
endpoint! {
    APP.url("/dashboard"),
    
    pub temperature_dashboard <HTTP> {
        // This macro automatically grabs dashboard.html, sets the correct 
        // HTML headers, and serves it to the browser!
        akari_render!("dashboard.html")
    }
}

// 3. Start the application
#[tokio::main]
async fn main() {
    println!("Dashboard running at http://localhost:3003/dashboard");
    APP.clone().run().await;
}
mod resource;
