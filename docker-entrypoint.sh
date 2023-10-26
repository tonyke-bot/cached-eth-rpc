#!/usr/bin/env bash

IFS=',' read -ra PARTS <<< "$ENDPOINTS"
ARGUMENTS=""

for part in "${PARTS[@]}"; do
  ARGUMENTS+="--endpoint $part "
done

# if DATA_PERSISTENCE = 1 is set, then use it
if [ "$DATA_PERSISTENCE" = "1" ]; then
  ARGUMENTS+="--datadir /data"
fi

exec $1 --port 8124 --bind 0.0.0.0 $ARGUMENTS
