#!/bin/sh

# copies wasm & denali files to the Arwen test folder
# expects 1 argument: the path to the Arwen repo root

ARWEN_PATH=$1

# building all contracts takes a lot of time, here are just the ones for Arwen:
moapy --verbose contract build ./contracts/examples/adder || return 1
moapy --verbose contract build ./contracts/examples/crowdfunding-dct || return 1
moapy --verbose contract build ./contracts/examples/ping-pong-moax || return 1
moapy --verbose contract build ./contracts/examples/multisig || return 1
moapy --verbose contract build ./contracts/examples/moax-dct-swap || return 1
moapy --verbose contract build ./contracts/examples/erc20 || return 1
moapy --verbose contract build ./contracts/feature-tests/basic-features || return 1
moapy --verbose contract build ./contracts/feature-tests/payable-features || return 1
moapy --verbose contract build ./contracts/feature-tests/async/async-alice || return 1
moapy --verbose contract build ./contracts/feature-tests/async/async-bob || return 1
moapy --verbose contract build ./contracts/feature-tests/async/forwarder || return 1
moapy --verbose contract build ./contracts/feature-tests/async/forwarder-raw || return 1
moapy --verbose contract build ./contracts/feature-tests/async/recursive-caller || return 1
moapy --verbose contract build ./contracts/feature-tests/async/vault || return 1

# if you still want to build all:
# ./build-wasm.sh


# copying the files to arwen here:
cp contracts/examples/adder/output/adder.wasm \
   $ARWEN_PATH/test/adder/output/adder.wasm
cp -R contracts/examples/adder/denali \
   $ARWEN_PATH/test/adder

cp contracts/examples/crowdfunding-dct/output/crowdfunding-dct.wasm \
   $ARWEN_PATH/test/crowdfunding-dct/output/crowdfunding-dct.wasm
cp -R contracts/examples/crowdfunding-dct/denali \
   $ARWEN_PATH/test/crowdfunding-dct

cp contracts/examples/ping-pong-moax/output/ping-pong-moax.wasm \
   $ARWEN_PATH/test/ping-pong-moax/output/ping-pong-moax.wasm
cp -R contracts/examples/ping-pong-moax/denali \
   $ARWEN_PATH/test/ping-pong-moax

cp contracts/examples/multisig/output/multisig.wasm \
   $ARWEN_PATH/test/multisig/output/multisig.wasm
cp -R contracts/examples/multisig/denali \
   $ARWEN_PATH/test/multisig
cp -R contracts/examples/multisig/test-contracts \
   $ARWEN_PATH/test/multisig

cp -R contracts/examples/moax-dct-swap/output/moax-dct-swap.wasm \
   $ARWEN_PATH/test/moax-dct-swap/output/moax-dct-swap.wasm
cp -R contracts/examples/moax-dct-swap/denali \
   $ARWEN_PATH/test/moax-dct-swap

cp -R contracts/examples/erc20/output/erc20.wasm \
   $ARWEN_PATH/test/erc20-rust/output/erc20.wasm
cp -R contracts/examples/erc20/denali \
   $ARWEN_PATH/test/erc20-rust

cp -R contracts/feature-tests/basic-features/output/basic-features.wasm \
   $ARWEN_PATH/test/features/basic-features/output/basic-features.wasm
cp -R contracts/feature-tests/basic-features/denali \
   $ARWEN_PATH/test/features/basic-features

cp -R contracts/feature-tests/payable-features/output/payable-features.wasm \
   $ARWEN_PATH/test/features/payable-features/output/payable-features.wasm
cp -R contracts/feature-tests/payable-features/denali \
   $ARWEN_PATH/test/features/payable-features

cp -R contracts/feature-tests/async/async-alice/output/async-alice.wasm \
   $ARWEN_PATH/test/features/async/async-alice/output/async-alice.wasm
cp -R contracts/feature-tests/async/async-bob/output/async-bob.wasm \
   $ARWEN_PATH/test/features/async/async-bob/output/async-bob.wasm
cp -R contracts/feature-tests/async/forwarder/output/forwarder.wasm \
   $ARWEN_PATH/test/features/async/forwarder/output/forwarder.wasm
cp -R contracts/feature-tests/async/forwarder-raw/output/forwarder-raw.wasm \
   $ARWEN_PATH/test/features/async/forwarder-raw/output/forwarder-raw.wasm
cp -R contracts/feature-tests/async/recursive-caller/output/recursive-caller.wasm \
   $ARWEN_PATH/test/features/async/recursive-caller/output/recursive-caller.wasm
cp -R contracts/feature-tests/async/vault/output/vault.wasm \
   $ARWEN_PATH/test/features/async/vault/output/vault.wasm
cp -R contracts/feature-tests/async/denali \
   $ARWEN_PATH/test/features/async
