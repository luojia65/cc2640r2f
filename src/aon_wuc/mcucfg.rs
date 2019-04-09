#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCUCFG {
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
pub struct RESERVED18R {
    bits: u16,
}
impl RESERVED18R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VIRT_OFFR {
    bits: bool,
}
impl VIRT_OFFR {
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
pub struct FIXED_WU_ENR {
    bits: bool,
}
impl FIXED_WU_ENR {
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
pub struct RESERVED4R {
    bits: u16,
}
impl RESERVED4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `SRAM_RET_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_RET_ENR {
    #[doc = "Retention on for all banks (SRAM:BANK0, SRAM:BANK1 ,SRAM:BANK2 and SRAM:BANK3) "]
    RET_FULL,
    #[doc = "Retention on for SRAM:BANK0, SRAM:BANK1 and SRAM:BANK2 "]
    RET_LEVEL3,
    #[doc = "Retention on for SRAM:BANK0 and SRAM:BANK1 "]
    RET_LEVEL2,
    #[doc = "Retention on for SRAM:BANK0 "]
    RET_LEVEL1,
    #[doc = "Retention is disabled"]
    RET_NONE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SRAM_RET_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRAM_RET_ENR::RET_FULL => 15,
            SRAM_RET_ENR::RET_LEVEL3 => 7,
            SRAM_RET_ENR::RET_LEVEL2 => 3,
            SRAM_RET_ENR::RET_LEVEL1 => 1,
            SRAM_RET_ENR::RET_NONE => 0,
            SRAM_RET_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRAM_RET_ENR {
        match value {
            15 => SRAM_RET_ENR::RET_FULL,
            7 => SRAM_RET_ENR::RET_LEVEL3,
            3 => SRAM_RET_ENR::RET_LEVEL2,
            1 => SRAM_RET_ENR::RET_LEVEL1,
            0 => SRAM_RET_ENR::RET_NONE,
            i => SRAM_RET_ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RET_FULL`"]
    #[inline]
    pub fn is_ret_full(&self) -> bool {
        *self == SRAM_RET_ENR::RET_FULL
    }
    #[doc = "Checks if the value of the field is `RET_LEVEL3`"]
    #[inline]
    pub fn is_ret_level3(&self) -> bool {
        *self == SRAM_RET_ENR::RET_LEVEL3
    }
    #[doc = "Checks if the value of the field is `RET_LEVEL2`"]
    #[inline]
    pub fn is_ret_level2(&self) -> bool {
        *self == SRAM_RET_ENR::RET_LEVEL2
    }
    #[doc = "Checks if the value of the field is `RET_LEVEL1`"]
    #[inline]
    pub fn is_ret_level1(&self) -> bool {
        *self == SRAM_RET_ENR::RET_LEVEL1
    }
    #[doc = "Checks if the value of the field is `RET_NONE`"]
    #[inline]
    pub fn is_ret_none(&self) -> bool {
        *self == SRAM_RET_ENR::RET_NONE
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED18W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED18W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 16383;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIRT_OFFW<'a> {
    w: &'a mut W,
}
impl<'a> _VIRT_OFFW<'a> {
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
#[doc = r" Proxy"]
pub struct _FIXED_WU_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FIXED_WU_ENW<'a> {
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
pub struct _RESERVED4W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRAM_RET_EN`"]
pub enum SRAM_RET_ENW {
    #[doc = "Retention on for all banks (SRAM:BANK0, SRAM:BANK1 ,SRAM:BANK2 and SRAM:BANK3) "]
    RET_FULL,
    #[doc = "Retention on for SRAM:BANK0, SRAM:BANK1 and SRAM:BANK2 "]
    RET_LEVEL3,
    #[doc = "Retention on for SRAM:BANK0 and SRAM:BANK1 "]
    RET_LEVEL2,
    #[doc = "Retention on for SRAM:BANK0 "]
    RET_LEVEL1,
    #[doc = "Retention is disabled"]
    RET_NONE,
}
impl SRAM_RET_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRAM_RET_ENW::RET_FULL => 15,
            SRAM_RET_ENW::RET_LEVEL3 => 7,
            SRAM_RET_ENW::RET_LEVEL2 => 3,
            SRAM_RET_ENW::RET_LEVEL1 => 1,
            SRAM_RET_ENW::RET_NONE => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRAM_RET_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAM_RET_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAM_RET_ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Retention on for all banks (SRAM:BANK0, SRAM:BANK1 ,SRAM:BANK2 and SRAM:BANK3)"]
    #[inline]
    pub fn ret_full(self) -> &'a mut W {
        self.variant(SRAM_RET_ENW::RET_FULL)
    }
    #[doc = "Retention on for SRAM:BANK0, SRAM:BANK1 and SRAM:BANK2"]
    #[inline]
    pub fn ret_level3(self) -> &'a mut W {
        self.variant(SRAM_RET_ENW::RET_LEVEL3)
    }
    #[doc = "Retention on for SRAM:BANK0 and SRAM:BANK1"]
    #[inline]
    pub fn ret_level2(self) -> &'a mut W {
        self.variant(SRAM_RET_ENW::RET_LEVEL2)
    }
    #[doc = "Retention on for SRAM:BANK0"]
    #[inline]
    pub fn ret_level1(self) -> &'a mut W {
        self.variant(SRAM_RET_ENW::RET_LEVEL1)
    }
    #[doc = "Retention is disabled"]
    #[inline]
    pub fn ret_none(self) -> &'a mut W {
        self.variant(SRAM_RET_ENW::RET_NONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 18:31 - 31:18\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved18(&self) -> RESERVED18R {
        let bits = {
            const MASK: u16 = 16383;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED18R { bits }
    }
    #[doc = "Bit 17 - 17:17\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn virt_off(&self) -> VIRT_OFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VIRT_OFFR { bits }
    }
    #[doc = "Bit 16 - 16:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn fixed_wu_en(&self) -> FIXED_WU_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FIXED_WU_ENR { bits }
    }
    #[doc = "Bits 4:15 - 15:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&self) -> RESERVED4R {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED4R { bits }
    }
    #[doc = "Bits 0:3 - 3:0\\] MCU SRAM is partitioned into 4 banks . This register controls which of the banks that has retention during MCU power off"]
    #[inline]
    pub fn sram_ret_en(&self) -> SRAM_RET_ENR {
        SRAM_RET_ENR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 15 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 18:31 - 31:18\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved18(&mut self) -> _RESERVED18W {
        _RESERVED18W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn virt_off(&mut self) -> _VIRT_OFFW {
        _VIRT_OFFW { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn fixed_wu_en(&mut self) -> _FIXED_WU_ENW {
        _FIXED_WU_ENW { w: self }
    }
    #[doc = "Bits 4:15 - 15:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&mut self) -> _RESERVED4W {
        _RESERVED4W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] MCU SRAM is partitioned into 4 banks . This register controls which of the banks that has retention during MCU power off"]
    #[inline]
    pub fn sram_ret_en(&mut self) -> _SRAM_RET_ENW {
        _SRAM_RET_ENW { w: self }
    }
}
