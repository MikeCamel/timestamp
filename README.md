# timestamp
Simple timestamp

Dependencies:
 - openssl-devel
 - perl-IPC-Cmd

Currently takes a message on `stdin`, and hashes it outputs the following to `stdout`:
- Input
- Hash + timestampit to stdout with a timestamp.

TODO:
- add hashing (DONE)
- add key signing
- consider implementing https://www.ietf.org/rfc/rfc3161.txt
