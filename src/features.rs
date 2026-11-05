use crate::models::{Interaction, UserId, VideoId, UserProfile, Video};
use std::collections::HashMap;

/// Very simple bag-of-tags embedding (one-hot style, fixed size for demo)
pub const EMBED_DIM: usize = 32;

pub fn video_embedding(video: &Video) -> Vec<f32> {
	let mut emb = vec![0.0f32; EMBED_DIM];
	for (i, tag) in video.tags.iter().enumerate() {
		let h = simple_hash(tag) % EMBED_DIM;
		emb[h] += 1.0;
	}
	// L2 normalize
	let norm: f32 = emb.iter().map(|x| x * x).sum::<f32>().sqrt().max(1e-8);
	emb.iter_mut().for_each(|x| *x /= norm);
	emb
}

pub fn user_embedding(profile: &UserProfile, videos: &HashMap<VideoId, Video>) -> Vec<f32> {
	if profile.watch_history.is_empty() {
		return vec![0.0; EMBED_DIM];
	}

	let mut emb = vec![0.0f32; EMBED_DIM];
	let mut total_weight = 0.0f32;

	for &(vid, secs) in &profile.watch_history {
		if let Some(v) = videos.get(&vid) {
			let ve = video_embedding(v);
			let w = secs.max(1.0);
			for i in 0..EMBED_DIM {
				emb[i] += ve[i] * w;
			}
			total_weight += w;
		}
	}

	if total_weight > 0.0 {
		emb.iter_mut().for_each(|x| *x /= total_weight);
	}

	// L2 normalize
	let norm: f32 = emb.iter().map(|x| x * x).sum::<f32>().sqrt().max(1e-8);
	emb.iter_mut().for_each(|x| *x /= norm);
	emb
}

fn simple_hash(s: &str) -> usize {
	s.bytes().fold(0usize, |acc, b| acc.wrapping_mul(31).wrapping_add(b as usize))
}

pub fn cosine(a: &[f32], b: &[f32]) -> f32 {
	let mut dot = 0.0;
	let mut na = 0.0;
	let mut nb = 0.0;
	for i in 0..a.len().min(b.len()) {
		dot += a[i] * b[i];
		na += a[i] * a[i];
		nb += b[i] * b[i];
	}
	if na == 0.0 || nb == 0.0 {
		0.0
	} else {
		dot / (na.sqrt() * nb.sqrt())
	}
}