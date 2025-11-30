#!/bin/bash

git add .
git commit -m "chore: update trnovel"
npm version patch

git push origin main --tags