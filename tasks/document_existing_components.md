---

title: "Add professional rustdoc to existing components"
area: documentation
status: open
------------

# Task · Documentation Sprint – Rustdoc for Components

## Context

Several public components distributed across `src/components/` and `src/ui/` still lack high‑quality rustdoc comments.  Comprehensive, example‑rich documentation is essential for external contributors and downstream users to understand the API surface and best‑practice usage.

## Goal

Iteratively add **professional‑grade rustdoc** to all existing components.  Each rustdoc block should follow the official [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/documentation.html), including:

* A concise one‑line summary.
* A detailed description with assumptions, edge cases, and invariants.
* At least one `/// ### Example` section with a *runnable* code snippet.
* Links to related components or external resources where appropriate.

## Acceptance Criteria

* Every public item (struct, enum, trait, function) in targeted files has rustdoc coverage ≥ 90 % according to `cargo llvm-cov --doc`.
* Code examples compile via `cargo test --doc` and demonstrate typical usage inside a `BevyApp`.
* Ran `cargo doc --no-deps` successfully **without warnings**.
* A demo scene for at least one newly documented component lives in `examples/` and is referenced in its rustdoc.

## Suggested Workflow

1. **Inventory** – Run `cargo doc --document-private-items` and note files with missing docs.
2. **Lock file** – Start a short‑lived branch (`docs/<component‑name>`).
3. **Write docs** – Add rustdoc, ensure example compiles with `rustdoc --test`.
4. **Commit** – Title format: `docs(<component>): add rustdoc`.
5. **Push / PR** – Small scope (1–3 files per PR) to keep reviews manageable.

## Helpful Resources

* Official Bevy examples for reference.
* `SOURCE.md` links to Radix UI patterns for consistency.

---

*Created 2025‑06‑27*

## Progress

- [x] `Button` - Added comprehensive rustdoc documentation
- [x] `Badge` - Already well documented with comprehensive rustdoc
- [] `Box`
- [] `Card`
- [] `Checkbox`
- [] `Flex`
- [] `Grid`
- [] `Heading`
- [] `HoverCard`
- [] `Progress`
- [] `Radio`
- [] `Section`
- [] `Select`
- [] `Separator`
- [] `Slider`
- [] `Switch`
- [] `Tabs`
- [] `Text`
- [x] `Toggle`
- [] `ToggleGroup`
- [] `RadioCard`
- [] `TabNav`
- [] `Spinner`
- [] `Menu (base)`
- [] `Menubar`
- [] `Navigation Menu`
- [] `Dropdown Menu`
- [] `Context Menu`
- [] `Dialog`
- [] `Alert Dialog`
- [] `Collapsible`
- [] `Checkbox Group`
- [] `Accordion`