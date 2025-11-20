# rust-lambda-cdk-s3-poc

A small phase-2 POC building on `rust-lambda-poc`.

Inspired by the recent GA of Rust on AWS Lambda and Rust performance benchmarks.

Rust + AWS CDK + API Gateway + (later) AWS SDK for Rust and S3 CRUD.

## Blog series

- **Blog 1** – [`rust-lambda-poc`](https://github.com/mattenarle10/rust-lambda-poc): single Rust Lambda deployed with Cargo Lambda, listing S3 buckets.
- **Blog 2** – this repo: CDK-managed HTTP API + Rust Lambda with `/health` and S3-backed `/items` (GET/POST/DELETE).

---

## Features (planned)

- Rust Lambda function(s) behind API Gateway
- Infrastructure-as-code with AWS CDK (TypeScript)
- Built and packaged with Cargo Lambda
- Simple `/health` endpoint (phase 1 – liveness)
- S3-backed `/items` using AWS SDK for Rust:
  - `GET /items` – list object keys
  - `POST /items?key=...` – create/overwrite an object with the request body
  - `DELETE /items?key=...` – delete a single object by key

## References

- [Building serverless applications with Rust on AWS Lambda](https://aws.amazon.com/blogs/compute/building-serverless-applications-with-rust-on-aws-lambda/)
- [AWS SDK for Rust – Developer Guide](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html)
- [AWS Lambda with Rust](https://docs.aws.amazon.com/lambda/latest/dg/lambda-rust.html)
- [The Rust Programming Language](https://doc.rust-lang.org/book/ch00-00-introduction.html)

## Deploy (phase 1 idea)

Assumes:
- AWS CLI + credentials are configured
- AWS CDK CLI is installed (`npm install -g aws-cdk`)

1. CDK app + Rust Lambda (in this repo):
   - `infra/` – CDK TypeScript app using `RustFunction` + `HttpApi`
   - `lambda/api` – Rust HTTP Lambda built with Cargo Lambda

2. Build & deploy stack (from `infra/`):

   ```bash
   npm install           # once, after cdk init
   cdk bootstrap         # once per account/region
   cdk deploy            # deploy API + Lambda
   ```

3. Call `/health` over HTTP (after deploy prints the API URL):

   ```bash
   curl "https://...execute-api.<region>.amazonaws.com/health"
   ```

4. Call `/items` over HTTP (S3-backed micro-endpoint):

   ```bash
   # List items
   curl "https://...execute-api.<region>.amazonaws.com/items"

   # Create or overwrite an object
   curl -X POST "https://...execute-api.<region>.amazonaws.com/items?key=notes.txt" \
     -H "content-type: text/plain" \
     -d "hello from POST /items"

   # Delete an object
   curl -X DELETE "https://...execute-api.<region>.amazonaws.com/items?key=notes.txt"
   ```

   `GET` returns `no items` for an empty bucket, or newline-separated S3 object keys (for example, `gregoria.png`).
