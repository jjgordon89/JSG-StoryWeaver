# Rust Codebase: Comprehensive Fixes Required

_Last updated: 2025-08-11_

This document outlines the root causes and concrete fixes required to resolve the main problems in the Rust/Tauri StoryWeaver backend. The fixes are grouped by error category with file references, code patterns, and implementation notes.

---

## 1. Error Type System and `Result` Usage

### **Root Cause**
- The `Result<T>` alias in `src-tauri/src/error.rs` only takes one type argument, but is used incorrectly as `Result<T, StoryWeaverError>` in many places.
- Missing error factory functions (e.g. `not_found`).
- Misused variant constructors (e.g. `StoryWeaverError::InvalidInput()` used as a function).

### **Actions Required**
- Update all Rust codebase to use just `Result<T>` for StoryWeaver operations.
- Add missing constructors (helper functions) in `StoryWeaverError` for all commonly-needed patterns (`not_found`, etc) to avoid using struct variants directly as functions.
- Refactor usages such as `Err(StoryWeaverError::InvalidInput("reason"))` to match enum variants or use the new factory functions.

---

## 2. Command Handler Return Types

### **Root Cause**
- Many Tauri command functions return raw types or `Result<T, E>` instead of using the standard `CommandResponse<T>`.
- Some handlers mix return patterns or have mismatches between actual code flow and type signature.

### **Actions Required**
- Ensure all Tauri command async handlers (in `src-tauri/src/commands/`) return `CommandResponse<T>`, using `.into()` for conversion from `Result<T>`.
- Remove direct returning of plain data or plain `Result<T, _>`.
- Refactor inner logic for uniform error propagation and response formatting.

---

## 3. Type Conversion & Option Handling

### **Root Cause**
- Errors where e.g. `Option<T>` is used in place of `T`, `.map()`/`.as_ref()` issues, or incorrect type casts (`Option<i64> as i32`).
- Missing/incorrect conversions between types (e.g. `chrono::NaiveDateTime` <-> Option/fields).
- Using types that don't match the schema or model structs.

### **Actions Required**
- Audit all field initializations, DB query assignments, and struct initializers for matching types.
- Add appropriate Option handling, `.map()`, or `.unwrap_or()` as necessary.
- Ensure conversions between types are explicit and lossless.

---

## 4. Database Query and Model Issues

### **Root Cause**
- Mismatches between SQL select fields and Rust model fields.
- Inconsistent or missing pool arguments in DB operation functions.
- Incorrect query result castings.

### **Actions Required**
- Review all queries and model field mappings in `src-tauri/src/database/operations/`, especially any fetching Result that is type-casted (`as i32`, etc.).
- Add or correct function pool parameters so every DB operation receives and uses the connection pool.
- Update model struct fields or SQL queries for consistency.

---

## 5. Async/Await and Future Usage

### **Root Cause**
- Attempts to `.await` on non-async types.
- Some inner helper functions aren't marked as `async` when used with `.await`.

### **Actions Required**
- Audit function signatures and internal flow to ensure only Future-returning expressions are awaited.
- Add or remove `async`/`.await` as appropriate.

---

## 6. Struct Field Issues

### **Root Cause**
- Structs missing required fields or containing renamed/obsolete fields.
- Type mismatches in field initializers.

### **Actions Required**
- Compare all model structs and command argument/result types to DB schema and API expectations.
- Add missing fields, remove obsolete ones, and correct types throughout code.

---

## 7. Error Propagation and Conversion

### **Root Cause**
- Missing or incomplete `From` trait implementations for external error types needing conversion to `StoryWeaverError`.
- Inconsistent handling of errors from libraries like `sqlx`, `anyhow`, `std::io`, etc.

### **Actions Required**
- Implement or update `From` traits for all common error types.
- Use `?` operator liberally, relying on automatic conversion where possible.

---

## 8. Testing & Validation

### **Actions Required**
- Unit test key error propagation, conversion, and command handler success/error paths.
- Add integration tests as needed to check DB operation coverage after type/factory refactor.

---

# Implementation Order

1. **Error type/Result fixes** (Items 1, 7)
2. **Command handler standardization** (Item 2)
3. **Model/type mapping and conversion** (Items 3, 4, 6)
4. **Async/await fixes** (Item 5)
5. **Testing** (Item 8)

---

# Example: Error Factory Addition

```rust
impl StoryWeaverError {
    /// Factory for not found errors
    pub fn not_found<S: Into<String>>(resource_type: S, id: S) -> Self {
        Self::NotFound {
            resource_type: resource_type.into(),
            id: id.into()
        }
    }
}
```
Update usages:
```rust
// Before:
Err(StoryWeaverError::NotFound { resource_type: "Project".into(), id: id })
// After:
Err(StoryWeaverError::not_found("Project", id))
```

---

# Next Steps

- Begin with error type/Result/factory refactor for consistent base API.
- Continue with handler and type standardization.
- Then update models/operations.
- Validate by running full build/test cycle.

---

**This file should be referenced by all Rust contributors during remediation. Update with new discoveries/decisions as fixes are implemented.**
