# Streamlog

A streaming log ingestion and processing pipeline written in Rust.

## Goal
Read log lines from stdin or a file, process them in a streaming fashion,
and produce aggregated statistics.

## Constraints
- Must process input line-by-line (no loading whole file)
- Must be memory efficient
- Must handle malformed input gracefully

## Fututre goals
- Networking
- Async
- Multithreading
- Persistent storage

## Current Phase
Phase - 1: Single-threaded, correct implementation

## Log format
<TIMESTAMP> <IP> <METHOD> <PATH> <STATUS> <LATENCY_MS>
