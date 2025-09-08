# Troj-n-Horse
This Rust project combines a simple CLI calculator with a background thread that collects system info (OS, architecture, username, hostname) and sends it to a remote server using reqwest. It also includes an Actix Web server that listens for incoming system data at /collect and logs it for verification.
