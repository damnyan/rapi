# Copilot Instructions for AI Coding Agents

## Project Overview

This repository uses a specification-driven workflow for feature development, centered around the `.specify/` directory and a set of bash scripts and markdown templates. The process is highly automated and expects agents to follow a strict sequence for planning, specifying, and implementing features.

## Key Workflows

- **Feature Planning:**
  - Run `.specify/scripts/bash/setup-plan.sh --json` to initialize a plan. Parse the output for `FEATURE_SPEC`, `IMPL_PLAN`, `SPECS_DIR`, and `BRANCH`.
  - Analyze the feature spec and constitution at `.specify/memory/constitution.md`.
  - Use `.specify/templates/plan-template.md` to generate design artifacts in `$SPECS_DIR` (e.g., `research.md`, `data-model.md`, `contracts/`, `quickstart.md`, `tasks.md`).
- **Feature Specification:**
  - Use `.specify/scripts/bash/create-new-feature.sh --json "$ARGUMENTS"` to create a new feature branch and spec file.
  - Write the spec using `.specify/templates/spec-template.md`.
- **Task Generation:**
  - Use `.specify/scripts/bash/check-task-prerequisites.sh --json` to discover available design docs.
  - Generate `tasks.md` using `.specify/templates/tasks-template.md`, referencing all available artifacts.

## Conventions & Patterns

- **Absolute Paths:** Always use absolute paths from the repo root for file operations.
- **Artifacts:** All design and implementation artifacts are stored in `$SPECS_DIR` for each feature.
- **Templates:** All markdown artifacts are generated from templates in `.specify/templates/`.
- **Branching:** Scripts create and check out feature branches automatically.
- **Task Numbering:** Tasks in `tasks.md` are numbered (T001, T002, ...) and grouped for parallel execution with `[P]`.
- **Error Handling:** Check for `ERROR` states in progress tracking and ensure all phases are complete before reporting success.

## Examples

- To start a new feature: `.specify/scripts/bash/create-new-feature.sh --json "Add login support"`
- To plan implementation: `.specify/scripts/bash/setup-plan.sh --json`
- To generate tasks: `.specify/scripts/bash/check-task-prerequisites.sh --json`

## Key Files & Directories

- `.specify/scripts/bash/` — Automation scripts for planning/specification
- `.specify/templates/` — Markdown templates for all artifacts
- `.specify/memory/constitution.md` — Project constitution and requirements
- `.github/prompts/` — Prompt files for agent workflows

## Agent Guidance

## Best Practice: Git Commit Messages

When making git commits, follow these best practices:

- **Use the imperative mood**: "Add feature" not "Added" or "Adds".
- **Start with a concise summary** (max 50 characters), capitalized, no period.
- **Separate summary from body with a blank line**.
- **Use the body to explain _what_ and _why_, not _how_** (wrap at 72 chars).
- **Reference issues or specs if relevant** (e.g., "See #42").
- **Example:**

  Add user authentication to API scaffold

  Implements JWT-based login and registration endpoints.
  Updates OpenAPI spec and adds tests. See 001-create-api-scaffold/spec.md.
