import * as cdk from 'aws-cdk-lib';
import { CfnOutput } from 'aws-cdk-lib';
import { Code, Function, Runtime, FunctionUrlAuthType } from 'aws-cdk-lib/aws-lambda';
import { Construct } from 'constructs';
// import * as sqs from 'aws-cdk-lib/aws-sqs';

export class InfraStack extends cdk.Stack {
    constructor(scope: Construct, id: string, props?: cdk.StackProps) {
        super(scope, id, props);

        const functionHandler = new Function(this, "Lambda-1", {
            code: Code.fromAsset("../target/lambda/lambda-1"),
            runtime: Runtime.PROVIDED_AL2,
            handler: "lambda-1-handler",
            memorySize: 1024,
        });

        const functionUrl = functionHandler.addFunctionUrl({
            authType: FunctionUrlAuthType.NONE,
        });


        new CfnOutput(this, 'Function Name', {
            value: functionHandler.functionName,
            description: "The name of the lambda-1 function"
        });

        new CfnOutput(this, 'Function URL', {
            value: functionUrl.url,
            description: "The public endpoint for the lambda-1 function"
        });
    }
}
