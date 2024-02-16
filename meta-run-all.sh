#!/bin/bash

cargo install dharitri-sc-meta

TARGET_DIR=$PWD/target

sc-meta all abi --path ./contracts
