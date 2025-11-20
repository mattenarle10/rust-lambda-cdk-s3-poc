use lambda_http::{Body, Error, Request, RequestExt, Response};
use std::env;

use crate::s3_store::{delete_item, list_items, put_item};

// Main HTTP entrypoint: route between /health and /items.
pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Read the path and HTTP method for simple routing.
    let path = event.uri().path();
    let method = event.method().as_str();

    // /health endpoint – simple liveness check.
    if path == "/health" && method == "GET" {
        let resp = Response::builder()
            .status(200)
            .header("content-type", "text/plain")
            .body("ok".into())
            .map_err(Box::new)?;
        return Ok(resp);
    }

    // /items endpoints – simple CRUD-style operations on S3.
    if path == "/items" {
        let bucket_name = env::var("BUCKET_NAME").map_err(Box::new)?;

        match method {
            // GET /items -> list all keys in the bucket
            "GET" => {
                let body_text = list_items(&bucket_name).await?;

                let resp = Response::builder()
                    .status(200)
                    .header("content-type", "text/plain")
                    .body(body_text.into())
                    .map_err(Box::new)?;
                return Ok(resp);
            }
            // POST /items?key=... -> create/overwrite an object with the request body as content
            "POST" => {
                let key_param = event
                    .query_string_parameters_ref()
                    .and_then(|params| params.first("key"));

                let key = match key_param {
                    Some(k) => k,
                    None => {
                        let resp = Response::builder()
                            .status(400)
                            .header("content-type", "text/plain")
                            .body("missing ?key=... for POST /items".into())
                            .map_err(Box::new)?;
                        return Ok(resp);
                    }
                };

                let body_bytes = event.body().to_vec();
                put_item(&bucket_name, key, body_bytes).await?;

                let message = format!("created: {key}");
                let resp = Response::builder()
                    .status(201)
                    .header("content-type", "text/plain")
                    .body(message.into())
                    .map_err(Box::new)?;
                return Ok(resp);
            }
            // DELETE /items?key=... -> delete a single object by key
            "DELETE" => {
                let key_param = event
                    .query_string_parameters_ref()
                    .and_then(|params| params.first("key"));

                let key = match key_param {
                    Some(k) => k,
                    None => {
                        let resp = Response::builder()
                            .status(400)
                            .header("content-type", "text/plain")
                            .body("missing ?key=... for DELETE /items".into())
                            .map_err(Box::new)?;
                        return Ok(resp);
                    }
                };

                delete_item(&bucket_name, key).await?;

                let message = format!("deleted: {key}");
                let resp = Response::builder()
                    .status(200)
                    .header("content-type", "text/plain")
                    .body(message.into())
                    .map_err(Box::new)?;
                return Ok(resp);
            }
            // Any other method on /items is not allowed.
            _ => {
                let resp = Response::builder()
                    .status(405)
                    .header("content-type", "text/plain")
                    .body("method not allowed".into())
                    .map_err(Box::new)?;
                return Ok(resp);
            }
        }
    }

    // Fallback 404 for unknown paths.
    let resp = Response::builder()
        .status(404)
        .header("content-type", "text/plain")
        .body("not found".into())
        .map_err(Box::new)?;
    Ok(resp)
}

// S3 helpers now live in s3_store.rs; http_handler just calls into them.

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::Request;

    // Smoke test to keep /health behavior stable.
    #[tokio::test]
    async fn test_health_handler_returns_ok() {
        let mut request = Request::default();
        *request.uri_mut() = "/health".parse().unwrap();

        let response = function_handler(request).await.unwrap();
        assert_eq!(response.status(), 200);

        let body_bytes = response.body().to_vec();
        let body_string = String::from_utf8(body_bytes).unwrap();

        assert_eq!(body_string, "ok");
    }
}
