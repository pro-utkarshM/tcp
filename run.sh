#!/bin/bash
cargo b --release
CARGO_TARGET_DIR=target
ext=$?
if [[ $ext -ne 0 ]]; then
	exit $ext
fi
sudo setcap cap_net_admin=eip $CARGO_TARGET_DIR/release/tcp
$CARGO_TARGET_DIR/release/tcp &
pid=$!
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
trap "kill $pid" INT TERM
wait $pid
