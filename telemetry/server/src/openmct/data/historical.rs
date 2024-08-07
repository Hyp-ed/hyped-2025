use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::{extract::Path, routing::get, Json, Router};
use axum_macros::debug_handler;
use chrono::{DateTime, FixedOffset};
use influxdb2::models::Query as InfluxQuery;
use influxdb2::FromDataPoint;
use serde::Serialize;

use crate::TelemetryServerState;

pub fn get_routes() -> Router<TelemetryServerState> {
    Router::new().route(
        "/pods/:pod/measurements/:measurement_key",
        get(get_historical_reading),
    )
}

#[derive(Debug, FromDataPoint, Serialize)]
pub struct InfluxHistoricalReading {
    // TODOLater: update this to not use camelCase
    measurementKey: String,
    _time: DateTime<FixedOffset>,
    value: f64,
}

#[derive(Debug, Serialize)]

pub struct HistoricalReading {
    id: String,
    timestamp: i64,
    value: f64,
}

impl Default for InfluxHistoricalReading {
    fn default() -> Self {
        Self {
            measurementKey: "".to_string(),
            _time: DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z").unwrap(),
            value: 0.0,
        }
    }
}

#[derive(serde::Deserialize)]

struct TimespanQuery {
    start: i64,
    end: i64,
}

#[debug_handler]
async fn get_historical_reading(
    Path((pod, measurement_key)): Path<(String, String)>,
    Query(query): Query<TimespanQuery>,
    State(TelemetryServerState {
        influxdb_client, ..
    }): State<TelemetryServerState>,
) -> Result<Json<Vec<HistoricalReading>>, (StatusCode, String)> {
    let telemetry_bucket = std::env::var("INFLUXDB_TELEMETRY_BUCKET").unwrap();

    let qs = format!(
        "from(bucket: \"{}\")
        |> range(start: {}, stop: {})
        |> filter(fn: (r) => r[\"measurementKey\"] == \"{}\")
        |> filter(fn: (r) => r[\"podId\"] == \"{}\")",
        telemetry_bucket,
        DateTime::from_timestamp_millis(query.start)
            .unwrap()
            .to_rfc3339(),
        DateTime::from_timestamp_millis(query.end)
            .unwrap()
            .to_rfc3339(),
        measurement_key,
        pod
    );

    let query = InfluxQuery::new(qs.to_string());
    let influx_res = influxdb_client
        .query::<InfluxHistoricalReading>(Some(query))
        .await;

    match influx_res {
        Ok(res) => {
            let res = Json(
                res.into_iter()
                    .map(|reading| HistoricalReading {
                        id: reading.measurementKey,
                        timestamp: reading._time.timestamp_millis(),
                        value: reading.value,
                    })
                    .collect(),
            );
            Ok(res)
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
