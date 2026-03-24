# CNC Machining Calculator (Rust + Tauri + React)

A domain-driven CNC calculation tool used in real production by a machine operator.

Designed to **prevent invalid machining input**, not just calculate values.

Includes a multi-layered test suite covering domain, application, and interface levels.

---

## Overview

This application supports CNC operators in daily work by solving:

- Cutting data (RPM, feed, chip load, cutting speed)
- Helix interpolation (pitch ↔ angle)
- Right triangle geometry
- Finishing workflows with step-by-step measurement feedback

The system is actively used in production, where incorrect results have **direct economic consequences**.

---

## Key Design Goal

> Make incorrect usage difficult — not just incorrect calculations impossible.

The system enforces:

- Physical constraints (units, geometry, machining rules)
- Process constraints (step order, forward progression)
- Input validation at multiple layers

---

## Architecture

The project follows a strict layered design:

Frontend (Tauri commands)  
        ↓  
Application layer (use cases)  
        ↓  
Domain layer (rules, invariants, physics)  
        ↓  
Units (type-safe primitives)  

### Principles

- **Domain owns all rules**
- **Use cases orchestrate, not compute**
- **UI layer is a thin adapter**
- **Invalid states are unrepresentable where possible**

---

## Core Features

### 1. Cutting Data Solver

- Accepts partial input (e.g. RPM + tool diameter)
- Iteratively resolves missing values
- Uses constraint propagation between:
  - Cutting speed ↔ RPM ↔ diameter
  - Feed rate ↔ chip load ↔ RPM ↔ teeth

---

### 2. Finishing Workflow Engine

A stateful execution model for machining finishing passes.

- Generates machining plan (by cuts or radial engagement)
- Tracks operator measurements per step
- Locks completed steps
- Recalculates remaining steps dynamically  
➡️ Models *real operator workflow*, not just math

---

### 3. Helix Solver

- Converts between pitch and helix angle
- Accounts for tool diameter offset (inner/outer paths)
- Enforces geometric constraints

---

### 4. Right Triangle Solver

- Supports multiple input combinations:
  - sides
  - side + angle
  - hypotenuse + angle
- Ensures numerically stable calculations

---

## Domain Modeling

### Strongly typed units

Examples:

- `Diameter`
- `Pitch`
- `ChipLoad`
- `CuttingSpeed`
- `FeedRate`
- `Rpm`

All units:

- enforce positivity and finiteness
- prevent invalid values at construction time

---

### Example invariant

```rust
pub fn rpm_from_cutting_speed(
    cutting_speed: CuttingSpeed,
    diameter: Diameter,
) -> Result<Rpm, CuttingError>```

## Error Handling

Layered error system:

- **Domain errors** → physical or logical violations  
- **Application errors** → validation aggregation  
- **Tauri errors** → UI-safe responses  

Includes:

- Field-level validation errors  
- Domain-specific error messages  

---

## Validation Strategy

Centralized through an `InputParser`:

- Collects multiple validation errors  
- Separates parsing from domain logic  
- Supports partial input workflows  

---

## Testing

The project includes a comprehensive, multi-layered test suite covering:

### Domain Layer
- Mathematical correctness (geometry, machining physics)
- Invariants and edge cases
- Numerical stability

### Application Layer
- Partial input solving (constraint propagation)
- Cross-path consistency (same result from different inputs)
- Validation and error handling
- Stateful workflows (finishing execution lifecycle)

### Interface Layer (Tauri)
- Request/response mapping
- Serialization and deserialization
- End-to-end command validation
- Happy path and error scenarios

### Test Types

- Unit tests for deterministic logic  
- Property-based tests (proptest) for invariants and identities  
- Scenario tests simulating real operator workflows  

---

### Example coverage

- Partial inputs (e.g. RPM only, feed only)
- Idempotency (same input → same output)
- Cross-path validation (different inputs → consistent results)
- Workflow constraints (step locking, forward progression)
---

##  Why This Project Is Different

This is not just a calculator.

It is:

- A **domain-driven engineering tool**  
- A system that **models machining reality**  
- A tool designed to **reduce operator error in production**  

---

## Tech Stack

- Rust  
- Tauri  
- Serde  
- ThisError  
- Proptest  

---

## Status

- Actively used by a CNC operator  
- Developed in close collaboration with end user  
- Iteratively refined based on real-world usage  

---

## Future Directions

- Improved explainability of calculations  
- Enhanced edge-case handling (real-world tolerances)  
- Expanded machining models  

---

## Author

Developed independently with focus on:

- Domain modeling  
- Correctness  
- Real-world usability  