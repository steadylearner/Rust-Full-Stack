#!/bin/bash

# Webpack dev server is based on Express. The structure of Rocket is similar to the Express project to simplify the test.

echo "[Web]"
yarn build

cp -R src/images/* server/python/public/src/images
cp -R dist/* server/python/public

(
  echo "[Server]"
  cd server/python
  source bin/activate
  python Vibora/main.py
)


