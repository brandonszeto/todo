use curl::easy::{Easy, List};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::io;
use std::io::{stdout, Write};

#[derive(Serialize, Deserialize)]
struct Task {
    id: String,
    project_id: String,
    section_id: String,
    content: String,
    description: String,
    is_completed: bool,
    labels: Vec<String>,
    parent_id: String,
    order: u64,
    priority: u8,
    due: Due,
    url: String,
    comment_count: u64,
    created_at: String,
    creator_id: String,
    assignee_id: String,
    assigner_id: String,
}

#[derive(Serialize, Deserialize)]
struct Due {
    string: String,
    date: String,
    is_recurring: bool,
    datetime: String,
    timezone: String,
}
