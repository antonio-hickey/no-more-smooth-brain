Instruction Set Architectures (ISA)
===

2 foundational questions in instruction set design:
---
* How do you balance transistor count and program complexity?
* How do you prioritize speed, power consumption, and cost?


Defining the processors
---

* x86 processor: In the architectual sense, the hardware components are all seperate from each other. Most components have separate chips called controllers. Components can be swapped or expanded without affecting connectivity or the overall hardware.

* ARM processor: ARM processors are in the same phsyical substrate as the other hardware controllers aka `integrated circuit`. Unlike Intel and AMD CPU's there is no ARM processor manufacturer, but a Arm Holdings licenses chip designs to other hardware manufacturers which then incorporate the ARM chip into their hardware designs. So unlike x86 chips ARM chips are not interchangeable and are very application specific. This type of manufacturing is called `system-on-a-chip (SoC)`.


Specifics
---
* x86
    * bits: 16-bit, 32-bit, & 64-bit
    * design: `Complex Instruction Set Computer (CISC)`
    * type: `Register-memory`
    * branching: `Condition Code`
    * open: partly

* ARM
    * bits: 32-bit, & 64-bit
    * design: `Reduced Instruction Set Computer (RISC)`
    * type: `Register-Register`
    * branching: `Condition Code`, `Compare & branch`
    * open: proprietary


