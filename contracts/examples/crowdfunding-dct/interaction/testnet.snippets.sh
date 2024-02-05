ALICE="${USERS}/alice.pem"
ADDRESS=$(moapy data load --key=address-testnet)
DEPLOY_TRANSACTION=$(moapy data load --key=deployTransaction-testnet)
DEPLOY_ARGUMENTS="12 4096 0xABBAABBA"
DEPLOY_GAS="80000000"
PROXY=https://testnet-api.dharitri.com

deploy() {
    moapy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} \
          --gas-limit=${DEPLOY_GAS} --arguments ${DEPLOY_ARGUMENTS} \
          --outfile="deploy-testnet.interaction.json" --send --proxy=${PROXY} --chain=T || return

    TRANSACTION=$(moapy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(moapy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['address']")

    moapy data store --key=address-testnet --value=${ADDRESS}
    moapy data store --key=deployTransaction-testnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

checkDeployment() {
    moapy tx get --hash=$DEPLOY_TRANSACTION --omit-fields="['data', 'signature']" --proxy=${PROXY}
    moapy account get --address=$ADDRESS --omit-fields="['code']" --proxy=${PROXY}
}

status() {
    moapy --verbose contract query ${ADDRESS} --function="status" --proxy=${PROXY}
}

currentFunds() {
    moapy --verbose contract query ${ADDRESS} --function="currentFunds" --proxy=${PROXY}
}

sendFunds() {
    moapy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=10000000 \
        --function="fund" --value=3\
        --send --proxy=${PROXY} --chain=T
}
