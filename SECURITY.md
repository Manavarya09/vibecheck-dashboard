# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability in VibeCheck, please report it responsibly.

**Email:** security@vibecheck.dev (or open a private GitHub advisory)

**Do not** open a public issue for security vulnerabilities.

## Scope

VibeCheck processes behavioral metadata (timestamps, app names, window titles). It does not capture code content, prompts, or file contents. All data is stored locally in SQLite.

## Data Handling

- All data stored locally at `~/Library/Application Support/com.vibecheck.dashboard/`
- No network requests made by default
- No telemetry or analytics
- Window titles are only used for classification, never transmitted

## Supported Versions

| Version | Supported |
| ------- | --------- |
| 0.1.x   | Yes       |
