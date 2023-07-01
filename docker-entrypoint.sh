#!/bin/bash

IFS=',' read -ra PARTS <<< "$ENDPOINTS"
ARGUMENTS=""

for part in "${PARTS[@]}"; do
  ARGUMENTS+="--endpoint $part "
done

$1 --port 8124 --bind 0.0.0.0 $ARGUMENTS
