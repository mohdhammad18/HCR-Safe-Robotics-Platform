# HCR Safe Robotics Platform

This is a small Rust service for reading a robot temperature sensor and showing it in a browser. A Hotaru server listens on MQTT topic `sensor/temperature`. When the sensor publishes a reading, the handler takes the payload and prints it. The same process serves an HTTP page at `http://127.0.0.1:3003/dashboard`.

The page is a robot sensor dashboard. It shows the current temperature and whether the service is connected to the broker. The running server renders `templates/dashboard.html`. The assignment notes next to the source also keep an MQTT subscriber and an inline version of that page.

The crate is built with Tokio and Hotaru 0.8. Run it with `cargo run`, then open the dashboard URL above. A broker must be publishing on `sensor/temperature` for a live reading to arrive.
