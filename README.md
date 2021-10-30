# confuse_server
Confuse bots by serving their traffic by this server

Currently this server replies with a `x-powered-by: <rce php version>`
  and `server: <rce apache version>` header, and a `<html><4kb of randomness></html> body after waiting 15 seconds.

if you know anything better to confuse, and potentially stop/rce bost, feel free to open an issue.
