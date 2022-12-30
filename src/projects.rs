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

fn list_projects(api: String) {}

fn create_project() {}

fn list_projects() {}
