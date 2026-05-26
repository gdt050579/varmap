# varmap

A Rust library for **heterogeneous, typed key–value maps** where each key holds exactly one value of a known type at a time. Values are stored in a compact 16-byte tagged representation; strings, byte slices, and large integers use an internal arena so reads return borrowed references (`&str`, `&[u8]`) without per-lookup allocation.

Three map types share the same value encoding and getter API, but differ in how keys are represented:

| Map                         | Key type                               | Best when                                       |
| --------------------------- | -------------------------------------- | ----------------------------------------------- |
| [`VarMap`](#varmap)         | Precomputed `Key` (64-bit hash)        | Keys are known at compile time (`var!` macro)   |
| [`StrVarMap`](#strvarmap)   | `&str` (FNV-1a hash at runtime)        | Keys come from user input or config at runtime  |
| [`EnumVarMap`](#enumvarmap) | Enum variant (`#[derive(EnumVarMap)]`) | Fixed, closed set of keys known at compile time |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
varmap = "0.1"
```

The proc-macros (`var!`, `#[derive(EnumVarMap)]`, `#[derive(VarMapValue)]`) are re-exported from the `varmap` crate.

## Supported value types

Built-in types (via `set` / `get` / typed getters):

- Integers: `i8` … `i64`, `u8` … `u64`, `i128`, `u128`
- Floats: `f32`, `f64`
- `bool`, `char`
- `&str`, `&[u8]` (stored in the arena; returned as borrows)
- `IpAddr`, `Ipv4Addr`, `Ipv6Addr`

Strings and byte slices up to **14 bytes** are stored inline in the map; larger payloads are arena-allocated.

Custom `Copy` types with alignment 1–16 can use `#[derive(VarMapValue)]` (see [Custom types](#custom-types)).

---

## StrVarMap

Use when variable names are ordinary strings at runtime (configuration keys, script variables, etc.). Keys are hashed with FNV-1a on each `set` / `get`.

```rust
use varmap::StrVarMap;

let mut map = StrVarMap::new();

map.set("port", 8080u16);
map.set("enabled", true);
map.set("host", "api.example.com");

assert_eq!(map.get_u16("port"), Some(8080));
assert_eq!(map.get_bool("enabled"), Some(true));
assert_eq!(map.get_str("host"), Some("api.example.com"));
assert_eq!(map.contains("port"), true);

// Generic get when the type is inferred or specified explicitly
let port: u16 = map.get("port").unwrap();
```

### Typed getters

All three maps provide the same convenience methods:

`get_bool`, `get_u8`, `get_u16`, `get_u32`, `get_u64`, `get_i8`, `get_i16`, `get_i32`, `get_i64`, `get_f32`, `get_f64`, `get_str`, `get_bytes`, `get_char`, `get_ip`, `get_ipv4`, `get_ipv6`

Or use the generic API:

```rust
map.get::<u32>("user.age");
map.get::<&str>("user.name");
```

### Other operations

```rust
map.clear();   // drop all keys and reset the arena
```

---

## VarMap

Use when key names are **fixed at compile time**. The `var!` macro expands to a `Key` holding the FNV-1a hash of the string literal, so lookups skip string hashing and comparison.

```rust
use varmap::{VarMap, Key, var};

let mut map = VarMap::new();

// Compile-time keys (recommended)
map.set(var!("user.age"), 32u32);
map.set(var!("user.name"), "Alice");

assert_eq!(map.get_u32(var!("user.age")), Some(32));
assert_eq!(map.get_str(var!("user.name")), Some("Alice"));

// Or supply a precomputed hash yourself
map.set(Key::new(0x0123_4567_89AB_CDEF), 1u8);
```

`StrVarMap` is a thin wrapper around `VarMap` that hashes `&str` keys at runtime; `VarMap` + `var!` is the faster option when names are static.

### Limits

`VarMap` (and therefore `StrVarMap`) supports at most **65 536** distinct keys (index stored in the low 16 bits of the internal hash entry).

---

## EnumVarMap

Use when the key set is a **closed enum** known at compile time. Each variant maps to a fixed slot (one per enum discriminant), giving **O(1)** access with no hashing or binary search.

Derive `EnumVarMapKey` on your enum (requires `#[repr(u16)]`, `Copy`, and at most 65 536 variants):

```rust
use varmap::{EnumVarMap, EnumVarMapKey};

#[derive(EnumVarMap, Copy, Clone, Debug)]
#[repr(u16)]
enum ConfigKey {
    Port,
    Enabled,
    Host,
}

let mut map = EnumVarMap::<ConfigKey>::new();

map.set(ConfigKey::Port, 8080u16);
map.set(ConfigKey::Enabled, true);
map.set(ConfigKey::Host, "api.example.com");

assert_eq!(map.get_u16(ConfigKey::Port), Some(8080));
assert_eq!(map.get_bool(ConfigKey::Enabled), Some(true));
assert_eq!(map.get_str(ConfigKey::Host), Some("api.example.com"));
assert_eq!(map.contains(ConfigKey::Port), true);
```

`EnumVarMap` owns its own arena (unlike `StrVarMap`, which delegates to an inner `VarMap`). Memory for every enum variant slot is reserved up front, including variants that have not been set yet.

---

## Custom types

Types that are `Copy`, have alignment between 1 and 16 bytes, and contain only plain data can derive `VarMapValue`:

```rust
use varmap::{StrVarMap, VarMapValue};

#[derive(VarMapValue, Copy, Clone, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

let mut map = StrVarMap::new();
let p = Point { x: 1, y: 2 };
map.set("origin", p);

let back: &Point = map.get("origin").unwrap();
assert_eq!(*back, p);
```

Generics and non-`Copy` types are not supported by the derive macro.

---

## Choosing a map

```
                    compile-time keys?
                           │
              ┌────────────┴────────────┐
             yes                        no
              │                          │
        fixed enum?              StrVarMap
              │
     ┌────────┴────────┐
    yes               no
     │                 │
EnumVarMap      VarMap + var!
```

- **`EnumVarMap`** — fastest reads/writes when the key set is an enum; pays fixed storage for every variant slot.
- **`VarMap` + `var!`** — same value layer as `StrVarMap`, but no per-operation string hashing.
- **`StrVarMap`** — simplest API when keys are dynamic strings.

---

## Performance

Benchmarks live in the `banches` crate. Each run performs **10 000** outer iterations; each iteration clears the map and writes (or, for read tests, reads) **256** string keys. Results below are averaged over repeated runs; columns are **time**, **allocation during init**, and **allocation during the timed loop** (via a tracking allocator).

| Implementation | Scenario               |   Time | Init alloc | Run alloc |
| -------------- | ---------------------- | -----: | ---------: | --------: |
| **EnumVarMap** | Update – large strings |  23 ms |    4 112 B |  16 384 B |
| **VarMap**     | Update – large strings |  74 ms |        0 B |  28 672 B |
| **StrVarMap**  | Update – large strings |  90 ms |        0 B |  29 696 B |
| HashMap        | Update – large strings | 175 ms |        0 B |  30 641 B |
| BTreeMap       | Update – large strings | 258 ms |        0 B |  26 985 B |
| **EnumVarMap** | Update – small strings |  31 ms |    4 112 B |       0 B |
| **VarMap**     | Update – small strings |  75 ms |        0 B |  12 288 B |
| **StrVarMap**  | Update – small strings |  92 ms |        0 B |  12 288 B |
| HashMap        | Update – small strings | 174 ms |        0 B |  22 026 B |
| BTreeMap       | Update – small strings | 255 ms |        0 B |  18 370 B |
| **EnumVarMap** | Read – large strings   |  16 ms |   20 496 B |       0 B |
| **VarMap**     | Read – large strings   |  59 ms |   28 672 B |       0 B |
| HashMap        | Read – large strings   |  58 ms |   30 641 B |       0 B |
| **StrVarMap**  | Read – large strings   |  85 ms |   28 672 B |       0 B |
| BTreeMap       | Read – large strings   |  93 ms |   26 985 B |       0 B |
| **EnumVarMap** | Read – small strings   |  10 ms |    4 112 B |       0 B |
| **VarMap**     | Read – small strings   |  35 ms |   12 288 B |       0 B |
| HashMap        | Read – small strings   |  52 ms |   22 026 B |       0 B |
| **StrVarMap**  | Read – small strings   |  54 ms |   12 288 B |       0 B |
| BTreeMap       | Read – small strings   |  90 ms |   18 370 B |       0 B |

*Large* values are long descriptive strings (~50+ bytes); *small* values are short tokens (a few bytes), many of which use inline storage (≤14 bytes) instead of the arena.

### Takeaways

1. **`EnumVarMap` is fastest** for this workload: direct indexing avoids hash tables and, for `VarMap`/`StrVarMap`, sorted hash-vector lookup. Read-heavy paths show the largest gap (e.g. 10 ms vs 35–54 ms for small strings).
2. **`VarMap` beats `StrVarMap`** by roughly 15–20% because `var!` removes runtime FNV-1a over key names.
3. **All three varmap types beat `HashMap` and `BTreeMap`** on updates (roughly 2–11× faster than `BTreeMap`). Reads are competitive with or faster than `HashMap`, especially for `EnumVarMap` and `VarMap`.
4. **`EnumVarMap` trades memory for speed**: ~4 KiB upfront per 256-key enum (empty slots), but **zero** run-time allocation on small-string read/update tests once initialized.
5. **Inline small strings** reduce arena churn: `EnumVarMap` reports 0 bytes allocated during small-string update loops.

Reproduce locally from the workspace root:

```bash
cargo run -p banches -- RUN 10000 10
```

Use `cargo run -p banches -- LIST` to see individual benchmark names and filter with an optional fourth argument (e.g. `enumvarmap`).

---

## License

MIT — see repository and crate metadata for details.
