# Scroll Rust Engineer Take-Home Assignments

This repo contains two take-home assignments for the Senior Rust Engineer role at Scroll.

## Assignments

**Assignment 1 ([`scroll-revm`](scroll-revm)):** Building on [revm](https://github.com/bluealloy/revm), create an EVM variant with a custom precompile. This custom precompile takes an address as its input, then reads the account's contract bytecode and returns the number of zero bytes in it. Add this custom precompile by implementing [`PrecompileCountZeroBytes`](scroll-revm/src/main.rs#L15) and modifying [`create_evm`](scroll-revm/src/main.rs#L22). For details on revm, please refer to the [revm book](https://bluealloy.github.io/revm/) and the [docs](https://docs.rs/revm/latest/revm/).

**Assignment 2 ([`exex-scroll-indexer`](exex-scroll-indexer)):** Building on [reth](https://github.com/paradigmxyz/reth), implement an [Executable Extension](https://www.paradigm.xyz/2024/05/reth-exex) (ExEx) that indexes all Scroll batches into a local database. Your indexer should process two events: [`CommitBatch`](https://github.com/scroll-tech/scroll/blob/6b11e20ca66f755b0cc9a0ef38c7ff17243af4df/contracts/src/L1/rollup/IScrollChain.sol#L15) and [`RevertBatch`](https://github.com/scroll-tech/scroll/blob/6b11e20ca66f755b0cc9a0ef38c7ff17243af4df/contracts/src/L1/rollup/IScrollChain.sol#L20). Implement the indexer by completing the [`scroll_indexer_exex`](exex-scroll-indexer/src/main.rs#L35) function. Feel free to refer to reth's [ExEx examples](https://github.com/paradigmxyz/reth/tree/main/examples/exex).

## How to Submit?

1. Clone this repo.
2. Modify [`scroll-revm/src/main.rs`](scroll-revm/src/main.rs) and [`exex-scroll-indexer/src/main.rs`](exex-scroll-indexer/src/main.rs).
3. Commit your changes.
4. Create a private repo on GitHub. (**Please do not push your solution to a public repo!**)
5. Push your changes.
6. Add [Thegaram](https://github.com/Thegaram) as an Admin to the repo.

## Evaluation Criteria

- Correctness.
- Readability, clean code, maintainability.
- Test coverage (if applicable).
- Performance (if applicable).
