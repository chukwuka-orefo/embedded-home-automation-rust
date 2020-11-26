# Embedded Presence Lighting Controller (Rust re-implementation)

A deterministic, presence-based lighting controller for embedded Linux systems,
re-implemented in Rust as a correctness-focused port of the original Python system.

This project preserves the original behaviour, state model, and timing semantics,
while expressing them in a stricter, more explicit systems language.

Original Python implementation:
https://github.com/chukwuka-orefo/embedded-presence-lighting

---

## Context and intent

The original controller was implemented in Python for Raspberry Pi class devices
and deployed as a long-running system service.

This Rust version has the following goals:

- Make all state transitions explicit and compiler-checked
- Enforce deterministic behaviour through a strongly typed FSM
- Separate policy, timing, and hardware concerns more rigidly
- Provide a foundation suitable for lower-level GPIO and embedded integration

All observable behaviour is intended to match the Python version exactly.

---

## System overview

The controller is built around a small number of well-defined responsibilities:

- Deterministic finite state machine controlling lighting behaviour
- Explicit separation between core logic and hardware interfaces
- Controller layer translating sensors → FSM → actuators
- Stub device implementations for development and testing
- GPIO implementations intended for Raspberry Pi deployment
- Long-running runtime loop suitable for systemd execution

All behaviour is deterministic and test-driven.

---

## Behaviour summary

- Motion detection triggers evaluation of ambient light
- Lighting activates only when ambient light is below a defined threshold
- Lighting remains active for a fixed timeout after the last motion event
- A cooldown period prevents rapid retriggering
- All behaviour is enforced via explicit state transitions

No implicit timing or background behaviour exists.

---

## Project structure

```
src/
├── controller/
│   ├── state_machine.rs      # deterministic FSM logic
│   └── controller.rs         # orchestration layer
├── interfaces/
│   ├── motion.rs             # motion sensor interface
│   ├── light_sensor.rs       # ambient light interface
│   ├── light_actuator.rs     # output actuator interface
│   └── stub_devices.rs       # development and test implementations
├── main.rs                   # runtime loop (stub mode)
└── lib.rs                    # library crate for testing and reuse

tests/
├── state_machine.rs          # FSM integration tests
└── controller.rs             # controller behaviour tests
```

The project builds both as a binary and as a library crate.

---

## Development and execution

By default, the runtime uses **stub devices** and can be executed on any machine:

```
cargo run
```

All state transitions and controller actions are logged to stdout.
Unit and integration tests enforce all FSM and controller behaviour:

```
cargo test
```

---

## Hardware deployment (future work)

GPIO-backed implementations are intentionally not included in the beta core.
They are expected to live under:

```
src/interfaces/gpio_*.rs
```

and will implement the same trait interfaces used by the stub devices.

The controller and FSM do not require modification for hardware deployment.

---
- FSM behaviour is complete and tested
- Controller wiring is stable
- Stub devices and runtime loop are functional
- Logging and observability are in place

