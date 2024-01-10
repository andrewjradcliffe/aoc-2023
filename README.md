# Hmm, what?

## For first-timers
```bash
git clone https://github.com/andrewjradcliffe/aoc-2023.git
cd aoc-2023
cargo build --release

# binaries:
ls ./target/release/day*
```

## Note to folks that read `/src`

Propagation of errors as `String` (and via `.map_err(|e|
e.to_string())`) is done as strictly a convenience; under any other
circumstances, I do design an `Error` type with classification,
`std::error::Error` implementation, etc. However, I realized on
approximately `day4` that it was far too tedious to write error types
for this hobby project.

The argument is easy to make by considering complexity:
- 1-4 new type definitions per day, each of which is typically parsed
  from `str` (hence, multiple error pathways); if the types are
  nested, then the error pathways nest, hence, the need for a tree of
  `From` impls. Admittedly, the tree depth is typically small (<3),
  but the required number of conversion impls is exponential nonetheless.
- 25 days

This represents a non-trivial amount of code to write.

If more people analyzed the computational complexity of their actions
(rather than just a machine's), the world would be a better place.
