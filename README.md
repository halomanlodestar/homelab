<!-- @format -->

# File management and Streaming system

Built to manage videos and stream them from local file system on demand

## Crates used

- Tokio
- Axum
- Serde
- Serde Json

Refer to Cargo.toml file for versions

## Module Structure

### `api`

Module contains end points for requests
This is divided into Router and Controllers for separation of concerns

### `File`

Custom File module to extract required file data using `std::fs` module.
