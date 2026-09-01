# Security Policy

## Supported versions

Security fixes are accepted against the latest published `0.1.x` release line of
this repository's crates (`tag`, `tag-app`).

## Reporting a vulnerability

Please **do not** open a public GitHub issue for security-sensitive reports.

Prefer one of the following:

1. **GitHub Security Advisories** — use
   [Report a vulnerability](https://github.com/unified-field-dev/tag/security/advisories/new)
   on this repository when available.
2. Contact the maintainers privately via the repository owner listed at
   https://github.com/unified-field-dev/tag.

Include:

- a description of the issue and its impact
- steps to reproduce or a proof of concept when possible
- affected crate names and versions

We will acknowledge receipt as soon as practical and coordinate a fix and
disclosure timeline with you.

## Scope

In scope: vulnerabilities in this repository's published crates and
documentation that could cause unsafe production defaults, plus CI/supply-chain
issues in this repository.

Out of scope: vulnerabilities solely in third-party dependencies unless this
project mishandles them in a security-relevant way.

## Catalog notes

- Catalog mutations run under Valence ownership privacy; non-owners cannot update
  or delete another user's tag.
- `tag_history` create/delete are `SYSTEM_ONLY`. `TagHistoryWriter` appends under
  a System actor and stores the request actor on the history row. Authenticated
  actors may still **read** shared catalog history (detail `HistoryTimeline`).
- `tag-app` server functions require a session (`require_session` / verified
  guard). Treat missing auth as deny. Catalog search clamps `limit` to `1..=50`.
- Product ManyToMany attachment edges live on consuming schemas — review those
  product policies separately from this catalog crate.
