use futures::TryStreamExt;

use crate::auth;

#[tracing::instrument(skip(credentials), fields(email = credentials.username.as_str()))]
pub async fn create_imap_session(
	imap_server: &str,
	credentials: auth::Credentials,
) -> async_imap::error::Result<
	async_imap::Session<async_native_tls::TlsStream<async_std::net::TcpStream>>,
> {
	let tls = async_native_tls::TlsConnector::new();
	let imap_addr = (imap_server, 993);

	// we pass in the imap_server twice to check that the server's TLS
	// certificate is valid for the imap_server we're connecting to.
	let client = async_imap::connect(imap_addr, imap_server, tls).await?;
	tracing::info!("connected to {}:{}", imap_addr.0, imap_addr.1);

	// the client we have here is unauthenticated.
	// to do anything useful with the e-mails, we need to log in
	let imap_session = client
		.login(&credentials.username, &credentials.password)
		.await
		.map_err(|e| e.0)?;

	tracing::info!("logged in as {}", credentials.username);

	Ok(imap_session)
}

pub async fn imap_test(
	imap_server: &str,
	login: &str,
	password: &str,
) -> async_imap::error::Result<()> {
	// use async_imap::error::Result;

	// let tls = async_native_tls::TlsConnector::new();
	// let client = async_imap::connect(("hrmny.sh", 993), "hrmny.sh", tls).await?;
	//
	// let mut session = match client
	// 	.login(
	// 		std::env::var("IMAP_LOGIN").unwrap(),
	// 		std::env::var("IMAP_PASSWORD").unwrap(),
	// 	)
	// 	.await
	// {
	// 	Ok(s) => s,
	// 	Err((e, _orig_client)) => {
	// 		tracing::error!("failed to login: {}", e);
	// 		return Err(e.into());
	// 	}
	// };
	//
	// let capabilities = session.capabilities().await?;
	// tracing::info!("got capabilities {:#?}", capabilities.iter());

	let tls = async_native_tls::TlsConnector::new();
	let imap_addr = (imap_server, 993);

	// we pass in the imap_server twice to check that the server's TLS
	// certificate is valid for the imap_server we're connecting to.
	let client = async_imap::connect(imap_addr, imap_server, tls).await?;
	tracing::info!("-- connected to {}:{}", imap_addr.0, imap_addr.1);

	// the client we have here is unauthenticated.
	// to do anything useful with the e-mails, we need to log in
	let mut imap_session = client.login(login, password).await.map_err(|e| e.0)?;
	tracing::info!("-- logged in a {}", login);

	// // we want to fetch the first email in the INBOX mailbox
	// imap_session.select("INBOX").await?;
	// tracing::info!("-- INBOX selected");
	//
	// // fetch message number 1 in this mailbox, along with its RFC822 field.
	// // RFC 822 dictates the format of the body of e-mails
	// let messages_stream = imap_session.fetch("1", "RFC822").await?;
	// let messages: Vec<_> = messages_stream.collect::<Result<_>>().await?;
	// let message = if let Some(m) = messages.first() {
	// 	m
	// } else {
	// 	return Ok(());
	// };
	//
	// // extract the message's body
	// let body = message.body().expect("message did not have a body!");
	// let body = std::str::from_utf8(body)
	// 	.expect("message was not valid utf-8")
	// 	.to_string();
	// tracing::info!("-- 1 message received, logging out");
	//
	// // be nice to the server and log out
	// imap_session.logout().await?;
	//
	// tracing::info!("result:\n{}", body);

	// let caps = imap_session.capabilities().await?;
	//
	// tracing::info!("capabilities: {:#?}", caps.iter());

	let mut list = imap_session.list(None, Some("*")).await?;
	// let list = list.collect::<Vec<_>>().await;

	while let Some(item) = list.try_next().await? {
		tracing::info!(
			"list-item: {} [{}] {:?}",
			item.name(),
			item.delimiter().unwrap_or("-none-"),
			item.attributes()
		);
	}

	Ok(())
}
