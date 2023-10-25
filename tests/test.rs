
// In tests/your_module_tests.rs

// Import necessary modules and types
use actix_rt::test; // Import actix-rt for testing
use actix_web::http::StatusCode; // Import StatusCode from actix-web
use sqlx::query; // Import query from sqlx
use soccer::AppState; // Import your AppState type, replacing "your_project" with the actual name of your project


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use sqlx::query;

    #[actix_rt::test]
    async fn test_get_games() {
        // Create a test AppState and database connection
        let data = test::init_service(AppState::default()).await;
        // Insert test data into the database (you might need to set up a test database)
        // ...

        // Make a test request to your get_games function
        let req = test::TestRequest::get().uri("/games").to_request();
        let resp = test::call_service(&data, req).await;

        // Assert the response
        assert_eq!(resp.status(), StatusCode::OK);
        // Add more assertions as needed
    }

   
}
