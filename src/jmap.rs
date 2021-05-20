mod mailbox;
pub mod method;
pub mod rfc8620;

use async_std::{future::Future, stream::StreamExt};
use futures::TryStreamExt;
pub use rfc8620::*;

use crate::{
	jmap::method::{Method, MethodCallResult, MethodResult},
	state,
};

pub struct JmapApi<'a> {
	session_id: &'a str,
	state:      &'a state::State,
}

impl JmapApi<'_> {
	pub fn new<'a>(session_id: &'a str, state: &'a state::State) -> JmapApi<'a> {
		JmapApi { session_id, state }
	}
}

impl JmapApi<'_> {
	pub async fn handle_request(&self, req: method::Request) -> tide::Result<method::Reponse> {
		let mut response = method::Reponse {
			method_responses: vec![],
			created_ids:      None,
			session_state:    "".to_string(),
		};

		for m in req.method_calls {
			let result: MethodResult = match m.method {
				Method::CoreEcho(map) => MethodResult::CoreEcho(map),
				Method::MailboxGet {
					account_id,
					ids,
					properties,
				} => self.handle_mailbox_get(account_id, ids, properties).await?,
			};

			response.method_responses.push(MethodCallResult {
				method_result: result,
				call_id:       m.call_id,
			});
		}

		Ok(response)
	}

	pub async fn handle_mailbox_get(
		&self,
		account_id: String,
		ids: Option<Vec<Id>>,
		_properties: Option<Vec<String>>,
	) -> tide::Result<MethodResult> {
		if ids.is_some() {
			return Err(tide::Error::from_str(
				tide::StatusCode::NotImplemented,
				"selecting ids is not supported yet",
			));
		}

		let _list = self
			.state
			.with_imap_session(self.session_id, |s| async move {
				let list: Vec<async_imap::types::Name> =
					s.list(None, Some("*")).await?.try_collect().await?;
				Ok(list)
			})
			.await?;

		Ok(MethodResult::MailboxGet {
			account_id,
			state: "".to_string(),
			list: vec![],
			not_found: vec![],
		})
	}
}
//
// impl JmapApi<'_> {
// 	pub async fn with_imap_session<'a, T, F, Fut>(&self, f: F) -> tide::Result<T>
// 	where
// 		F: Send + Sync + FnOnce(&'a mut state::ImapSession) -> Fut + 'a,
// 		Fut: Future<Output = tide::Result<T>> + Send + 'a,
// 		T: 'static,
// 	{
// 		self.state.with_imap_session(self.session_id, f).await
// 	}
// }
