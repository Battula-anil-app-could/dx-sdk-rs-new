#!/bin/sh

# helper script for checking that all contracts and debug projects compile

### BUILD ALL CONTRACTS ###

moapy --verbose contract build "contracts/benchmarks/str-repeat"
moapy --verbose contract build "contracts/examples/adder"
moapy --verbose contract build "contracts/examples/crowdfunding-moax"
moapy --verbose contract build "contracts/examples/crowdfunding-erc20"
moapy --verbose contract build "contracts/examples/crowdfunding-dct"
moapy --verbose contract build "contracts/examples/crypto-bubbles"
moapy --verbose contract build "contracts/examples/factorial"
moapy --verbose contract build "contracts/examples/lottery-moax"
moapy --verbose contract build "contracts/examples/lottery-erc20"
moapy --verbose contract build "contracts/examples/lottery-dct"
moapy --verbose contract build "contracts/examples/multisig"
moapy --verbose contract build "contracts/examples/non-fungible-tokens"
moapy --verbose contract build "contracts/examples/simple-erc20"
moapy --verbose contract build "contracts/feature-tests/abi-tester"
moapy --verbose contract build "contracts/feature-tests/basic-features"
moapy --verbose contract build "contracts/feature-tests/async/async-alice"
moapy --verbose contract build "contracts/feature-tests/async/async-bob"
moapy --verbose contract build "contracts/feature-tests/use-module"

### CREATE ALL ABIs ###

./abi.sh "contracts/examples/adder"
./abi.sh "contracts/examples/crowdfunding-moax"
./abi.sh "contracts/examples/crowdfunding-erc20"
./abi.sh "contracts/examples/crowdfunding-dct"
./abi.sh "contracts/examples/crypto-bubbles"
./abi.sh "contracts/examples/factorial"
./abi.sh "contracts/examples/lottery-moax"
./abi.sh "contracts/examples/lottery-erc20"
./abi.sh "contracts/examples/lottery-dct"
./abi.sh "contracts/examples/multisig"
./abi.sh "contracts/examples/non-fungible-tokens"
./abi.sh "contracts/examples/simple-erc20"
./abi.sh "contracts/feature-tests/abi-tester"
./abi.sh "contracts/feature-tests/basic-features"
./abi.sh "contracts/feature-tests/use-module"
