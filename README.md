# NetDump

## What is it?

NetDump is a command-line tool designed to facilitate network analysis and troubleshooting. With NetDump, you can capture and analyze packets in real time, manage network interfaces, send ICMP requests (ping), perform traceroutes, and resolve domain names (DNS). The tool supports Berkeley Packet Filter (BPF) filters so you can specify the type of traffic you want to capture, allowing for more precise analysis.

> Currently it does not work for Windows, I only tested it on Linux systems.

# Commands

* **cap**: Capture packets from your network.
* **interface**: Get a list of available network interfaces.
* **ping**: Ping sends an Internet Control Message Protocol (ICMP) echo request.
* **tracerouter**: Performs a traceroute to the given IP address.
* **resolve**: Resolve the IP address of a host.

## `Cap` Command

* **Capture TCP/UDP/ICMP packets**:

  ```bash

  sudo netdump cap -i eth0

  ```
* **Capture TCP packets using the filter**:

  ```bash

  sudo netdump cap -i eth0 -f "tcp"

  ```

## `Interface` Command

* **List available network interfaces**:

  ```bash

  sudo netdump interface -l

  ```
* **Get information about a specific interface**:

  ```bash

  sudo netdump interface -f eth0

  ```

## `Ping` Command

* **Send a basic ping**:

  ```bash

  sudo netdump ping -d 192.168.0.1

  ```

  ```bash

  sudo netdump ping -d google.com

  ```
* **Define time to live (TTL)**:

  ```bash

  sudo netdump ping -d 192.168.0.1 -t 80

  ```
* **Set the interval between sent packets (in seconds)**:

  ```bash

  sudo netdump ping -d 192.168.0.1 -m 1

  ```
* **Stop after a specific number of replies**:

  ```bash

  sudo netdump ping -d 192.168.0.1 -c 10

  ```
* **Send an ICMPv6 packet**:

  ```bash

  sudo netdump ping -d ::1 -ipv6

  ```
* **Combine options**:

  ```bash

  sudo netdump ping -d 192.168.0.1 -t 80 -m 1 -c 10

  ```

## `Tracerouter` Command

* **Perform a traceroute to an IP address**:

  ```bash

  sudo netdump tracerouter -r 192.168.0.1

  ```

## `Resolve` Command

* **Resolve the IP address of a host**:

  ```bash

  sudo netdump resolve -d google.com

  ```

# BPF Filter Cheat Sheet for Packet Capture

## Protocol Filters

* **ICMP**: `icmp`
* **TCP**: `tcp`
* **UDP**: `udp`
* **ICMPv6**: `icmp6`

## IP Address Filters

* **Source IP Address**: `src host <IP>`

  * Example: `src host 192.168.0.1`
* **Destination IP Address**: `dst host <IP>`

  * Example: `dst host 192.168.0.2`
* **Any IP Address (either source or destination)**: `host <IP>`

  * Example: `host 192.168.0.1`

## Port Filters

* **TCP/UDP Port**: `port <PORT>`

  * Example: `port 80`
* **TCP Port**: `tcp port <PORT>`

  * Example: `tcp port 22`
* **UDP Port**: `udp port <PORT>`

  * Example: `udp port 53`
* **Port Range**: `portrange <START>-<END>`

  * Example: `portrange 1000-2000`

## Combination Filters

* **Multiple Filters (AND condition)**: `tcp and host 192.168.0.1 and port 80`
* **Negation (NOT)**: `not <FILTER>`

  * Example: `not port 80`
* **OR Condition**: `filter1 or filter2`

  * Example: `tcp or udp`

## Example Commands (using `cap` command)

* **Capture TCP packets**:

  ```bash

  sudo netdump cap -i eth0 -f "tcp"

  ```
* **Capture ICMP packets**:

  ```bash

  sudo netdump cap -i eth0 -f "icmp"

  ```
* **Capture traffic from a specific host**:

  ```bash

  sudo netdump cap -i eth0 -f "host 192.168.0.1"

  ```
* **Capture traffic to/from a specific port**:

  ```bash

  sudo netdump cap -i eth0 -f "port 80"

  ```
* **Capture UDP traffic on a port range**:

  ```bash

  sudo netdump cap -i eth0 -f "udp portrange 1000-2000"

  ```
* **Capture traffic excluding port 80**:

  ```bash

  sudo netdump cap -i eth0 -f "not port 80"

  ```


**Install the necessary dependencies**:

``sudo apt install build-essential pkg-config libpcap-dev     ``
