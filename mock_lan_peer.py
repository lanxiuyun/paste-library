#!/usr/bin/env python3
import argparse
import json
import socket
import sys
import time
import uuid
from datetime import datetime, timezone
from typing import Any


PROTOCOL_VERSION = 1


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Mock a LAN clipboard peer for single-machine testing."
    )
    parser.add_argument(
        "--action",
        choices=["heartbeat", "pair-request", "sync-text", "all"],
        default="all",
        help="Which test flow to run.",
    )
    parser.add_argument(
        "--target-host",
        default="127.0.0.1",
        help="Target app host.",
    )
    parser.add_argument(
        "--target-discovery-port",
        type=int,
        default=48572,
        help="Target app UDP discovery port.",
    )
    parser.add_argument(
        "--target-tcp-port",
        type=int,
        default=48571,
        help="Target app TCP sync port.",
    )
    parser.add_argument(
        "--mock-listen-port",
        type=int,
        default=48581,
        help="Mock peer TCP listen port for PairResponse.",
    )
    parser.add_argument(
        "--mock-device-id",
        default=f"mock-peer-{uuid.uuid4().hex[:8]}",
        help="Mock peer device id.",
    )
    parser.add_argument(
        "--mock-device-name",
        default="Mock LAN Peer",
        help="Mock peer device name.",
    )
    parser.add_argument(
        "--text",
        default="Hello from mock LAN peer",
        help="Text payload for sync-text or all.",
    )
    parser.add_argument(
        "--wait-seconds",
        type=int,
        default=60,
        help="How long to wait for PairResponse.",
    )
    return parser


def print_section(title: str) -> None:
    print(f"\n==== {title} ====")


def send_udp_json(host: str, port: int, payload: dict[str, Any]) -> None:
    raw = json.dumps(payload, ensure_ascii=False).encode("utf-8")
    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as sock:
        if host == "255.255.255.255":
            sock.setsockopt(socket.SOL_SOCKET, socket.SO_BROADCAST, 1)
        sock.sendto(raw, (host, port))


def send_tcp_json(host: str, port: int, payload: dict[str, Any]) -> None:
    raw = json.dumps(payload, ensure_ascii=False).encode("utf-8")
    with socket.create_connection((host, port), timeout=3) as sock:
        sock.sendall(raw)
        try:
            sock.shutdown(socket.SHUT_WR)
        except OSError:
            pass


def wait_for_one_tcp_message(port: int, timeout_seconds: int) -> str | None:
    deadline = time.time() + timeout_seconds
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as server:
        server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        server.bind(("0.0.0.0", port))
        server.listen(1)
        server.settimeout(0.5)

        while time.time() < deadline:
            try:
                conn, _addr = server.accept()
            except TimeoutError:
                continue

            with conn:
                chunks: list[bytes] = []
                conn.settimeout(1.0)
                while True:
                    try:
                        chunk = conn.recv(4096)
                    except TimeoutError:
                        break
                    if not chunk:
                        break
                    chunks.append(chunk)

                if chunks:
                    return b"".join(chunks).decode("utf-8", errors="replace")

    return None


def heartbeat_message(device_id: str, device_name: str, listen_port: int) -> dict[str, Any]:
    return {
        "kind": "heartbeat",
        "protocol_version": PROTOCOL_VERSION,
        "device_id": device_id,
        "device_name": device_name,
        "tcp_port": listen_port,
    }


def pair_request_message(device_id: str, device_name: str, listen_port: int) -> dict[str, Any]:
    return {
        "kind": "pair_request",
        "protocol_version": PROTOCOL_VERSION,
        "device_id": device_id,
        "device_name": device_name,
        "tcp_port": listen_port,
    }


def sync_text_message(
    device_id: str,
    device_name: str,
    listen_port: int,
    text: str,
) -> dict[str, Any]:
    return {
        "kind": "sync_text",
        "protocol_version": PROTOCOL_VERSION,
        "device_id": device_id,
        "device_name": device_name,
        "tcp_port": listen_port,
        "message_id": f"{device_id}-{uuid.uuid4().hex}",
        "created_at": datetime.now(timezone.utc).isoformat(),
        "text": text,
    }


def run_heartbeat(args: argparse.Namespace) -> None:
    print_section("Send Heartbeat")
    payload = heartbeat_message(
        args.mock_device_id,
        args.mock_device_name,
        args.mock_listen_port,
    )
    print(json.dumps(payload, ensure_ascii=False, indent=2))
    send_udp_json(args.target_host, args.target_discovery_port, payload)
    print(f"Heartbeat sent to {args.target_host}:{args.target_discovery_port}")


def run_pair_request(args: argparse.Namespace) -> None:
    run_heartbeat(args)
    time.sleep(0.5)

    print_section("Send PairRequest")
    payload = pair_request_message(
        args.mock_device_id,
        args.mock_device_name,
        args.mock_listen_port,
    )
    print(json.dumps(payload, ensure_ascii=False, indent=2))
    send_tcp_json(args.target_host, args.target_tcp_port, payload)
    print(f"PairRequest sent to {args.target_host}:{args.target_tcp_port}")

    print_section("Wait PairResponse")
    print("Approve the device in app settings -> LAN sync -> pending requests")
    response = wait_for_one_tcp_message(args.mock_listen_port, args.wait_seconds)
    if response is None:
        print("Timed out waiting for PairResponse", file=sys.stderr)
        sys.exit(1)

    print(response)


def run_sync_text(args: argparse.Namespace) -> None:
    print_section("Send SyncText")
    payload = sync_text_message(
        args.mock_device_id,
        args.mock_device_name,
        args.mock_listen_port,
        args.text,
    )
    print(json.dumps(payload, ensure_ascii=False, indent=2))
    send_tcp_json(args.target_host, args.target_tcp_port, payload)
    print(f"SyncText sent to {args.target_host}:{args.target_tcp_port}")


def run_all(args: argparse.Namespace) -> None:
    run_heartbeat(args)
    time.sleep(0.5)

    print_section("Send PairRequest")
    pair_payload = pair_request_message(
        args.mock_device_id,
        args.mock_device_name,
        args.mock_listen_port,
    )
    print(json.dumps(pair_payload, ensure_ascii=False, indent=2))
    send_tcp_json(args.target_host, args.target_tcp_port, pair_payload)
    print(f"PairRequest sent to {args.target_host}:{args.target_tcp_port}")

    print_section("Wait PairResponse")
    print("Approve the device in app settings -> LAN sync -> pending requests")
    response = wait_for_one_tcp_message(args.mock_listen_port, args.wait_seconds)
    if response is None:
        print("Timed out waiting for PairResponse", file=sys.stderr)
        sys.exit(1)

    print(response)
    try:
        response_obj = json.loads(response)
    except json.JSONDecodeError:
        print("PairResponse is not valid JSON", file=sys.stderr)
        sys.exit(1)

    if not response_obj.get("accepted"):
        print("Pair request was rejected", file=sys.stderr)
        sys.exit(1)

    time.sleep(0.5)
    run_sync_text(args)

    print_section("Done")
    print("Now check in the app:")
    print(f"1. discovered devices contains {args.mock_device_name}")
    print(f"2. trusted devices contains {args.mock_device_name}")
    print(f"3. clipboard history contains: {args.text}")


def main() -> None:
    parser = build_parser()
    args = parser.parse_args()

    print_section("Parameters")
    print(f"action               : {args.action}")
    print(f"target_host          : {args.target_host}")
    print(f"target_discovery_port: {args.target_discovery_port}")
    print(f"target_tcp_port      : {args.target_tcp_port}")
    print(f"mock_listen_port     : {args.mock_listen_port}")
    print(f"mock_device_id       : {args.mock_device_id}")
    print(f"mock_device_name     : {args.mock_device_name}")

    if args.action == "heartbeat":
        run_heartbeat(args)
    elif args.action == "pair-request":
        run_pair_request(args)
    elif args.action == "sync-text":
        run_sync_text(args)
    else:
        run_all(args)


if __name__ == "__main__":
    main()
