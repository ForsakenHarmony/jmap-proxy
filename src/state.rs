use std::{ops::DerefMut, sync::Arc};

use async_imap::Session;
use async_native_tls::TlsStream;
use async_std::{future::Future, net::TcpStream, sync::Mutex};
use flurry::HashMap;

use crate::{auth, imap};

pub type ImapSession = async_imap::Session<async_native_tls::TlsStream<TcpStream>>;

#[derive(Clone)]
pub struct State {
	imap_sessions: Arc<HashMap<String, Arc<Mutex<ImapSession>>>>,
}

impl State {
	pub fn new() -> Self {
		State {
			imap_sessions: Arc::new(HashMap::new()),
		}
	}

	pub async fn with_imap_session<'a, T, F, Fut>(&self, session_id: &str, f: F) -> tide::Result<T>
	where
		F: Send + Sync + 'a + FnOnce(&'a mut ImapSession) -> Fut,
		Fut: Future<Output = tide::Result<T>> + Send + 'a,
		T: 'static,
	{
		let guard = self.imap_sessions.guard();
		let s = match self.imap_sessions.get(session_id, &guard) {
			Some(s) => s.clone(),
			None => {
				return Err(tide::Error::from_str(
					tide::StatusCode::InternalServerError,
					"no imap session found for session id",
				))
			}
		};
		let mut s = s.lock().await;
		let fut = f(s.deref_mut());
		let res = fut.await?;
		drop(s);
		Ok(res)
	}
}

impl State {
	#[tracing::instrument(skip(self, credentials), fields(email = credentials.username.as_str()))]
	pub async fn authenticate(
		&self,
		session_id: String,
		credentials: auth::Credentials,
	) -> tide::Result<()> {
		tracing::info!("get_user");

		if self
			.imap_sessions
			.contains_key(&session_id, &self.imap_sessions.guard())
		{
			return Ok(());
		}

		let session = match imap::create_imap_session("hrmny.sh", credentials).await {
			Ok(s) => s,
			Err(e) => {
				tracing::error!("failed to create imap session: {:#?}", e);
				return Err(e.into());
			}
		};
		self.imap_sessions.insert(
			session_id,
			Arc::new(Mutex::new(session)),
			&self.imap_sessions.guard(),
		);

		Ok(())
	}
}
