use std::{env, sync::Arc};

use clicksend_lib::{
    messaging::{SmsMessage, SmsRecipient, SmsRequest},
    ClickSendClient,
};
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let api_key = env::var("CLICKSEND_API_KEY").expect("No API Key");
    let username = env::var("CLICKSEND_USERNAME").expect("No Username");

    let clicksend_client = Arc::new(
        ClickSendClient::new("https://rest.clicksend.com", &username, &api_key, "v3").unwrap(),
    );

    let message = SmsMessage {
        body: "Hello World".to_string(),
        to: SmsRecipient::Number("61400588588".to_string()),
        from: None,
        source: None,
        schedule: None,
        custom_string: None,
        country: None,
        from_email: None,
        exclude_no_sender_id_recipients: None,
    };

    let request = SmsRequest {
        messages: vec![message],
    };

    dbg!(&request);

    let response = clicksend_client
        .send_sms(request)
        .expect("Could not send message");

    dbg!(&response);

    println!("Hello, world!");
}
