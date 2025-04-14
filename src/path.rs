use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Path {
	waypoints: Vec<Waypoint>,

	#[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Waypoint {
	anchor: Point,
	prev_control: Option<Point>,
	next_control: Option<Point>,
	is_locked: bool,
	linked_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Point {
	x: f64,
	y: f64,
}