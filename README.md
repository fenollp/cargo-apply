# `cargo apply`

A [`Coccinelle`](https://rust-for-linux.com/coccinelle-for-rust)-based approach to applying modifications to a `cargo` workspace.

Scope of possible manipulations may also encompass TOML files changes and `build.rs` edits.

Tiniest scope is to be able to:
* find crates that use a given lib's version
* try any crate with a pass of the semantic code patch being developped

cf https://www.reddit.com/r/rust/comments/18a2ugn/coccinelle_for_rust_a_tool_designed_for_program/

```
cargo apply updates

  Auto-patch workspace code after dependencies were updated.

# To run after e.g. `cargo update --recursive toml`
# in directory containing /Cargo.lock
#     filter with `  -e, --edges <KINDS>            The kinds of dependencies to display (features, normal, build, dev, all, no-normal, no-build, no-dev, no-proc-macro)` ?

# For all crates in workspace,
# find the direct dependencies that changed
#     `cargo tree --frozen --target=all --all-features  --depth 1`
#     `git diff -- Cargo.lock` => abort if not git
# For all these deps,
# https://doc.rust-lang.org/cargo/commands/cargo-add.html each somewhere to find coccinelle files in their source
```

```
cargo apply patch [COCCINELLE OPTIONS] [ --allow-dirty | --allow-staged ] COCCIFILE.cocci [ FILEPATH+ ]

  Applies COCCIFILE to FILEPATHs

      --allow-dirty
          Fix code even if the working directory is dirty or has staged changes
      --allow-staged
          Fix code even if the working directory has staged changes
```



---
CLI needs more thought...
```
cargo apply updates [ -p CRATE ]
cargo apply patch ( [ -p CRATE ] COCCIFNAME | -p CRATE | { -f COCCIFNAME.cocci }+ ) [ --allow-dirty | --allow-staged ] [ -- { SOME? COCCINELLE OPTIONS } ]

fname ordering: alphanum
```