use crate::candidate::generate_candidates;
use crate::models::*;
use crate::ranking::rank_candidates;
use chrono::Utc;
use std::collections::HashMap;

pub struct Recommender {
	pub users: HashMap<UserId, UserProfile>,
	pub videos: HashMap<VideoId, Video>,
	pub interactions: Vec<Interaction>,
}

impl Recommender {
	pub fn new() -> Self {
		Self {
			users: HashMap::new(),
			videos: HashMap::new(),
			interactions: Vec::new(),
		}
	}

	pub fn add_user(&mut self, id: UserId) {
		self.users.entry(id).or_default();
	}

	pub fn add_video(&mut self, id: VideoId, title: String, tags: Vec<String>) {
		self.videos.insert(
			id,
			Video {
				id,
				title,
				tags,
				created_at: Utc::now(),
				view_count: 0,
				total_watch_secs: 0.0,
			},
		);
	}

	pub fn record_interaction(&mut self, inter: Interaction) {
		// update video stats
		if let Some(v) = self.videos.get_mut(&inter.video) {
			v.view_count += 1;
			v.total_watch_secs += inter.watch_secs as f64;
		}

		// update user profile
		let profile = self.users.entry(inter.user).or_default();
		profile.watch_history.push((inter.video, inter.watch_secs));
		if inter.liked {
			profile.liked.push(inter.video);
		}

		// keep history reasonable
		if profile.watch_history.len() > 100 {
			profile.watch_history.drain(0..20);
		}

		self.interactions.push(inter);
	}

	/// Main entry point
	pub fn recommend(&self, user: UserId, n: usize) -> Vec<(VideoId, f32)> {
		let candidates = generate_candidates(
			user,
			&self.users,
			&self.videos,
			&self.interactions,
			200, // candidate pool size
		);

		let ranked = rank_candidates(user, &candidates, &self.users, &self.videos);

		ranked.into_iter().take(n).collect()
	}
}