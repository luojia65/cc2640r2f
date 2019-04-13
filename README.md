# CC2640R2F

Device support for TI CC2640R2F microcontrollers.

This crate provides an auto-generated API interacting with 
CC2640R2F as well as its peripherals. It is generated via 
[svd2rust] 0.14.0 through [tixml2svd] 0.1.3. Missing Interrupt
Vector descriptions are manually patched according to the 
reference manual, regarding unassigned interrupts as reserved.

[svd2rust]: https://github.com/rust-embedded/svd2rust
[tixml2svd]: https://github.com/dhoove/tixml2svd

An example usage for this bare-metal device support library
may be found at [`cc2640r2f-hal`].

[`cc2640r2f-hal`]: https://github.com/luojia65/cc2640r2f-hal
