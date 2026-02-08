# Project Conventions

This document describes some of the conventions used in this project. It is
intnended for the developers who wish to contribute to project or simply read
the source code of the project.

## Design by contract

We believe contracts are a necessary part of code and thus we follow
Design by Contract really seriously. For getting started with the same, please
read [Contracts Chapter of Better Code book](https://github.com/stlab/better-code/blob/main/better-code/src/chapter-2-contracts.md).

These are project wide policy used for contract documentation:

- Every declaration outside a function body must have a documentation comment that describes its contract.
    - Start with a summary sentence fragment.
        - Describe what a function or method does and what it returns.
        - Describe what a property or type is.
        - Separate the fragment from any additional documentation with a blank line and end it with a period.
    - Preconditions, postconditions and invariants obviously implied by the summary need not be explicitly documented.
    - Declarations that fulfill protocol requirements are exempted when nothing useful can be added to the documentation of the protocol requirement itself.
- Document the time complexity of every operation, except for constructors which operate in constant time.
- Document the space complexity of every operation that doesn't operate in constant space.

## API design guidelines

We take most of our inspirations for API design guidelines from:
[Swift API Guidelines](https://www.swift.org/documentation/api-design-guidelines/).
Although, this is not a Swift project, but we can just put away the swift specific
things from the above document and follow the rest.

The only exception is Swift uses camelCase for variable names. But we follow the
rust style snake_case.

## Code Formatting

We use `rustfmt` for formatting every code. Please ensure to run rustfmt before
submitting the same. There are PR checks, that checks for the same.

## File Names

All Rust files end with `.rust` extension.

In general, a file is named after the main entity it defines. For more complex
situtations, please exercise your best judgement.
