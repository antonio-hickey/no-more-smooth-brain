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

Sequence Number
---
The sequence number of the first data octect in this segment (except when SYN is present). If SYN is present the sequence number is the initial sequence number (ISN) and the first data octect is ISN+1.

Acknowledgement Number
---
If the ACK control bit is set this field contains the value of the next sequence number the sender of the segment is expecting to recieve. *Once a connection is established this is always sent.*


