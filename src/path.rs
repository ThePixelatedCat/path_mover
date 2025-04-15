use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Path {
    version: String,
    pub waypoints: Vec<Waypoint>,
    #[serde(flatten)]
    zones_and_constraints: HashMap<String, Value>,
    pub goal_end_state: State,
    #[serde(flatten)]
    start_state_misc: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Waypoint {
    pub anchor: Point,
    pub prev_control: Option<Point>,
    pub next_control: Option<Point>,
    is_locked: bool,
    linked_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    velocity: f64,
    pub rotation: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
