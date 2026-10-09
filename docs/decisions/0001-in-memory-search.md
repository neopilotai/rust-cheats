# ADR 0001: Start with deterministic in-memory search

Status: accepted

The initial search backend scans validated sheets in memory and applies explicit ranking stages for exact IDs/titles, prefixes, tags, descriptions, and fuzzy matches. This keeps the offline binary small and makes behavior easy to test. A persisted index can be added behind the search API after benchmarks demonstrate that the bundled dataset requires it; Tantivy is not introduced speculatively.

The tie-breaker is title order, which makes CLI and editor output reproducible.
