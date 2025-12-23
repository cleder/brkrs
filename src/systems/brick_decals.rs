//! Brick decal assignment system for brick-type-decals feature.
//!
//! This system assigns decals to bricks based on their BrickType during level loading.
//! Decals are visual hints centered on the top side of bricks, supporting normal/bump mapping.

use crate::level_format::brick_types::{BrickType, Decal};
use crate::BrickTypeId;
use bevy::prelude::*;

/// System that assigns decals to bricks based on their type.
/// Runs during level loading to ensure all bricks have appropriate decals.
pub fn assign_brick_decals(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &BrickType), Without<Decal>>,
) {
    for (entity, brick_type) in query.iter() {
        let decal = create_decal_for_type(brick_type, &asset_server);
        commands.entity(entity).insert(decal);
    }
}

/// System that assigns decals to bricks based on their BrickTypeId.
/// This is a fallback system for bricks that don't have a BrickType component.
/// It converts the ID to a type and assigns a decal, with fallback for unknown types.
pub fn assign_brick_decals_fallback(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &BrickTypeId), (Without<Decal>, Without<BrickType>)>,
) {
    for (entity, brick_type_id) in query.iter() {
        let decal = if let Some(brick_type) = BrickType::from_id(brick_type_id.0) {
            create_decal_for_type(&brick_type, &asset_server)
        } else {
            create_fallback_decal(brick_type_id.0, &asset_server)
        };
        commands.entity(entity).insert(decal);
    }
}

/// Creates a Decal component for the given BrickType.
/// In a full implementation, this would load actual assets.
/// For now, returns a default Decal with the correct type.
fn create_decal_for_type(brick_type: &BrickType, asset_server: &AssetServer) -> Decal {
    let normal_map_path = match brick_type {
        BrickType::Standard => "textures/decals/standard_normal.png",
        BrickType::Indestructible => "textures/decals/indestructible_normal.png",
        BrickType::MultiHit => "textures/decals/multihit_normal.png",
    };

    Decal {
        brick_type: *brick_type,
        normal_map_handle: Some(asset_server.load(normal_map_path)),
    }
}

/// Creates a fallback decal for unknown brick types.
/// Logs a warning and returns a default decal.
fn create_fallback_decal(brick_type_id: u8, asset_server: &AssetServer) -> Decal {
    warn!(
        "Unknown brick type ID: {}, using fallback decal",
        brick_type_id
    );
    Decal {
        brick_type: BrickType::Standard, // Fallback to standard
        normal_map_handle: Some(asset_server.load("textures/decals/standard_normal.png")),
    }
}
