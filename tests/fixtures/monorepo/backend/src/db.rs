use std::collections::HashMap;

// in-memory store for demo purposes
pub struct Store {
	data: HashMap<String, String>,
}

impl Store {
	pub fn new() -> Self {
		Self { data: HashMap::new() }
	}

	pub fn get(&self, key: &str) -> Option<&String> {
		self.data.get(key)
	}

	pub fn set(&mut self, key: String, val: String) {
		self.data.insert(key, val);
	}
}
