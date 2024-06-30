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
- [ ] Ping
- [ ] Traceroute
- [ ] Wifi Speed 

## Goals 2
# Ping and Tracerouter
- [ ] Add stop after count replies like ping 
- [ ] Add ICMP6(echo) and IPV6
- [ ] Add TCP SYN (IPV4 and IPV6)
- [ ] Add UPD (IPV4 and IPV6)

# Packet Capture 
- [ ] **Add new protocols** 
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
  - [x] ICMP
  - [ ] SNMP
  - [ ] Telnet and SSH
  - [ ] NTP
  - [ ] VoIP Protocols (SIP, RTP)
   
- [ ] **Filter** 
  - [ ] Port
  - [ ] Ip Address 
  - [ ] Protocols
