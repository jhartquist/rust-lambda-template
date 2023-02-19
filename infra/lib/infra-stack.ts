import { CfnOutput, Stack, StackProps } from 'aws-cdk-lib';
import { Architecture, Code, Function, FunctionUrlAuthType, Runtime } from 'aws-cdk-lib/aws-lambda';
import { Construct } from 'constructs';

export class InfraStack extends Stack {
    constructor(scope: Construct, id: string, props?: StackProps) {
        super(scope, id, props);

        const functionHandler = new Function(this, "Lambda-1", {
            code: Code.fromAsset("../target/lambda/lambda-1"),
            runtime: Runtime.PROVIDED_AL2,
            architecture: Architecture.ARM_64,
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
            description: "The public URL of the lambda-1 function"
        });
    }
}
