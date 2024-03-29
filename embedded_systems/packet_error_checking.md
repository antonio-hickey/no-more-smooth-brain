Packet Error Checking (PEC)
===

Abstract
---
"Packet-Error Checking” (PEC) is an error detection mechanism wildly used during data transmission. Some Maxim Integrated products featured PEC mode to increase reliability of data transfer. This application note discussed detailed implementation of PEC byte on temperature sensor product with 1-wire and 2-wire interface.

---

Background
---
Communication happens everywhere, and errors can decrease communication efficiency. To prevent this people use different methods to detect communication errors.

Data communications can be subject to errors like channel noise, electrical distortion, random bit errors, and cross talk.

---

Cyclic Redundancy Check (CRC)
---
* An error-detecting code used to detect accidental errors in data transmissions. 

My MLX90614 infrared sensor implements CCs in the form of Packet Error Checking (PEC), which is a mechanism that was originally defined in SMBus.

In a data transmission system, a PEC byte can be appended at the end of each transaction as error-detecting code.

The PEC byte is calculated based on a CRC-8 byte represented by the polynomial: $C(X) = X^8 + X^2 + X^1 + 1$

The PEC mechanism improves reliability and communication robustness and PEC implementation is optional for SMBus devices.

---

$I^2C$/SMBus PEC
---
* Write transaction:
    * The master writes the sensor address, waits for an ACK bit from the sensor, and the master transmits the target register, followed by another ACK bit from the sensor.

    * The master writes 2 data bytes and recieves ACK bits from the sensor for each data byte.

    * With PEC mode on, the master sends one more CRC byte and recieves the last ACK bit from the sensor, stopping the transaction.

    * The CRC byte is calculated using the slave address, register address, and transmitted data.

* Read transaction:
    * The master transmits the sensor address and the target register address and recieves an ACK bit for each transmission from the slave.

    * The master generates a REPEAT START (Sr) byte and writes the sensor address and a read bit.

    * The sensor then sends an ACK bit for the address/read byte and transmits the 2 data bytes.

    * With PEC mode on, a PEC byte is appended by the sensor after the data transmission.

    * The CRC byte is calculated using a slave address with a write bit, a register address, a slave address with a read bit, and the transmitted data.

---

Example write with tempature sensor
---
* The temperature threshold register (TOS) is used to set the temperature limit of the sensor. If the measured temperature exceeds TOS, the configuration register shows an overtemperature status. The TOS has a power-on state of 80°C (0x5000), where the address is 0x03. To set TOS to 95°C (0x5F00), the master writes to the sesnor.

* M = Master | S = Slave

| Direction | M -> S | M -> S           | M -> S | S -> M | M -> S               | S -> M | M -> S       | S -> M | M -> S       | S -> M | M -> S       | S -> M |
|-----------|--------|------------------|--------|--------|----------------------|--------|--------------|--------|--------------|--------|--------------|--------|
| Content   | S      | Slave<br>Address | WR     | ACK    | Register <br>Address | ACK    | Data High    | ACK    | Data Low     | ACK    | PEC Byte     | ACK    |
| Binary    |        | 1001 <br>000     | 0      |        | 0000<br>0011         |        | 0101<br>1111 |        | 0000<br>0000 |        | 0010<br>0100 |        |

* The master calculates the PEC-8 byte starting from the first bit of the slave address (MSB), 0x90035F00 is shifted into the shift register one bit at a time to calculate 0x24.

* The master sends 0x90035F0024 to the sesnor and receives an ACK because 0x24 matches the PEC byte generated by the slave. The slave sends an ACK to the master if the received PEC byte is a match.

* PEC generator for $CRC = X^8 + X^2 + X^1 + 1$ :
    * ![CRC Generation](https://www.analog.com/-/media/analog/en/landing-pages/design-notes/how-to-use-packeterror-checking-to-secure-your-temperature-reading/6797fig06.png)

---

Conclusion
---
By using CRC or PEC, the master and slave can verify the data recieved and detect transmission errors. This is especially useful in scenarios where multiple devices are connecting with the same host at the same time, the cyclic redundancy check provides an efficient error checking solution.
