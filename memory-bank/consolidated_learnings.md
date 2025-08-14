## 2025-08-14 — Validators & Command Hardening

### Pattern: Centralize validation for command boundaries
- Move repeated length/byte/security checks into a small helpers module (`src-tauri/src/security/validators.rs`) to ensure consistent enforcement and error messages.
- Helpers implemented:
  - `validate_id(field_name, value, max_len)` — trims, checks char/byte limits, security scan.
  - `validate_non_empty_str(field_name, value, max_len)` — enforces non-empty + length/security.
  - `validate_optional_str(field_name, &Option<String>, max_len, allow_empty)` — optional string validation.
  - `validate_body_limits(field_name, body, max_bytes, max_chars)` — enforces both byte and char limits and security checks.
  - `validate_order_index_default(order_index)` and rating shortcuts for common numeric ranges.
- Rationale: Prevents mismatches between byte and char limits, reduces copy/paste errors, and produces consistent error messages.

### Applied Changes (high-impact)
- Replaced ad-hoc validation across core command handlers:
  - `src-tauri/src/commands/projects.rs` — now uses `validate_id`, `validate_optional_str`, consolidating project-level checks.
  - `src-tauri/src/commands/documents.rs` — now uses `validate_body_limits`, `validate_id`, `validate_optional_id`, `validate_order_index_default`.
  - `src-tauri/src/commands/ai_writing.rs` — now uses `validate_non_empty_str`, `validate_body_limits`, `validate_optional_str` for AI inputs.
- Result: Standardized validation on high-risk surfaces (project/document creation, AI inputs). Reduced chance of missing byte-length checks or inconsistent messages.

### Best Practices
- Prefer `validate_body_limits` when content may contain multi-byte characters or when both byte and char length matter (uploads, large text fields).
- Use `validate_non_empty_str` for short text fields that must not be blank (titles, prompts).
- Use `validate_optional_str` for optional inputs where empty string may be allowed or disallowed explicitly.
- For identifier-like fields, use `validate_id` and `validate_optional_id` with conservative max lengths (e.g., 64 or 255 depending on the domain).

### Next consolidation steps
- Expand validators use to remaining command handlers (templates, collaboration, plugin, story_bible, canvas).
- Add unit tests for validators covering non-ASCII multibyte inputs and edge byte-length cases.
- Add CI gate enforcing use of `validate_*` helpers for exported Tauri commands that accept user-provided strings.
