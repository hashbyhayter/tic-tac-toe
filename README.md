# tic-tac-toe
Simple Rust implementation of tic tac toe using web assembly.

## Setup

First install npm, rustup, and wasm-pack on your development enviroment then follow steps below: 

1. Run `wasm-pack build` in the root folder
1. Run `npm install` in _tic-tac-toe/www_
1. Run `npm link` in _tic-tac-toe/pkg_
1. Run `npm link tic-tac-toe` in _tic-tac-toe/www_
1. Run `npm start` to run locally
