#!/bin/bash

set -e #$help set

# build frontend assets and put them in a place the Rocket server
# expects


echo "building web"
pushd web #$help pushd
yarn build
popd #$help popd
echo "web build complete"

cp web/target/wasm32-unknown-unknown/release/index.js server/web/index.js
cp web/target/wasm32-unknown-unknown/release/index.wasm server/web/index.wasm
cp web/static/index.html server/web/index.html
cp web/static/index.css server/web/index.css
cp web/static/favicon.ico server/web/favicon.ico

cp web/static/normalize.css server/web/normalize.css
cp web/static/steadylearner.css server/web/steadylearner.css
cp web/static/markdown.css server/web/markdown.css
cp web/static/modal.css server/web/modal.css

cp web/static/bundle.js server/web/bundle.js
cp -R web/static/node_modules server/web/node_moduels

(
  echo "running server"
  cd server
  cargo run --release
)


