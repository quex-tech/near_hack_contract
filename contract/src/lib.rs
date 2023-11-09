extern crate serde;
extern crate serde_json;

use serde_json::Value;
use near_sdk::{
	near_bindgen, AccountId, BorshStorageKey, PanicOnDefault,
	borsh::{self, BorshDeserialize, BorshSerialize},
	collections::{UnorderedMap}, log, env, Promise,
    serde::{Deserialize, Serialize}
};
use near_sdk::PublicKey;
use hex;


fn verification(pk_string: &[u8; 32], message: &str, sig_string: &[u8; 64]) -> bool {
	env::ed25519_verify(&sig_string, &message.as_bytes(), pk_string)
}

type PrivateKey = String;
type AI_Response = String;
type Signature = String;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
	owner_id: AccountId,
	public_key: String,
	messages: UnorderedMap<AccountId, Vec<(Sender, String)>>
}


#[derive(BorshDeserialize, BorshSerialize)]
pub struct Message {
	text: String,
	sender: Sender
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
pub enum Sender {
	User,
	Bot
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
	Messages
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, public_key: String) -> Self {
		Self {
			owner_id,
			public_key,
			messages: UnorderedMap::new(StorageKey::Messages),
		}
	}

	pub fn getMessages(&self, account_id: AccountId) -> Option<Vec<(Sender, String)>> {
		self.messages.get(&account_id)
	}

	pub fn initializeAI(&self, pk: PrivateKey, signatures: Vec<PrivateKey>){
		log!("{}", pk)
	}

	#[payable]
	pub fn addRequest(&mut self, msg: String) {
		let account_id = env::predecessor_account_id();
		let mut all_messages = self.messages.get(&account_id).unwrap_or(vec![]);
		all_messages.push((Sender::User, msg.to_string()));

		self.messages.insert(&account_id, &all_messages);


		log!("{}", msg)
	}


	pub fn addResponse(&self, pk_string: String, message: String, sig_string: String) {
		let mut pk = [0u8; 32];
		//let _pk_bytes = hex::decode_to_slice(pk_string, &mut pk as &mut [u8]);
		let _pk_bytes = hex::decode_to_slice(self.public_key.clone(), &mut pk as &mut [u8]);

		let mut sig = [0u8; 64];
		let _sig_string = hex::decode_to_slice(sig_string, &mut sig as &mut [u8]);

		let verified = verification(&pk, &message, &sig);
		log!("{}", verified);
	}

	pub fn addResponse1(&mut self, pk_string: String, message: String, sig_string: String) {
		let mut pk = [0u8; 32];
		//let _pk_bytes = hex::decode_to_slice(pk_string, &mut pk as &mut [u8]);
		let _pk_bytes = hex::decode_to_slice(self.public_key.clone(), &mut pk as &mut [u8]);

		let mut sig = [0u8; 64];
		let _sig_string = hex::decode_to_slice(sig_string, &mut sig as &mut [u8]);

		let parsed_data: Result<Value, serde_json::Error> = serde_json::from_str(&message);

		match parsed_data {
			Ok(parsed_json) => {
				//log!("Content parsed_data: {}", parsed_data.clone().unwrap_or_default());
				log!("Content parsed_json: {:?}", parsed_json);


				let account_id:AccountId = parsed_json["address"].as_str().expect("ERR_NO_SENDER").parse().unwrap();

				// Access fields from the parsed JSON
				let content = parsed_json["response"]["choices"][0]["message"]["content"].as_str().expect("ERR_NO_MESSAGE");
				log!("Content content: {:?}", content);

				if content.to_lowercase() == "funded" || content.to_lowercase() == "funded." {
					Promise::new(account_id.clone()).transfer(1000000000000000000000000);
				}

				let mut all_messages = self.messages.get(&account_id).unwrap_or(vec![]);
				all_messages.push((Sender::Bot, content.to_string()));

				self.messages.insert(&account_id, &all_messages);

			}
			Err(e) => {
				log!("Error parsing JSON: {}", e);
			}
		}

		let verified = verification(&pk, &message, &sig);
		log!("{}", verified);
	}




	pub fn testResponse(&self) {
		let pk_string = "15896f5e867762ba4e1f75c3243e231c118ea2d5b3a2e99cede1190527e819f4";
		let message = "{\"data\":{\"respose\":{\"id\":\"chatcmpl-abc123\",\"object\":\"chat.completion\",\"created\":1677858242,\"model\":\"gpt-3.5-turbo-1006\",\"usage\":{\"prompt_tokens\":13,\"completion_tokens\":7,\"total_tokens\":20},\"choices\":[{\"message\":{\"role\":\"assistant\",\"content\":\"\n\nThis is a test!\"},\"finish_reason\":\"stop\",\"index\":0}]},\"address\":\"\"},\"signature\":\"\"}";

		let sig_string = "e6ecb57c1d8bf9e7751f6b52634d01f5402f162e9f91eb68f1e161e4b675baba9a432b1a1413ef39522116e6839b7ed295796803342db36a24ed595427bdf000";

		let mut pk = [0u8; 32];
		let _pk_bytes = hex::decode_to_slice(pk_string, &mut pk as &mut [u8]);

		let mut sig = [0u8; 64];
		let _sig_string = hex::decode_to_slice(sig_string, &mut sig as &mut [u8]);

		log!("r {:?}", _pk_bytes);
		log!("{:?}", pk);

		log!("r {:?}", _sig_string);
		log!("{:?}", sig);

		let verified = verification(&pk, &message, &sig);
		log!("{}", verified)
	}
}
