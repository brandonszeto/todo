use chrono::DateTime;
use chrono::NaiveDate;
use chrono_tz::Tz;
use colored::*;
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;

use crate::config::Config;
use crate::{config, items, request, time};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Item {
    pub id: String,
    pub content: String,
    pub priority: u8,
    pub checked: bool,
    pub description: String,
    pub due: Option<DateInfo>,
    pub is_deleted: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct DateInfo {
    pub date: String,
    pub is_recurring: bool,
    pub timezone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Body {
    items: Vec<Item>,
}

enum DateTimeInfo {
    NoDateTime,
    Date {
        date: NaiveDate,
        is_recurring: bool,
    },
    DateTime {
        datetime: DateTime<Tz>,
        is_recurring: bool,
    },
}

impl Item {
    pub fn fmt(&self, config: &Config) -> String {
        let content = match self.priority {
            2 => self.content.blue(),
            3 => self.content.yellow(),
            4 => self.content.red(),
            _ => self.content.normal(),
        };

        let description = match &*self.description {
            "" => String::from(""),
            _ => format!("\n{}", self.description),
        };

        let due = match &self.datetimeinfo(config) {
            Ok(DateTimeInfo::Date { date, is_recurring }) => {
                let recurring_icon = if *is_recurring { " ↻" } else { "" };
                let date_string = time::format_date(date, config);

                format!("\nDue: {}{}", date_string, recurring_icon)
            }
            Ok(DateTimeInfo::DateTime {
                datetime,
                is_recurring,
            }) => {
                let recurring_icon = if *is_recurring { " ↻" } else { "" };
                let datetime_string = time::format_datetime(datetime, config);

                format!("\nDue: {}{}", datetime_string, recurring_icon)
            }
            Ok(DateTimeInfo::NoDateTime) => String::from(""),
            Err(string) => string.clone(),
        };

        format!("\n{}{}{}", content, description, due)
    }

    /// Determines the numeric value of an item for sorting
    fn value(&self, config: &Config) -> u32 {
        let date_value: u8 = self.date_value(config);
        let priority_value: u8 = self.priority_value();

        date_value as u32 + priority_value as u32
    }

    /// Return the value of the due field
    fn date_value(&self, config: &Config) -> u8 {
        match &self.datetimeinfo(config) {
            Ok(DateTimeInfo::NoDateTime) => 80,
            Ok(DateTimeInfo::Date { date, is_recurring }) => {
                let today_value = if *date == time::today_date(config) {
                    100
                } else {
                    0
                };
                let overdue_value = if self.is_overdue(config) { 150 } else { 0 };
                let recurring_value = if is_recurring.to_owned() { 0 } else { 50 };
                today_value + overdue_value + recurring_value
            }
            Ok(DateTimeInfo::DateTime {
                datetime,
                is_recurring,
            }) => {
                let recurring_value = if is_recurring.to_owned() { 0 } else { 50 };
                let duration = *datetime - time::now(config);
                match duration.num_minutes() {
                    -15..=15 => 200 + recurring_value,
                    _ => recurring_value,
                }
            }
            Err(_) => 50,
        }
    }

    /// Return the value of the due field
    fn datetime(&self, config: &Config) -> Option<DateTime<Tz>> {
        match self.datetimeinfo(config) {
            Ok(DateTimeInfo::DateTime { datetime, .. }) => Some(datetime),
            _ => None,
        }
    }

    fn priority_value(&self) -> u8 {
        match self.priority {
            2 => 1,
            3 => 3,
            4 => 4,
            _ => 2,
        }
    }

    /// Converts the JSON date representation into Date or Datetime
    fn datetimeinfo(&self, config: &Config) -> Result<DateTimeInfo, String> {
        let tz = match (self.clone().due, config.clone().timezone) {
            (None, Some(tz_string)) => time::timezone_from_str(&Some(tz_string)),
            (None, None) => Tz::UTC,
            (Some(DateInfo { timezone: None, .. }), Some(tz_string)) => time::timezone_from_str(&Some(tz_string)),
            (Some(DateInfo { timezone: None, .. }), None) => Tz::UTC,
            (Some(DateInfo {
                timezone: Some(tz_string),
                ..
                // Remove the Some here
            }), _) => time::timezone_from_str(&Some(tz_string)),
        };
        match self.clone().due {
            None => Ok(DateTimeInfo::NoDateTime),
            Some(DateInfo {
                date, is_recurring, ..
            }) if date.len() == 10 => Ok(DateTimeInfo::Date {
                date: time::date_from_str(&date, tz)?,
                is_recurring,
            }),
            Some(DateInfo {
                date, is_recurring, ..
            }) => Ok(DateTimeInfo::DateTime {
                datetime: time::datetime_from_str(&date, tz)?,
                is_recurring,
            }),
        }
    }

    fn has_no_date(&self) -> bool {
        self.due.is_none()
    }

    // Returns true if the datetime is today and there is a time
    fn is_today(&self, config: &Config) -> bool {
        match self.datetimeinfo(config) {
            Ok(DateTimeInfo::NoDateTime) => false,
            Ok(DateTimeInfo::Date { date, .. }) => date == time::today_date(config),
            Ok(DateTimeInfo::DateTime { datetime, .. }) => {
                time::datetime_is_today(datetime, config)
            }
            Err(_) => false,
        }
    }

    fn is_overdue(&self, config: &Config) -> bool {
        match self.clone().datetimeinfo(config) {
            Ok(DateTimeInfo::NoDateTime) => false,
            Ok(DateTimeInfo::Date { date, .. }) => time::is_date_in_past(date, config),
            Ok(DateTimeInfo::DateTime { datetime, .. }) => {
                time::is_date_in_past(datetime.date_naive(), config)
            }
            Err(_) => false,
        }
    }

    /// Returns true when it is a datetime, otherwise false
    fn has_time(&self, config: &Config) -> bool {
        matches!(
            self.clone().datetimeinfo(config),
            Ok(DateTimeInfo::DateTime { .. })
        )
    }
}

pub fn json_to_items(json: String) -> Result<Vec<Item>, String> {
    let result: Result<Body, _> = serde_json::from_str(&json);
    match result {
        Ok(body) => Ok(body.items),
        Err(err) => Err(format!("Could not parse response for item: {:?}", err)),
    }
}

pub fn json_to_item(json: String) -> Result<Item, String> {
    match serde_json::from_str(&json) {
        Ok(item) => Ok(item),
        Err(err) => Err(format!("Could not parse response for item: {:?}", err)),
    }
}

pub fn sort_by_value(mut items: Vec<Item>, config: &Config) -> Vec<Item> {
    items.sort_by_key(|b| Reverse(b.value(config)));
    items
}

pub fn sort_by_datetime(mut items: Vec<Item>, config: &Config) -> Vec<Item> {
    items.sort_by_key(|i| i.datetime(config));
    items
}

pub fn filter_not_in_future(items: Vec<Item>, config: &Config) -> Result<Vec<Item>, String> {
    let items = items
        .into_iter()
        .filter(|item| item.is_today(config) || item.has_no_date() || item.is_overdue(config))
        .collect();

    Ok(items)
}

pub fn filter_today_and_has_time(items: Vec<Item>, config: &Config) -> Vec<Item> {
    items
        .into_iter()
        .filter(|item| item.is_today(config) && item.has_time(config))
        .collect()
}

pub fn set_priority(config: Config, item: items::Item) {
    println!("{}", item.fmt(&config));

    let priority = config::get_input("Choose a priority from 1 (lowest) to 3 (highest):")
        .expect("Please enter a number from 1 to 3");

    match priority.as_str() {
        "1" => {
            let config = config.set_next_id(item.id.clone());
            request::update_item_priority(config, item, 2).expect("could not set priority");
        }
        "2" => {
            let config = config.set_next_id(item.id.clone());
            request::update_item_priority(config, item, 3).expect("could not set priority");
        }
        "3" => {
            let config = config.set_next_id(item.id.clone());
            request::update_item_priority(config, item, 4).expect("could not set priority");
        }
        _ => println!("Not a valid input, please enter 1, 2, or 3"),
    }
}
