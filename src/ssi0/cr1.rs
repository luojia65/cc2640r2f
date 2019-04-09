#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR1 {
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
#[doc = r" Value of the field"]
pub struct SODR {
    bits: bool,
}
impl SODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `MS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSR {
    #[doc = "Device configured as slave"]
    SLAVE,
    #[doc = "Device configured as master"]
    MASTER,
}
impl MSR {
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
            MSR::SLAVE => true,
            MSR::MASTER => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSR {
        match value {
            true => MSR::SLAVE,
            false => MSR::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline]
    pub fn is_slave(&self) -> bool {
        *self == MSR::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline]
    pub fn is_master(&self) -> bool {
        *self == MSR::MASTER
    }
}
#[doc = "Possible values of the field `SSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSER {
    #[doc = "Operation enabled"]
    SSI_ENABLED,
    #[doc = "Operation disabled"]
    SSI_DISABLED,
}
impl SSER {
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
            SSER::SSI_ENABLED => true,
            SSER::SSI_DISABLED => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSER {
        match value {
            true => SSER::SSI_ENABLED,
            false => SSER::SSI_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `SSI_ENABLED`"]
    #[inline]
    pub fn is_ssi_enabled(&self) -> bool {
        *self == SSER::SSI_ENABLED
    }
    #[doc = "Checks if the value of the field is `SSI_DISABLED`"]
    #[inline]
    pub fn is_ssi_disabled(&self) -> bool {
        *self == SSER::SSI_DISABLED
    }
}
#[doc = r" Value of the field"]
pub struct LBMR {
    bits: bool,
}
impl LBMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Proxy"]
pub struct _SODW<'a> {
    w: &'a mut W,
}
impl<'a> _SODW<'a> {
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
#[doc = "Values that can be written to the field `MS`"]
pub enum MSW {
    #[doc = "Device configured as slave"]
    SLAVE,
    #[doc = "Device configured as master"]
    MASTER,
}
impl MSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSW::SLAVE => true,
            MSW::MASTER => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSW<'a> {
    w: &'a mut W,
}
impl<'a> _MSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Device configured as slave"]
    #[inline]
    pub fn slave(self) -> &'a mut W {
        self.variant(MSW::SLAVE)
    }
    #[doc = "Device configured as master"]
    #[inline]
    pub fn master(self) -> &'a mut W {
        self.variant(MSW::MASTER)
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
#[doc = "Values that can be written to the field `SSE`"]
pub enum SSEW {
    #[doc = "Operation enabled"]
    SSI_ENABLED,
    #[doc = "Operation disabled"]
    SSI_DISABLED,
}
impl SSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSEW::SSI_ENABLED => true,
            SSEW::SSI_DISABLED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Operation enabled"]
    #[inline]
    pub fn ssi_enabled(self) -> &'a mut W {
        self.variant(SSEW::SSI_ENABLED)
    }
    #[doc = "Operation disabled"]
    #[inline]
    pub fn ssi_disabled(self) -> &'a mut W {
        self.variant(SSEW::SSI_DISABLED)
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
#[doc = r" Proxy"]
pub struct _LBMW<'a> {
    w: &'a mut W,
}
impl<'a> _LBMW<'a> {
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
    #[doc = "Bit 3 - 3:3\\] Slave-mode output disabled This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an SSI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SSI slave is not supposed to drive the TXD line: 0: SSI can drive the TXD output in slave mode. 1: SSI cannot drive the TXD output in slave mode."]
    #[inline]
    pub fn sod(&self) -> SODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SODR { bits }
    }
    #[doc = "Bit 2 - 2:2\\] Master or slave mode select. This bit can be modified only when SSI is disabled, SSE=0."]
    #[inline]
    pub fn ms(&self) -> MSR {
        MSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - 1:1\\] Synchronous serial interface enable."]
    #[inline]
    pub fn sse(&self) -> SSER {
        SSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - 0:0\\] Loop back mode: 0: Normal serial port operation enabled. 1: Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    #[inline]
    pub fn lbm(&self) -> LBMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LBMR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 3 - 3:3\\] Slave-mode output disabled This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an SSI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SSI slave is not supposed to drive the TXD line: 0: SSI can drive the TXD output in slave mode. 1: SSI cannot drive the TXD output in slave mode."]
    #[inline]
    pub fn sod(&mut self) -> _SODW {
        _SODW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Master or slave mode select. This bit can be modified only when SSI is disabled, SSE=0."]
    #[inline]
    pub fn ms(&mut self) -> _MSW {
        _MSW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Synchronous serial interface enable."]
    #[inline]
    pub fn sse(&mut self) -> _SSEW {
        _SSEW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Loop back mode: 0: Normal serial port operation enabled. 1: Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    #[inline]
    pub fn lbm(&mut self) -> _LBMW {
        _LBMW { w: self }
    }
}
