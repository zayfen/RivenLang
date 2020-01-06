# zlang
a language implemented in rust

## grammar

### Node Objects
#### Node
```rust
struct Node {
  type: String,
  loc: SourceLocation | None
}
```

#### Location
```rust
struct Location {
  row: usize,
  column: usize
}
```

#### SourceLocation
```rust
struct SourceLocation {
  source: String | None,
  start: Location,
  end: Location
}
```

### Programs


