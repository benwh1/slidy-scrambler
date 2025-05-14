#!/bin/bash
wasm-pack build --target web;
cp static/* deploy;
cp pkg/slidy_scrambler.js pkg/slidy_scrambler_bg.wasm deploy;
