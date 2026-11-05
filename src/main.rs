mod models;
mod features;
mod candidate;
mod ranking;
mod engine;

use crate::engine::Recommender;
use crate::models::{Interaction, UserId, VideoId};
use chrono::Utc;
use std::collections::HashMap;

fn main() {
	let mut reco = Recommender::new();

	// ---------- seed some data ----------
	// Videos
	for i in 1..=50 {
		reco.add_video(VideoId(i), format!("Video {}", i), vec!["tech".into(), "rust".into()]);
	}

	// Users + interactions
	for u in 1..=10 {
		let uid = UserId(u);
		reco.add_user(uid);

		// Simulate watch history
		for v in 1..=8 {
			let vid = VideoId(((u * 3 + v) % 50) + 1);
			reco.record_interaction(Interaction {
				user: uid,
				video: vid,
				watch_secs: 120.0 + (v as f32 * 15.0),
				liked: v % 3 == 0,
				timestamp: Utc::now(),
			});
		}
	}

	// ---------- ask for recommendations ----------
	let user = UserId(3);
	let recommendations = reco.recommend(user, 8);

	println!("Recommendations for user {:?}:", user);
	for (rank, (vid, score)) in recommendations.iter().enumerate() {
		println!("{:>2}. {:?}  score = {:.4}", rank + 1, vid, score);
	}
}