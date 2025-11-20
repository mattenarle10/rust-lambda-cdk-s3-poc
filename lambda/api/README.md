# Introduction

api is a Rust project that implements an AWS Lambda function in Rust.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo Lambda](https://www.cargo-lambda.info/guide/installation.html)
 - [AWS CDK CLI](https://docs.aws.amazon.com/cdk/v2/guide/getting_started.html#getting_started_prerequisites)

## Building

To build the project for production, run `cargo lambda build --release`. Remove the `--release` flag to build for development.

Read more about building your lambda function in [the Cargo Lambda documentation](https://www.cargo-lambda.info/commands/build.html).

## Testing

You can run regular Rust unit tests with `cargo test`.

If you want to run integration tests locally, you can use the `cargo lambda watch` and `cargo lambda invoke` commands to do it.

First, run `cargo lambda watch` to start a local server. When you make changes to the code, the server will automatically restart.

Second, you'll need a way to pass the event data to the lambda function.

You can use the existent [event payloads](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-events/src/fixtures) in the Rust Runtime repository if your lambda function is using one of the supported event types.

You can use those examples directly with the `--data-example` flag, where the value is the name of the file in the [lambda-events](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-events/src/fixtures) repository without the `example_` prefix and the `.json` extension.

```bash
cargo lambda invoke --data-example apigw-request
```

For generic events, where you define the event data structure, you can create a JSON file with the data you want to test with. For example:

```json
{
    "command": "test"
}
```

Then, run `cargo lambda invoke --data-file ./data.json` to invoke the function with the data in `data.json`.

For HTTP events, you can also call the function directly with cURL or any other HTTP client. For example:

```bash
curl https://localhost:9000
```

Read more about running the local server in [the Cargo Lambda documentation for the `watch` command](https://www.cargo-lambda.info/commands/watch.html).
Read more about invoking the function in [the Cargo Lambda documentation for the `invoke` command](https://www.cargo-lambda.info/commands/invoke.html).

## Deploying (via CDK in this POC)

In this project, the `api` function is deployed by the CDK stack in `../infra` using the `RustFunction` construct from `cargo-lambda-cdk`.

From the repo root, you typically do:

```bash
cd infra
npm install           # first time
cdk bootstrap         # first time per account/region
cdk deploy            # builds this Rust Lambda + deploys HttpApi + /health and /items
```

After deploy, CDK prints an `api_url` output. You can then call the `/health` endpoint:

```bash
curl "$API_URL/health"
```

Expected response body:

```text
ok
```

### HTTP endpoints exposed by this Lambda

- `GET /health` – liveness probe returning plain `"ok"`.
- `GET /items` – list S3 object keys (or `no items` if bucket is empty).
- `POST /items?key=...` – create/overwrite S3 object with the request body as content.
- `DELETE /items?key=...` – delete a single S3 object by key.

You can still use `cargo lambda build` locally for testing, but the normal deployment flow for this repo is handled by CDK.
