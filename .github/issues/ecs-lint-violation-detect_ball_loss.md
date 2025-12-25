---
title: "[LINT] MessageWriter misuse: detect_ball_loss in src/systems/respawn.rs"
labels: ["lint", "ecs-architecture", "bevy-0.17", "message-event-separation"]
---

## Message/Event Separation Violation

**File:** src/systems/respawn.rs
**Function:** detect_ball_loss

The Message/Event static analysis linter detected likely misuse of `MessageWriter<T>` in this function, where an immediate side-effect (e.g., `commands.spawn`, `commands.entity`, `audio.play`, etc.) occurs in the same function. This violates the Bevy 0.17 architecture mandate for Message-Event Separation:

> Use `MessageWriter<T>` strictly for double-buffered, frame-agnostic data streams (e.g., telemetry, combat logs, physics collisions). Use observer systems and `Trigger<T>` for immediate, reactive logic (e.g., UI interactions, Sound effects, OnAdd/OnRemove lifecycle hooks).

### Guidance
- Move immediate side-effects into a separate observer system that reacts to the message/event.
- Or, if the logic is truly immediate, use an observer/trigger pattern instead of a buffered message.

---

**Automated finding. Please review and refactor as needed.**
