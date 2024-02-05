#!/bin/sh

# cleans all wasm targets

moapy --verbose contract clean "contracts/benchmarks/str-repeat"
moapy --verbose contract clean "contracts/examples/adder"
moapy --verbose contract clean "contracts/examples/crowdfunding-moax"
moapy --verbose contract clean "contracts/examples/crowdfunding-erc20"
moapy --verbose contract clean "contracts/examples/crowdfunding-dct"
moapy --verbose contract clean "contracts/examples/crypto-bubbles"
moapy --verbose contract clean "contracts/examples/factorial"
moapy --verbose contract clean "contracts/examples/lottery-moax"
moapy --verbose contract clean "contracts/examples/lottery-erc20"
moapy --verbose contract clean "contracts/examples/multisig"
moapy --verbose contract clean "contracts/examples/simple-erc20"
moapy --verbose contract clean "contracts/feature-tests/basic-features"
moapy --verbose contract clean "contracts/feature-tests/async/async-alice"
moapy --verbose contract clean "contracts/feature-tests/async/async-bob"
moapy --verbose contract clean "contracts/feature-tests/use-module"
