# TAISS [Substreams](https://substreams.streamingfast.io)

[![Build Status](https://github.com/pinax-network/taiss-substreams/actions/workflows/ci.yml/badge.svg)](https://github.com/pinax-network/taiss-substreams/actions/workflows/ci.yml)
![License](https://img.shields.io/github/license/pinax-network/taiss-substreams)

> [`Substreams`](https://substreams.streamingfast.io) for TAISS project

## Graph

```mermaid
graph TD;
  prom_out[map: prom_out]
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> prom_out
```

## Related

- [TAISS Smart Contract](https://github.com/pinax-network/taiss-contract)

## References

- [Substreams](https://substreams.streamingfast.io)
- [Prometheus](https://prometheus.io)
  - [Substreams Prometheus](https://github.com/pinax-network/substreams-sink-prometheus)
  - [Substreams Prometheus Rust](https://github.com/pinax-network/substreams-sink-prometheus.rs)

## Features

- [Prometheus `GAUGE`](https://prometheus.io/docs/concepts/metric_types/#gauge)
  - [ ] temperature
