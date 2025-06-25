
# Agent Workflow: Creating a New UI Component

This document outlines the structured process for planning a new UI component for the Bevy Radix-inspired library. Follow these phases sequentially. **Do not write implementation code until the plan is approved.**

## Phase 1: Discovery & Research

Your goal in this phase is to fully understand the requirements and technical context.

### 1.1. Clarify Requirements (PRD Generation)
-   Ask the user clarifying questions based on the initial prompt. Use the PRD structure (Goals, User Stories, Requirements, Non-Goals) as a guide.
-   Generate a preliminary `prd-[feature-name].md` for confirmation.

### 1.2. Research Sources
-   **Radix Principles:** Review the corresponding component documentation listed in `SOURCE.md`. Identify the core API, props, and anatomy of the component.
-   **Bevy Implementation:** Research `docs.rs` for the relevant Bevy components and concepts needed (e.g., `NodeBundle`, `Style`, `Interaction`, `FocusPolicy`).

### 1.3. Analyze Existing Code
-   Review the project's `src/` directory.
-   Identify existing components, utilities, or theme elements that can be reused (e.g., `Text` component, theme colors, `ButtonBuilder` pattern).
-   List the specific modules/structs you intend to reuse.

## Phase 2: Implementation Plan

Based on your research, create a detailed implementation plan. Present this plan to the user for approval.

### 2.1. Proposed File Structure
-   List the new files to be created (e.g., `src/components/new_component.rs`).
-   List existing files that will be modified (e.g., `src/components/mod.rs`, `src/lib.rs`).

### 2.2. Component API & Data Structures
-   Define the main public structs (e.g., `NewComponent`, `NewComponentBuilder`, enums for variants/sizes).
-   Show a code example of how the component will be used (Builder Pattern).

### 2.3. Bevy ECS Integration
-   Describe the Bevy `Bundle` that will be created. What Bevy components will it contain? (`NodeBundle`, `Interaction`, etc.).
-   List the systems that will be needed (e.g., a system to handle interactions, a system to update visuals based on state).
-   Explain how the component will integrate with the theme (`Theme`, `Colors`, `Typography`).

### 2.4. Open Questions
-   List any remaining ambiguities or technical challenges that require user input.

---
**Wait for user approval of the plan before proceeding to implementation.**
---
