# timestamp
Simple timestamp with signature.  Intended as a test workload for Enarx.

Dependencies:
 - openssl-devel
 - perl-IPC-Cmd
 - perl-FindBin
 - perl-File-Copy
 - perl-File-Compare

Currently takes a message on `stdin`, and hashes it outputs the following to `stdout`:
- Input
- Hash + timestampit to stdout with a timestamp.

TODO:
- add hashing (DONE)
- add key signing (DONE - doesn't currently compile to wasm32-wasi cargo target)
- consider implementing https://www.ietf.org/rfc/rfc3161.txt
