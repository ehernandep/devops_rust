use actix_web::test;
use actix_web::{App, test::TestRequest};
use serde_json::Value;
use crate::routes::game::get_games;

#[actix_rt::test]
async fn test_get_games() {
    // Create an App instance for testing.
    let mut app = test::init_service(App::new().service(get_games)).await;

    // Simulate a GET request to /games.
    let req = test::TestRequest::get().uri("/games").to_request();
    let resp = test::call_service(&mut app, req).await;

    // Check the response status.
    assert!(resp.status().is_success());

    // Parse the response body as JSON.
    let body: Value =  test::read_response_json(&mut app, req).await;


    // Verify the response JSON structure.
    assert_eq!(body["status"], "success");
    assert_eq!(body["no. games"], 2); // Adjust this based on your expected test data.

    // Verify the content of the "games" array.
    let games = body["games"].as_array().expect("Expected 'games' to be an array");
    assert_eq!(games.len(), 2); // Adjust this based on your expected test data.

    // Verify the properties of individual games.
    let first_game = &games[0];
    assert_eq!(first_game["address"], "calle 1");
    // Add similar assertions for other properties of the first game.

    let second_game = &games[1];
    assert_eq!(second_game["address"], "calle 2");
    // Add similar assertions for other properties of the second game.
}
