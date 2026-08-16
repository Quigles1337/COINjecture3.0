# `main` branch protection contract

P-001 documents the protection contract so that repository settings can be read back
and compared mechanically. Applying a setting is not claimed by this document.

## Required settings

- Require a pull request before merging to `main`.
- Require branches to be up to date before merging.
- Require every `D6 CI` job below to succeed:
  - `format`
  - `clippy`
  - `dependency-policy`
  - `dependency-audit`
  - `unsafe-audit`
  - `unit-and-property-tests`
  - `conservation-invariant`
  - `lean-conformance`
  - `codec-fuzz-smoke`
  - `genesis-spend-test`
  - `locked-build`
- Dismiss stale approvals when new commits are pushed.
- Require conversation resolution when review conversations exist.
- Apply protections to administrators.
- Disallow force pushes and branch deletion.
- Allow no bypass actor for a red required check.

AUTO-lane packets may merge after their mandatory adversary pass and D17 re-check;
HUMAN-lane packets still stop for the governing human decision. No status name may be
removed from this contract merely to make a red pipeline mergeable.

## Verification command

Once settings are applied, read them back without mutation:

```text
gh api repos/Quigles1337/COINjecture3.0/branches/main/protection
```

The response URL or a captured artifact is required before any report claims that
protection is active.
