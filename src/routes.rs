use std::{
	collections::{hash_map::DefaultHasher, HashMap},
	hash::{Hash, Hasher},
};

use crate::{auth::User, jmap, state};

pub async fn jmap(mut req: tide::Request<state::State>) -> tide::Result<tide::Response> {
	// let user = req.ext::<User>().unwrap();
	let request: jmap::method::Request = req.body_json().await?;

	let session_id = req.session().id();

	let jmap_api = jmap::JmapApi::new(session_id, req.state());
	let response = jmap_api.handle_request(request).await?;

	let body = serde_json::to_value(&response)?;

	Ok(body.into())
}

pub async fn session<State>(req: tide::Request<State>) -> tide::Result<tide::Response> {
	let user = req.ext::<User>().unwrap();

	let account_id: jmap::Id = user.email.clone();

	let mut accounts = HashMap::new();
	accounts.insert(
		account_id.clone(),
		jmap::Account {
			name:                 user.email.clone(),
			is_personal:          true,
			is_read_only:         false,
			account_capabilities: jmap::AcountCapabilities {
				mail: jmap::AccountMailCapabilities {
					max_mailboxes_per_email:        Some(1000),
					max_mailbox_depth:              None,
					max_size_mailbox_name:          490,
					max_size_attachments_per_email: 50000000,
					email_query_sort_options:       vec![
						"receivedAt".to_owned(),
						"from".to_owned(),
						"to".to_owned(),
						"subject".to_owned(),
						"size".to_owned(),
						"header.x-spam-score".to_owned(),
					],
					may_create_top_level_mailbox:   true,
				},
			},
		},
	);

	let mut primary_accounts = HashMap::new();
	primary_accounts.insert("urn:ietf:params:jmap:mail".to_owned(), account_id.clone());

	let session = jmap::JmapSession {
		capabilities: jmap::Capabilities {
			core: jmap::CoreCapabilities {
				max_size_upload:         50_000_000,
				max_concurrent_upload:   4,
				max_size_request:        10_000_000,
				max_concurrent_requests: 4,
				max_calls_in_request:    16,
				max_objects_in_get:      500,
				max_objects_in_set:      500,
				collation_algorithms:    vec![],
			},
			mail: jmap::EmptyCapabilities {},
		},
		accounts,
		primary_accounts,
		username: user.email.clone(),
		api_url: "/jmap".to_string(),
		download_url: "/download/{accountId}/{blobId}/{name}?accept={type}".to_string(),
		upload_url: "/upload/{accountId}".to_string(),
		event_source_url: "/eventsource?types={types}&closeafter={closeafter}&ping={ping}"
			.to_string(),
		state: Default::default(),
	};

	let mut body = serde_json::to_value(&session)?;

	let hash = calculate_hash(&format!("{}", body));
	let obj = body.as_object_mut().unwrap();
	obj.insert(
		"state".to_owned(),
		serde_json::Value::String(format!("{:X}", hash)),
	);

	Ok(body.into())
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
	let mut s = DefaultHasher::new();
	t.hash(&mut s);
	s.finish()
}
