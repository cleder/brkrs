use bevy::prelude::*;

/// Marker component for the gravity indicator UI element.
#[derive(Component)]
pub struct GravityIndicator;

/// Cached gravity indicator textures.
#[derive(Resource, Reflect)]
pub struct GravityIndicatorTextures {
    pub question: Handle<Image>,
    pub weight0: Handle<Image>,
    pub weight2: Handle<Image>,
    pub weight10: Handle<Image>,
    pub weight20: Handle<Image>,
}

/// Enumerated gravity levels for mapping tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GravityLevel {
    L0,
    L2,
    L10,
    L20,
    Unknown,
}

impl GravityLevel {
    /// Get the asset file name for this gravity level.
    pub fn asset_name(self) -> &'static str {
        match self {
            GravityLevel::L0 => "weight-0",
            GravityLevel::L2 => "weight-2",
            GravityLevel::L10 => "weight-10",
            GravityLevel::L20 => "weight-20",
            GravityLevel::Unknown => "weight-question",
        }
    }
}

/// Round a component and check tolerance against its nearest integer.
fn within_tol_round(val: f32, tol: f32) -> Option<i32> {
    if !val.is_finite() {
        return None;
    }
    let r = val.round();
    if (val - r).abs() <= tol {
        Some(r as i32)
    } else {
        None
    }
}

/// Map current gravity vector to a discrete level using ±0.5 tolerance on X and Z axes only.
/// Per spec: Y axis is always 0 and ignored.
pub fn map_gravity_to_level(g: Vec3) -> GravityLevel {
    let tol = 0.5;
    let mut best = 0;

    for c in [g.x, g.z] {
        if let Some(r) = within_tol_round(c, tol) {
            let v = r.abs();
            if v > best {
                best = v;
            }
        }
    }

    match best {
        20 => GravityLevel::L20,
        10 => GravityLevel::L10,
        2 => GravityLevel::L2,
        0 => GravityLevel::L0,
        _ => GravityLevel::Unknown,
    }
}

/// Select the image handle based on the mapped gravity level.
pub fn select_texture(level: GravityLevel, textures: &GravityIndicatorTextures) -> &Handle<Image> {
    match level {
        GravityLevel::L20 => &textures.weight20,
        GravityLevel::L10 => &textures.weight10,
        GravityLevel::L2 => &textures.weight2,
        GravityLevel::L0 => &textures.weight0,
        GravityLevel::Unknown => &textures.question,
    }
}

/// Spawn the gravity indicator anchored at the lower-left corner if it doesn't exist.
pub fn spawn_gravity_indicator(
    mut commands: Commands,
    textures: Option<Res<GravityIndicatorTextures>>,
    existing: Query<Entity, With<GravityIndicator>>,
    gravity_cfg: Option<Res<crate::GravityConfiguration>>,
) {
    if !existing.is_empty() {
        return;
    }
    let Some(textures) = textures else {
        warn!("GravityIndicatorTextures resource missing; skipping indicator spawn");
        return;
    };

    let level = map_gravity_to_level(gravity_cfg.map(|r| r.current).unwrap_or(Vec3::ZERO));
    let handle = select_texture(level, &textures).clone();

    commands.spawn((
        bevy::prelude::ImageNode::new(handle),
        bevy::prelude::Node {
            position_type: bevy::prelude::PositionType::Absolute,
            left: bevy::prelude::Val::Px(12.0),
            bottom: bevy::prelude::Val::Px(12.0),
            ..Default::default()
        },
        GravityIndicator,
    ));
}

/// Update the gravity indicator image when gravity changes.
pub fn update_gravity_indicator(
    gravity_cfg: Option<Res<crate::GravityConfiguration>>,
    textures: Option<Res<GravityIndicatorTextures>>,
    mut query: Query<&mut bevy::prelude::ImageNode, With<GravityIndicator>>,
) {
    let (Some(gravity_cfg), Some(textures)) = (gravity_cfg, textures) else {
        return;
    };

    // Only update if resource changed to avoid per-frame churn
    if !gravity_cfg.is_changed() {
        return;
    }

    let level = map_gravity_to_level(gravity_cfg.current);
    let new_handle = select_texture(level, &textures).clone();

    for mut image_node in query.iter_mut() {
        // Replace the image; ImageNode is a simple wrapper that can be reassigned
        *image_node = bevy::prelude::ImageNode::new(new_handle.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tolerance_rounding_maps_correctly() {
        assert_eq!(
            map_gravity_to_level(Vec3::new(0.0, 0.0, 0.0)),
            GravityLevel::L0
        );
        assert_eq!(
            map_gravity_to_level(Vec3::new(2.0, 0.0, 0.0)),
            GravityLevel::L2
        );
        assert_eq!(
            map_gravity_to_level(Vec3::new(10.0, 0.0, 0.0)),
            GravityLevel::L10
        );
        assert_eq!(
            map_gravity_to_level(Vec3::new(20.0, 0.0, 0.0)),
            GravityLevel::L20
        );
    }

    #[test]
    fn mixed_axes_selects_highest_level() {
        // Y axis ignored; use X/Z combinations
        assert_eq!(
            map_gravity_to_level(Vec3::new(2.0, 0.0, 10.0)),
            GravityLevel::L10
        );
        assert_eq!(
            map_gravity_to_level(Vec3::new(0.0, 0.0, 20.0)),
            GravityLevel::L20
        );
        assert_eq!(
            map_gravity_to_level(Vec3::new(2.49, 0.0, 0.0)),
            GravityLevel::L2
        ); // within tol
        assert_eq!(
            map_gravity_to_level(Vec3::new(9.51, 0.0, 0.0)),
            GravityLevel::L10
        ); // within tol
    }

    #[test]
    fn unknown_when_outside_tolerance() {
        assert_eq!(
            map_gravity_to_level(Vec3::new(2.6, 0.0, 0.0)),
            GravityLevel::Unknown
        );
        assert_eq!(
            map_gravity_to_level(Vec3::new(9.4, 0.0, 0.0)),
            GravityLevel::Unknown
        );
    }
}
