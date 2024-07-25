use crate::{config::AssetManifest, run::AppState};

use super::Actor;

#[derive(Clone)]
pub struct TemplateData {
    pub title: String,
    pub assets: AssetManifest,
    pub styles: Vec<String>,
    pub scripts: Vec<String>,
    pub async_scripts: Vec<String>,
    pub actor: Option<Actor>,
}

impl TemplateData {
    pub fn new(state: &AppState, actor: Option<Actor>) -> TemplateData {
        let config = state.config.clone();
        let assets = config.assets.clone();

        TemplateData {
            title: String::from(""),
            assets,
            styles: Vec::new(),
            scripts: Vec::new(),
            async_scripts: Vec::new(),
            actor,
        }
    }
}
