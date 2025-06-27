# Agent Handbook

## 1 – Project Goal

We are building a cross‑platform, interactive application with the **Bevy 0.16** game engine.  A key focus is a modular UI layer that adapts **Radix UI** design principles for the Bevy ecosystem.  Agents collaborate to research, design, and implement new UI components while maintaining high code quality, documentation, and test coverage.

---

## 2 – What to Read First (session bootstrap)

When a new conversation starts, the agent should …

1. **Open the Bevy API docs** for the current engine version: [https://docs.rs/bevy/0.16.0/bevy/index.html](https://docs.rs/bevy/0.16.0/bevy/index.html)
2. **Scan `SOURCE.md`** for the curated list of Radix links, existing UI crates, and integration notes.
3. **Check the `tasks/` directory** for any open task files matching today’s scope.
4. Ensure local tooling (`rustup`, `cargo`, nightly toolchain) matches the repository’s `rust-toolchain.toml`.

> *If one of these items is missing or outdated, create a task card in `tasks/maintenance/` before proceeding.*

---

## 3 – Behaviour & Workflow Guidelines

| Phase         | Expected output                                                              |
| ------------- | ---------------------------------------------------------------------------- |
| **Discover**  | Short summary of problem, links to relevant docs, quick feasibility check    |
| **Plan**      | Bullet‑point plan or PRD fragment; get approval if the change is non‑trivial |
| **Implement** | Code, examples, docs, assets                                                 |

After finishing work **in any session**:

1. **Commit** *all* touched files.
2. **Commit title:** English, ≤ 72 chars, imperative mood. e.g. `feat(ui): add radial‑menu component`
3. **Commit body (optional):** 1–3 sentences with context, decisions, and follow‑ups.
4. **Push** to the remote branch.

> "No commit → No progress." Every agent session ends with a push, even for WIP branches.

---

## 4 – Directory Structure & Conventions

```
/
├─ Agent.md               # you are here
├─ SOURCE.md              # curated external resources
├─ tasks/                 # specific task briefs, one file per task
│   ├─ ui/new_navbar.md
│   ├─ maintenance/update_bevy.md
│   └─ …
├─ src/                   # Rust codebase (Bevy app & plugins)
└─ examples/              # runnable demos & showcase scenes
```

* **Rust style:** `rustfmt` defaults, Clippy must pass.
* **Naming:** snake\_case for files/modules, CamelCase for types.
* **Docs:** All public items require rustdoc comments.

---

## 5 – CI & Quality Gates

* `cargo test` **must pass** locally before every push.
* `cargo fmt -- --check` enforced in CI.
* New UI components must include at least one **demo scene** in `examples/` demonstrating intended usage.
* Pull requests trigger automated linting, tests, and a visual diff on UI snapshots.

---

## 6 – Communication Norms

* **Commit messages & code comments:** English.
* **Issue / PR discussion:** German or English, whichever is clearer.
* Keep responses concise and actionable.
* Tag reviewers early; avoid “big‑bang” PRs.

---

## 7 – Finding Specific Tasks

Detailed briefs live in `tasks/`.  Filenames follow `<area>/<short_name>.md`, for example:

* `ui/add_slider_control.md`
* `ai/smart_camera_follow.md`
* `maintenance/upgrade_bevy.md`

Each task file contains:

1. **Context**  – why the task matters
2. **Acceptance criteria**  – done when …
3. **Links / assets**  – designs, reference code, docs

Agents should process one task at a time, updating its status in the task front‑matter (`status: in‑progress/blocked/done`).

---

*Last updated: 2025‑06‑27*
