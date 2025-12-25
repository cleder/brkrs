# Physics Config Edge Case Tests

This file contains unit tests for edge cases in BallPhysicsConfig, PaddlePhysicsConfig, and BrickPhysicsConfig.
It verifies validation logic for zero, maximum, and out-of-bounds values.

- Zero values (all fields set to 0.0) must pass validation.
- Maximum recommended values (restitution/friction: 2.0, damping: 10.0) must pass validation.
- Out-of-bounds values (negative, excessive) must fail validation.

See [src/physics_config.rs](../src/physics_config.rs) for config structs and validation logic.
