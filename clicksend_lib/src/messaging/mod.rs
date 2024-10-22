use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

use crate::{error::AppResult, ClickSendClient};

impl ClickSendClient {
    pub fn send_sms(&self, req: SmsRequest) -> AppResult<SmsResponse> {
        self.post("sms/send", &req)
    }
}

#[derive(Debug)]
pub struct SmsMessage {
    pub body: String,
    pub to: SmsRecipient,
    pub from: Option<String>,
    pub source: Option<String>,
    pub schedule: Option<u32>,
    pub custom_string: Option<String>,
    pub country: Option<String>,
    pub from_email: Option<String>,
    pub exclude_no_sender_id_recipients: Option<bool>,
}

impl Serialize for SmsMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SmsMessage", 8)?;

        // Serialize the common fields
        state.serialize_field("body", &self.body)?;
        if let Some(ref from) = self.from {
            state.serialize_field("from", from)?;
        }
        if let Some(ref source) = self.source {
            state.serialize_field("source", source)?;
        }
        if let Some(schedule) = self.schedule {
            state.serialize_field("schedule", &schedule)?;
        }
        if let Some(ref custom_string) = self.custom_string {
            state.serialize_field("custom_string", custom_string)?;
        }
        if let Some(ref country) = self.country {
            state.serialize_field("country", country)?;
        }
        if let Some(ref from_email) = self.from_email {
            state.serialize_field("from_email", from_email)?;
        }
        if let Some(exclude_no_sender_id_recipients) = self.exclude_no_sender_id_recipients {
            state.serialize_field(
                "exclude_no_sender_id_recipients",
                &exclude_no_sender_id_recipients,
            )?;
        }

        // Manually serialize the recipient field based on its variant
        match &self.to {
            SmsRecipient::Number(number) => {
                state.serialize_field("to", number)?;
            }
            SmsRecipient::ContactList(list_id) => {
                state.serialize_field("list_id", list_id)?;
            }
        }

        state.end()
    }
}

#[derive(Debug, Serialize)]
pub struct SmsRequest {
    pub messages: Vec<SmsMessage>,
}

#[derive(Debug, Deserialize)]
pub struct SmsResponse {
    pub http_code: u32,
    pub response_code: String,
    pub response_msg: String,
    pub data: SmsResponseData,
}

#[derive(Debug, Deserialize)]
pub struct SmsResponseData {
    pub total_price: f32,
    pub total_count: u32,
    pub queued_count: u32,
    pub messages: Vec<MessageData>,
    pub _currency: CurrencyData,
    pub blocked_count: u32,
}

#[derive(Debug)]
pub enum SmsRecipient {
    Number(String),
    ContactList(String),
}
impl Serialize for SmsRecipient {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SmsRecipient::Number(number) => {
                let mut state = serializer.serialize_struct("SmsRecipient", 1)?;
                state.serialize_field("to", number)?;
                state.end()
            }
            SmsRecipient::ContactList(list_id) => {
                let mut state = serializer.serialize_struct("SmsRecipient", 1)?;
                state.serialize_field("list_id", list_id)?;
                state.end()
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageDirection {
    In,
    Out,
}

#[derive(Debug, Deserialize)]
pub struct MessageData {
    pub direction: MessageDirection,
    pub date: u32,
    pub to: String,
    pub body: String,
    pub from: String,
    pub schedule: u32,
    pub message_id: String,
    pub message_parts: u32,
    pub message_price: String,
    pub from_email: Option<String>,
    pub list_id: Option<String>,
    pub custom_string: Option<String>,
    pub contact_id: Option<String>,
    pub user_id: u32,
    pub subaccount_id: u32,
    pub is_shared_system_number: bool,
    pub country: String,
    pub carrier: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CurrencyData {
    pub currency_name_short: String,
    pub currency_prefix_d: String,
    pub currency_prefix_c: String,
    pub currency_name_long: String,
}
