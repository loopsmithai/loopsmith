# GitHub App Authentication Specification

## Purpose

`smith-agent` authenticates to GitHub as a GitHub App: it mints a short-lived JWT from the App's
private key, exchanges it for an installation access token, and uses that token for API calls. This
spec defines the auth flow and its security invariants.

## Requirements

### Requirement: JWT minting

The system SHALL generate a JWT signed with RS256 from the App `client_id` (as issuer) and the App's
private-key PEM, with an issued-at backdated slightly and a short expiry (≈10 minutes).

#### Scenario: Mint a JWT
- GIVEN a valid App `client_id` and private-key PEM
- WHEN the system generates a JWT
- THEN the JWT is RS256-signed with the `client_id` as issuer and a near-term expiry

### Requirement: Installation token exchange

The system SHALL exchange the JWT for an installation access token via
`POST /app/installations/{installation_id}/access_tokens`. Installation tokens expire after ≈1 hour.

#### Scenario: Exchange JWT for installation token
- GIVEN a valid JWT and installation id
- WHEN the system requests an installation token
- THEN it receives a token and an expiry timestamp ≈1 hour out

### Requirement: Uninstall

The system SHALL uninstall an App installation via `DELETE /app/installations/{installation_id}`
authenticated with the App JWT.

### Requirement: Secret redaction in status output

`smith-agent github auth-status` SHALL report a boolean `has_private_key`, never the private-key
material itself.

#### Scenario: auth-status hides key material
- GIVEN stored App credentials including a private key
- WHEN `smith-agent github --org acme auth-status` runs
- THEN the `result` contains `has_private_key: true`
- AND the `result` contains no private-key bytes

## Invariants

- Installation tokens MUST NOT be validated via the GitHub `/user` endpoint — GitHub returns 403 for
  installation tokens. (Validation, if needed, uses an installation-scoped endpoint.)
- A JWT is short-lived (≈10 min); an installation token is ≈1 hour. Neither is persisted beyond its
  use.

## Limitations & Known Issues

### Known gaps & accepted constraints
- Beyond `auth-status` redaction and keyring storage, there is no *verified* enforcement that secret
  material (private-key PEM, installation tokens) is kept out of stderr/logs. This is unverified, not
  a guaranteed contract.
