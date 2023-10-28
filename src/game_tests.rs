use actix_web::{App, test};
use crate::routes::game::get_games;

#[actix_rt::test]
async fn test_get_games() {

    let app = App::new().service(get_games); 

    let mut app = test::init_service(app).await;


    let req = test::TestRequest::get().uri("/api/games").to_request();
    let resp = test::call_service(&mut app, req).await;


    assert!(resp.status().is_success());

    let response_bytes = test::read_body(resp).await;

   
    let response_str = String::from_utf8_lossy(&response_bytes);
    let response_json: serde_json::Value =
        serde_json::from_str(&response_str).expect("Failed to parse JSON response");


    assert_eq!(response_json["some_key"], "some_value"); 
}

