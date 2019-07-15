# Electrust
Rust in Electron boilerplater

# Installation

## Requirements 
- [rust](https://www.rust-lang.org/tools/install)
- [nodejs](https://nodejs.org/en/download/)

## Build & Run

1- build your rust library:
```bash
cd lib 
cargo build
``` 
2- install all required npm packages:
```bash
cd main
npm install
```
3- rebuild electron files:
```bash
npm run rebuild
```
4- and finally! to start your app:
```bash
npm start
```
or
```bash
npm run start
```
