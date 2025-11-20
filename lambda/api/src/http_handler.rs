use lambda_http::{Body, Error, Request, Response};

// Simple health-style handler: always returns 200 with body "ok".
pub(crate) async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("ok".into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::Request;

    #[tokio::test]
    async fn test_health_handler_returns_ok() {
        let request = Request::default();

        let response = function_handler(request).await.unwrap();
        assert_eq!(response.status(), 200);

        let body_bytes = response.body().to_vec();
        let body_string = String::from_utf8(body_bytes).unwrap();

        assert_eq!(body_string, "ok");
    }
}
