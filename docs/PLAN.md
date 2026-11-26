# TOMACCO — Project Plan

> Your Browser. Your Rules. Your Weird Little World.

A small, personal, highly customizable desktop browser for Windows 10/11.
Built with Tauri 2 + WebView2 + Rust + vanilla HTML/CSS/JS, configured via
a JSON file ("Browser DNA"). Built in CI only (GitHub Actions) — Rule 11.

## Constraints

- 2–4 week timeline, one developer + AI pair, zero budget.
- Prefer existing open-source libraries; no reinventing wheels.
- All compiled artifacts are produced by GitHub Actions (no local Rust env).
- Progress report submitted at the end of every phase using the fixed template.

## Features

1. **Custom Interface** — user-shaped UI driven by Browser DNA.
2. **Smart Spaces** — grouped contexts with their own tabs and themes.
3. **Command Center** — quick command palette (Ctrl+K style).
4. **Flexible View** — tab layout flexibility (single / split).
5. **Dramatic Loading** — fun loading messages while pages load.
6. **Optional Profile Password** — Argon2id-protected profile unlock.
7. **Panic Button** — one-click switch to the Safe Profile.

## Phases

### Phase 0 — Research & spike (done)

- Environment audit: Node 24 / npm / git available; Rust not installed locally (not a blocker, CI builds).
- Multi-webview in Tauri 2 requires the `unstable` feature; known bug tauri#10011.
- Profile isolation via `WebviewWindowBuilder::data_directory()` — cookies/storage per profile. Feasibility confirmed: Panic Button + password protection are viable.
- Risk identified: UDF lock — all webviews of the current profile must be closed before switching.
- Architecture decision: Plan A (multi-webview + `unstable`, 3-day spike) with fallback Plan B (one window per tab); iframe approach rejected.
- Reference projects: Bushido, Cloaksession. Key deps: tauri-plugin-store/sql/global-shortcut, argon2 crate, tauri-action.

### Phase 1a — Skeleton (done)

- CI workflow (windows-latest, Rust cache, icon decoded from base64, `npx tauri build`, NSIS artifact upload).
- Tauri config: 1200x800 main window, NSIS bundle, capabilities.
- Welcome UI with midnight theme and pure-CSS tomato logo.
- Browser DNA v1 structure skeleton (`dna/default.json`).

### Phase 1b — Browser Core (next)

- Spike: verify multi-webview + dual `data_directory` isolation (3 days max, then decide Plan A vs Plan B).
- Tabs, address bar, back/forward/refresh, search integration.
- WebView Controller module in Rust; UI chrome in vanilla JS.

### Phase 2 — Spaces & profiles

- Smart Spaces UI + persistence via tauri-plugin-store.
- Per-profile `data_directory` isolation; Panic Button switching flow.

### Phase 3 — Command Center & password

- Command palette (Ctrl+K) with fuzzy matching.
- Optional profile password via Argon2 (argon2 crate); locked/unlocked states.

### Phase 4 — Polish & release

- Dramatic loading messages, theming polish, DNA editor.
- NSIS installer released via GitHub Releases (tauri-action).
