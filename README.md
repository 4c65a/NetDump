<!--toc:start-->
- [NetDump](#netdump)
  - [What is?](#what-is)
    - [Command](#command)
- [Goals](#goals)
  - [Goals 2](#goals-2)
    - [Ping and Tracerouter](#ping-and-tracerouter)
    - [Packet Capture](#packet-capture)
<!--toc:end-->

# NetDump

## What is?
Is a project with basic functions for network packet capture.

### Command
netdump [--list] [--interface <interface_name>] [--ping <ip>] [--traceroute <ip>] [--wifi_speed]

## Goals
- [x] List Interface of your own system
- [x] Packet Capture
- [x] Ping
- [ ] Traceroute
- [ ] Wifi Speed 
- [ ] Add clap command

## Goals 2
# Ping and Tracerouter
- [ ] Add stop after count replies like ping 
- [ ] Add ICMP6(echo) and IPV6
- [ ] Seconds between sending each packe
- [ ] Know about that if the device is living and uses a file containing multiple IPs or hosts for verification
  
# Packet Capture 
- [ ] Add new protocols 
  - [ ] Ethernet
  - [x] IPv4 and IPv6
  - [x] TCP
  - [x] UDP
  - [ ] HTTP/HTTPS
  - [ ] FTP
  - [ ] SMTP
  - [ ] IMAP/POP3
  - [ ] DNS
  - [ ] DHCP
  - [ ] ARP
  - [x] ICMP and ICMP6
  - [ ] SNMP
  - [ ] Telnet and SSH
  - [ ] NTP
  - [ ] VoIP Protocols (SIP, RTP)
   
- [ ] Filter
  - [ ] Port
  - [ ] Ip Address 
  - [ ] Protocols
