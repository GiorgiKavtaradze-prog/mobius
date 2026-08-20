# Security Policy

## Supported Versions

The following Mobius release lines currently receive security attention:

| Version | Support status |
| :------ | :------------- |
| 0.5.x   | Active support |
| 0.4.x   | Security fixes only, best effort |
| < 0.4   | Unsupported |

Use the latest stable release whenever possible so you receive the most recent
security fixes.

## Reporting a Vulnerability

Please do not open a public GitHub issue for security vulnerabilities.

Report vulnerabilities privately through GitHub Security Advisories:

- [Open a private vulnerability report](https://github.com/GiorgiKavtaradze-prog/mobius/security/advisories/new)

If GitHub private reporting is unavailable, contact the maintainer through the
GitHub profile and request a private disclosure channel without sharing exploit
details publicly.

Include as much of the following information as you can:

- A concise description of the vulnerability.
- Affected Mobius versions, commits, platforms, and configuration.
- Minimal reproduction steps or a proof of concept.
- Expected and observed behavior.
- Impact analysis, such as code execution, privilege escalation, information
  disclosure, denial of service, or sandbox escape.
- Suggested remediation, if you already have one.

## Response Timeline

| Stage | Target timeframe | What happens |
| :---- | :--------------- | :----------- |
| Acknowledgment | Within 48 hours | Maintainers confirm receipt of the report. |
| Triage | Within 5 business days | Maintainers assess severity, impact, and affected versions. |
| Fix and release | Within 30 days when practical | Maintainers prepare, test, and publish a patched release. |
| Disclosure | After a fix is available | Maintainers publish an advisory and credit the reporter unless anonymity is requested. |

Complex vulnerabilities may require more time. In those cases, maintainers will
share progress updates through the private reporting channel.

## Security Considerations

Mobius is a terminal emulator that parses untrusted PTY output and renders GPU
content. Security-sensitive areas include:

- RGP escape-sequence parsing and inline object lifecycle management.
- OBJ, GLB, and STL asset loading from local paths or embedded payloads.
- Chunked base64 payload accumulation and temporary asset handling.
- Clipboard copy/paste and bracketed paste sanitization.
- Shell spawning and configuration-driven path resolution.
- GPU backend and driver interaction through wgpu.

Run untrusted terminal applications, remote shells, and downloaded model assets
with the same caution you would use in any terminal emulator.

## Disclosure Policy

- Maintainers will not intentionally disclose a vulnerability publicly before a
  fix is available.
- Reporters should not publicly disclose the issue until a fix and advisory have
  been published.
- Maintainers will credit the reporter unless they request anonymity.

## Hall of Fame

Security researchers and community members who report valid, previously unknown
vulnerabilities may be acknowledged here with their consent.

## Contact

- Maintainer: Giorgi Kavtaradze
- GitHub: [@GiorgiKavtaradze-prog](https://github.com/GiorgiKavtaradze-prog)
