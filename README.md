# Send Emails via AWS Lambda
Use Rust on AWS Lambda to send a contact form E-Mail to a predefined email address. This allows many simple websites to be hosted on AWS S3 + Cloudfront, even if they have a contact form.

# Further reading
 - https://crates.io/crates/lambda_runtime
 - https://github.com/awslabs/aws-lambda-rust-runtime
 - https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/

# WARNING
Make sure you use locked down AWS access credentials!! The credentials should only be able to invoke the one lambda function and absolutely nothing else. If you do not follow this advice, someone may use your credentials to do bad things, such as max out your VM quota to mine cryptocoins...

Lambda User Policy:

    {
      "Version": "2012-10-17",
      "Statement": [
        {
          "Sid": "Stmt1431568401000",
          "Effect": "Allow",
          "Action": ["lambda:InvokeFunction"],
          "Resource": ["arn:aws:lambda:eu-west-1:9757988339:function:contactemail"]
        }
      ]
   }

Lambda Role Policy:
    
    {
      "Version": "2012-10-17",
      "Statement": [
        {
            "Effect": "Allow",
            "Action": [
                "logs:*"
            ],
            "Resource": "arn:aws:logs:*:*:*"
        },
        {
           "Effect":"Allow",
           "Action":["ses:SendEmail", "ses:SendRawEmail"],
           "Resource":"*"
        }
      ]
    }

# TODO from 2021 WIP
 - Modify html in contact form to load a lambda for jquery.
 - Modify the CloudFront settings to fireoff a lambda on a certain file path being called.
 - Finish writing the rust lambda function to connect to SES and send the email.
 - Do we still the need lambda version in the cloudformation?

# Prep
Install rust
Install zip command line tool e.g. on ubuntu `apt install zip`
Install aws command line tool
Generate a certificate for your desired url in AWS certificate manager, you will need the certificates ARN for cloudformation. This certificate must be created US-EAST-1.
You will need AWS ses setup to use this. Verify a domain via DNS entry, verify an email address and then request production use.

# Deploy, build, redeploy
We can deploy almost all of our infra via cloudformation, including a placeholder lambda. However we can't deploy a rust lambda directly with cloudformation. Therefore we use this method of updating it in place once it's deployed via cloudformation.
```
aws cloudformation deploy --stack-name www-yourwebsite-com --capabilities CAPABILITY_NAMED_IAM --region us-east-1 --template-file cloudformation.yml --parameter-overrides Application=yourwebsite WebsiteURL=www.yourwebsite.com

# now manually upload files to populate your new S3 bucket with html/css/images. Ensure each file as public read access.

# now manually deploy the lambda function to lambda@edge via the console, keeping the existing trigger.

cargo build --release --target x86_64-unknown-linux-musl

cp ./target/x86_64-unknown-linux-musl/release/bootstrap ./bootstrap && zip lambda.zip bootstrap && rm bootstrap

aws lambda update-function-code --function-name cloudformation-contact-form-send-email \
  --zip-file fileb://./lambda.zip \
  --region us-east-1

aws lambda update-function-configuration --function-name cloudformation-contact-form-send-email \
  --handler doesnt.matter \
  --runtime provided \
  --environment Variables={RUST_BACKTRACE=1} \
  --tracing-config Mode=Active \
  --region us-east-1

# now "deploy api" from the gui or perhaps there is a one liner we can add here?
```

aws deploy production version of function <--- might not be needed now as we don't need to specify a lambda function version