# Tiger Style

```
Author: John Lin @john-s-lin
Date Created: 2024-04-11
```

## Introduction

This document describes a software engineering practice called "Tiger Style", first coined by Joran Dirk Greef of [TigerBeetle](https://tigerbeetle.com/) in [this talk](https://youtu.be/w3WYdYyjek4?feature=shared) and in [this document](https://github.com/tigerbeetle/tigerbeetle/blob/81806334726ef7438c74d5f630b126b32431663a/docs/TIGER_STYLE.md).

The purpose of Tiger Style is to develop safe, performant systems in a short amount of time. `hermes-proto` will be developed with a focus on three concepts mentioned in Tiger Style:

1. Spending a little more time on design speeds up development and prevents technical debt from working around a bad, non-optimal design.
2. Assertions within source code ensures safe failovers and correct code.
3. Deterministic simulations allow for faster reliability testing on the order of years.

## Premise

### Why invest in design first, code after?

Bad design compounds over time. That's why we're spending a little more time on the drawing board, figuring out the optimal architecture for `hermes-proto`. Good design has _relatively_ good performance baked in.

### Why not relegate assertions to tests only, instead of also in source code?

Assertions in production code ensure that a system immediately exits if an unknown condition occurs. [This blog post](https://ratfactor.com/cards/tiger-style) explains it better.

### Why use deterministic simulations?

If we can simulate a 24h period of emergency room triage in a few seconds, why shouldn't we? We may not be building a mission-critical system, but who wouldn't want to build a resilient system that can simulate high traffic load under testing?

## References

- Tiger Style: https://github.com/tigerbeetle/tigerbeetle/blob/main/docs/TIGER_STYLE.md
- TigerStyle! (Or How To Design Safer Systems in Less Time) by Joran Dirk Greef [YouTube]: https://youtu.be/w3WYdYyjek4?feature=shared
