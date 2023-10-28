use actix_web::{App, test};
use crate::routes::game::get_games; // Replace with the actual import for your route handler.

#[actix_rt::test]
async fn test_get_games() {
    // Create a TestServer with the App.
    let app = App::new().service(get_games); // Add your route handler to the App.

    let mut app = test::init_service(app).await;

    // Simulate a GET request to /games.
    let req = test::TestRequest::get().uri("/api/games").to_request();
    let resp = test::call_service(&mut app, req).await;

    // Check the response status.
    assert!(resp.status().is_success());

    // Read the response body as bytes.
    let response_bytes = test::read_body(resp).await;

    // Now you can work with the response body as needed.
    // For example, if you expect a JSON response:
    let response_str = String::from_utf8_lossy(&response_bytes);
    let response_json: serde_json::Value =
        serde_json::from_str(&response_str).expect("Failed to parse JSON response");

    // Now you can assert and work with the JSON response as needed.
    assert_eq!(response_json["some_key"], "some_value"); // Replace with your expected JSON structure.
}

