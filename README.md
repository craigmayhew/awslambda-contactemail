# Send Emails via AWS Lambda
Use AWS lamda to send an Email on behalf of a contact form to predefined email address. This allows many simple websites to be hosted on AWS S3, even if they have a contact form.

# WARNING
Make sure you use locked down AWS access credentials!! The credentials should only be able to invoke the one lambda function and absolutly nothing else. If you do not follow this advice, someone may use your credentials to do bad things, such as max out your VM quota to mine bitcoin...

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

#AWS SES
You will need AWS ses setup to use this. Verify a domain via DNS entry, verify an email address and then request production use.
