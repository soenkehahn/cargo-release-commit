# `cargo-release-commit`

Small script that:

- clones the repo of the crate you're in into a temporary directory,
- checks out a given revision,
- makes sure that given revision is merged into master,
- runs `cargo publish --dry-run`,
- asks for confirmation to continue,
- runs `cargo publish` and
- runs `git tag v$VERSION` and `git push origin v$VERSION`.
