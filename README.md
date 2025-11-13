# G4SSG-Core
Multiverse, universe, galaxy, and/or star system generator core. Built
more or less based on *GURPS 4e Space*.

```
MIT License

Copyright (c) 2025 Markku Sukanen

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

**W.I.P.** - YMMV, etc.

# Requirements
* [g4ssg-pm](https://github.com/msukanen/g4ssg-pm.git)

# Multi-threading + Other "Noteworthy Stuff"
G4SSG-Core uses a massively parallel modus operandi relying (at the time of
writing at least) on Rayon.

## Clustering Goals
The goal at some point is to harness e.g. Tokio (alongside local Rayon, of course)
for freely scalable clustering, etc.

### [G4SSG-CLI](https://github.com/msukanen/g4ssg-cli.git)
Future clustering will enable efficient usage of
`--scope galaxy` and possibly
`--scope universe`, or theoretically even the megalomanic
`--scope multiverse`, of which none are truly feasible on a single machine.
