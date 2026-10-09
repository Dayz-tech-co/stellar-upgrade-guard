# Security Policy

## Supported Versions

This project has not made a stable release yet. Security fixes are handled on the default branch until the first release line exists.

| Version | Supported |
| --- | --- |
| `main` before `0.1.0` | Best effort |
| Released versions | Not yet available |

## Reporting A Vulnerability

Do not open a public issue for a real vulnerability. Use GitHub private vulnerability reporting if it is enabled for the repository, or contact the repository owner privately through GitHub.

Before the first public release, maintainers should confirm that GitHub private vulnerability reporting is enabled or that another private reporting path is documented here.

Reports should include:

- affected commit or version;
- operating system and command used, if relevant;
- whether untrusted WASM, RPC data, or GitHub Action inputs are involved;
- a minimal reproduction when safe to share;
- logs with secrets removed.

Do not include private keys, seed phrases, API keys, RPC authorization headers, or production contract secrets.

## What Counts As Security-Relevant

Examples:

- parser crashes or excessive resource use on crafted WASM;
- incorrect handling of untrusted RPC responses;
- leaking RPC credentials, GitHub secrets, Stellar private keys, or environment variables;
- GitHub Action behavior that exposes sensitive inputs;
- dependency vulnerabilities affecting normal use;
- denial-of-service cases caused by unexpectedly large or malformed inputs.

Compatibility classification disagreements are usually product bugs unless they cause a security-sensitive false claim or secret exposure.

## RPC And Secret Handling

Users may pass RPC URLs or provider credentials in CI. The CLI and action must avoid printing authorization headers, private keys, API keys, and secrets. If an RPC URL contains credentials or query tokens, do not echo it verbatim in logs.

## Untrusted WASM

Stellar Upgrade Guard parses WASM metadata. Treat WASM files as untrusted input. Bugs that cause panics, uncontrolled memory use, or long-running parsing on crafted files are valid security concerns.

## GitHub Action Security

The action should use least-privilege workflow permissions and should not require write permissions. Consumer workflows should pass sensitive RPC configuration through GitHub secrets and avoid printing command invocations containing credentials.

## Dependencies

Dependency vulnerabilities should be reported privately when they affect the project in a practical way. Routine dependency updates can be filed as normal issues or pull requests.

## Disclosure

The project is maintainer-led and cannot guarantee a fixed response time. Maintainers will coordinate remediation and public disclosure based on severity, available fixes, and reporter needs.
