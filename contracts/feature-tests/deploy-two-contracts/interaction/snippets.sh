ALICE="/home/dharitri/MySandbox/testnet/wallets/users/alice.pem"
ADDRESS=$(moapy data load --key=address-testnet)
DEPLOY_TRANSACTION=$(moapy data load --key=deployTransaction-testnet)
PROXY=https://testnet-api.dharitri.com

deploy() {
    moapy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} --gas-limit=50000000 --send --outfile="deploy-testnet.interaction.json" --proxy=${PROXY} --chain=T || return

    TRANSACTION=$(moapy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(moapy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['address']")

    moapy data store --key=address-testnet --value=${ADDRESS}
    moapy data store --key=deployTransaction-testnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

deployContract() {
    moapy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=5000000 --function="deployContract" --send --proxy=${PROXY} --chain=T
}

deployTwoContracts() {
    moapy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=5000000 --function="deployTwoContracts" --send --proxy=${PROXY} --chain=T
}