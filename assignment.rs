/// ======== Question 1 START ========
// Answer: out
// Explanation: In an MQTT architecture, the broker is the central hub. 
// A sensor "publishes" data (data flows IN to the broker). 
// An LCD "subscribes" to read that data (data flows OUT from the broker to the LCD).
/// ======== Question 1 END ========


/// ======== Question 2 START ========
// We implement an endpoint tagged with <MQTT> to subscribe to the sensor's topic.
// When the simulator publishes a temperature, this function intercepts it.

endpoint! {
    // The MQTT topic we are listening to
    APP.url("sensor/temperature"),
    
    // The MQTT handler function
    pub read_temperature <MQTT> {
        // Extract the raw byte payload from the incoming MQTT message
        let payload = req.incoming.as_ref().unwrap().payload();
        
        // Convert the bytes into a readable string (optional, for logging/debugging)
        let temp_string = String::from_utf8_lossy(&payload);
        println!("Received temperature from simulator: {}", temp_string);

        // Acknowledge the message was processed successfully
        Ok(req)
    }
}
/// ======== Question 2 END ========


/// ======== Question 3 START ========
// We implement an <HTTP> endpoint to serve a web dashboard to the user's browser.
// In a full application, this would dynamically read the data captured by Question 2.

endpoint! {
    // The URL path for the browser (e.g., http://localhost:3003/dashboard)
    APP.url("/dashboard"),
    
    // The HTTP handler function
    pub temperature_dashboard <HTTP> {
        // A simple HTML dashboard to display the data
        let html_dashboard = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>HCR Temperature Dashboard</title>
            <style>
                body { font-family: Arial, sans-serif; text-align: center; margin-top: 50px; }
                .temp-box { font-size: 48px; color: #ff5722; font-weight: bold; }
            </style>
        </head>
        <body>
            <h1>Robot Sensor Dashboard</h1>
            <p>Current Temperature Reading:</p>
            <div class="temp-box">24.5 °C</div>
            <p><em>Status: Connected to Broker</em></p>
        </body>
        </html>
        "#;
        
        // Return the HTML string to the browser
        html_response(html_dashboard)
    }
}
/// ======== Question 3 END ========