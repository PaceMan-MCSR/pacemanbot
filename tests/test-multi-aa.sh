#!/bin/bash

for file in "$@"; do
  echo "Sending $file to /api/ars/aa/sendEvent"
  curl -X POST -H "Content-Type: application/json" \
       -d "$(cat "$file")" \
       http://localhost:8001/api/ars/aa/sendEvent
  sleep 10
done