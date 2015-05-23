# awslambda-contactemail
Use AWS lamda to send an Email on behalf of a contact form to predefined email address. This allows many simple websites to be hosted on AWS S3, even if they have a contact form.

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
