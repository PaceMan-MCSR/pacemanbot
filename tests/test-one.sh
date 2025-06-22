#!/bin/bash
curl -X POST -H "Content-Type: application/json" -d "$(cat $1)" http://host.docker.internal:8001/api/ars/sendevent

