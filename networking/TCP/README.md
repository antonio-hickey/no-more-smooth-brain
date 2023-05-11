Transmission Control Protocol (TCP)
===
[RFC 793](https://datatracker.ietf.org/doc/html/rfc793)

One of the main protocols of the IP suite. Providing reliable, ordered, and error checked delivery of a stream of bytes between 
applications running on hosts communicating via an IP network.

TCP is [connection-oriented](https://en.wikipedia.org/wiki/Connection-oriented_communication), and a connection between client and 
server must be established before data can be sent. The server must be listening (passive open) for connection requests from clients
before a connection is established. Three way handshake (active open), retransmission, & error detection makes TCP reliable but lengthens
the latency of TCP.

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

