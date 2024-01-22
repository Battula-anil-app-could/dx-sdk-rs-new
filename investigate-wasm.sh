#!/bin/sh

# generates all contract wasm bytecodes, without removing the original function names
# very useful for investigating bytecode sizes

# export RUSTFLAGS=${RUSTFLAGS-'-C link-arg=-s'}

### BENCHMARKS ###

cd contracts/benchmarks/str-repeat/wasm
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/str_repeat_wasm.wasm output/str-repeat-dbg.wasm
cd ../../..

### EXAMPLES ###

cd contracts/examples/adder/wasm
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/adder_wasm.wasm output/adder-dbg.wasm
cd ../../..

cd contracts/examples/crowdfunding-moax/wasm
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/crowdfunding_moax_wasm.wasm output/crowdfunding-moax-dbg.wasm
cd ../../..

cd contracts/examples/crowdfunding-erc20/wasm
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/crowdfunding_erc20_wasm.wasm output/crowdfunding-erc20-dbg.wasm
cd ../../..

cd contracts/examples/crowdfunding-dct/wasm
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/crowdfunding_dct_wasm.wasm output/crowdfunding-dct-dbg.wasm
cd ../../..

cd contracts/examples/crypto-bubbles/wasm
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/crypto_bubbles_wasm.wasm output/crypto-bubbles-dbg.wasm
cd ../../..

cd contracts/examples/factorial/wasm
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/factorial_wasm.wasm output/factorial-dbg.wasm
cd ../../..

cd contracts/examples/lottery-moax/wasm
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/lottery_moax_wasm.wasm output/lottery-moax-dbg.wasm
cd ../../..

cd contracts/examples/lottery-erc20/wasm
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/lottery_erc20_wasm.wasm output/lottery-erc20-dbg.wasm
cd ../../..

cd contracts/examples/lottery-dct/wasm
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/lottery_dct_wasm.wasm output/lottery-dct-dbg.wasm
cd ../../..

cd contracts/examples/multisig/wasm
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/multisig_wasm.wasm output/multisig-dbg.wasm
cd ../../..

cd contracts/examples/simple-erc20/wasm
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/simple_erc20_wasm.wasm output/simple-erc20-dbg.wasm
cd ../../..


### TEST CONTRACTS ###

cd contracts/feature-tests/basic-features/wasm
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/basic_features_wasm.wasm output/basic-features-dbg.wasm
cd ../../..

cd contracts/feature-tests/async/async-alice/wasm
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/async_alice_wasm.wasm output/alice-dbg.wasm
cd ../../../..

cd contracts/feature-tests/async/async-bob/wasm
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/async_bob_wasm.wasm output/bob-dbg.wasm
cd ../../../..

cd contracts/feature-tests/use-module/wasm
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/use_module_wasm.wasm output/use_module-dbg.wasm
cd ../../..
