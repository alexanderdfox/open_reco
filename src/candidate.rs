use crate::features::{cosine, user_embedding, video_embedding, EMBED_DIM};
use crate::models::{UserId, VideoId, UserProfile, Video};
use std::collections::{HashMap, HashSet};

/// Candidate generation – high recall
pub fn generate_candidates(
	user: UserId,
	profiles: &HashMap<UserId, UserProfile>,
	videos: &HashMap<VideoId, Video>,
	interactions: &[crate::models::Interaction],
	k: usize,
) -> Vec<VideoId> {
	let mut candidates = HashSet::new();

	let profile = match profiles.get(&user) {
		Some(p) => p,
		None => return vec![],
	};

	// 1. Embedding retrieval (nearest neighbors in embedding space)
	let user_emb = user_embedding(profile, videos);
	let mut scored: Vec<(VideoId, f32)> = videos
		.iter()
		.map(|(vid, v)| {
			let ve = video_embedding(v);
			(*vid, cosine(&user_emb, &ve))
		})
		.collect();

	scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
	for (vid, _) in scored.into_iter().take(k * 2) {
		candidates.insert(vid);
	}

	// 2. Collaborative filtering – co-watch
	let mut co_watch: HashMap<VideoId, f32> = HashMap::new();
	for inter in interactions {
		if inter.user == user {
			continue;
		}
		// if this user watched something we also watched
		if profile.watch_history.iter().any(|(v, _)| *v == inter.video) {
			*co_watch.entry(inter.video).or_default() += inter.watch_secs;
		}
	}
	let mut co: Vec<_> = co_watch.into_iter().collect();
	co.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
	for (vid, _) in co.into_iter().take(k) {
		candidates.insert(vid);
	}

	// 3. Popularity prior (for cold-start / exploration)
	let mut popular: Vec<_> = videos
		.iter()
		.map(|(vid, v)| (*vid, v.view_count as f32 + v.total_watch_secs as f32 * 0.01))
		.collect();
	popular.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
	for (vid, _) in popular.into_iter().take(k / 2) {
		candidates.insert(vid);
	}

	// Remove already watched
	for (vid, _) in &profile.watch_history {
		candidates.remove(vid);
	}

	candidates.into_iter().take(k * 3).collect() // return a few hundred max
}