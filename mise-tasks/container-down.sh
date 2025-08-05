#!/bin/bash

ids=$(docker ps -q --filter "label=id_label=$BRANCH_NAME")

if [ -z "$ids" ]; then
    echo "No containers found with label '$BRANCH_NAME'"
    exit 1
fi

docker rm -f $ids