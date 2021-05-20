mod auth;
mod error;
mod imap;
mod jmap;
mod routes;
mod state;

use tide_tracing::TraceMiddleware;

use crate::{auth::Authentication, state::State};

#[async_std::main]
async fn main() -> tide::Result<()> {
	dotenv::dotenv()?;

	tracing_subscriber::fmt()
		.with_max_level(tracing::Level::INFO)
		.init();

	let mut app = tide::with_state(State::new());

	app.with(tide::utils::After(|mut res: tide::Response| async move {
		if res.error().is_some() && res.is_empty().unwrap_or(false) {
			let body = serde_json::to_value(error::ProblemDetails {
				r#type: "unexpected error".to_string(),
				limit:  None,
				status: res.status().into(),
				detail: "unexpected error".to_string(),
			})?;

			res.set_body(body);
		}

		Ok(res)
	}));
	app.with(TraceMiddleware::new());
	app.with(tide::sessions::SessionMiddleware::new(
		tide::sessions::MemoryStore::new(),
		std::env::var("SESSION_SECRET").unwrap().as_bytes(),
	));
	app.with(Authentication::new());

	app.at("/.well-known/jmap").get(routes::session);
	app.at("/jmap").post(routes::jmap);

	app.listen("127.0.0.1:8080").await?;

	// let req = jmap::method::Request {
	// 	using:        vec!["urn:ietf:params:jmap:mail".to_string()],
	// 	method_calls: vec![jmap::method::MethodCall {
	// 		method:  jmap::method::Method::CoreEcho(serde_json::Map::new()),
	// 		call_id: "abc".to_string(),
	// 	}],
	// 	created_ids:  None,
	// };
	//
	// let str = serde_json::to_string(&req)?;
	// tracing::info!("req as str: `{}`", str);
	//
	// let req: jmap::method::Request = serde_json::from_str(&str)?;
	// tracing::info!("req from str: `{:?}`", req);

	// imap::imap_test(
	// 	"hrmny.sh",
	// 	&std::env::var("IMAP_LOGIN").unwrap(),
	// 	&std::env::var("IMAP_PASSWORD").unwrap(),
	// )
	// .await?;

	Ok(())
}
