# Credential Store Specification

## Purpose

`smith` and `smith-agent` persist GitHub App credentials and installation tokens in the operating
system's Secret Service keyring. This spec defines the service-naming scheme, the keys, the legacy
fallback chain, and the relevant overrides.

## Requirements

### Requirement: Credential domains and service names

The system SHALL store credentials under two domains, each with its own keyring service name:

| Domain | Service name | Holds |
|--------|--------------|-------|
| App credentials | `loopsmith.app.{agent_id}` | App id, client id, private key |
| Installation token | `loopsmith.{org}.installation` | Installation id / token material |

### Requirement: Credential keys

Within a service, credentials SHALL be keyed as `{member}/github-app-id`,
`{member}/github-app-client-id`, `{member}/github-app-private-key`, and
`{member}/github-installation-id`.

### Requirement: Legacy fallback chain (backward compatibility)

When the installation-token service has no stored installation id, the system SHALL probe two legacy
service names in order before giving up:

1. `loopsmith.{org}.github-app` (pre-split format)
2. `botminter.{org}.github-app` (oldest format)

If a legacy service holds the credentials, the system SHALL use it. This preserves access to
credentials written by earlier versions and the original BotMinter tooling.

#### Scenario: Read credentials written by an older version
- GIVEN installation credentials stored only under `botminter.acme.github-app`
- WHEN a new `smith-agent --org acme` operation looks up the installation token
- THEN the new service is checked first, then `loopsmith.acme.github-app`, then `botminter.acme.github-app`
- AND the credentials are found via the oldest-format fallback

### Requirement: D-Bus override

When `SMITH_KEYRING_DBUS` is set, the system SHALL use it as the D-Bus session bus address for
keyring access.

## Limitations & Known Issues

### Known gaps & accepted constraints
- The installation-store fallback **probe key is hardcoded** to the literal
  `smith/github-installation-id`, rather than being parameterized by `--id`/`agent_id` like the key
  builder (`{member}/...`). For non-default agent identities the legacy probe would not match. This
  inconsistency is acknowledged, not a desired contract.
- **App-to-installation relationship is not modeled.** The App credentials (`loopsmith.app.{id}`)
  and the per-org installation credentials (`loopsmith.{org}.installation`) are stored in two
  unlinked keyring services. The correlation is purely conventional (one App per `--id`). If
  `smith init` is run twice under the same `--id`, the second App's credentials silently overwrite
  the first, orphaning all existing installations — they hold installation IDs that belong to the
  old App, and the new App's JWT will fail to exchange tokens for them. No error until runtime.
  The fix requires either storing the App ID alongside each installation, or keying the App store
  by App ID rather than agent identity.
- **`smith init` hardcodes `--id` to `"smith"`.** Unlike `smith-agent` and `smith install`, which
  accept `--id` with a default of `"smith"`, `smith init` has no `--id` flag and writes credentials
  under the literal key prefix `smith/`. Multi-agent setups on one machine would require `init` to
  accept `--id`.
