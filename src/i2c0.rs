#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Slave Own Address This register consists of seven address bits that identify this I2C device on the I2C bus."]
    pub soar: SOAR,
    #[doc = "0x04 - Slave Status Note: This register shares address with SCTL, meaning that this register functions as a control register when written, and a status register when read."]
    pub sstat: SSTAT,
    #[doc = "0x08 - Slave Data This register contains the data to be transmitted when in the Slave Transmit state, and the data received when in the Slave Receive state."]
    pub sdr: SDR,
    #[doc = "0x0c - Slave Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
    pub simr: SIMR,
    #[doc = "0x10 - Slave Raw Interrupt Status This register shows the unmasked interrupt status."]
    pub sris: SRIS,
    #[doc = "0x14 - Slave Masked Interrupt Status This register show which interrupt is active (based on result from SRIS and SIMR)."]
    pub smis: SMIS,
    #[doc = "0x18 - Slave Interrupt Clear This register clears the raw interrupt SRIS."]
    pub sicr: SICR,
    _reserved0: [u8; 2020usize],
    #[doc = "0x800 - Master Salve Address This register contains seven address bits of the slave to be accessed by the master (a6-a0), and an RS bit determining if the next operation is a receive or transmit."]
    pub msa: MSA,
    #[doc = "0x804 - Master Status"]
    pub mstat: MSTAT,
    #[doc = "0x808 - Master Data This register contains the data to be transmitted when in the Master Transmit state and the data received when in the Master Receive state."]
    pub mdr: MDR,
    #[doc = "0x80c - I2C Master Timer Period This register specifies the period of the SCL clock."]
    pub mtpr: MTPR,
    #[doc = "0x810 - Master Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
    pub mimr: MIMR,
    #[doc = "0x814 - Master Raw Interrupt Status This register show the unmasked interrupt status."]
    pub mris: MRIS,
    #[doc = "0x818 - Master Masked Interrupt Status This register show which interrupt is active (based on result from MRIS and MIMR)."]
    pub mmis: MMIS,
    #[doc = "0x81c - Master Interrupt Clear This register clears the raw and masked interrupt."]
    pub micr: MICR,
    #[doc = "0x820 - Master Configuration This register configures the mode (Master or Slave) and sets the interface for test mode loopback."]
    pub mcr: MCR,
}
#[doc = "Slave Own Address This register consists of seven address bits that identify this I2C device on the I2C bus."]
pub struct SOAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave Own Address This register consists of seven address bits that identify this I2C device on the I2C bus."]
pub mod soar;
#[doc = "Slave Status Note: This register shares address with SCTL, meaning that this register functions as a control register when written, and a status register when read."]
pub struct SSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave Status Note: This register shares address with SCTL, meaning that this register functions as a control register when written, and a status register when read."]
pub mod sstat;
#[doc = "Slave Control Note: This register shares address with SSTAT, meaning that this register functions as a control register when written, and a status register when read."]
pub struct SCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave Control Note: This register shares address with SSTAT, meaning that this register functions as a control register when written, and a status register when read."]
pub mod sctl;
#[doc = "Slave Data This register contains the data to be transmitted when in the Slave Transmit state, and the data received when in the Slave Receive state."]
pub struct SDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave Data This register contains the data to be transmitted when in the Slave Transmit state, and the data received when in the Slave Receive state."]
pub mod sdr;
#[doc = "Slave Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
pub struct SIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
pub mod simr;
#[doc = "Slave Raw Interrupt Status This register shows the unmasked interrupt status."]
pub struct SRIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave Raw Interrupt Status This register shows the unmasked interrupt status."]
pub mod sris;
#[doc = "Slave Masked Interrupt Status This register show which interrupt is active (based on result from SRIS and SIMR)."]
pub struct SMIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave Masked Interrupt Status This register show which interrupt is active (based on result from SRIS and SIMR)."]
pub mod smis;
#[doc = "Slave Interrupt Clear This register clears the raw interrupt SRIS."]
pub struct SICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave Interrupt Clear This register clears the raw interrupt SRIS."]
pub mod sicr;
#[doc = "Master Salve Address This register contains seven address bits of the slave to be accessed by the master (a6-a0), and an RS bit determining if the next operation is a receive or transmit."]
pub struct MSA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Salve Address This register contains seven address bits of the slave to be accessed by the master (a6-a0), and an RS bit determining if the next operation is a receive or transmit."]
pub mod msa;
#[doc = "Master Status"]
pub struct MSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Status"]
pub mod mstat;
#[doc = "Master Control This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller as stated in MSTAT. When written, the control register configures the I2C controller operation. To generate a single transmit cycle, the I2C Master Slave Address (MSA) register is written with the desired address, the MSA.RS bit is cleared, and this register is written with * ACK=X (0 or 1), * STOP=1, * START=1, * RUN=1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the MDR register."]
pub struct MCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Control This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller as stated in MSTAT. When written, the control register configures the I2C controller operation. To generate a single transmit cycle, the I2C Master Slave Address (MSA) register is written with the desired address, the MSA.RS bit is cleared, and this register is written with * ACK=X (0 or 1), * STOP=1, * START=1, * RUN=1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the MDR register."]
pub mod mctrl;
#[doc = "Master Data This register contains the data to be transmitted when in the Master Transmit state and the data received when in the Master Receive state."]
pub struct MDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Data This register contains the data to be transmitted when in the Master Transmit state and the data received when in the Master Receive state."]
pub mod mdr;
#[doc = "I2C Master Timer Period This register specifies the period of the SCL clock."]
pub struct MTPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Master Timer Period This register specifies the period of the SCL clock."]
pub mod mtpr;
#[doc = "Master Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
pub struct MIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
pub mod mimr;
#[doc = "Master Raw Interrupt Status This register show the unmasked interrupt status."]
pub struct MRIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Raw Interrupt Status This register show the unmasked interrupt status."]
pub mod mris;
#[doc = "Master Masked Interrupt Status This register show which interrupt is active (based on result from MRIS and MIMR)."]
pub struct MMIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Masked Interrupt Status This register show which interrupt is active (based on result from MRIS and MIMR)."]
pub mod mmis;
#[doc = "Master Interrupt Clear This register clears the raw and masked interrupt."]
pub struct MICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Interrupt Clear This register clears the raw and masked interrupt."]
pub mod micr;
#[doc = "Master Configuration This register configures the mode (Master or Slave) and sets the interface for test mode loopback."]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Configuration This register configures the mode (Master or Slave) and sets the interface for test mode loopback."]
pub mod mcr;
