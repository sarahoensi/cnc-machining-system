# Canonical Units – Machining System

All domain logic operates on canonical units.
Unit conversion is only allowed at system boundaries (DTO mapping).

## Cutting Data

| Concept              | Canonical Unit         | Notes |
|----------------------|------------------------|-------|
| Tool diameter (D)    | millimeters            | > 0 |
| Cutting speed (Vc)   | meters/min             | > 0 |
| Spindle speed (n)    | revolutions/min (RPM)  | > 0 |
| Feed rate (F)        | millimeters/min        | > 0 |
| Feed per tooth (fz)  | millimeters/tooth      | > 0 |
| Number of teeth (z)  | integer                | > 0 |

## Rules
- Domain code never receives strings.
- Domain code never formats numbers.
- Domain code never converts between units except via explicit constructors.
