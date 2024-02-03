ALICE="/home/dharitri/MySandbox/testnet/wallets/users/alice.pem"
ADDRESS=$(moapy data load --key=address-testnet)
DEPLOY_TRANSACTION=$(moapy data load --key=deployTransaction-testnet)
PROXY=https://testnet-api.dharitri.com
CHAIN_ID=T

SC_PARENT_ADDRESS_BECH32=moa1qqqqqqqqqqqqqpgqfzydqmdw7m2vazsp6u5p95yxz76t2p9rd8ssz682hq

SC_CHILD_ADDRESS_HEX=0000000000000000050011d9d2104d1bb4703accbf6dd06b4ffa87a125bd69e1
SC_CHILD_ADDRESS_BECH32=$(moapy wallet bech32 --encode ${SC_CHILD_ADDRESS_HEX})

deploy() {
    moapy --verbose contract deploy \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --recall-nonce \
        --pem=${ALICE} \
        --bytecode="../output/forwarder.wasm" \
        --gas-limit=50000000 \
        --send \
        --outfile="deploy-testnet.interaction.json" \
         || return

    TRANSACTION=$(moapy data parse --file="deploy-testnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(moapy data parse --file="deploy-testnet.interaction.json" --expression="data['contractAddress']")

    moapy data store --key=address-testnet --value=${ADDRESS}
    moapy data store --key=deployTransaction-testnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

deployFactorialChild() {
    local FACTORIAL_CODE=0x"$(xxd -p ../../examples/factorial/output/factorial.wasm | tr -d '\n')"

    moapy --verbose contract call ${SC_PARENT_ADDRESS_BECH32} --recall-nonce --pem=${ALICE} --gas-limit=10000000 --function="deployContract" --arguments ${FACTORIAL_CODE} --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

callChildFactorial() {
    moapy --verbose contract call ${SC_CHILD_ADDRESS_BECH32} --recall-nonce --pem=${ALICE} --gas-limit=10000000 --function="factorial" --arguments 0x05 --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

upgradeChildToAdder() {
    local ADDER_CODE=0x"$(xxd -p ../../examples/adder/output/adder.wasm | tr -d '\n')"
    local INIT_VALUE=42

    moapy --verbose contract call ${SC_PARENT_ADDRESS_BECH32} --recall-nonce --pem=${ALICE} --gas-limit=500000000 --function="upgradeChildContract" --arguments 0x${SC_CHILD_ADDRESS_HEX} ${ADDER_CODE} ${INIT_VALUE} --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

deployTwoContracts() {
    moapy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=5000000 --function="deployTwoContracts" --send --proxy=${PROXY} --chain=${CHAIN_ID}
}
