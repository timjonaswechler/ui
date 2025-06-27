---

title: "Refactor <ComponentName>.rs into smaller modules"
area: refactor
status: open
------------

# Task · Modularise Large Component File

## Context

`<ComponentName>.rs` has grown beyond 400 LOC and presently mixes data definitions, systems, helpers, and UI widgets in a single file.  This hampers readability, reuse, and targeted testing.

## Goal

Split `<ComponentName>.rs` into logically separated modules while preserving the public API.  A thin **root module** will re‑export the new internal sub‑modules to avoid breaking external code.


## Acceptance Criteria

* Public API (functions, structs, type aliases) remains **backward‑compatible** (no breaking changes).
* File structure mirrors the chosen breakdown and stays under 300 LOC per file.
* All imports updated; existing examples compile and `cargo test` passes.
* Each new sub‑module has top‑level rustdoc explaining its purpose.

## Steps

1. Analyse `<ComponentName>.rs` to identify natural boundaries.
2. Create new directory & files under `src/ui/<component_name>/`.
3. Move code, adjust `use` paths, update module tree in `mod.rs`.
4. Run full test + doc build.
5. Commit with title `refactor(<component>): split into modules`.

## Notes

* Keep commits small; one component per branch.
* Coordinate with the documentation task to update examples if paths change.

---

*Created 2025‑06‑27*

## Progress
- [] `Button`
- [] `Badge`
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
- [] `Toggle`
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