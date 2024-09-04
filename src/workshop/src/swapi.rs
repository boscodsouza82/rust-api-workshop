/*
:clear
// */

use std::time::Duration;

pub struct SwapiClient {
    base_url: String
}

pub struct Person {
}

impl SwapiClient {

    pub fn new(base_url_a: String, timeout_a: Duration) -> Result<Self, ()> {
        Ok(Self {base_url: base_url_a})
    }
}
