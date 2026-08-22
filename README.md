# Rust Template

This repo contains a standard rust app template. It imports some opinionated
libraries that you might not need, most notably `tokio` as an async runtime and
`clap` to parse command-line arguments.

## Getting started

```sh
./scripts/init.sh ../my-service
```

This copies the tracked files into the target directory and renames the
template to the project name (the basename of the path, or an optional second
argument). The directory must be empty, though an existing `.git` is fine, so
it can populate a repo you have already created or cloned.

Creating the repo and the first commit is left to you:

```sh
cd ../my-service && git init && git add -A && git commit -m "chore: init"
```

## Development

- `./scripts/validate.sh` — clippy, tests, unused deps, formatting.
- `./scripts/fix.sh` — apply formatting & drop unused deps.

Both take `-n`/`--nix` to run the underlying tools via `nix develop`.

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
