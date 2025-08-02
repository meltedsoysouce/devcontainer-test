#!/bin/bash

ids=$(docker ps -q --filter "label=id_label=$ID_LABEL")

if [ -z "$ids" ]; then
    echo "No containers found with label '$ID_LABEL'"
    exit 1
fi

docker rm -f $ids