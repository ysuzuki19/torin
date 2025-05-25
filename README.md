# torin

Development tool to track and enforce scheduled code deletions via comment annotations.

## What is torin?

Torin helps manage the lifecycle of your code by allowing developers to schedule parts of their codebase for deletion or to flag them as errors under certain conditions. It uses special comment annotations to mark code blocks and defines rules or dates for when these actions should be triggered.

This is particularly useful for:

- Managing feature flags and cleaning up old code paths.
- Ensuring temporary code, experiments, or debug utilities are removed or flagged.
- Keeping track of deprecation timelines and enforcing them.
- Preventing accidental commits of code that should not go into production (e.g., via `ERROR` annotations).

## Features

- **Scheduled Code Actions**: Mark code for future deletion or to trigger errors using simple comment annotations.
- **Flexible Triggers**: Define actions based on:
  - **Custom rules**: e.g., `rule=debug`, `rule=experimental_feature` (activated via `.torin.toml`).
  - **Specific dates**: e.g., `date=2025-01-01`.
- **Operational Modes**:
  - `plan`: Show a diff of what would be changed (deletions) or what errors would be flagged, without applying.
  - `check`: Verify if any scheduled deletions are due or if any error annotations are active. Exits with a non-zero status code if actions are pending, making it suitable for CI checks.
  - `apply`: Apply the scheduled deletions to the codebase.
- **Configuration File**: Manage included/excluded files and project-wide active rules via a `.torin.toml` file.
- **Annotation System**: Uses clear and parseable comment annotations like:
  - `// torin DELETE BEGIN ...` and `// torin DELETE END` for blocks.
  - `// torin DELETE NEIGHBOR ...` for code segments.
  - `// torin ERROR NEIGHBOR ...` or `// torin ERROR BEGIN ...` to flag code.

## How it works

Torin scans your source files (as specified in your project's `.torin.toml`) for special comment annotations. These annotations define:

1.  A **Command**:
    - `DELETE`: Marks code for deletion.
    - `ERROR`: Marks code that, if its trigger is active, will cause `torin check` to fail.
2.  A **Target**:
    - `BEGIN ... END`: Defines a block of code between a `BEGIN` and `END` annotation.
    - `NEIGHBOR`: Refers to the contiguous block of code containing the annotation, typically delimited by empty lines or file/block boundaries.
3.  A **Trigger**:
    - `rule=<rulename>`: The action is active if `<rulename>` is listed in the `rules` array in `.torin.toml`.
    - `date=<YYYY-MM-DD>`: The action is active if the current date is on or after the specified date.

When `torin` runs:

- `torin plan`: Evaluates annotations and shows what deletions would occur or which `ERROR` annotations are active.
- `torin check`: Reports if any `DELETE` actions are due or `ERROR` annotations are active.
- `torin apply`: Modifies the files to perform the `DELETE` actions. `ERROR` annotations do not cause modifications but will still cause `check` to fail if active.

## Installation

Currently, you can build Torin from source:

```sh
git clone https://github.com/ysuzuki19/torin
cd torin
cargo install --path .
```

You can then copy this binary to a location in your PATH.

## Usage

### 1. Configuration (`.torin.toml`)

Create a `.torin.toml` file in the root of your project. This file tells Torin which files to scan and which rules are currently active.

```toml
// filepath: .torin.toml
[project]
# Glob patterns for files to include in scanning
includes = ["src/**/*.rs", "examples/**/*.rs"]

# Glob patterns for files/directories to exclude
excludes = ["src/vendor/**/*.rs", "target/**"]

# List of active rules. Annotations with these rules will be processed.
rules = ["debug", "experimental_feature_x"]
```

### 2. Annotations in Code

Embed annotations directly into your code as comments.

**Block Deletion (Date-based):**

```rust
// filepath: src/example.rs
// ...existing code...
// torin DELETE BEGIN date=2025-01-01
fn old_feature_code() {
    // This code will be marked for deletion on or after January 1, 2025
    println!("Old feature is running");
}
// torin DELETE END
// ...existing code...
```

**Neighbor Deletion (Rule-based):**
Deletes the contiguous block of code containing the annotation, typically delimited by empty lines.

```rust
// filepath: src/example.rs
// ...existing code...

// torin DELETE NEIGHBOR rule=cleanup_feature_y
fn old_helper_for_feature_y() {
    // This function and its annotation will be removed
    // if 'cleanup_feature_y' rule is active in .torin.toml.
    // The deletion includes this entire block,
    // usually from the preceding empty line to the
    // subsequent empty line (or file boundaries).
}

// ...existing code...
```

**Error Annotation (Rule-based):**
Marks code that should cause `torin check` to fail if the rule is active. This is useful for preventing commits of temporary debug code.

```rust
// filepath: src/example.rs
// ...existing code...

// torin ERROR NEIGHBOR rule=production_block
fn temporary_debug_function() {
    // If 'production_block' rule is active in .torin.toml,
    // `torin check` will fail, indicating this code should not be present.
    // `torin apply` will not remove this code; it only flags it.
    eprintln!("Temporary debug function called!");
}

// ...existing code...
```

### 3. CLI Commands

- **Plan changes**:
  See what Torin would do without actually changing files.

  ```sh
  torin plan
  ```

- **Check for due actions**:
  Ideal for CI/CD pipelines to ensure code hygiene. Exits with `0` if no actions are pending, `1` otherwise.

  ```sh
  torin check
  ```

- **Apply changes**:
  Modifies files in place to remove code marked with `DELETE` annotations whose triggers are active.
  **Warning: This modifies files. Ensure your code is under version control.**

  ```sh
  torin apply
  ```

- **Generate Shell Completion**:
  Torin can generate completion scripts for various shells.
  ```sh
  torin completion bash > /usr/local/etc/bash_completion.d/torin # Example for bash
  torin completion zsh > ~/.zfunc/_torin # Example for zsh
  # For fish:
  # torin completion fish > ~/.config/fish/completions/torin.fish
  ```
  Follow your shell's instructions for installing completion scripts.

## License

This project is licensed under the Mozilla Public License Version 2.0.
See the [LICENSE](LICENSE) file for details.
