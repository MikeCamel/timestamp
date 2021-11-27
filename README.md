# timestamp
Simple timestamp with signature.  Intended as a test workload for Enarx.

Currently takes a message on `stdin`, and hashes it outputs the following to `stdout`:
- Input
- public key
- Hash + timestamp to stdout with a signature.

TODO:
- add hashing (DONE)
- add key signing (DONE - doesn't currently compile to wasm32-wasi cargo target)
- add sntp
- consider implementing https://www.ietf.org/rfc/rfc3161.txt
