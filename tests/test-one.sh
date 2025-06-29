#!/bin/bash
curl -X POST -H "Content-Type: application/json" -d "$(cat $1)" http://localhost:8001/api/ars/sendevent
