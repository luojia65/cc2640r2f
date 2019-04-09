#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RFCPEISL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `INTERNAL_ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTERNAL_ERRORR {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl INTERNAL_ERRORR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            INTERNAL_ERRORR::CPE1 => true,
            INTERNAL_ERRORR::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTERNAL_ERRORR {
        match value {
            true => INTERNAL_ERRORR::CPE1,
            false => INTERNAL_ERRORR::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == INTERNAL_ERRORR::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == INTERNAL_ERRORR::CPE0
    }
}
#[doc = "Possible values of the field `BOOT_DONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOT_DONER {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl BOOT_DONER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BOOT_DONER::CPE1 => true,
            BOOT_DONER::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOOT_DONER {
        match value {
            true => BOOT_DONER::CPE1,
            false => BOOT_DONER::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == BOOT_DONER::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == BOOT_DONER::CPE0
    }
}
#[doc = "Possible values of the field `MODULES_UNLOCKED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODULES_UNLOCKEDR {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl MODULES_UNLOCKEDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MODULES_UNLOCKEDR::CPE1 => true,
            MODULES_UNLOCKEDR::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODULES_UNLOCKEDR {
        match value {
            true => MODULES_UNLOCKEDR::CPE1,
            false => MODULES_UNLOCKEDR::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == MODULES_UNLOCKEDR::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == MODULES_UNLOCKEDR::CPE0
    }
}
#[doc = "Possible values of the field `SYNTH_NO_LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNTH_NO_LOCKR {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl SYNTH_NO_LOCKR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SYNTH_NO_LOCKR::CPE1 => true,
            SYNTH_NO_LOCKR::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNTH_NO_LOCKR {
        match value {
            true => SYNTH_NO_LOCKR::CPE1,
            false => SYNTH_NO_LOCKR::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == SYNTH_NO_LOCKR::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == SYNTH_NO_LOCKR::CPE0
    }
}
#[doc = "Possible values of the field `IRQ27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ27R {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl IRQ27R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            IRQ27R::CPE1 => true,
            IRQ27R::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRQ27R {
        match value {
            true => IRQ27R::CPE1,
            false => IRQ27R::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == IRQ27R::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == IRQ27R::CPE0
    }
}
#[doc = "Possible values of the field `RX_ABORTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_ABORTEDR {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl RX_ABORTEDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RX_ABORTEDR::CPE1 => true,
            RX_ABORTEDR::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_ABORTEDR {
        match value {
            true => RX_ABORTEDR::CPE1,
            false => RX_ABORTEDR::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_ABORTEDR::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_ABORTEDR::CPE0
    }
}
#[doc = "Possible values of the field `RX_N_DATA_WRITTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_N_DATA_WRITTENR {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl RX_N_DATA_WRITTENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RX_N_DATA_WRITTENR::CPE1 => true,
            RX_N_DATA_WRITTENR::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_N_DATA_WRITTENR {
        match value {
            true => RX_N_DATA_WRITTENR::CPE1,
            false => RX_N_DATA_WRITTENR::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_N_DATA_WRITTENR::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_N_DATA_WRITTENR::CPE0
    }
}
#[doc = "Possible values of the field `RX_DATA_WRITTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DATA_WRITTENR {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl RX_DATA_WRITTENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RX_DATA_WRITTENR::CPE1 => true,
            RX_DATA_WRITTENR::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_DATA_WRITTENR {
        match value {
            true => RX_DATA_WRITTENR::CPE1,
            false => RX_DATA_WRITTENR::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_DATA_WRITTENR::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_DATA_WRITTENR::CPE0
    }
}
#[doc = "Possible values of the field `RX_ENTRY_DONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_ENTRY_DONER {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl RX_ENTRY_DONER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RX_ENTRY_DONER::CPE1 => true,
            RX_ENTRY_DONER::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_ENTRY_DONER {
        match value {
            true => RX_ENTRY_DONER::CPE1,
            false => RX_ENTRY_DONER::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_ENTRY_DONER::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_ENTRY_DONER::CPE0
    }
}
#[doc = "Possible values of the field `RX_BUF_FULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_BUF_FULLR {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl RX_BUF_FULLR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RX_BUF_FULLR::CPE1 => true,
            RX_BUF_FULLR::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_BUF_FULLR {
        match value {
            true => RX_BUF_FULLR::CPE1,
            false => RX_BUF_FULLR::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_BUF_FULLR::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_BUF_FULLR::CPE0
    }
}
#[doc = "Possible values of the field `RX_CTRL_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_CTRL_ACKR {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl RX_CTRL_ACKR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RX_CTRL_ACKR::CPE1 => true,
            RX_CTRL_ACKR::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_CTRL_ACKR {
        match value {
            true => RX_CTRL_ACKR::CPE1,
            false => RX_CTRL_ACKR::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_CTRL_ACKR::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_CTRL_ACKR::CPE0
    }
}
#[doc = "Possible values of the field `RX_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_CTRLR {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl RX_CTRLR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RX_CTRLR::CPE1 => true,
            RX_CTRLR::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_CTRLR {
        match value {
            true => RX_CTRLR::CPE1,
            false => RX_CTRLR::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_CTRLR::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_CTRLR::CPE0
    }
}
#[doc = "Possible values of the field `RX_EMPTY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_EMPTYR {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl RX_EMPTYR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RX_EMPTYR::CPE1 => true,
            RX_EMPTYR::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_EMPTYR {
        match value {
            true => RX_EMPTYR::CPE1,
            false => RX_EMPTYR::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_EMPTYR::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_EMPTYR::CPE0
    }
}
#[doc = "Possible values of the field `RX_IGNORED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_IGNOREDR {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl RX_IGNOREDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RX_IGNOREDR::CPE1 => true,
            RX_IGNOREDR::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_IGNOREDR {
        match value {
            true => RX_IGNOREDR::CPE1,
            false => RX_IGNOREDR::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_IGNOREDR::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_IGNOREDR::CPE0
    }
}
#[doc = "Possible values of the field `RX_NOK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_NOKR {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl RX_NOKR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RX_NOKR::CPE1 => true,
            RX_NOKR::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_NOKR {
        match value {
            true => RX_NOKR::CPE1,
            false => RX_NOKR::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_NOKR::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_NOKR::CPE0
    }
}
#[doc = "Possible values of the field `RX_OK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_OKR {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl RX_OKR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RX_OKR::CPE1 => true,
            RX_OKR::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_OKR {
        match value {
            true => RX_OKR::CPE1,
            false => RX_OKR::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_OKR::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_OKR::CPE0
    }
}
#[doc = "Possible values of the field `IRQ15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ15R {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl IRQ15R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            IRQ15R::CPE1 => true,
            IRQ15R::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRQ15R {
        match value {
            true => IRQ15R::CPE1,
            false => IRQ15R::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == IRQ15R::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == IRQ15R::CPE0
    }
}
#[doc = "Possible values of the field `IRQ14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ14R {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl IRQ14R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            IRQ14R::CPE1 => true,
            IRQ14R::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRQ14R {
        match value {
            true => IRQ14R::CPE1,
            false => IRQ14R::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == IRQ14R::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == IRQ14R::CPE0
    }
}
#[doc = "Possible values of the field `IRQ13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ13R {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl IRQ13R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            IRQ13R::CPE1 => true,
            IRQ13R::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRQ13R {
        match value {
            true => IRQ13R::CPE1,
            false => IRQ13R::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == IRQ13R::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == IRQ13R::CPE0
    }
}
#[doc = "Possible values of the field `IRQ12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ12R {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl IRQ12R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            IRQ12R::CPE1 => true,
            IRQ12R::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRQ12R {
        match value {
            true => IRQ12R::CPE1,
            false => IRQ12R::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == IRQ12R::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == IRQ12R::CPE0
    }
}
#[doc = "Possible values of the field `TX_BUFFER_CHANGED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_BUFFER_CHANGEDR {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl TX_BUFFER_CHANGEDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TX_BUFFER_CHANGEDR::CPE1 => true,
            TX_BUFFER_CHANGEDR::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_BUFFER_CHANGEDR {
        match value {
            true => TX_BUFFER_CHANGEDR::CPE1,
            false => TX_BUFFER_CHANGEDR::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_BUFFER_CHANGEDR::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_BUFFER_CHANGEDR::CPE0
    }
}
#[doc = "Possible values of the field `TX_ENTRY_DONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_ENTRY_DONER {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl TX_ENTRY_DONER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TX_ENTRY_DONER::CPE1 => true,
            TX_ENTRY_DONER::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_ENTRY_DONER {
        match value {
            true => TX_ENTRY_DONER::CPE1,
            false => TX_ENTRY_DONER::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_ENTRY_DONER::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_ENTRY_DONER::CPE0
    }
}
#[doc = "Possible values of the field `TX_RETRANS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_RETRANSR {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl TX_RETRANSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TX_RETRANSR::CPE1 => true,
            TX_RETRANSR::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_RETRANSR {
        match value {
            true => TX_RETRANSR::CPE1,
            false => TX_RETRANSR::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_RETRANSR::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_RETRANSR::CPE0
    }
}
#[doc = "Possible values of the field `TX_CTRL_ACK_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_CTRL_ACK_ACKR {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl TX_CTRL_ACK_ACKR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TX_CTRL_ACK_ACKR::CPE1 => true,
            TX_CTRL_ACK_ACKR::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_CTRL_ACK_ACKR {
        match value {
            true => TX_CTRL_ACK_ACKR::CPE1,
            false => TX_CTRL_ACK_ACKR::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_CTRL_ACK_ACKR::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_CTRL_ACK_ACKR::CPE0
    }
}
#[doc = "Possible values of the field `TX_CTRL_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_CTRL_ACKR {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl TX_CTRL_ACKR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TX_CTRL_ACKR::CPE1 => true,
            TX_CTRL_ACKR::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_CTRL_ACKR {
        match value {
            true => TX_CTRL_ACKR::CPE1,
            false => TX_CTRL_ACKR::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_CTRL_ACKR::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_CTRL_ACKR::CPE0
    }
}
#[doc = "Possible values of the field `TX_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_CTRLR {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl TX_CTRLR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TX_CTRLR::CPE1 => true,
            TX_CTRLR::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_CTRLR {
        match value {
            true => TX_CTRLR::CPE1,
            false => TX_CTRLR::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_CTRLR::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_CTRLR::CPE0
    }
}
#[doc = "Possible values of the field `TX_ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_ACKR {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl TX_ACKR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TX_ACKR::CPE1 => true,
            TX_ACKR::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_ACKR {
        match value {
            true => TX_ACKR::CPE1,
            false => TX_ACKR::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_ACKR::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_ACKR::CPE0
    }
}
#[doc = "Possible values of the field `TX_DONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_DONER {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl TX_DONER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TX_DONER::CPE1 => true,
            TX_DONER::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_DONER {
        match value {
            true => TX_DONER::CPE1,
            false => TX_DONER::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_DONER::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_DONER::CPE0
    }
}
#[doc = "Possible values of the field `LAST_FG_COMMAND_DONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LAST_FG_COMMAND_DONER {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl LAST_FG_COMMAND_DONER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LAST_FG_COMMAND_DONER::CPE1 => true,
            LAST_FG_COMMAND_DONER::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LAST_FG_COMMAND_DONER {
        match value {
            true => LAST_FG_COMMAND_DONER::CPE1,
            false => LAST_FG_COMMAND_DONER::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == LAST_FG_COMMAND_DONER::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == LAST_FG_COMMAND_DONER::CPE0
    }
}
#[doc = "Possible values of the field `FG_COMMAND_DONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FG_COMMAND_DONER {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl FG_COMMAND_DONER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FG_COMMAND_DONER::CPE1 => true,
            FG_COMMAND_DONER::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FG_COMMAND_DONER {
        match value {
            true => FG_COMMAND_DONER::CPE1,
            false => FG_COMMAND_DONER::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == FG_COMMAND_DONER::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == FG_COMMAND_DONER::CPE0
    }
}
#[doc = "Possible values of the field `LAST_COMMAND_DONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LAST_COMMAND_DONER {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl LAST_COMMAND_DONER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LAST_COMMAND_DONER::CPE1 => true,
            LAST_COMMAND_DONER::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LAST_COMMAND_DONER {
        match value {
            true => LAST_COMMAND_DONER::CPE1,
            false => LAST_COMMAND_DONER::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == LAST_COMMAND_DONER::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == LAST_COMMAND_DONER::CPE0
    }
}
#[doc = "Possible values of the field `COMMAND_DONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMMAND_DONER {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl COMMAND_DONER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            COMMAND_DONER::CPE1 => true,
            COMMAND_DONER::CPE0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMMAND_DONER {
        match value {
            true => COMMAND_DONER::CPE1,
            false => COMMAND_DONER::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline]
    pub fn is_cpe1(&self) -> bool {
        *self == COMMAND_DONER::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline]
    pub fn is_cpe0(&self) -> bool {
        *self == COMMAND_DONER::CPE0
    }
}
#[doc = "Values that can be written to the field `INTERNAL_ERROR`"]
pub enum INTERNAL_ERRORW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl INTERNAL_ERRORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTERNAL_ERRORW::CPE1 => true,
            INTERNAL_ERRORW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTERNAL_ERRORW<'a> {
    w: &'a mut W,
}
impl<'a> _INTERNAL_ERRORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTERNAL_ERRORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(INTERNAL_ERRORW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(INTERNAL_ERRORW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BOOT_DONE`"]
pub enum BOOT_DONEW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl BOOT_DONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOOT_DONEW::CPE1 => true,
            BOOT_DONEW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOOT_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOT_DONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOOT_DONEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(BOOT_DONEW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(BOOT_DONEW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODULES_UNLOCKED`"]
pub enum MODULES_UNLOCKEDW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl MODULES_UNLOCKEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODULES_UNLOCKEDW::CPE1 => true,
            MODULES_UNLOCKEDW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODULES_UNLOCKEDW<'a> {
    w: &'a mut W,
}
impl<'a> _MODULES_UNLOCKEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODULES_UNLOCKEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(MODULES_UNLOCKEDW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(MODULES_UNLOCKEDW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNTH_NO_LOCK`"]
pub enum SYNTH_NO_LOCKW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl SYNTH_NO_LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNTH_NO_LOCKW::CPE1 => true,
            SYNTH_NO_LOCKW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNTH_NO_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNTH_NO_LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNTH_NO_LOCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(SYNTH_NO_LOCKW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(SYNTH_NO_LOCKW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IRQ27`"]
pub enum IRQ27W {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl IRQ27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRQ27W::CPE1 => true,
            IRQ27W::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRQ27W<'a> {
    w: &'a mut W,
}
impl<'a> _IRQ27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRQ27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(IRQ27W::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(IRQ27W::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_ABORTED`"]
pub enum RX_ABORTEDW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl RX_ABORTEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_ABORTEDW::CPE1 => true,
            RX_ABORTEDW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_ABORTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_ABORTEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_ABORTEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_ABORTEDW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_ABORTEDW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_N_DATA_WRITTEN`"]
pub enum RX_N_DATA_WRITTENW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl RX_N_DATA_WRITTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_N_DATA_WRITTENW::CPE1 => true,
            RX_N_DATA_WRITTENW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_N_DATA_WRITTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_N_DATA_WRITTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_N_DATA_WRITTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_N_DATA_WRITTENW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_N_DATA_WRITTENW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_DATA_WRITTEN`"]
pub enum RX_DATA_WRITTENW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl RX_DATA_WRITTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_DATA_WRITTENW::CPE1 => true,
            RX_DATA_WRITTENW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_DATA_WRITTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_DATA_WRITTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_DATA_WRITTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_DATA_WRITTENW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_DATA_WRITTENW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_ENTRY_DONE`"]
pub enum RX_ENTRY_DONEW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl RX_ENTRY_DONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_ENTRY_DONEW::CPE1 => true,
            RX_ENTRY_DONEW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_ENTRY_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_ENTRY_DONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_ENTRY_DONEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_ENTRY_DONEW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_ENTRY_DONEW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_BUF_FULL`"]
pub enum RX_BUF_FULLW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl RX_BUF_FULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_BUF_FULLW::CPE1 => true,
            RX_BUF_FULLW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_BUF_FULLW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_BUF_FULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_BUF_FULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_BUF_FULLW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_BUF_FULLW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_CTRL_ACK`"]
pub enum RX_CTRL_ACKW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl RX_CTRL_ACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_CTRL_ACKW::CPE1 => true,
            RX_CTRL_ACKW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_CTRL_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_CTRL_ACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_CTRL_ACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_CTRL_ACKW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_CTRL_ACKW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_CTRL`"]
pub enum RX_CTRLW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl RX_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_CTRLW::CPE1 => true,
            RX_CTRLW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_CTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_CTRLW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_CTRLW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_EMPTY`"]
pub enum RX_EMPTYW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl RX_EMPTYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_EMPTYW::CPE1 => true,
            RX_EMPTYW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_EMPTYW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_EMPTYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_EMPTYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_EMPTYW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_EMPTYW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_IGNORED`"]
pub enum RX_IGNOREDW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl RX_IGNOREDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_IGNOREDW::CPE1 => true,
            RX_IGNOREDW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_IGNOREDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_IGNOREDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_IGNOREDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_IGNOREDW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_IGNOREDW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_NOK`"]
pub enum RX_NOKW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl RX_NOKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_NOKW::CPE1 => true,
            RX_NOKW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_NOKW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_NOKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_NOKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_NOKW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_NOKW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_OK`"]
pub enum RX_OKW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl RX_OKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_OKW::CPE1 => true,
            RX_OKW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_OKW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_OKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_OKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_OKW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_OKW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IRQ15`"]
pub enum IRQ15W {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl IRQ15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRQ15W::CPE1 => true,
            IRQ15W::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRQ15W<'a> {
    w: &'a mut W,
}
impl<'a> _IRQ15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRQ15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(IRQ15W::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(IRQ15W::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IRQ14`"]
pub enum IRQ14W {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl IRQ14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRQ14W::CPE1 => true,
            IRQ14W::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRQ14W<'a> {
    w: &'a mut W,
}
impl<'a> _IRQ14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRQ14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(IRQ14W::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(IRQ14W::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IRQ13`"]
pub enum IRQ13W {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl IRQ13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRQ13W::CPE1 => true,
            IRQ13W::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRQ13W<'a> {
    w: &'a mut W,
}
impl<'a> _IRQ13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRQ13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(IRQ13W::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(IRQ13W::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IRQ12`"]
pub enum IRQ12W {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl IRQ12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRQ12W::CPE1 => true,
            IRQ12W::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRQ12W<'a> {
    w: &'a mut W,
}
impl<'a> _IRQ12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRQ12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(IRQ12W::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(IRQ12W::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TX_BUFFER_CHANGED`"]
pub enum TX_BUFFER_CHANGEDW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl TX_BUFFER_CHANGEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_BUFFER_CHANGEDW::CPE1 => true,
            TX_BUFFER_CHANGEDW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_BUFFER_CHANGEDW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_BUFFER_CHANGEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_BUFFER_CHANGEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_BUFFER_CHANGEDW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_BUFFER_CHANGEDW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TX_ENTRY_DONE`"]
pub enum TX_ENTRY_DONEW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl TX_ENTRY_DONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_ENTRY_DONEW::CPE1 => true,
            TX_ENTRY_DONEW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_ENTRY_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_ENTRY_DONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_ENTRY_DONEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_ENTRY_DONEW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_ENTRY_DONEW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TX_RETRANS`"]
pub enum TX_RETRANSW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl TX_RETRANSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_RETRANSW::CPE1 => true,
            TX_RETRANSW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_RETRANSW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_RETRANSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_RETRANSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_RETRANSW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_RETRANSW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TX_CTRL_ACK_ACK`"]
pub enum TX_CTRL_ACK_ACKW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl TX_CTRL_ACK_ACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_CTRL_ACK_ACKW::CPE1 => true,
            TX_CTRL_ACK_ACKW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_CTRL_ACK_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_CTRL_ACK_ACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_CTRL_ACK_ACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_CTRL_ACK_ACKW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_CTRL_ACK_ACKW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TX_CTRL_ACK`"]
pub enum TX_CTRL_ACKW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl TX_CTRL_ACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_CTRL_ACKW::CPE1 => true,
            TX_CTRL_ACKW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_CTRL_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_CTRL_ACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_CTRL_ACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_CTRL_ACKW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_CTRL_ACKW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TX_CTRL`"]
pub enum TX_CTRLW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl TX_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_CTRLW::CPE1 => true,
            TX_CTRLW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_CTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_CTRLW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_CTRLW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TX_ACK`"]
pub enum TX_ACKW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl TX_ACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_ACKW::CPE1 => true,
            TX_ACKW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_ACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_ACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_ACKW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_ACKW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TX_DONE`"]
pub enum TX_DONEW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl TX_DONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_DONEW::CPE1 => true,
            TX_DONEW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_DONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_DONEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_DONEW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_DONEW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LAST_FG_COMMAND_DONE`"]
pub enum LAST_FG_COMMAND_DONEW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl LAST_FG_COMMAND_DONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LAST_FG_COMMAND_DONEW::CPE1 => true,
            LAST_FG_COMMAND_DONEW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LAST_FG_COMMAND_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _LAST_FG_COMMAND_DONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LAST_FG_COMMAND_DONEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(LAST_FG_COMMAND_DONEW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(LAST_FG_COMMAND_DONEW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FG_COMMAND_DONE`"]
pub enum FG_COMMAND_DONEW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl FG_COMMAND_DONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FG_COMMAND_DONEW::CPE1 => true,
            FG_COMMAND_DONEW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FG_COMMAND_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _FG_COMMAND_DONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FG_COMMAND_DONEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(FG_COMMAND_DONEW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(FG_COMMAND_DONEW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LAST_COMMAND_DONE`"]
pub enum LAST_COMMAND_DONEW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl LAST_COMMAND_DONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LAST_COMMAND_DONEW::CPE1 => true,
            LAST_COMMAND_DONEW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LAST_COMMAND_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _LAST_COMMAND_DONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LAST_COMMAND_DONEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(LAST_COMMAND_DONEW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(LAST_COMMAND_DONEW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COMMAND_DONE`"]
pub enum COMMAND_DONEW {
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1,
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0,
}
impl COMMAND_DONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMMAND_DONEW::CPE1 => true,
            COMMAND_DONEW::CPE0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMMAND_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _COMMAND_DONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMMAND_DONEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(COMMAND_DONEW::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(COMMAND_DONEW::CPE0)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 31 - 31:31\\] Select which CPU interrupt vector the RFCPEIFG.INTERNAL_ERROR interrupt should use."]
    #[inline]
    pub fn internal_error(&self) -> INTERNAL_ERRORR {
        INTERNAL_ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - 30:30\\] Select which CPU interrupt vector the RFCPEIFG.BOOT_DONE interrupt should use."]
    #[inline]
    pub fn boot_done(&self) -> BOOT_DONER {
        BOOT_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - 29:29\\] Select which CPU interrupt vector the RFCPEIFG.MODULES_UNLOCKED interrupt should use."]
    #[inline]
    pub fn modules_unlocked(&self) -> MODULES_UNLOCKEDR {
        MODULES_UNLOCKEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - 28:28\\] Select which CPU interrupt vector the RFCPEIFG.SYNTH_NO_LOCK interrupt should use."]
    #[inline]
    pub fn synth_no_lock(&self) -> SYNTH_NO_LOCKR {
        SYNTH_NO_LOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - 27:27\\] Select which CPU interrupt vector the RFCPEIFG.IRQ27 interrupt should use."]
    #[inline]
    pub fn irq27(&self) -> IRQ27R {
        IRQ27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - 26:26\\] Select which CPU interrupt vector the RFCPEIFG.RX_ABORTED interrupt should use."]
    #[inline]
    pub fn rx_aborted(&self) -> RX_ABORTEDR {
        RX_ABORTEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - 25:25\\] Select which CPU interrupt vector the RFCPEIFG.RX_N_DATA_WRITTEN interrupt should use."]
    #[inline]
    pub fn rx_n_data_written(&self) -> RX_N_DATA_WRITTENR {
        RX_N_DATA_WRITTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - 24:24\\] Select which CPU interrupt vector the RFCPEIFG.RX_DATA_WRITTEN interrupt should use."]
    #[inline]
    pub fn rx_data_written(&self) -> RX_DATA_WRITTENR {
        RX_DATA_WRITTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - 23:23\\] Select which CPU interrupt vector the RFCPEIFG.RX_ENTRY_DONE interrupt should use."]
    #[inline]
    pub fn rx_entry_done(&self) -> RX_ENTRY_DONER {
        RX_ENTRY_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - 22:22\\] Select which CPU interrupt vector the RFCPEIFG.RX_BUF_FULL interrupt should use."]
    #[inline]
    pub fn rx_buf_full(&self) -> RX_BUF_FULLR {
        RX_BUF_FULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - 21:21\\] Select which CPU interrupt vector the RFCPEIFG.RX_CTRL_ACK interrupt should use."]
    #[inline]
    pub fn rx_ctrl_ack(&self) -> RX_CTRL_ACKR {
        RX_CTRL_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - 20:20\\] Select which CPU interrupt vector the RFCPEIFG.RX_CTRL interrupt should use."]
    #[inline]
    pub fn rx_ctrl(&self) -> RX_CTRLR {
        RX_CTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - 19:19\\] Select which CPU interrupt vector the RFCPEIFG.RX_EMPTY interrupt should use."]
    #[inline]
    pub fn rx_empty(&self) -> RX_EMPTYR {
        RX_EMPTYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - 18:18\\] Select which CPU interrupt vector the RFCPEIFG.RX_IGNORED interrupt should use."]
    #[inline]
    pub fn rx_ignored(&self) -> RX_IGNOREDR {
        RX_IGNOREDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - 17:17\\] Select which CPU interrupt vector the RFCPEIFG.RX_NOK interrupt should use."]
    #[inline]
    pub fn rx_nok(&self) -> RX_NOKR {
        RX_NOKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - 16:16\\] Select which CPU interrupt vector the RFCPEIFG.RX_OK interrupt should use."]
    #[inline]
    pub fn rx_ok(&self) -> RX_OKR {
        RX_OKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - 15:15\\] Select which CPU interrupt vector the RFCPEIFG.IRQ15 interrupt should use."]
    #[inline]
    pub fn irq15(&self) -> IRQ15R {
        IRQ15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - 14:14\\] Select which CPU interrupt vector the RFCPEIFG.IRQ14 interrupt should use."]
    #[inline]
    pub fn irq14(&self) -> IRQ14R {
        IRQ14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - 13:13\\] Select which CPU interrupt vector the RFCPEIFG.IRQ13 interrupt should use."]
    #[inline]
    pub fn irq13(&self) -> IRQ13R {
        IRQ13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - 12:12\\] Select which CPU interrupt vector the RFCPEIFG.IRQ12 interrupt should use."]
    #[inline]
    pub fn irq12(&self) -> IRQ12R {
        IRQ12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - 11:11\\] Select which CPU interrupt vector the RFCPEIFG.TX_BUFFER_CHANGED interrupt should use."]
    #[inline]
    pub fn tx_buffer_changed(&self) -> TX_BUFFER_CHANGEDR {
        TX_BUFFER_CHANGEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - 10:10\\] Select which CPU interrupt vector the RFCPEIFG.TX_ENTRY_DONE interrupt should use."]
    #[inline]
    pub fn tx_entry_done(&self) -> TX_ENTRY_DONER {
        TX_ENTRY_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - 9:9\\] Select which CPU interrupt vector the RFCPEIFG.TX_RETRANS interrupt should use."]
    #[inline]
    pub fn tx_retrans(&self) -> TX_RETRANSR {
        TX_RETRANSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - 8:8\\] Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK_ACK interrupt should use."]
    #[inline]
    pub fn tx_ctrl_ack_ack(&self) -> TX_CTRL_ACK_ACKR {
        TX_CTRL_ACK_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - 7:7\\] Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK interrupt should use."]
    #[inline]
    pub fn tx_ctrl_ack(&self) -> TX_CTRL_ACKR {
        TX_CTRL_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - 6:6\\] Select which CPU interrupt vector the RFCPEIFG.TX_CTRL interrupt should use."]
    #[inline]
    pub fn tx_ctrl(&self) -> TX_CTRLR {
        TX_CTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - 5:5\\] Select which CPU interrupt vector the RFCPEIFG.TX_ACK interrupt should use."]
    #[inline]
    pub fn tx_ack(&self) -> TX_ACKR {
        TX_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - 4:4\\] Select which CPU interrupt vector the RFCPEIFG.TX_DONE interrupt should use."]
    #[inline]
    pub fn tx_done(&self) -> TX_DONER {
        TX_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - 3:3\\] Select which CPU interrupt vector the RFCPEIFG.LAST_FG_COMMAND_DONE interrupt should use."]
    #[inline]
    pub fn last_fg_command_done(&self) -> LAST_FG_COMMAND_DONER {
        LAST_FG_COMMAND_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - 2:2\\] Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_DONE interrupt should use."]
    #[inline]
    pub fn fg_command_done(&self) -> FG_COMMAND_DONER {
        FG_COMMAND_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - 1:1\\] Select which CPU interrupt vector the RFCPEIFG.LAST_COMMAND_DONE interrupt should use."]
    #[inline]
    pub fn last_command_done(&self) -> LAST_COMMAND_DONER {
        LAST_COMMAND_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - 0:0\\] Select which CPU interrupt vector the RFCPEIFG.COMMAND_DONE interrupt should use."]
    #[inline]
    pub fn command_done(&self) -> COMMAND_DONER {
        COMMAND_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4294901760 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 31 - 31:31\\] Select which CPU interrupt vector the RFCPEIFG.INTERNAL_ERROR interrupt should use."]
    #[inline]
    pub fn internal_error(&mut self) -> _INTERNAL_ERRORW {
        _INTERNAL_ERRORW { w: self }
    }
    #[doc = "Bit 30 - 30:30\\] Select which CPU interrupt vector the RFCPEIFG.BOOT_DONE interrupt should use."]
    #[inline]
    pub fn boot_done(&mut self) -> _BOOT_DONEW {
        _BOOT_DONEW { w: self }
    }
    #[doc = "Bit 29 - 29:29\\] Select which CPU interrupt vector the RFCPEIFG.MODULES_UNLOCKED interrupt should use."]
    #[inline]
    pub fn modules_unlocked(&mut self) -> _MODULES_UNLOCKEDW {
        _MODULES_UNLOCKEDW { w: self }
    }
    #[doc = "Bit 28 - 28:28\\] Select which CPU interrupt vector the RFCPEIFG.SYNTH_NO_LOCK interrupt should use."]
    #[inline]
    pub fn synth_no_lock(&mut self) -> _SYNTH_NO_LOCKW {
        _SYNTH_NO_LOCKW { w: self }
    }
    #[doc = "Bit 27 - 27:27\\] Select which CPU interrupt vector the RFCPEIFG.IRQ27 interrupt should use."]
    #[inline]
    pub fn irq27(&mut self) -> _IRQ27W {
        _IRQ27W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\] Select which CPU interrupt vector the RFCPEIFG.RX_ABORTED interrupt should use."]
    #[inline]
    pub fn rx_aborted(&mut self) -> _RX_ABORTEDW {
        _RX_ABORTEDW { w: self }
    }
    #[doc = "Bit 25 - 25:25\\] Select which CPU interrupt vector the RFCPEIFG.RX_N_DATA_WRITTEN interrupt should use."]
    #[inline]
    pub fn rx_n_data_written(&mut self) -> _RX_N_DATA_WRITTENW {
        _RX_N_DATA_WRITTENW { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] Select which CPU interrupt vector the RFCPEIFG.RX_DATA_WRITTEN interrupt should use."]
    #[inline]
    pub fn rx_data_written(&mut self) -> _RX_DATA_WRITTENW {
        _RX_DATA_WRITTENW { w: self }
    }
    #[doc = "Bit 23 - 23:23\\] Select which CPU interrupt vector the RFCPEIFG.RX_ENTRY_DONE interrupt should use."]
    #[inline]
    pub fn rx_entry_done(&mut self) -> _RX_ENTRY_DONEW {
        _RX_ENTRY_DONEW { w: self }
    }
    #[doc = "Bit 22 - 22:22\\] Select which CPU interrupt vector the RFCPEIFG.RX_BUF_FULL interrupt should use."]
    #[inline]
    pub fn rx_buf_full(&mut self) -> _RX_BUF_FULLW {
        _RX_BUF_FULLW { w: self }
    }
    #[doc = "Bit 21 - 21:21\\] Select which CPU interrupt vector the RFCPEIFG.RX_CTRL_ACK interrupt should use."]
    #[inline]
    pub fn rx_ctrl_ack(&mut self) -> _RX_CTRL_ACKW {
        _RX_CTRL_ACKW { w: self }
    }
    #[doc = "Bit 20 - 20:20\\] Select which CPU interrupt vector the RFCPEIFG.RX_CTRL interrupt should use."]
    #[inline]
    pub fn rx_ctrl(&mut self) -> _RX_CTRLW {
        _RX_CTRLW { w: self }
    }
    #[doc = "Bit 19 - 19:19\\] Select which CPU interrupt vector the RFCPEIFG.RX_EMPTY interrupt should use."]
    #[inline]
    pub fn rx_empty(&mut self) -> _RX_EMPTYW {
        _RX_EMPTYW { w: self }
    }
    #[doc = "Bit 18 - 18:18\\] Select which CPU interrupt vector the RFCPEIFG.RX_IGNORED interrupt should use."]
    #[inline]
    pub fn rx_ignored(&mut self) -> _RX_IGNOREDW {
        _RX_IGNOREDW { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] Select which CPU interrupt vector the RFCPEIFG.RX_NOK interrupt should use."]
    #[inline]
    pub fn rx_nok(&mut self) -> _RX_NOKW {
        _RX_NOKW { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] Select which CPU interrupt vector the RFCPEIFG.RX_OK interrupt should use."]
    #[inline]
    pub fn rx_ok(&mut self) -> _RX_OKW {
        _RX_OKW { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] Select which CPU interrupt vector the RFCPEIFG.IRQ15 interrupt should use."]
    #[inline]
    pub fn irq15(&mut self) -> _IRQ15W {
        _IRQ15W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] Select which CPU interrupt vector the RFCPEIFG.IRQ14 interrupt should use."]
    #[inline]
    pub fn irq14(&mut self) -> _IRQ14W {
        _IRQ14W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] Select which CPU interrupt vector the RFCPEIFG.IRQ13 interrupt should use."]
    #[inline]
    pub fn irq13(&mut self) -> _IRQ13W {
        _IRQ13W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] Select which CPU interrupt vector the RFCPEIFG.IRQ12 interrupt should use."]
    #[inline]
    pub fn irq12(&mut self) -> _IRQ12W {
        _IRQ12W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Select which CPU interrupt vector the RFCPEIFG.TX_BUFFER_CHANGED interrupt should use."]
    #[inline]
    pub fn tx_buffer_changed(&mut self) -> _TX_BUFFER_CHANGEDW {
        _TX_BUFFER_CHANGEDW { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Select which CPU interrupt vector the RFCPEIFG.TX_ENTRY_DONE interrupt should use."]
    #[inline]
    pub fn tx_entry_done(&mut self) -> _TX_ENTRY_DONEW {
        _TX_ENTRY_DONEW { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Select which CPU interrupt vector the RFCPEIFG.TX_RETRANS interrupt should use."]
    #[inline]
    pub fn tx_retrans(&mut self) -> _TX_RETRANSW {
        _TX_RETRANSW { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK_ACK interrupt should use."]
    #[inline]
    pub fn tx_ctrl_ack_ack(&mut self) -> _TX_CTRL_ACK_ACKW {
        _TX_CTRL_ACK_ACKW { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK interrupt should use."]
    #[inline]
    pub fn tx_ctrl_ack(&mut self) -> _TX_CTRL_ACKW {
        _TX_CTRL_ACKW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Select which CPU interrupt vector the RFCPEIFG.TX_CTRL interrupt should use."]
    #[inline]
    pub fn tx_ctrl(&mut self) -> _TX_CTRLW {
        _TX_CTRLW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Select which CPU interrupt vector the RFCPEIFG.TX_ACK interrupt should use."]
    #[inline]
    pub fn tx_ack(&mut self) -> _TX_ACKW {
        _TX_ACKW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Select which CPU interrupt vector the RFCPEIFG.TX_DONE interrupt should use."]
    #[inline]
    pub fn tx_done(&mut self) -> _TX_DONEW {
        _TX_DONEW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Select which CPU interrupt vector the RFCPEIFG.LAST_FG_COMMAND_DONE interrupt should use."]
    #[inline]
    pub fn last_fg_command_done(&mut self) -> _LAST_FG_COMMAND_DONEW {
        _LAST_FG_COMMAND_DONEW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_DONE interrupt should use."]
    #[inline]
    pub fn fg_command_done(&mut self) -> _FG_COMMAND_DONEW {
        _FG_COMMAND_DONEW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Select which CPU interrupt vector the RFCPEIFG.LAST_COMMAND_DONE interrupt should use."]
    #[inline]
    pub fn last_command_done(&mut self) -> _LAST_COMMAND_DONEW {
        _LAST_COMMAND_DONEW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Select which CPU interrupt vector the RFCPEIFG.COMMAND_DONE interrupt should use."]
    #[inline]
    pub fn command_done(&mut self) -> _COMMAND_DONEW {
        _COMMAND_DONEW { w: self }
    }
}
