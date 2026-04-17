module.exports = {
  "*.{md,json,yaml,yml}": ["prettier --write"],
  "*.rs": [
    () => "cargo fmt",
    (files) => `cargo fmt --check -- ${files.join(" ")}`,
    () => "cargo clippy --all-targets --all-features",
  ],
};
