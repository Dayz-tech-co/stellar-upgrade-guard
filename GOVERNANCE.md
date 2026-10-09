# Governance

Stellar Upgrade Guard is currently a small maintainer-led project.

## Maintainer Model

The repository owner is responsible for final decisions about scope, releases, compatibility semantics, and repository administration. Contributors are welcome to propose changes, but no steering committee or foundation governance exists.

## Decision Making

Most decisions happen through issues and pull requests. The maintainer weighs:

- correctness and evidence;
- deterministic behavior;
- user impact;
- implementation complexity;
- maintenance cost;
- alignment with the roadmap.

## Compatibility Rule Changes

Compatibility rules are core product behavior. Changes require clear evidence from Soroban/Stellar semantics, XDR structures, SDK behavior, or well-documented fixtures. When evidence is incomplete, conservative `unknown` classifications are preferred.

## Architecture Changes

Changes to parser structure, normalized models, RPC behavior, CLI output contracts, or GitHub Action behavior should be proposed before large implementation work. Architecture changes should preserve local/offline tests wherever possible.

## Review Expectations

Reviews focus on correctness, determinism, test coverage, security hygiene, and whether docs match behavior. Maintainers may ask for smaller PRs when a change mixes unrelated concerns.

## Release Authority

The maintainer decides when a release is ready, updates changelog and version metadata, creates tags, and publishes GitHub releases or crates when that becomes part of the project process.

## Contributor Recognition

Contributors are recognized through Git history, release notes where relevant, and issue/PR discussion. The project does not currently maintain a separate credits file.

## Adding Maintainers

Future maintainers may be added after sustained, high-quality contributions and demonstrated care for the project's conservative compatibility model. Maintainer access should be granted gradually and reviewed periodically.

## Conflict Resolution

Technical disagreements should be resolved with evidence, fixtures, and small experiments. Conduct concerns should be handled privately where possible, using the Code of Conduct and GitHub reporting tools.
