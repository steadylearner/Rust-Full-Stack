#!/bin/bash

# Webpack dev server is based on Express. Then, use it to test the single page app first.

echo "[Web]"
yarn build

cp -R src/images/* server/restify/public/src/images
cp -R dist/* server/restify/public

(
  echo "[Server]"
  cd server/restify
  yarn
  yarn serve
)


