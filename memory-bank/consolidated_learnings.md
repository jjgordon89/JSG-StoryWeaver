# Consolidated Learnings: StoryWeaver

## Key Insights and Patterns

### Action Plan Audit Pattern (2025-01-14)
**Discovery:** Action plans can become outdated as development progresses. Two items marked as incomplete in CODEBASE_ACTION_PLAN.md were actually fully implemented:
- AI card filter implementations: Complete in `AIResponseCard::get_filtered` with comprehensive filtering
- AIResponseCache time-based clearing: Complete with TTL, background sweeper, and manual cleanup

**Learning:** Always verify current implementation status before starting work on action plan items. Code analysis should precede implementation to avoid duplicate work.

**Pattern:** When auditing action plans:
1. Search for existing implementations in the codebase
2. Examine test coverage to understand feature completeness
3. Update action plan documentation with current status
4. Focus efforts on genuinely incomplete items

---

# Rust Error Handling and API Patterns (2025-08-11)

### Standard Patterns for StoryWeaver Rust/Tauri Backend

---

### 1. Result Aliasing

- **Always use the project alias `Result<T>`** (defined as `std::result::Result<T, StoryWeaverError>`) instead of the standard `Result<T, E>` style when StoryWeaver errors are expected.
- Never write `Result<T, StoryWeaverError>` — use `Result<T>`.

---

### 2. Error Factory Functions

- **All frequently-used error variants should have a factory/helper function**, e.g. `not_found`, to create error objects with proper fields.  
- Internally, use:  

  ```rust
  Err(StoryWeaverError::not_found("ResourceType", id))
  ```

  **Do not** create error variants directly with struct literal syntax unless rare/specialized.

---

### 3. Tauri Command Return Convention

- **Tauri command async handlers must always return `CommandResponse<T>`** (project pattern).
- Convert from internal `Result<T>` using `.into()`.
- Never return plain values or raw `Result<T, _>` types from Tauri-exposed commands.

---

### 4. Model and Type Conversions

- For all struct initializations and DB field mapping, **be explicit with Option handling**  
  - Use `.map()`, `.unwrap_or()`, or direct assignment.
  - Types must always match between DB/query return and model field/struct signature.

---

### 5. Use of `?` and `From` Traits

- **Implement `From` trait for all third-party error types that flow through the system** (e.g., `sqlx::Error`, `anyhow::Error`, `std::io::Error`, etc.) to support clean propagation with `?` operator.
- Avoid direct `map_err()` calls for common conversions unless context string is essential.

---

### 6. Struct Consistency

- All data models and command argument/result types must align field-for-field with API and DB schema.
- Remove obsolete fields and add new required ones as project grows.

---

### 7. Command Handler Logic

- Separate inner business logic as private async functions returning `Result<T>`.
- Command-exposed functions should stay lean, only responsible for conversion and response wrapping.

---

### 8. Test Coverage

- Add and maintain unit tests for error propagation and conversion.
- Ensure integration tests for key DB operation commands.

---

_Always refactor legacy code to these standard patterns. Use these conventions for all future code reviews and remediation cycles._
