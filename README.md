# G4SSG-Core

Multiverse, universe, galaxy, and/or star system generator core. Built
more or less based on *GURPS 4e Space*, although gradually going for more
scientific approach.

**W.I.P.** - YMMV, etc.

## `Cargo.toml`

If/when Population-III stars get more or less confidently confirmed
candidates, enable the "popiii_candidate" feature.

## Multi-threading + Other "Noteworthy Stuff"

TODO.

### Clustering Goals

The goal at some point is to harness e.g. Tokio (alongside local Rayon, of course)
for freely scalable clustering, etc.

## [G4SSG-CLI](https://github.com/msukanen/g4ssg-cli.git)

Future clustering will enable efficient usage of
`--scope galaxy` and possibly
`--scope universe`, or theoretically even the megalomanic
`--scope multiverse`, of which none are truly feasible on a single machine.
