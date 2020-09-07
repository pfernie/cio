use std::collections::HashMap;

use airtable_api::Airtable;
use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

use crate::airtable::{
    airtable_api_key, AIRTABLE_BASE_ID_CUSTOMER_LEADS, AIRTABLE_GRID_VIEW,
    AIRTABLE_MAILING_LIST_SIGNUPS_TABLE,
};
use crate::models::NewMailingListSubscriber;

/// Get all the mailing list subscribers from Airtable.
pub async fn get_all_subscribers() -> Vec<NewMailingListSubscriber> {
    // Initialize the Airtable client.
    let airtable =
        Airtable::new(airtable_api_key(), AIRTABLE_BASE_ID_CUSTOMER_LEADS);

    let records = airtable
        .list_records(AIRTABLE_MAILING_LIST_SIGNUPS_TABLE, AIRTABLE_GRID_VIEW)
        .await
        .unwrap();

    let mut subscribers: Vec<NewMailingListSubscriber> = Default::default();
    for record in records {
        let fields: NewMailingListSubscriber =
            serde_json::from_value(record.fields.clone()).unwrap();

        subscribers.push(fields);
    }
    subscribers
}

/// The data type for the webhook from Mailchimp.
///
/// Docs:
/// https://mailchimp.com/developer/guides/sync-audience-data-with-webhooks/#handling-the-webhook-response-in-your-application
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MailchimpWebhook {
    #[serde(rename = "type")]
    pub webhook_type: String,
    fired_at: DateTime<Utc>,
    data: MailchimpWebhookData,
}

impl MailchimpWebhook {
    /// Convert to a signup data type.
    pub fn as_signup(&self) -> NewMailingListSubscriber {
        let mut signup: NewMailingListSubscriber = Default::default();

        if self.data.merges.is_some() {
            let merges = self.data.merges.as_ref().unwrap();

            if let Some(e) = &merges.email {
                signup.email = e.trim().to_string();
            }
            if let Some(f) = &merges.first_name {
                signup.first_name = f.trim().to_string();
            }
            if let Some(l) = &merges.last_name {
                signup.last_name = l.trim().to_string();
            }
            if let Some(c) = &merges.company {
                signup.company = c.trim().to_string();
            }
            if let Some(i) = &merges.interest {
                signup.interest = i.trim().to_string();
            }

            if merges.groupings.is_some() {
                let groupings = merges.groupings.as_ref().unwrap();

                signup.wants_podcast_updates =
                    groupings.get(&0).unwrap().groups.is_some();
                signup.wants_newsletter =
                    groupings.get(&1).unwrap().groups.is_some();
                signup.wants_product_updates =
                    groupings.get(&2).unwrap().groups.is_some();
            }
        }

        signup.date_added = self.fired_at;
        signup.date_optin = self.fired_at;
        signup.date_last_changed = self.fired_at;
        signup.name = format!("{} {}", signup.first_name, signup.last_name);

        signup
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MailchimpWebhookData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_opt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_signup: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merges: Option<MailchimpWebhookMerges>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MailchimpWebhookMerges {
    #[serde(skip_serializing_if = "Option::is_none", rename = "FNAME")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "LNAME")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "EMAIL")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ADDRESS")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "PHONE")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "COMPANY")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "INTEREST")]
    pub interest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "BIRTHDAY")]
    pub birthday: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "GROUPINGS")]
    pub groupings: Option<HashMap<i32, MailchimpWebhookGrouping>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MailchimpWebhookGrouping {
    pub id: String,
    pub unique_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<String>,
}
