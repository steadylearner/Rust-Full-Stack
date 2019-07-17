#!/bin/bash

set -e #$help set

# build frontend assets and put them in a place the Rocket server
# expects

echo "install NPM pacakages for Browserify to work"
pushd static #$help pushd
yarn
popd #$help popd
echo "ready to use Browserify to use JavaScript modules in Rust frontend"

echo "install NPM packages to use live Rust yew editor"

yarn

echo "install cargo-web and set Rust environment to nightly to use it"

rustup default nightly
cargo install cargo-web

echo "https://www.steadylearner.com/blog/read/How-to-use-Rust-Yew for more information."

echo "ready to use rollup to develop Rust Yew frontend"

echo "start a Rust yew live editor"

yarn watch:rs


