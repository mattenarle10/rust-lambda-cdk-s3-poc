# rust-lambda-cdk-s3-poc

A small phase-2 POC building on `rust-lambda-poc`.

Rust + AWS CDK + API Gateway + (later) AWS SDK for Rust and S3 CRUD.

---

## Features (planned)

- Rust Lambda function(s) behind API Gateway
- Infrastructure-as-code with AWS CDK (TypeScript)
- Built and packaged with Cargo Lambda
- Simple `/health` endpoint (phase 1)
- Later: S3 list/CRUD using AWS SDK for Rust on `/items`

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

