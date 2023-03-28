# README Boilerplate

Going through the [fasterthanlime inuraddressspace tutorial](https://www.youtube.com/watch?v=xN5WjaeeklA)

## Table of Contents

- [Summary](#summary)
- [Order](#order)

## Summary

This is a monorepo which contains (mostly) rust projects

## Order

The projects were created in the following order:

- asm/nataliedotexe
- asm/cli (debugging asm/nataliedotexe)
- libinjection

### libinjection

`nataliedotexe` runs uses the dll built by `natalib` 
the run command lives in the Justfile. `choco install just; just`, or just `just`  to run