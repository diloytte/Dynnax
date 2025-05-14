#!/bin/bash

# Kill any previously running processes that might conflict
pkill -f target/release/backend
pkill -f target/release/dead_x_sniper

sleep 1  # short wait to ensure ports are released

# Run backend
./run_backend.sh &
PID1=$!

# Run sniper
./run_dead_x_sniper.sh &
PID2=$!

# Run frontend
cd frontend || exit 1
npm run dev &
PID3=$!

trap "echo 'Stopping processes...'; kill $PID1 $PID2 $PID3; exit" SIGINT SIGTERM

wait
