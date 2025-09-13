# Data Model: API Scaffold

## Entities

### ErrorResponse
- code: String (e.g., "domain.error_code")
- message: String
- description: String

### ValidationErrorResponse
- code: "validation.error"
- message: String
- description: String
- errors: Map<String, Vec<String>>

### PaginatedList<T>
- data: Vec<T>
- meta: PaginationMeta (extensible)

### SingleResource<T>
- data: T
- meta: Meta (extensible)

### PaginationMeta
- page: u32
- per_page: u32
- total: u64
- [additional fields allowed]

### Meta
- [arbitrary key-value pairs for extensibility]

## Validation Rules
- All error and validation fields are required.
- Pagination meta must include page, per_page, total.
- Meta objects must be extensible for developer use.

## State Transitions
- N/A (scaffold only)
