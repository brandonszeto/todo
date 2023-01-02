use reqwest::{
    blocking::Client,
    header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
};

use crate::config::UserConfig;
use uuid::Uuid;

pub fn post_todoist_rest(
    token: String,
    url: String,
    body: serde_json::Value,
) -> Result<String, String> {
    let todoist_url: &str = "https://api.todoist.com";
    let request_url = format!("{}{}", todoist_url, url);
    let authorization: &str = &format!("Bearer {}", token);

    let response = Client::new()
        .post(request_url)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, authorization)
        .header("X-Request-Id", new_uuid())
        .json(&body)
        .send()
        .or(Err("Did not get response from server"))?;

    if response.status().is_success() {
        Ok(response.text().or(Err("Could not read response text"))?)
    } else {
        Err(format!("Error: {:#?}", response.text()))
    }
}

fn new_uuid() -> String {
    Uuid::new_v4().to_string()
}

// pub fn add_item_to_inbox(config: &UserConfig, task: &str) -> Result<Item, String> {
//     let url = String::from(QUICK_ADD_URL);
//     let body = json!({"text": task, "auto_reminder": true});

//     let json = post_todoist_rest(config.token.clone(), url, body)?;
//     items::json_to_item(json)
// }
