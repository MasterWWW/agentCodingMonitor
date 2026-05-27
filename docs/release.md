# Release

## Tags

Push a tag `v*` to trigger GitHub Actions build artifacts for macOS and Windows.

## Code signing (optional)

- **macOS**: notarize with Developer ID for distribution outside Gatekeeper warnings.
- **Windows**: Authenticode signing to reduce SmartScreen prompts.

v0.1.0 releases may ship unsigned; see README for local build instructions.
