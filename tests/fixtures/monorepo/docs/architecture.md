# Architecture

## Overview

This monorepo contains three components:

- **backend** — Rust HTTP API
- **frontend** — TypeScript/React SPA
- **infra** — deployment scripts and Docker config

## Data flow

```text
browser -> frontend -> backend -> database
```

## Notes

- Backend exposes REST endpoints on port 8080
- Frontend proxies API calls via Next.js rewrites
