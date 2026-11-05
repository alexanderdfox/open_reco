use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VideoId(pub u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
	pub id: VideoId,
	pub title: String,
	pub tags: Vec<String>,
	pub created_at: DateTime<Utc>,
	// simple popularity counters (updated from interactions)
	pub view_count: u64,
	pub total_watch_secs: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
	pub user: UserId,
	pub video: VideoId,
	pub watch_secs: f32,
	pub liked: bool,
	pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Default)]
pub struct UserProfile {
	pub watch_history: Vec<(VideoId, f32)>, // (video, watch_secs)
	pub liked: Vec<VideoId>,
	pub search_tokens: Vec<String>,
	// simple average embedding of watched videos
	pub embedding: Vec<f32>,
}