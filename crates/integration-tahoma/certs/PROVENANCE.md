# TaHoma trust and upstream application constants

## Manufacturer trust root

`overkiz-root-ca-2048.pem` was retrieved on October 23, 2026 over validated
HTTPS from [Overkiz](https://ca.overkiz.com/overkiz-root-ca-2048.crt).
It is a public CA certificate; no private key is included. Configure its path
as `ca_file` for local mode. The client uses only the configured CA, verifies
the hostname, and never downloads a replacement certificate at runtime.

- Subject/issuer: `O=Overkiz, OU=Root CA, CN=Overkiz Root CA, C=FR`.
- Validity: April 27, 2016 through April 27, 2036; RSA 2048, SHA-256.
- Certificate SHA-256: `BB:FA:96:11:1E:AE:15:E3:C6:ED:15:F3:43:27:6C:0C:27:26:03:2E:A1:9D:F5:BB:D0:67:E2:F1:CF:FC:44:DB`.
- PEM artifact SHA-256: `5b4dbce7322da0718235d8d54149a3665e37822224b6a8801447f6d4d72ded15`.

The CA is distributed by Overkiz for gateway trust. No separate certificate
license was supplied by its download endpoint. This records its origin rather
than asserting ownership of the certificate. Real gateway certificate
compatibility with strict Rustls verification remains untested.

## SomfyEurope OAuth constants

The public mobile application ID and application secret in `src/client.rs`
come from [PyOverkiz const.py at commit 7b5a0034](https://github.com/imicknl/python-overkiz-api/blob/7b5a0034e550e4ae27536c4d8d1a073f796d733e/pyoverkiz/const.py).
Upstream identifies these constants as public values embedded in mobile
applications. They are distinct from a household username/password, gateway
selection, access token, and refresh token. These private values are never
included in the repository or public diagnostics.

The authentication strategy and fixed provider endpoints follow that same
commit, as linked in [the P11 design](../../../docs/integrations/tahoma-client-design.md).
The copied constants retain the following upstream notice:

MIT License

Copyright (c) 2020 Mick Vleeshouwer

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## Fictional TLS fixtures

`tests/fixtures/tls/ca.pem`, `server.der` and `server-key.der` were generated
for this ticket with Python cryptography 49.0.0. They contain no gateway or
account data. The test-only key is deliberately public and must never be used
outside loopback tests. The leaf SANs are the fictional configured gateway DNS,
`account.test`, and `api.test`; both certificates expire January 1, 2035.
Tests inject DNS resolution to loopback and use their own root. The fixture
root private key was discarded. No real mDNS, household HTTPS or Somfy login
is needed by automated tests.
