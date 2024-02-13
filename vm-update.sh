#!/bin/bash

# Locally update the VM version

VM_TAG="v1.4.51"

echo "Before:"
moapy config dump
moapy config set dependencies.vmtools.tag $VM_TAG
echo "After:"
moapy config dump

moapy deps install vmtools --overwrite
