import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { HttpApi } from 'aws-cdk-lib/aws-apigatewayv2';
import { HttpLambdaIntegration } from 'aws-cdk-lib/aws-apigatewayv2-integrations';
import { HttpMethod } from 'aws-cdk-lib/aws-apigatewayv2';
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

    const httpApi = new HttpApi(this, 'http_api');
    const healthIntegration = new HttpLambdaIntegration('health_integration', apiLambda);

    httpApi.addRoutes({
      path: '/health',
      methods: [HttpMethod.GET],
      integration: healthIntegration,
    });

    new cdk.CfnOutput(this, 'api_url', {
      value: httpApi.apiEndpoint,
    });
  }
}
