# open_reco

A clean-room, fully independent video recommendation system written in Rust.

This project implements a two-stage recommendation pipeline inspired by the publicly described architecture of large video platforms (candidate generation → ranking). It uses only standard techniques and data you own. No proprietary code, models, or reverse-engineered logic is included.

## Architecture Overview

The system follows the classic high-recall → high-precision design:

```
User request
    │
    ▼
┌─────────────────────────┐
│  Candidate Generation   │  ← Retrieve a few hundred plausible videos
│  (high recall)          │
└───────────┬─────────────┘
            │
            ▼
┌─────────────────────────┐
│       Ranking           │  ← Score and order the candidates
│  (high precision)       │
└───────────┬─────────────┘
            │
            ▼
     Top-N recommendations
```

### 1. Candidate Generation (`src/candidate.rs`)

This stage reduces a large catalog to a manageable set of candidates. Three complementary strategies are used:

- **Embedding retrieval**  
  Both users and videos are projected into a shared vector space (simple bag-of-tags embeddings). Cosine similarity finds videos close to the user’s taste.

- **Collaborative filtering (co-watch)**  
  Videos frequently watched by users with overlapping history are promoted.

- **Popularity prior**  
  A small amount of globally popular content is mixed in. This helps cold-start users and encourages exploration.

Already-watched videos are filtered out.

### 2. Ranking (`src/ranking.rs`)

Every candidate receives a relevance score for the specific user. The current scorer is a transparent linear combination of:

- Content similarity (cosine of user and video embeddings)
- Average watch percentage / retention signal
- Popularity (log-scaled view count)

You can replace this with any ranking model (gradient boosting, neural net, etc.) without changing the rest of the pipeline.

### 3. User & Video Profiles (`src/models.rs` + `src/features.rs`)

- **UserProfile** keeps a rolling watch history, liked videos, and a derived embedding.
- **Video** stores metadata, tags, and aggregate engagement statistics.
- Embeddings are computed on the fly from tags and weighted by watch time.

## Data Flow

1. You call `record_interaction()` whenever a user watches (or likes) a video.  
   This updates both the user’s history and the video’s popularity counters.

2. When you call `recommend(user, n)`:
   - Candidate generation builds a pool of ~200–600 videos.
   - Ranking scores every candidate.
   - The top-N results are returned with their scores.

## Running the Example

```bash
cargo run
```

The demo seeds 50 videos and 10 users with synthetic watch histories, then prints recommendations for user 3.

## Extending the System

| Goal                        | Where to change                          |
|----------------------------|------------------------------------------|
| Better embeddings          | `features.rs` – replace bag-of-tags     |
| Real neural ranking model  | `ranking.rs`                             |
| Approximate nearest neighbors | `candidate.rs` (add FAISS/HNSW)       |
| Persistence                | Add `serde` + SQLite / Sled / RocksDB   |
| Negative feedback          | Extend `UserProfile` + ranking penalty  |
| Multi-objective ranking    | Predict click + watch time + satisfaction|

## Design Principles (Clean-Room)

- Only public knowledge of recommendation system architecture was used.
- All algorithms, feature engineering, and scoring logic were written from scratch.
- No YouTube, TikTok, or other platform code, model weights, or internal documentation was consulted or reproduced.
- The system is intentionally simple and transparent so you can understand and modify every part.

## License

MIT (or whatever you prefer). Use it, fork it, productize it.
