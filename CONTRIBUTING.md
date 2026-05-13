# Contributing to Stellar Lending Protocol

Thanks for your interest! This project runs on the **Stellar Wave** program via [Drips](https://drips.network/wave) — contributors earn on-chain rewards for merged PRs.

## Quick Start

```bash
git clone https://github.com/<your-org>/stellar-lending-protocol
cd stellar-lending-protocol
make build
make test
```

## How to Contribute

1. Browse issues labelled **`Stellar Wave`** — these are bounty-eligible.
2. Apply to an issue via the [Drips Wave app](https://drips.network/wave) or leave a comment on GitHub.
3. Wait to be assigned before opening a PR (one contributor per issue per Wave).
4. Open a PR against `develop`, referencing the issue (`Closes #N`).
5. Address review feedback; maintainer merges and marks the issue resolved.
6. Drips automatically calculates your share of the reward pool.

## Code Standards

- `cargo fmt` before committing
- `cargo clippy -- -D warnings` must pass
- Every new function needs at least one unit test
- No `unsafe` code without prior discussion in the issue

## Complexity Levels (Points)

| Label | Points | Examples |
|-------|--------|---------|
| Trivial | 100 | Typo fix, doc update, small refactor |
| Medium | 150 | Bug fix, new view function, test coverage |
| High | 200 | New feature, cross-contract integration, security fix |

## Commit Style

```
<type>(<scope>): <short description>

Types: feat, fix, docs, test, refactor, chore
Scope: lending-pool, oracle, token, liquidator, ci
```

## Questions?

Open a GitHub Discussion or ping us in the issue thread.
