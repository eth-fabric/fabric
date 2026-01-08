#!/usr/bin/env bash
set -euo pipefail

ENCLAVE="${1:-preconf-testnet}"
FABRIC_CONFIG="${2:?usage: update_chain_line.sh [enclave] <path-to-fabric-config.toml>}"

TMP_DIR="/tmp/genesis_data"
CONFIG_YAML="$TMP_DIR/config.yaml"

rm -rf "$TMP_DIR"
kurtosis files download "$ENCLAVE" el_cl_genesis_data "$TMP_DIR" >/dev/null

gt=$(grep -E '^MIN_GENESIS_TIME:' "$CONFIG_YAML" | head -n1 | awk '{print $2}')
sps=$(grep -E '^SECONDS_PER_SLOT:' "$CONFIG_YAML" | head -n1 | awk '{print $2}')
gfv=$(grep -E '^GENESIS_FORK_VERSION:' "$CONFIG_YAML" | head -n1 | awk '{print $2}')
cid=$(grep -E '^DEPOSIT_CHAIN_ID:' "$CONFIG_YAML" | head -n1 | awk '{print $2}')

chain_line=$(printf 'chain = { genesis_time_secs = %s, slot_time_secs = %s, genesis_fork_version = "%s", chain_id = %s}' \
  "$gt" "$sps" "$gfv" "$cid")

cp "$FABRIC_CONFIG" "$FABRIC_CONFIG.bak"

# Replace the first line that starts with "chain =" (allow leading whitespace)
perl -0777 -i -pe "s/^[ \t]*chain[ \t]*=[^\n]*\$/${chain_line}/m" "$FABRIC_CONFIG"

echo "Updated chain line in $FABRIC_CONFIG"
echo "Backup: $FABRIC_CONFIG.bak"
echo "New line: $chain_line"