use reqwest::blocking::Client;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::config::Config;
use crate::items::Item;
use crate::{items, projects};

// TODOIST URLS
const QUICK_ADD_URL: &str = "/sync/v9/quick/add";
const PROJECT_DATA_URL: &str = "/sync/v9/projects/get_data";
const SYNC_URL: &str = "/sync/v9/sync";
const REST_V2_TASKS_URL: &str = "/rest/v2/tasks/";

// CRATES.IO URLS
const VERSIONS_URL: &str = "/v1/crates/tod/versions";

#[derive(Deserialize)]
struct CargoResponse {
    versions: Vec<Version>,
}

#[derive(Deserialize)]
struct Version {
    num: String,
}

/// Add a new item to the inbox with natural language support
pub fn add_item_to_inbox(config: &Config, task: &str) -> Result<Item, String> {
    let url = String::from(QUICK_ADD_URL);
    let body = json!({"text": task, "auto_reminder": true});

    let json = post_todoist_sync(config.token.clone(), url, body)?;
    items::json_to_item(json)
}

/// Get a vector of all items for a project
pub fn items_for_project(config: &Config, project_id: &str) -> Result<Vec<Item>, String> {
    let url = String::from(PROJECT_DATA_URL);
    let body = json!({ "project_id": project_id });
    let json = post_todoist_sync(config.token.clone(), url, body)?;
    items::json_to_items(json)
}

/// Move an item to a different project
pub fn move_item(config: Config, item: Item, project_name: &str) -> Result<String, String> {
    let project_id = projects::project_id(&config, project_name)?;
    let body = json!({"commands": [{"type": "item_move", "uuid": new_uuid(), "args": {"id": item.id, "project_id": project_id}}]});
    let url = String::from(SYNC_URL);

    post_todoist_sync(config.token, url, body)?;
    Ok(String::from("✓"))
}

/// Update the priority of an item by ID
pub fn update_item_priority(config: Config, item: Item, priority: u8) -> Result<String, String> {
    let body = json!({ "priority": priority });
    let url = format!("{}{}", REST_V2_TASKS_URL, item.id);

    post_todoist_rest(config.token, url, body)?;
    // Does not pass back an item
    Ok(String::from("✓"))
}

/// Complete the last item returned by "next item"
pub fn complete_item(config: Config) -> Result<String, String> {
    let body = json!({"commands": [{"type": "item_close", "uuid": new_uuid(), "temp_id": new_uuid(), "args": {"id": config.next_id}}]});
    let url = String::from(SYNC_URL);

    post_todoist_sync(config.token.clone(), url, body)?;

    config.clear_next_id().save()?;

    // Does not pass back an item
    Ok(String::from("✓"))
}

/// Post to Todoist via sync API
fn post_todoist_sync(
    token: String,
    url: String,
    body: serde_json::Value,
) -> Result<String, String> {
    let todoist_url: &str = "https://api.todoist.com";

    let request_url = format!("{}{}", todoist_url, url);

    let response = Client::new()
        .post(request_url)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .json(&body)
        .send()
        .or(Err("Did not get response from server"))?;

    if response.status().is_success() {
        Ok(response.text().or(Err("Could not read response text"))?)
    } else {
        Err(format!("Error: {:#?}", response.text()))
    }
}

/// Post to Todoist via REST api
fn post_todoist_rest(
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

/// Get latest version number from Cargo.io
pub fn get_latest_version() -> Result<String, String> {
    let cargo_url: &str = "https://crates.io/api";

    let request_url = format!("{}{}", cargo_url, VERSIONS_URL);

    let response = Client::new()
        .get(request_url)
        .header(USER_AGENT, "Tod")
        .send()
        .or(Err("Did not get response from server"))?;

    if response.status().is_success() {
        let cr: CargoResponse =
            serde_json::from_str(&response.text().or(Err("Could not read response text"))?)
                .or(Err("Could not serialize to CargoResponse"))?;
        Ok(cr.versions.first().unwrap().num.clone())
    } else {
        Err(format!("Error: {:#?}", response.text()))
    }
}

/// Create a new UUID, required for Todoist API
fn new_uuid() -> String {
    Uuid::new_v4().to_string()
}
