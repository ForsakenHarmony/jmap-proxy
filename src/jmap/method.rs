use std::{collections::HashMap, fmt};

use serde::{
	de::{SeqAccess, Visitor},
	ser::SerializeTuple,
	Deserialize,
	Deserializer,
	Serialize,
	Serializer,
};

use crate::jmap::{mailbox::Mailbox, Id};

#[derive(Deserialize, Debug)]
#[serde(tag = "t", content = "c")]
#[serde(rename_all = "camelCase")]
pub enum Method {
	#[serde(rename = "Core/echo")]
	CoreEcho(serde_json::Map<String, serde_json::Value>),
	#[serde(rename = "Mailbox/get")]
	MailboxGet {
		account_id: String,
		ids:        Option<Vec<Id>>,
		properties: Option<Vec<String>>,
	},
}

#[derive(Serialize, Debug)]
#[serde(tag = "t", content = "c")]
#[serde(rename_all = "camelCase")]
pub enum MethodResult {
	#[serde(rename = "Core/echo")]
	CoreEcho(serde_json::Map<String, serde_json::Value>),
	#[serde(rename = "Mailbox/get")]
	MailboxGet {
		account_id: String,
		state:      String,
		list:       Vec<Mailbox>,
		not_found:  Vec<Id>,
	},
}

#[derive(Debug)]
pub struct MethodCallResult {
	pub method_result: MethodResult,
	pub call_id:       String,
}

#[derive(Debug)]
pub struct MethodCall {
	pub method:  Method,
	pub call_id: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Request {
	pub using:        Vec<String>,
	pub method_calls: Vec<MethodCall>,
	pub created_ids:  Option<HashMap<Id, Id>>,
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Reponse {
	pub method_responses: Vec<MethodCallResult>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub created_ids:      Option<HashMap<Id, Id>>,
	pub session_state:    String,
}

impl Serialize for MethodCallResult {
	fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
	where
		S: Serializer,
	{
		let mut s = serializer.serialize_tuple(3)?;

		let method =
			serde_json::to_value(&self.method_result).map_err(|e| serde::ser::Error::custom(e))?;
		let obj = method.as_object().unwrap();

		s.serialize_element(&obj["t"])?;
		s.serialize_element(&obj["c"])?;
		s.serialize_element(&self.call_id)?;
		s.end()
	}
}

impl<'de> Deserialize<'de> for MethodCall {
	fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
	where
		D: Deserializer<'de>,
	{
		struct MethodCallVisitor;

		struct MethodCallPartial {
			method_name: String,
			method_args: serde_json::Map<String, serde_json::Value>,
			call_id:     String,
		}

		impl<'de> Visitor<'de> for MethodCallVisitor {
			type Value = MethodCallPartial;

			fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> std::fmt::Result {
				formatter.write_str("a json array / tuple with 3 elements")
			}

			fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
			where
				A: SeqAccess<'de>,
			{
				let method_name: String = seq.next_element()?.unwrap();
				let method_args: serde_json::Map<String, serde_json::Value> =
					seq.next_element()?.unwrap();
				let call_id: String = seq.next_element()?.unwrap();

				Ok(MethodCallPartial {
					method_name,
					method_args,
					call_id,
				})
			}
		}

		let v = deserializer.deserialize_tuple(3, MethodCallVisitor)?;

		let m = Method::deserialize(serde_json::json!({
			"t": v.method_name,
			"c": v.method_args,
		}))
		.map_err(|e| serde::de::Error::custom(e))?;

		Ok(MethodCall {
			method:  m,
			call_id: v.call_id,
		})
	}
}
