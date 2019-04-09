#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TEST {
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
pub struct RESERVED9R {
    bits: u32,
}
impl RESERVED9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `STALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALLR {
    #[doc = "Enable STALL"]
    EN,
    #[doc = "Disable STALL"]
    DIS,
}
impl STALLR {
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
            STALLR::EN => true,
            STALLR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STALLR {
        match value {
            true => STALLR::EN,
            false => STALLR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == STALLR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == STALLR::DIS
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED1R {
    bits: u8,
}
impl RESERVED1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TEST_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEST_ENR {
    #[doc = "Test mode Enabled"]
    EN,
    #[doc = "Test mode Disabled"]
    DIS,
}
impl TEST_ENR {
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
            TEST_ENR::EN => true,
            TEST_ENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEST_ENR {
        match value {
            true => TEST_ENR::EN,
            false => TEST_ENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TEST_ENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TEST_ENR::DIS
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED9W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED9W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 8388607;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STALL`"]
pub enum STALLW {
    #[doc = "Enable STALL"]
    EN,
    #[doc = "Disable STALL"]
    DIS,
}
impl STALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STALLW::EN => true,
            STALLW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STALLW<'a> {
    w: &'a mut W,
}
impl<'a> _STALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STALLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable STALL"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(STALLW::EN)
    }
    #[doc = "Disable STALL"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(STALLW::DIS)
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
#[doc = r" Proxy"]
pub struct _RESERVED1W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TEST_EN`"]
pub enum TEST_ENW {
    #[doc = "Test mode Enabled"]
    EN,
    #[doc = "Test mode Disabled"]
    DIS,
}
impl TEST_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEST_ENW::EN => true,
            TEST_ENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEST_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TEST_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEST_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Test mode Enabled"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TEST_ENW::EN)
    }
    #[doc = "Test mode Disabled"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TEST_ENW::DIS)
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
    #[doc = "Bits 9:31 - 31:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved9(&self) -> RESERVED9R {
        let bits = {
            const MASK: u32 = 8388607;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED9R { bits }
    }
    #[doc = "Bit 8 - 8:8\\] WDT Stall Enable 0: The WDT timer continues counting if the CPU is stopped with a debugger. 1: If the CPU is stopped with a debugger, the WDT stops counting. Once the CPU is restarted, the WDT resumes counting."]
    #[inline]
    pub fn stall(&self) -> STALLR {
        STALLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:7 - 7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved1(&self) -> RESERVED1R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED1R { bits }
    }
    #[doc = "Bit 0 - 0:0\\] The test enable bit 0: Enable external reset 1: Disables the generation of an external reset. Instead bit 1 of the INT_CAUS register is set and an interrupt is generated"]
    #[inline]
    pub fn test_en(&self) -> TEST_ENR {
        TEST_ENR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 9:31 - 31:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved9(&mut self) -> _RESERVED9W {
        _RESERVED9W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] WDT Stall Enable 0: The WDT timer continues counting if the CPU is stopped with a debugger. 1: If the CPU is stopped with a debugger, the WDT stops counting. Once the CPU is restarted, the WDT resumes counting."]
    #[inline]
    pub fn stall(&mut self) -> _STALLW {
        _STALLW { w: self }
    }
    #[doc = "Bits 1:7 - 7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved1(&mut self) -> _RESERVED1W {
        _RESERVED1W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] The test enable bit 0: Enable external reset 1: Disables the generation of an external reset. Instead bit 1 of the INT_CAUS register is set and an interrupt is generated"]
    #[inline]
    pub fn test_en(&mut self) -> _TEST_ENW {
        _TEST_ENW { w: self }
    }
}
