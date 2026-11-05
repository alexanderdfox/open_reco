use crate::features::{cosine, user_embedding, video_embedding};
use crate::models::{UserId, VideoId, UserProfile, Video};
use std::collections::HashMap;

/// Ranking – high precision scoring
pub fn rank_candidates(
	user: UserId,
	candidates: &[VideoId],
	profiles: &HashMap<UserId, UserProfile>,
	videos: &HashMap<VideoId, Video>,
) -> Vec<(VideoId, f32)> {
	let profile = match profiles.get(&user) {
		Some(p) => p,
		None => return vec![],
	};

	let user_emb = user_embedding(profile, videos);

	let mut scored = Vec::with_capacity(candidates.len());

	for &vid in candidates {
		let video = match videos.get(&vid) {
			Some(v) => v,
			None => continue,
		};

		let ve = video_embedding(video);

		// ---- clean-room ranking features ----
		let sim = cosine(&user_emb, &ve);                     // content similarity
		let popularity = (video.view_count as f32).ln_1p() * 0.15;
		let avg_watch = if video.view_count > 0 {
			(video.total_watch_secs as f32 / video.view_count as f32).min(600.0) / 600.0
		} else {
			0.3
		};

		// simple linear combination (you can replace with a real model later)
		let score = 0.55 * sim + 0.25 * avg_watch + 0.20 * popularity;

		scored.push((vid, score));
	}

	scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
	scored
}