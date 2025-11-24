# Brkrs

`Brkrs` is a classic Breakout/Arkanoid style game implemented in Rust with the Bevy game engine.
It's a feature-rich clone with advanced gameplay mechanics beyond the basic Breakout formula.
It features a paddle that can be controlled with the mouse, in all directions (left/right (x), up/down (y)).
If the player is moving the paddle to the right when the ball makes contact, the game calculates a greater horizontal velocity component in the rightward direction, sending the ball off at a sharper horizontal angle. Conversely, moving the paddle to the left imparts a leftward "english." The mouse wheel controls the rotation of the paddle.
It uses 3D rendering to display the bricks, the walls, and the ball.
The game will be implemented in 3D but constrained to a 2D plane above the ground.

The game area is divided into a 22x22 grid, the stones are placed into this grid and fill a grid cell.

## Architecture Overview

## Core Systems

1. **Physics Layer (Bevy Rapier3D)**
   - 3D physics constrained to Y=2.0 plane
   - Collision detection and response
   - Ball dynamics with restitution and friction
   - Paddle kinematic movement

2. **Game State Management**
   - Menu state
   - Playing state
   - Paused state
   - Game over state
   - Level transition state

3. **Level System**
   - Map/level loader
   - Brick spawning and management
   - Level progression (77 levels total)

4. **Brick System**
   - 37+ different brick types with unique behaviors
   - Component-based brick properties
   - Event-driven brick collision handlers

## Technical Considerations

### 3D to 2D Plane Constraint

- All gameplay occurs at Y=2.0
- Use `LockedAxes::TRANSLATION_LOCKED_Y` for rigid bodies
- Camera positioned above looking down
- Maintain 3D aesthetics with lighting and shadows

### Collision Detection

- Use Rapier's collision events (automatically handles ball reflection)
- Rapier's restitution provides the bounce physics
- For paddle-ball collision: Add steering impulse based on mouse movement (allows player to control ball direction)
- For brick-ball collision: Determine collision side from contact normal
- Some bricks modify velocity/apply additional impulses after Rapier's collision response
