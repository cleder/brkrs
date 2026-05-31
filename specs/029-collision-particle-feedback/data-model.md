# Data Model: Collision Particle Feedback

## Entities

### 1. CollisionFeedbackTrigger

- Purpose: Immutable event payload representing one qualifying collision that should produce visual feedback.
- Fields:
  - `ball_entity: Entity`
  - `target_entity: Entity`
  - `target_kind: enum { Wall, Paddle, Brick }`
  - `contact_point: Vec3` (exact world-space collision point)
  - `collision_frame: u64` (diagnostic/testing aid)
- Validation rules:
  - `contact_point` must be finite (`is_finite` on all components).
  - Trigger is ignored when game state is paused or non-playing.

### 2. FeedbackEffectInstance

- Purpose: Runtime ECS state for one spawned sparkly burst.
- Fields:
  - `spawn_time: f32` (seconds)
  - `lifetime_seconds: f32` (must be in 0.20-0.35)
  - `particle_count: u8` (must be in 8-16)
  - `origin_contact_point: Vec3`
  - `source_kind: enum { Wall, Paddle, Brick }`
  - `source_brick_destroyed: bool` (true when collision destroyed the brick)
- Validation rules:
  - `lifetime_seconds` in inclusive range [0.20, 0.35].
  - `particle_count` in inclusive range [8, 16].
  - Instance must despawn automatically when elapsed >= lifetime.

### 3. FeedbackProfile

- Purpose: Tunable style/config resource shared by burst spawning systems.
- Fields:
  - `min_particles: u8` = 8
  - `max_particles: u8` = 16
  - `min_lifetime_seconds: f32` = 0.20
  - `max_lifetime_seconds: f32` = 0.35
  - `spark_style_id: &'static str` (logical style selector)
- Validation rules:
  - `min_particles <= max_particles`
  - `min_lifetime_seconds <= max_lifetime_seconds`

## Relationships

- One `CollisionFeedbackTrigger` creates exactly one `FeedbackEffectInstance` per qualifying collision.
- `FeedbackEffectInstance` instances are independent; burst collisions create multiple concurrent instances.
- `FeedbackProfile` is a shared resource referenced by spawn logic when materializing effect instances.

## State Transitions

### FeedbackEffectInstance lifecycle

1. `Triggered`: collision classified as eligible (wall/paddle/brick).
2. `Spawned`: effect entities/components created at exact `contact_point`.
3. `Active`: effect is visible and updated for elapsed time.
4. `Expired`: elapsed time reaches configured lifetime.
5. `Despawned`: entities removed; no residual state remains.

## Special Rules from Clarifications

- Burst collisions: no merge, no cap, no overflow queue; spawn once per qualifying collision.
- Pause/non-playing states: suppress new spawn transitions entirely; do not replay skipped effects later.
- Brick destruction on impact: trigger still transitions to `Spawned` before brick cleanup completes.
