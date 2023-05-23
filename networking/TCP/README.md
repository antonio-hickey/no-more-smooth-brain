Transmission Control Protocol (TCP)
===
[RFC 793](https://datatracker.ietf.org/doc/html/rfc793)

General Overview
---
One of the main protocols of the IP suite. Providing reliable, ordered, and error checked delivery of a stream of bytes between 
applications running on hosts communicating via an IP network.

TCP is [connection-oriented](https://en.wikipedia.org/wiki/Connection-oriented_communication), and a connection between client and 
server must be established before data can be sent. The server must be listening (passive open) for connection requests from clients
before a connection is established. Three way handshake (active open), retransmission, & error detection makes TCP reliable but lengthens
the latency of TCP.

Pros vs Cons
---
Pros:
* In Order Delivery
* Error Detection
* Unique Identification
* Congestion Control
* Data Retransmission

Cons:
* Slow Start
* Image Blockings
* Loss Result of Congestion
* Slow Handshake
* Not Optimized For Small Networks

TCP's 3-way Handshake
---
* Step 1 (SYN): The client sends a segment with SYN(Synchronize Sequence Number) which informs the server the client is likely to start communication and with 
what sequence number to starts segments with.

* Step 2 (SYN + ACK): The server responds with SYN-ACK signal bits set. ACK(Acknowledgement) signifies the server got the SYN and sends it's own SYN like in step 1
but to the client.

* step 3 (ACK): The client responds with ACK telling the server it got it's SYN.

* Example:
    * steps:
        * `./run.sh` to set up everything for the server side (compile into binary, set up tuntap interface, etc) 
        * in a new terminal window run `nc 192.168.0.2 80` to send tcp packets to the server (from client) 
        * in a new terminal window run `sudo tshark -i tun0` to read the packets sent to tuntap interface (server)

    * `192.168.0.1` = Client
    * `192.168.0.2` = Server
    * ```
        2 9.906303600  192.168.0.1 → 192.168.0.2  TCP 60 48042 → 80 [SYN] Seq=0 Win=64240 Len=0 MSS=1460 SACK_PERM=1 TSval=722769728 TSecr=0 WS=128
        3 9.907042100  192.168.0.2 → 192.168.0.1  TCP 40 80 → 48042 [SYN, ACK] Seq=0 Ack=1 Win=10 Len=0
        4 9.907969000  192.168.0.1 → 192.168.0.2  TCP 40 48042 → 80 [ACK] Seq=1 Ack=1 Win=64240 Len=0
    ```

The client sends the server a SYN, the server responds with SYN, ACK, and the client sends back a ACK to finish the handshake (connection established).

Sequence Number
---
The sequence number of the first data octect in this segment (except when SYN is present). If SYN is present the sequence number is the initial sequence number (ISN) and the first data octect is ISN+1.

Acknowledgement Number
---
If the ACK control bit is set this field contains the value of the next sequence number the sender of the segment is expecting to recieve. *Once a connection is established this is always sent.*


