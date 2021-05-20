use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProblemDetails {
	pub r#type: String,
	pub limit:  Option<String>,
	pub status: u16,
	pub detail: String,
}
