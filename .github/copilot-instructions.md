---
description: AI rules derived by SpecStory from the project AI interaction history
globs: *
---

## <headers/>

## AI CODING AGENT GUIDELINES

The AI coding assistant must adhere to the following guidelines when working on this project:

*   Focus on providing actionable and concise instructions (20-50 lines) using markdown.
*   Include specific examples from the codebase when describing patterns.
*   Avoid generic advice (e.g., "write tests," "handle errors") and focus on project-specific approaches.
*   Document only discoverable patterns, not aspirational practices.
*   Reference key files/directories that exemplify important patterns.
*   When updating `.github/copilot-instructions.md`, merge intelligently, preserving valuable content while updating outdated sections.

## WORKFLOW & RELEASE RULES

## TECH STACK

## PROJECT DOCUMENTATION & CONTEXT SYSTEM

When creating or updating project documentation, including `.github/copilot-instructions.md`, source existing AI conventions from the following locations: `**/{.github/copilot-instructions.md,AGENT.md,AGENTS.md,CLAUDE.md,.cursorrules,.windsurfrules,.clinerules,.cursor/rules/**,.windsurf/rules/**,.clinerules/**,README.md}`.

When analyzing the codebase to generate or update `.github/copilot-instructions.md`, focus on discovering the essential knowledge that would help an AI agents be immediately productive in this codebase. Consider aspects like:
- The "big picture" architecture that requires reading multiple files to understand - major components, service boundaries, data flows, and the "why" behind structural decisions
- Critical developer workflows (builds, tests, debugging) especially commands that aren't obvious from file inspection alone
- Project-specific conventions and patterns that differ from common practices
- Integration points, external dependencies, and cross-component communication patterns

## DEBUGGING