#!/bin/bash

# Check if a port number was provided
if [ -z "$1" ]; then
  echo "Usage: $0 <port-number>"
  exit 1
fi

port="$1"
wait_time=1 # Time in seconds to wait between checks

echo "Waiting for process to start on port ${port}..."

# Loop until a connection can be made to the specified port
while ! nc -z localhost "${port}" &>/dev/null; do
  sleep "$wait_time"
done

echo "Process started on port ${port}."
