use tide::StatusCode;
use tracing::{error, info};

use crate::{error, state::State};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Credentials {
	pub username: String,
	pub password: String,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct User {
	pub email: String,
}

pub struct Authentication;

impl Authentication {
	pub fn new() -> Self {
		Authentication
	}
}

fn unauthorized_response() -> tide::Result {
	let status = StatusCode::Unauthorized;

	let body = serde_json::to_value(error::ProblemDetails {
		r#type: "unauthorized".to_string(),
		limit:  None,
		status: status.into(),
		detail: "unauthorized".to_string(),
	})?;

	let response = tide::Response::builder(status)
		.body(body)
		.header("WWW-Authenticate", "Basic")
		.build();

	return Ok(response);
}

fn decode_basic_auth(auth_param: &str) -> tide::Result<Credentials> {
	let bytes = base64::decode(auth_param);
	if bytes.is_err() {
		// This is invalid. Fail the request.
		return Err(tide::Error::from_str(
			StatusCode::Unauthorized,
			"Basic auth param must be valid base64.",
		));
	}

	let as_utf8 = String::from_utf8(bytes.unwrap());
	if as_utf8.is_err() {
		// You know the drill.
		return Err(tide::Error::from_str(
			StatusCode::Unauthorized,
			"Basic auth param base64 must contain valid utf-8.",
		));
	}

	let as_utf8 = as_utf8.unwrap();
	let (username, password) = match as_utf8.split_once(':') {
		Some(t) => t,
		None => {
			return Err(tide::Error::from_str(
				StatusCode::Unauthorized,
				"Basic auth must contain username and password separated by `:`",
			));
		}
	};

	Ok(Credentials {
		username: username.to_owned(),
		password: password.to_owned(),
	})
}

#[async_trait::async_trait]
impl tide::Middleware<State> for Authentication {
	async fn handle(
		&self,
		mut req: tide::Request<State>,
		next: tide::Next<'_, State>,
	) -> tide::Result {
		let auth_header = req.header("Authorization");
		if auth_header.is_none() {
			info!("no auth header, bailing");
			return Ok(unauthorized_response()?);
		}

		let value: Vec<_> = auth_header.unwrap().into_iter().collect();
		if value.is_empty() {
			info!("empty auth header, bailing");
			return Ok(unauthorized_response()?);
		}

		if value.len() > 1 {
			error!("multiple auth headers, bailing");
			return Ok(unauthorized_response()?);
		}

		let scheme = "Basic ";
		let value = value[0].as_str();
		if !value.starts_with(scheme) {
			error!("received invalid auth value: `{:?}`", value);
			return Ok(unauthorized_response()?);
		}

		let auth_param = &value[scheme.len()..];

		info!("saw auth header, attempting to auth");
		let credentials = decode_basic_auth(auth_param)?;
		let email = credentials.username.clone();

		let session_id = req.session().id().to_owned();
		let state = req.state();
		if let Err(e) = state.authenticate(session_id, credentials).await {
			error!("failed to authenticate: {}", e);
			return Ok(unauthorized_response()?);
		}

		req.set_ext(User { email });

		Ok(next.run(req).await)
	}
}
