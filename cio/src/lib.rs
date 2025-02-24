#![recursion_limit = "256"]
#![allow(clippy::field_reassign_with_default)]

pub mod airtable;
pub mod analytics;
pub mod api_tokens;
pub mod applicant_status;
pub mod applicants;
pub mod auth_logins;
pub mod certs;
pub mod configs;
pub mod core;
pub mod db;
pub mod finance;
pub mod gsuite;
pub mod huddles;
pub mod interviews;
pub mod journal_clubs;
pub mod mailchimp;
pub mod mailing_list;
pub mod models;
pub mod png;
pub mod rack_line;
pub mod recorded_meetings;
pub mod rfds;
pub mod schema;
pub mod shipments;
pub mod shorturls;
pub mod slack;
pub mod states;
pub mod swag_inventory;
pub mod swag_store;
pub mod tailscale;
pub mod templates;
pub mod utils;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lopdf;
