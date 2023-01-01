use crate::UserConfig;
use curl::easy::{Easy, List};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::io;
use std::io::{stdout, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    id: String,
    name: String,
    color: String,
    parent_id: String,
    order: u64,
    comment_count: u64,
    is_shared: bool,
    is_favorite: bool,
    is_inbox_project: bool,
    is_team_inbox: bool,
    view_style: String,
    url: String,
}

pub fn list_projects() {
    let mut easy = Easy::new();
    easy.url("https://api.todoist.com/rest/v2/projects")
        .unwrap();
    let mut list = List::new();
    let authorization_str = String::from("Authorization: Bearer ").to_owned();
    let mut client_config = UserConfig::new();
    client_config.load_config().unwrap();
    let token = client_config.token.to_owned();
    let authorization_str = authorization_str + &token;
    list.append(&authorization_str).unwrap();
    easy.http_headers(list).unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })
    .unwrap();

    easy.perform().unwrap();
}

fn create_project(project_name: &String) {}

fn update_project(project_name: &String) {}

fn delete_project(project_name: &String) {}

fn get_project_collaborators() {}
