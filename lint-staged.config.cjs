module.exports = {
  "*.{md,json,yaml,yml}": ["prettier --write --list-different"],
  "*.rs": [
    () => "cargo fmt",
    () => "cargo clippy --fix --allow-dirty --all-targets --all-features",
  ],
};
