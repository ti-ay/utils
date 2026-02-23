# ut (Utility Macros)

`ut` provides a suite of utility macros.
All macros can be enabled or disabled (default) at compile time.
If macros are disables, then they are entirely stripped from the binary. 

## Macros

| Macro | Description |
| :--- | :--- |
| `ut_note` | Quick annotations or breadcrumbs in your code. |
| `ut_time` | Measures the execution time of a block or expression. |
| `ut_view` | Inspects variables and expressions (similar to `dbg!`). |
| `ut_out` | Standard output printing. |
| `ut_outln` | Standard output printing with a newline. |

---

## Feature Flag System

`ut` uses a granular feature flag system. You can toggle groups of macros or individual ones. If a macro is disabled, it expands to an empty statement, and the compiler removes it entirely.

### Feature Hierarchy

* **Global Toggles:**
    * `ut_on`: Enables all macros.
    * `ut_off`: Disables all macros.
* **Group Toggles:**
    * `dbg_on` / `dbg_off`: Controls `view`, `time`, and `note`.
    * `out_on` / `out_off`: Controls `out` and `outln`.
* **Individual Toggles:**
    * `note_on` / `note_off`
    * `time_on` / `time_off`
    * `view_on` / `view_off`

---

## Example
```rust
use ut::{ut_note, ut_time, ut_view, ut_outln};

fn main() {
    ut_note!("Starting heavy computation");

    let result = ut_time!({
        let val = fib(40);
        ut_view!(val);
        val
    });

    ut_outln!("Calculation finished: {}", result);
}
```
```bash
cargo run -p bin1 -F ut/ut_on
```
```bash
[NOTE] Starting heavy computation
[VIEW] "val" => 165580141
[TIME] [{ let val = fib(40); ut_view!(val); val }] took: 430 ms
Calculation finished: 165580141
```

## Setup

Add the dependency to your `Cargo.toml`. You can define your own feature sets to control how `ut` behaves in different build profiles.

```toml
[dependencies]
ut = { git = "https://github.com/ti-ay/utils", package = "ut"}

[features]
default = ["all_on"]

# Bundle flags for convenience
all_off = ["ut/note_off", "ut/time_off", "ut/view_off", "ut/out_off"]
all_on = ["ut/note_on", "ut/time_on", "ut/view_on", "ut/out_on"]
```
