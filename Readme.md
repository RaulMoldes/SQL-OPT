# ANSI-Compatible SQL Parser and Optimizer

This project implements a modular **SQL parser and optimizer** designed to comply with the ANSI SQL standard. It is written in Rust and designed to be extensible, efficient, and expressive. The parser builds an **Abstract Syntax Tree (AST)** from raw SQL input and provides a visitor-based architecture for query transformation and optimization.

## Parsing Architecture Overview

The parser is based on the **Pratt parsing algorithm** (also known as *Top-Down Operator Precedence Parsing*).
This method offers a simple yet powerful way to parse expressions with varying operator precedence without needing a traditional grammar or parser generator.
