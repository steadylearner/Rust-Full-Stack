#!/bin/bash

# Webpack dev server is based on Express. Then, use it to test the single page app first.

echo "[Web]"
yarn build

cp -R src/images/* server/express/public/src/images
cp -R dist/* server/express/public

(
  echo "[Server]"
  cd server/express
  yarn
  yarn serve
)


