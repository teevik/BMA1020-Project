run-native:
  cargo run

run-web:
  trunk serve --release

build-web:
  trunk build --release

deploy-web:
  git subtree push --prefix dist origin gh-pages