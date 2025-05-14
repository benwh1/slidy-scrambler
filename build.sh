#!/bin/bash
wasm-pack build --target web;
rm -r deploy;
cp -r static deploy;
cp pkg/slidy_scrambler.js pkg/slidy_scrambler_bg.wasm deploy;
