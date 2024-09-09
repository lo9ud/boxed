# Rationale

This document is intended to provide a high-level overview of the project, its goals, and the rationale behind the decisions made during its development.

## Goals

- Demonstrate my ability to design and implement a complex system
  - Multiple interacting components (UI, datamodel, etc.)
  - Complex algorithms for many different moving parts
- Demonstrate my ability to work with complex data structures
  - Trees, graphs, etc. for expression parsing, evalutaion and dependency tracking
- Demonstrate my ability to work with complex UIs
  - Maintaining state, keeping backend and frontend in sync, etc.

## Design Decisions

- **Expressions**: I chose a recursive descent parser, as it naturally models the structure of the simple expression grammar. Additionally, it is easy to implement typechecking in the parsing stage when usnig this method.

- [**Evaluation**](./expressions.md): I chose a simple recursive, bottom-up evaluation strategy. This is simple to implement and understand, and is sufficient for the simple expressions I am working with.

- [**Data Model**](./model.md): I chose a simple key-value store for the workspace, and a column-based data model for the cells. This allows for static typing and easy typechecking.

- [**UI**](./ui.md): I chose to use React.js built on top of Tauri. This allws me to use Rust, which I am more comfortable with, for the backend, and React, which I wanted to learn more about, for the frontend.

  - React is a good choice for this project because it allows for easy state management and updating, which is important for a spreadsheet-like application, as well as rapid prototyping and development.

- [**File I/O**](./files.md): I chose to use a custom binary format for saving and loading as a challenge for myself. This format is based on the Parquet format, but is much simpler. I chose not to support Excel files due to the complexity of the ISO 29500 standard.

- [**Type System**](./types.md): I chose to use a simple type system with no implicit type conversions. This is to avoid the ambiguity that can arise from implicit type conversions, and to make the system easier to reason about.
