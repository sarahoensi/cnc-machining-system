# Error Policy for Form and Calculation Modules

## 1) Error Levels

### Field-level error
- Use when one specific field can be corrected by the user.
- Render inline at the field.
- On Calculate, focus the first field-level error in visual order.
- Examples:
  - `Material is required`
  - `Length is required`
  - `Inner diameter must be smaller than outer diameter`

### Form-level error
- Use when the error applies to a combination of fields or general form state.
- Render using `FormError`.
- May include a suggested focus field (future support), otherwise focus first relevant empty input.
- Examples:
  - `Provide either pitch or angle`
  - `Provide either cuts or radial engagement`

### System/backend error
- Use when failure is outside direct field correction.
- Render as `FormError` or module-specific error surface.
- Example:
  - Material database could not be loaded

## 2) Validation Responsibilities

### Input primitives
- Restrict syntax only (number/text/select interaction).
- Must not perform domain math or business rules.

### Frontend/domain validators
- Validate required fields.
- Validate simple field-specific rules.
- Validate simple cross-field rules.
- Return field-level errors when a specific field is correctable.
- Return form-level errors for combination/global rules.

### Backend/domain
- Own robust domain validation:
  - range
  - positivity
  - geometry
  - mode-dependent rules
- Return field errors when tied to a concrete field.
- Return general error when not attributable to one field.

## 3) Focus Policy

On Calculate:
1. Focus first field-level error in visual order.
2. If a form-level error has suggested focus field, focus it.
3. Otherwise focus first relevant empty input field.

Never focus disabled/readOnly/displayOnly/hidden/output fields.

## 4) Clearing Policy

When a field changes:
- Clear that field's field-level error.
- Clear relevant form-level errors when the changed field affects that rule.

On Clear form:
- Clear all calculation errors.
- Saved results may be preserved based on feature UX.
- Persistent material state may be preserved based on feature UX.

## 5) App Examples

- Triangle: "at least two inputs" is form-level.
- Helix: "pitch or angle" is form-level.
- Cylinder Weight: "material required" is field-level.
- Cylinder Weight: "inner diameter must be smaller than outer diameter" is field-level.
- Finishing: "cuts or radial engagement" is form-level.
- Backend math/domain errors should map to field-level when possible.

## 6) Current Shared Support (status)

- Shared engine supports backend field errors through `fieldErrors -> fields[key].error`.
- `formError` supports string/string[] for form-level/general errors.
- `FormSelectMenuField` supports inline field error rendering.
- Form navigation can focus select trigger elements (not only numeric inputs).
- Suggested focus field for form-level errors is not yet implemented.
