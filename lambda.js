var emailRecipient = [];
var subject = 'Message From S3';
//delethe this whole file
exports.handler_emailContactForm = function(event, context) {
    //a very basic form of authorization so random calls don't fire off nonsense emails
    if (event.authorization == '8uhui9ihiu83he3032hteogo') {
      emailRecipient = ['example@websitedomain.com'];
    }else{
      context.done('error', 'Authorization Failed');
      return;
    }
    
    var aws = require('aws-sdk');
    var ses = new aws.SES({apiVersion: '2010-12-01'});
    var details = [];
    var arrayLength = event.fields.length;
    for (var i = 0; i < arrayLength; i++) {
      details.push(event.fields[i]+': '+event.datas[i]);
    }
    
    var body = "\n\n\n" + details.join("\n\n");

    var joy = ses.sendEmail({
      Source: event.fields.email,
      Destination: { ToAddresses: emailRecipient },
      Message: {
        Subject: {
          Data: subject
        },
        Body: {
          Text: {
            Data: body
          }
        }
      }
    }, function(err, data) {
      if(err) {
        callback(err);
      } else {
        callback(null);
      }
    });
    console.log(joy.response);
    context.done(joy.response);
}
