use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    Json,
};
use chrono::Local;
use reqwest::Client;
use serde::Deserialize;
use utoipa::IntoParams;

use crate::routes::AnyhowError;

#[derive(Deserialize, IntoParams)]
pub struct Params {
    /// The target date to query for given in the format of YYYY-MM-DD
    date: Option<String>,
}


#[utoipa::path(
    get,
    path = "/schedule/library",
    params(Params),
    responses(
        (
            status = 200,
            body = HashMap<String, String>, 
            description = "The corresponding opening hours of the requested date",
            example = json!({
                "G/F Entrance": "8:00am - 11:00pm",
                "LG5 Entrance": "3:00pm - 11:00pm",
                "Learning Commons": "24 Hours"
            })
        )
    )
)]
pub async fn get(
    State(client): State<Client>,
    Query(params): Query<Params>,
) -> Result<Json<HashMap<String, String>>, AnyhowError> {
    let date = params
        .date
        .unwrap_or(Local::now().format("%Y-%m-%d").to_string());
    #[derive(Deserialize)]
    struct Entry {
        name: String,
        desc: String,
        #[serde(rename = "start")]
        date: String,
    }
    let response = client
        .get("https://lbcone.hkust.edu.hk/hours/hoursapi/gethours?func=calendar")
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<Entry>>()
        .await?
        .into_iter()
        .filter_map(|entry| {
            if entry.date == date {
                Some((entry.name, entry.desc))
            } else {
                None
            }
        })
        .collect::<HashMap<String, String>>();
    Ok(Json(response))
}
