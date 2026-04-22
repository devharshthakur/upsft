module.exports = {
  "*.{md,json,yaml,yml}": ["prettier --write"],
  "*.rs": [
    () => "cargo fmt",
    () => "cargo clippy --fix --all-targets --all-features",
  ],
};
