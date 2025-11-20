import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { HttpApi } from 'aws-cdk-lib/aws-apigatewayv2';
import { HttpLambdaIntegration } from 'aws-cdk-lib/aws-apigatewayv2-integrations';
import { HttpMethod } from 'aws-cdk-lib/aws-apigatewayv2';
import * as s3 from 'aws-cdk-lib/aws-s3';
import { RustFunction } from 'cargo-lambda-cdk';

export class InfraStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // rust lambda built from lambda/api
    const apiLambda = new RustFunction(this, 'api_lambda', {
      manifestPath: '../lambda/api',
      runtime: 'provided.al2023',
      timeout: cdk.Duration.seconds(10),
    });

    // S3 bucket where the /items endpoint will read/write objects
    const itemsBucket = new s3.Bucket(this, 'items_bucket', {
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
    });

    // allow the Lambda to access the bucket and pass the name via env var
    itemsBucket.grantReadWrite(apiLambda);
    apiLambda.addEnvironment('BUCKET_NAME', itemsBucket.bucketName);

    // simple HTTP API with /health and /items routes
    const httpApi = new HttpApi(this, 'http_api');
    const healthIntegration = new HttpLambdaIntegration('health_integration', apiLambda);
    const itemsIntegration = new HttpLambdaIntegration('items_integration', apiLambda);

    httpApi.addRoutes({
      path: '/health',
      methods: [HttpMethod.GET],
      integration: healthIntegration,
    });

    httpApi.addRoutes({
      path: '/items',
      methods: [HttpMethod.GET],
      integration: itemsIntegration,
    });

    // output the base URL so we can curl the API
    new cdk.CfnOutput(this, 'api_url', {
      value: httpApi.apiEndpoint,
    });

    // output the bucket name so we can upload objects and debug
    new cdk.CfnOutput(this, 'itemsBucketName', {
      value: itemsBucket.bucketName,
    });
  }
}
