module.exports = {
  '*.{md,json,yaml,yml}': ['prettier --write'],
  '*.rs': [
    (files) => `cargo fmt --check -- ${files.join(' ')}`,
    () => 'cargo clippy --all-targets --all-features -- -D warnings',
  ],
};
