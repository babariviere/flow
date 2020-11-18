#!/bin/sh

# TODO: check if version is correct

[ ! $(command -v toml) ] && cargo install toml-cli

toml set Cargo.toml package.version "$1" > /tmp/flow.Cargo.toml
mv /tmp/flow.Cargo.toml Cargo.toml

git add Cargo.toml
git commit -m "release: version $1"

git tag "v$1"
git push --tags
