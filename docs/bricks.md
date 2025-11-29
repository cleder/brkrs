# Brick Documentation

This document describes all brick types, their indices, and the actions triggered when hit.

## Overview

The map file contains a 20x20 grid of bricks, each identified by a unique index.
They are grouped into the following ranges:

- **Brick Index Range**: 10-57, 90-97
- **Destructible Bricks**: 10-57 (must be cleared to complete a level)
- **Solid/Indestructible Bricks**: 90-97 (cannot be destroyed, don't count toward level completion)

## Brick Types

The following tables detail the various brick types.
The `Index` column refers to the brick's identifier in the map file.

### Simple Bricks (Index 20)

| Index | Image | Name | Score | Description |
|-------|-------|------|-------|-------------|
| 20 | ![Stone](Stone.gif) | Simple Stone | 25 | Basic brick, destroyed on first hit |

### Multi-Hit Bricks (Index 10-13)

| Index | Image | Name | Score | Description |
|-------|-------|------|-------|-------------|
| 10 | ![Hit 1](Stonehit1.gif) | Hit 1 | 50 | Needs 1 more hit to become simple stone |
| 11 | ![Hit 2](Stonehit2.gif) | Hit 2 | 50 | Needs 2 more hits |
| 12 | ![Hit 3](Stonehit3.gif) | Hit 3 | 50 | Needs 3 more hits |
| 13 | ![Hit 4](Stonehit4.gif) | Hit 4 | 50 | Needs 4 more hits |

### Gravity Bricks (Index 21-25)

| Index | Image | Name | Score | Description |
|-------|-------|------|-------|-------------|
| 21 | ![Zero G](Stone00g.gif) | Zero Gravity | 125 | Turns off gravity |
| 22 | ![5 G](Stone05g.gif) | 5G | 75 | Light gravity (like Mars) |
| 23 | ![10 G](Stone10g.gif) | 10G | 125 | Normal gravity (Earth) |
| 24 | ![20 G](Stone20g.gif) | 20G | 150 | High gravity |
| 25 | ![Queer G](Stone22g.gif) | Queer Gravity | 250 | Random changing gravity in intensity and direction |

### Score Multiplier Bricks (Index 26-29)

| Index | Image | Name | Score | Description |
|-------|-------|------|-------|-------------|
| 26 | ![1X](Stone1x.gif) | Times 1 | 25 | Reset to single score |
| 27 | ![2X](Stone2x.gif) | Times 2 | 25 | Double all points |
| 28 | ![3X](Stone3x.gif) | Times 3 | 25 | Triple all points |
| 29 | ![4X](Stone4x.gif) | Times 4 | 25 | Quadruple all points |

### Paddle Effect Bricks (Index 30-32)

| Index | Image | Name | Score | Description |
|-------|-------|------|-------|-------------|
| 30 | ![Apple](Stoneapple.gif) | Apple | 300 | Shrinks paddle (temporary) |
| 31 | ![Sun](Stonesun.gif) | Sun | 200 | Spawns a deadly skull enemy |
| 32 | ![Yin Yang](Stoneyinyan.gif) | Yin Yang | 225 | Enlarges paddle |

### Ball Size Bricks (Index 33-35)

| Index | Image | Name | Score | Description |
|-------|-------|------|-------|-------------|
| 33 | ![Small Ball](Stonesmallball.gif) | Small Ball | 25 | Changes ball to small size |
| 34 | ![Medium Ball](Stonemediumball.gif) | Medium Ball | 25 | Changes ball to medium size |
| 35 | ![Big Ball](Stonebigball.gif) | Big Ball | 25 | Changes ball to large size |

### Enemy Spawn Bricks (Index 36)

| Index | Image | Name | Score | Description |
|-------|-------|------|-------|-------------|
| 36 | ![Donut](Stonedonut.gif) | Donut/Rotor | 75 | Spawns a bouncing rotor enemy |

### Ball Count Bricks (Index 37-39)

| Index | Image | Name | Score | Description |
|-------|-------|------|-------|-------------|
| 37 | ![Red 1](Stonered1.gif) | Red 1 | 100 | Reduces to 1 ball in play |
| 38 | ![Red 2](Stonered2.gif) | Red 2 | 100 | Changes to 2 balls in play |
| 39 | ![Red 3](Stonered3.gif) | Red 3 | 100 | Changes to 3 balls in play |

### Hazard Bricks (Index 40-42)

| Index | Image | Name | Score | Description |
|-------|-------|------|-------|-------------|
| 40 | ![Bomb](Stonebomb.gif) | Bomb | 100 | Explodes, spawns deadly fragments |
| 41 | ![Extra](Stoneextra.gif) | Extra Ball | +1 Ball | Gives an extra ball (life) |
| 42 | ![Killer](Stonekill.gif) | Killer | 90 | **Deadly** - Touching with paddle kills you |

### Direction Bricks (Index 43-48)

| Index | Image | Name | Score | Description |
|-------|-------|------|-------|-------------|
| 43 | ![Down](Stonedown.gif) | Down | 125 | Accelerates ball downward |
| 44 | ![Left](Stonelleft.gif) | Left | 125 | Accelerates ball leftward |
| 45 | ![Right](Stoneright.gif) | Right | 125 | Accelerates ball rightward |
| 46 | ![Up](Stoneup.gif) | Up | 125 | Accelerates ball upward |
| 47 | | Up-Right | 125 | Accelerates ball up and right |
| 48 | | Up-Left | 125 | Accelerates ball up and left |

### Special Bricks (Index 49-56)

| Index | Image | Name | Score | Description |
|-------|-------|------|-------|-------------|
| 49 | ![Teleport](Stoneteleport.gif) | Teleport | 150 | Teleports ball to another random teleport brick |
| 50 | ![Level Up](Stonelevelup.gif) | Smiley/Level Up | 300 | Advances to next level |
| 51 | ![Slow](Stoneslow.gif) | Hourglass/Slow | 30 | Slows down ball and mouse |
| 52 | ![Phone](Stonephone.gif) | Phone | 40 | Randomizes ball velocity |
| 53 | ![Question](Stonequestion.gif) | Question | Random | Transforms into a random brick type |
| 54 | ![Level Down](Stoneleveldown.gif) | Level Down | - | Returns to previous level |
| 55 | ![Magnet](Stonemagnet.gif) | Magnet (Enabled) | - | Active magnet - attracts ball |
| 56 | ![Magnet Disabled](Stonemagnetdis.gif) | Magnet (Disabled) | - | Inactive magnet - becomes active when enabled one is destroyed |

### Paddle-Destroyable Brick (Index 57)

| Index | Image | Name | Score | Description |
|-------|-------|------|-------|-------------|
| 57 | ![Bat](Stonebat.gif) | Bat | 250 (paddle) | Cannot be destroyed by ball, only by paddle |

### Solid/Indestructible Bricks (Index 90-97)

These bricks cannot be destroyed and don't count toward level completion.

| Index | Image | Name | Description |
|-------|-------|------|-------------|
| 90 | ![Solid](Stonesolid.gif) | Solid | Indestructible, ball bounces off |
| 91 | | Solid Die | **Deadly** - Kills paddle on contact |
| 92 | | Solid Down | Solid + accelerates ball downward on bottom hit |
| 93 | | Solid Left | Solid + accelerates ball leftward |
| 94 | | Solid Right | Solid + accelerates ball rightward |
| 95 | | Solid Up | Solid + accelerates ball upward on top hit |
| 96 | | Solid Up-Left | Solid + accelerates ball up and left |
| 97 | | Solid Up-Right | Solid + accelerates ball up and right |

### Magnet System

- Only one magnet can be active at a time
- When an enabled magnet (51) is destroyed, a random disabled magnet (52) becomes enabled
- The magnet creates a force field that attracts the ball

### LIFE Levels

Some levels use Conway's Game of Life algorithm (2-3-3 variant):

- Bricks in rows/columns 0 and 19 don't die at generation change
- Generation stones in columns 3, 4, 5 of row 0 determine respawning brick types
- Ball/paddle starting position determines if LIFE mode is active

## Additional Images

| Image | Description |
|-------|-------------|
| ![Invisible](stoneinv.gif) | Invisible brick (proposed) |
| ![Rubber](stoneRubber.gif) | Rubber brick (proposed) |

## Sound Effects

Each brick type triggers a specific sound when hit
