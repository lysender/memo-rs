use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Pref {
    pub theme: String,
}

impl Pref {
    pub fn new() -> Self {
        Self {
            theme: String::from("light"),
        }
    }
}
