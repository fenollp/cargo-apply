# `cargo apply`

A `Coccinelle`-based approach to applying modifications to a `cargo` workspace.

Scope of possible manipulations may also encompass TOML files changes and `build.rs` edits.

Tiniest scope is to be able to:
* find crates that use a given lib's version
* try any crate with a pass of the semantic code patch being developped

cf https://www.reddit.com/r/rust/comments/18a2ugn/coccinelle_for_rust_a_tool_designed_for_program/
