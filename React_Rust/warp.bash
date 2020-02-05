#!/bin/bash

# Webpack dev server is based on Express. The structure of Acti is similar to the Express project to simplify the test.

echo "[Web]"
yarn build

cp -R src/images/* server/actix/public/src/images
cp -R dist/* server/actix/public

(
  echo "[Server]"
  cd server/warp
  cargo run --release
)


