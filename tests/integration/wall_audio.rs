// Integration test harness for wall collision audio
// This file will contain all tests for the Audio Wall Delay Fix feature.

use bevy::prelude::*;
use brkrs::signals::BallWallHit;

#[derive(Resource, Default)]
struct WallHitTestFlag(pub bool);

fn test_wall_hit_observer(trigger: On<BallWallHit>, mut flag: ResMut<WallHitTestFlag>) {
    let _event = trigger.event();
    flag.0 = true;
}

#[test]
fn emits_ball_wall_hit_and_observes() {
    let mut app = App::new();
    app.add_event::<BallWallHit>();
    app.init_resource::<WallHitTestFlag>();
    app.add_observer(test_wall_hit_observer);

    // Spawn dummy entities for ball and wall
    let ball = app.world.spawn_empty().id();
    let wall = app.world.spawn_empty().id();

    // Trigger the event
    app.world.send_event(BallWallHit {
        ball_entity: ball,
        wall_entity: wall,
    });

    // Run a frame to process the observer
    app.update();

    // Assert the observer ran
    let flag = app.world.resource::<WallHitTestFlag>();
    assert!(flag.0, "BallWallHit observer was not called");
}
