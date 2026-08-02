# CNC Calculation Engine – Architecture Overview

## Purpose of This Document

This document describes the architectural design of the CNC Calculation Engine backend. It serves as a long-term reference to help maintain consistency, avoid architectural drift, and guide future development decisions.

The primary goal of this architecture is to:

- Maintain strong separation of concerns
- Support long-term scalability
- Enable high testability
- Model the engineering domain accurately
- Minimize refactoring when adding new features

---

# System Scope

This application is an **engineering calculation system**.

It is responsible for:

- Calculating machining parameters
- Solving geometric machining problems
- Generating machining pass strategies
- Providing deterministic calculation results

The application does NOT:

- Generate G-code
- Control CNC machines
- Perform scheduling or real-time machine execution

---

# Architectural Style

The system follows principles from:

- Domain Driven Design (DDD)
- Clean Architecture
- Hexagonal Architecture (Ports & Adapters)

The architecture separates the system into independent layers with strict dependency direction.

---

# High-Level Layer Structure

```
Interface Layer (Tauri / UI Adapter)
        ↓
Application Layer (Use Cases)
        ↓
Domain Layer (Core Engineering Logic)
```

Dependencies always flow downward.

Upper layers depend on lower layers.
Lower layers must NEVER depend on upper layers.

---

# Crate / Module Overview

The backend is divided into logical crates or modules:

```
cnc-domain
cnc-application
cnc-interface-tauri
```

---

# Domain Layer

## Responsibility

The domain layer contains all core engineering logic. This is the most important and most stable part of the system.

The domain layer:

- Models machining knowledge
- Enforces engineering rules
- Contains mathematical algorithms
- Is deterministic
- Has no external dependencies

The domain layer MUST NOT depend on:

- UI
- Tauri
- Serialization frameworks
- Databases
- File systems
- Async runtime

---

## Domain Structure

```
domain/
   units/
   geometry/
   machining/
   planning/
```

---

## Units Module

### Purpose

Provides strongly typed engineering measurement types.

### Examples

- Length
- Diameter
- Angle
- RPM
- FeedRate
- CuttingSpeed

### Why Units Exist

- Prevents mixing incompatible measurements
- Improves domain readability
- Enables compile-time safety
- Reduces calculation bugs

---

## Geometry Module

### Purpose

Provides pure mathematical modeling.

### Responsibilities

- Triangle solving
- Circle geometry
- Helix mathematics
- Vector calculations

### Design Rules

- Contains NO machining knowledge
- Fully reusable mathematical engine
- Only depends on units

---

## Machining Module

### Purpose

Models machining physics and engineering formulas.

### Responsibilities

- Chip load calculations
- Cutting speed calculations
- Feed rate calculations
- Spindle speed formulas
- Tool engagement calculations
- Helix calculations

### Design Rules

- Stateless calculation logic
- Uses units and geometry
- Represents physical machining relationships

---

## Planning Module

### Purpose

Contains algorithms for machining strategy calculations.

### Responsibilities

- Finishing pass generation
- Step distribution strategies

### Typical Aggregates

Example:

- FinishingPlan
- Pass
- StepStrategy

---

## Domain Design Patterns

### Value Objects

Represent measurement or conceptual values.

Examples:

- Diameter
- FeedRate
- Angle

Value objects must:

- Be immutable
- Validate invariants during construction

---

### Aggregates

Represent domain structures that maintain internal consistency.

Examples:

- FinishingPlan
- MachiningCondition

Aggregates enforce domain rules and invariants.

---

### Domain Services

Contain stateless algorithms operating on domain objects.

Examples:

- ChipLoadCalculator
- HelixCalculator
- PassDistributionStrategy

---

# Application Layer

## Responsibility

The application layer coordinates workflows and use cases.

It:

- Receives validated domain input
- Calls domain logic
- Returns calculation results

It does NOT:

- Contain engineering formulas
- Know about UI frameworks
- Handle serialization or transport protocols

---

## Application Structure

```
application/
   calculate_cutting_data/
   generate_finishing_plan/
   calculate_helix/
   triangle_solver/
```

Each module represents a user use case.

---

## Use Case Pattern

Each use case typically follows:

```
Input Model → Domain Calls → Output Model
```

Application models should use domain types rather than primitive types.

---

# Interface Layer (Tauri Adapter)

## Responsibility

Acts as the translation boundary between the frontend and the application layer.

This layer:

- Receives UI input
- Converts DTOs to domain types
- Calls application use cases
- Converts domain results back to UI DTOs

---

## Interface Structure

```
commands/
dto/
mappers/
```

---

## DTOs (Data Transfer Objects)

DTOs represent UI-friendly data formats.

DTOs:

- Contain primitive types
- Represent serialized transport data
- Are allowed to use serialization frameworks

DTOs MUST NOT leak into domain or application layers.

---

## Mappers

Responsible for converting:

```
DTO ↔ Domain Models
```

This creates an anti-corruption layer between UI and domain.

---

# Validation Strategy

Validation occurs at multiple levels:

---

## UI Validation

Purpose:

- Improve user experience
- Prevent incomplete forms
- Handle formatting errors

---

## Domain Validation

Purpose:

- Enforce engineering correctness
- Guarantee invariant safety

Examples:

- Diameter must be positive
- Target dimension must exceed start dimension
- Feed rate cannot be zero

Domain validation MUST exist even if UI validates.

---

# Dependency Rules

## Allowed Dependencies

```
Interface → Application → Domain
Application → Domain
Planning → Machining → Geometry → Units
```

---

## Forbidden Dependencies

```
Domain → Application
Domain → Interface
Geometry → Machining
Units → Anything
```

---

# Testing Strategy

## Domain Tests

Highest priority.

Test:

- Mathematical correctness
- Edge cases
- Engineering invariants
- Numerical stability

Domain tests must NOT require runtime frameworks.

---

## Application Tests

Test:

- Workflow correctness
- Use case orchestration

---

## Interface Tests

Minimal testing. Focus primarily on mapping correctness.

---

# Design Principles

## Deterministic Domain

All domain calculations must produce identical outputs for identical inputs.

No randomness.
No external side effects.

---

## Domain Language Consistency

The domain model should reflect real engineering terminology.

Avoid generic names like:

- Data
- Helper
- Utils
- Service (without clear domain meaning)

---

## Feature Independence

UI features are considered workflows using domain logic.

Domain models must remain stable regardless of UI changes.

---

# Future Extension Guidelines

When adding new features:

1. Determine if logic belongs in domain or application
2. Expand domain if new engineering knowledge is introduced
3. Add new use cases in application layer
4. Extend DTOs and mappers only in interface layer

---

# Architectural Goals

This architecture is designed to:

- Reduce long-term refactoring cost
- Improve testability
- Protect core engineering logic
- Allow UI flexibility
- Encourage explicit domain modeling
- Provide compile-time correctness through strong typing

---

# Mental Model Summary

Think of the system as:

- Domain = Engineering brain
- Application = Workflow coordinator
- Interface = Translator between UI and domain

---

# Final Rule

If domain logic cannot be reused in a CLI tool, test harness, or alternative UI without modification, the architecture boundary has likely been violated.
