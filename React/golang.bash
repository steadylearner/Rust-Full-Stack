#!/bin/bash

# Webpack dev server is based on Express. The structure of Rocket is similar to the Express project to simplify the test.

echo "[Web]"
yarn build

cp -R src/images/* server/golang/public/src/images
cp -R dist/* server/golang/public

(
  echo "[Server]"
  cd server/golang
  go run index.go
)


