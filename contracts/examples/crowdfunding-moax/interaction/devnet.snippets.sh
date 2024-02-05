ALICE="${USERS}/alice.pem"
ADDRESS=$(moapy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(moapy data load --key=deployTransaction-devnet)
DEPLOY_ARGUMENTS="12 4096"
DEPLOY_GAS="80000000"

deploy() {
    moapy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} \
          --gas-limit=${DEPLOY_GAS} --arguments ${DEPLOY_ARGUMENTS} \
          --outfile="deploy-devnet.interaction.json" --send || return

    TRANSACTION=$(moapy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(moapy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['address']")

    moapy data store --key=address-devnet --value=${ADDRESS}
    moapy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

deploySimulate() {
    moapy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} \
          --gas-limit=${DEPLOY_GAS} --arguments ${DEPLOY_ARGUMENTS} \
          --outfile="simulate-devnet.interaction.json" --simulate || return

    TRANSACTION=$(moapy data parse --file="simulate-devnet.interaction.json" --expression="data['result']['hash']")
    ADDRESS=$(moapy data parse --file="simulate-devnet.interaction.json" --expression="data['emitted_tx']['address']")
    RETCODE=$(moapy data parse --file="simulate-devnet.interaction.json" --expression="data['result']['returnCode']")
    RETMSG=$(moapy data parse --file="simulate-devnet.interaction.json" --expression="data['result']['returnMessage']")

    echo ""
    echo "Simulated transaction: ${TRANSACTION}"
    echo "Smart contract address: ${ADDRESS}"
    echo "Deployment return code: ${RETCODE}"
    echo "Deployment return message: ${RETMSG}"
}

checkDeployment() {
    moapy tx get --hash=$DEPLOY_TRANSACTION --omit-fields="['data', 'signature']"
    moapy account get --address=$ADDRESS --omit-fields="['code']"
}

status() {
    moapy --verbose contract query ${ADDRESS} --function="status"
}

currentFunds() {
    moapy --verbose contract query ${ADDRESS} --function="currentFunds"
}

sendFunds() {
    moapy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=10000000 \
        --function="fund" --value=3\
        --send
}
