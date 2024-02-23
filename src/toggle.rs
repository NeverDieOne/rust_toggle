use serde::Deserialize;

use crate::config::ToggleConfig;

extern crate serde_json;
extern crate chrono;


#[derive(Debug, Deserialize)]
pub struct TimeEntry {
    pub id: u32,
    pub description: String
}


pub fn set_proxy(config: &mut ToggleConfig, proxy_url: String) {
    config.proxy = proxy_url;
}

pub fn auth(config: &mut ToggleConfig, api_key: String) {
    config.api_key = api_key;

}

pub fn logout(config: &mut ToggleConfig) {
    config.api_key = String::from("");
}

pub fn get_current_time_entry(client: &reqwest::blocking::Client, config: &ToggleConfig) -> Option<TimeEntry> {
    let res: Option<TimeEntry> = client.get("https://api.track.toggl.com/api/v9/me/time_entries/current".to_string())
        .basic_auth(&config.api_key, Some("api_token"))
        .send()
        .unwrap()
        .json().expect("Can not create time entry");
    res
}

pub fn start_entry(client: &reqwest::blocking::Client, config: &ToggleConfig, description: String) {
    let res = client.post(format!("https://api.track.toggl.com/api/v9/workspaces/{}/time_entries", config.workspace_id))
        .basic_auth(&config.api_key, Some("api_token"))
        .json(&serde_json::json!({
            "description": description,
            "start": chrono::Local::now().to_utc().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            "created_with": String::from("API"),
            "workspace_id": config.workspace_id,
            "duration": -1,
        }))
        .send()
        .unwrap()
        .text();

    println!("{:?}", res)
}

pub fn stop_entry(client: &reqwest::blocking::Client, config: &ToggleConfig) {
    let current_entry = get_current_time_entry(client, config);
    match current_entry {
        Some(entry) => {
            let res = client.patch(format!("https://api.track.toggl.com/api/v9/workspaces/{}/time_entries/{}/stop", config.workspace_id, entry.id))
                .basic_auth(&config.api_key, Some("api_token"))
                .send()
                .unwrap()
                .text();

            println!("{:?}", res)
        }
        None => println!("There is no time entry to stop")
    }
}
