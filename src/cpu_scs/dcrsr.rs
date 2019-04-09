#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCRSR {
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
pub struct RESERVED17R {
    bits: u16,
}
impl RESERVED17R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REGWNRR {
    bits: bool,
}
impl REGWNRR {
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
#[doc = r" Value of the field"]
pub struct RESERVED5R {
    bits: u16,
}
impl RESERVED5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REGSELR {
    bits: u8,
}
impl REGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED17W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED17W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 32767;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _REGWNRW<'a> {
    w: &'a mut W,
}
impl<'a> _REGWNRW<'a> {
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
#[doc = r" Proxy"]
pub struct _RESERVED5W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED5W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 2047;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _REGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _REGSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
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
    #[doc = "Bits 17:31 - 31:17\\] Software should not rely on the value of a reserved. Write 0."]
    #[inline]
    pub fn reserved17(&self) -> RESERVED17R {
        let bits = {
            const MASK: u16 = 32767;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED17R { bits }
    }
    #[doc = "Bit 16 - 16:16\\] 1: Write 0: Read"]
    #[inline]
    pub fn regwnr(&self) -> REGWNRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        REGWNRR { bits }
    }
    #[doc = "Bits 5:15 - 15:5\\] Software should not rely on the value of a reserved. Write 0."]
    #[inline]
    pub fn reserved5(&self) -> RESERVED5R {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED5R { bits }
    }
    #[doc = "Bits 0:4 - 4:0\\] Register select 0x00: R0 0x01: R1 0x02: R2 0x03: R3 0x04: R4 0x05: R5 0x06: R6 0x07: R7 0x08: R8 0x09: R9 0x0A: R10 0x0B: R11 0x0C: R12 0x0D: Current SP 0x0E: LR 0x0F: DebugReturnAddress 0x10: XPSR/flags, execution state information, and exception number 0x11: MSP (Main SP) 0x12: PSP (Process SP) 0x14: CONTROL<<24 | FAULTMASK<<16 | BASEPRI<<8 | PRIMASK"]
    #[inline]
    pub fn regsel(&self) -> REGSELR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REGSELR { bits }
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
    #[doc = "Bits 17:31 - 31:17\\] Software should not rely on the value of a reserved. Write 0."]
    #[inline]
    pub fn reserved17(&mut self) -> _RESERVED17W {
        _RESERVED17W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] 1: Write 0: Read"]
    #[inline]
    pub fn regwnr(&mut self) -> _REGWNRW {
        _REGWNRW { w: self }
    }
    #[doc = "Bits 5:15 - 15:5\\] Software should not rely on the value of a reserved. Write 0."]
    #[inline]
    pub fn reserved5(&mut self) -> _RESERVED5W {
        _RESERVED5W { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\] Register select 0x00: R0 0x01: R1 0x02: R2 0x03: R3 0x04: R4 0x05: R5 0x06: R6 0x07: R7 0x08: R8 0x09: R9 0x0A: R10 0x0B: R11 0x0C: R12 0x0D: Current SP 0x0E: LR 0x0F: DebugReturnAddress 0x10: XPSR/flags, execution state information, and exception number 0x11: MSP (Main SP) 0x12: PSP (Process SP) 0x14: CONTROL<<24 | FAULTMASK<<16 | BASEPRI<<8 | PRIMASK"]
    #[inline]
    pub fn regsel(&mut self) -> _REGSELW {
        _REGSELW { w: self }
    }
}
