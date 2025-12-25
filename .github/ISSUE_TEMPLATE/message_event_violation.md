---
title: "[LINT] MessageWriter misuse: [function] in [file]"
labels: ["lint", "ecs-architecture", "bevy-0.17", "message-event-separation"]
---

## Message/Event Separation Violation

**File:** [file]
**Function:** [function]

The Message/Event static analysis linter detected likely misuse of `MessageWriter<T>` in this function, where an immediate side-effect (e.g., `commands.spawn`, `commands.entity`, `audio.play`, etc.) occurs in the same function. This violates the Bevy 0.17 architecture mandate for Message-Event Separation:

> Use `MessageWriter<T>` strictly for double-buffered, frame-agnostic data streams (e.g., telemetry, combat logs, physics collisions). Use observer systems and `Trigger<T>` for immediate, reactive logic (e.g., UI interactions, Sound effects, OnAdd/OnRemove lifecycle hooks).

### Guidance
- If the side-effect is immediate and reactive (e.g., UI, sound, spawning), use an `Event` (with `#[derive(Event)]`) and an observer system (`commands.observe()`).
- If the data is meant to be buffered and processed later (e.g., logs, telemetry), use `MessageWriter`/`MessageReader` but ensure no immediate side-effects are in the same function.
- Refactor by moving immediate side-effects into a separate system that observes the appropriate `Event` or `Trigger<T>`. **Do not create observer systems for Messages.**

---

**Automated finding. Please review and refactor as needed.**
