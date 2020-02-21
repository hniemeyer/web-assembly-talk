#!/bin/bash
wasm-pack build
cd www/ || exit
npm install
npm run start
