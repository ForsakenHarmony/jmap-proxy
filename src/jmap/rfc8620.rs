use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type Id = String;
pub type SessionState = String;

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct JmapSession {
	pub capabilities:     Capabilities,
	pub accounts:         HashMap<Id, Account>,
	pub primary_accounts: HashMap<String, Id>,
	pub username:         String,
	pub api_url:          String,
	pub download_url:     String,

	pub upload_url:       String,
	pub event_source_url: String,
	pub state:            SessionState,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
	#[serde(rename = "urn:ietf:params:jmap:core")]
	pub core: CoreCapabilities,
	#[serde(rename = "urn:ietf:params:jmap:mail")]
	pub mail: EmptyCapabilities,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct CoreCapabilities {
	pub max_size_upload:         u64,
	pub max_concurrent_upload:   u64,
	pub max_size_request:        u64,
	pub max_concurrent_requests: u64,
	pub max_calls_in_request:    u64,
	pub max_objects_in_get:      u64,
	pub max_objects_in_set:      u64,
	pub collation_algorithms:    Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmptyCapabilities {}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Account {
	pub name:                 String,
	pub is_personal:          bool,
	pub is_read_only:         bool,
	pub account_capabilities: AcountCapabilities,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AcountCapabilities {
	#[serde(rename = "urn:ietf:params:jmap:mail")]
	pub mail: AccountMailCapabilities,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct AccountMailCapabilities {
	pub max_mailboxes_per_email:        Option<u64>,
	pub max_mailbox_depth:              Option<u64>,
	pub max_size_mailbox_name:          u64,
	pub max_size_attachments_per_email: u64,
	pub email_query_sort_options:       Vec<String>,
	pub may_create_top_level_mailbox:   bool,
}
