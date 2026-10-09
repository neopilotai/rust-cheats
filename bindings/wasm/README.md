# WASM binding plan

Create a dedicated `cdylib` crate with `wasm-bindgen` and `serde-wasm-bindgen`. Initially expose pure functions that accept sheet JSON and return search/completion JSON. Do not depend on filesystem walking or Tokio in the browser build. A browser UI can ship pre-generated JSON indexes with the static site.
