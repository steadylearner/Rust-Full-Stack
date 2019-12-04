#!/bin/bash

(
  echo "[Server]"
  cd server/python
  source bin/activate
  python Django/manage.py runserver 0.0.0.0:8000
)

