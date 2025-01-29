use actix_web::HttpRequest;
use std::env;

pub async fn is_proxy_request(_req: HttpRequest) -> bool {
    let api_token = env::var("API_TOKEN").expect("No API_TOKEN");
    let api_token_header = _req.headers().get("X-API-Token");
    match api_token_header {
        Some(header) => *header == *api_token,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use actix_web::test;
    use std::env;

    #[actix_web::test]
    async fn test_lb_header_correct() {
        // Set the environment variable
        env::set_var("API_TOKEN", "test-token");

        // Create a request with the correct header
        let request = test::TestRequest::default()
            .insert_header(("X-API-Token", "test-token"))
            .to_http_request();

        // Call the function
        let result = super::is_proxy_request(request).await;

        // Assert the result
        assert!(result);
    }

    #[actix_web::test]
    async fn test_lb_header_incorrect() {
        // Set the environment variable
        env::set_var("API_TOKEN", "test-token");

        // Create a request with an incorrect header
        let request = test::TestRequest::default()
            .insert_header(("X-API-Token", "wrong-token"))
            .to_http_request();

        // Call the function
        let result = super::is_proxy_request(request).await;

        // Assert the result
        assert!(!result);
    }

    #[actix_web::test]
    async fn test_lb_header_present() {
        // Set the environment variable
        env::set_var("API_TOKEN", "test-token");

        // Create a request with a random header
        let request = test::TestRequest::default()
            .insert_header(("Some-Other-Header", "value"))
            .to_http_request();

        // Call the function
        let result = super::is_proxy_request(request).await;

        // Assert the result
        assert!(!result);
    }

    #[actix_web::test]
    async fn test_lb_header_not_present() {
        // Set the environment variable
        env::set_var("API_TOKEN", "test-token");

        // Create a request without the "X-API-Token" header
        let request = test::TestRequest::default().to_http_request();

        // Call the function
        let result = super::is_proxy_request(request).await;

        // Assert the result
        assert!(!result);
    }
}
