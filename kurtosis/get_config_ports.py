#!/usr/bin/env python3
import argparse
import re
import subprocess
from pathlib import Path
from typing import Dict

def run_port_print(enclave: str, service: str, port_id: str) -> int:
    """
    kurtosis port print <enclave> <service> <port_id>
    returns something like: 127.0.0.1:58976
    """
    p = subprocess.run(
        ["kurtosis", "port", "print", enclave, service, port_id],
        check=True,
        capture_output=True,
        text=True,
    )
    out = p.stdout.strip()
    m = re.search(r":(\d+)\s*$", out)
    if not m:
        raise RuntimeError(f"Unexpected output from kurtosis port print: {out}")
    return int(m.group(1))

def replace_scalar_int(toml_text: str, key: str, value: int) -> str:
    pattern = rf"(?m)^(?P<prefix>\s*{re.escape(key)}\s*=\s*)(?P<val>.*?)(?P<suffix>\s*)$"
    if not re.search(pattern, toml_text):
        raise RuntimeError(f"Did not find key '{key}' in fabric config")
    return re.sub(pattern, rf"\g<prefix>{value}\g<suffix>", toml_text, count=1)

def write_with_backup(path: Path, new_text: str) -> None:
    old_text = path.read_text(encoding="utf-8")
    bak = path.with_suffix(path.suffix + ".bak")
    bak.write_text(old_text, encoding="utf-8")
    path.write_text(new_text, encoding="utf-8")

def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--enclave", default="preconf-testnet")
    ap.add_argument("--fabric-config", required=True, type=Path)
    args = ap.parse_args()

    ports: Dict[str, int] = {}
    ports["BEACON_PORT"] = run_port_print(args.enclave, "cl-1-lighthouse-geth", "http")
    ports["EXECUTION_PORT"] = run_port_print(args.enclave, "el-1-geth-lighthouse", "rpc")
    ports["RELAY_PORT"] = run_port_print(args.enclave, "mev-relay-api", "http")

    # Update fabric config
    fabric_text = args.fabric_config.read_text(encoding="utf-8")
    fabric_text = replace_scalar_int(fabric_text, "beacon_port", ports["BEACON_PORT"])
    fabric_text = replace_scalar_int(fabric_text, "execution_client_port", ports["EXECUTION_PORT"])
    fabric_text = replace_scalar_int(fabric_text, "downstream_relay_port", ports["RELAY_PORT"])
    write_with_backup(args.fabric_config, fabric_text)

    print("Updated configs using Kurtosis port print:")
    print(f"  beacon_port            = {ports['BEACON_PORT']}")
    print(f"  execution_client_port  = {ports['EXECUTION_PORT']}")
    print(f"  downstream_relay_port  = {ports['RELAY_PORT']}")
    print("")
    print("Backups written as:")
    print(f"  {args.fabric_config}.bak")

if __name__ == "__main__":
    main()