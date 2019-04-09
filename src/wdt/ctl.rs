#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTL {
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
pub struct RESERVED3R {
    bits: u32,
}
impl RESERVED3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `INTTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTTYPER {
    #[doc = "Non-maskable interrupt"]
    NONMASKABLE,
    #[doc = "Maskable interrupt"]
    MASKABLE,
}
impl INTTYPER {
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
            INTTYPER::NONMASKABLE => true,
            INTTYPER::MASKABLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTTYPER {
        match value {
            true => INTTYPER::NONMASKABLE,
            false => INTTYPER::MASKABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NONMASKABLE`"]
    #[inline]
    pub fn is_nonmaskable(&self) -> bool {
        *self == INTTYPER::NONMASKABLE
    }
    #[doc = "Checks if the value of the field is `MASKABLE`"]
    #[inline]
    pub fn is_maskable(&self) -> bool {
        *self == INTTYPER::MASKABLE
    }
}
#[doc = "Possible values of the field `RESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESENR {
    #[doc = "Reset output Enabled"]
    EN,
    #[doc = "Reset output Disabled"]
    DIS,
}
impl RESENR {
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
            RESENR::EN => true,
            RESENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESENR {
        match value {
            true => RESENR::EN,
            false => RESENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == RESENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == RESENR::DIS
    }
}
#[doc = "Possible values of the field `INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTENR {
    #[doc = "Interrupt Enabled"]
    EN,
    #[doc = "Interrupt Disabled"]
    DIS,
}
impl INTENR {
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
            INTENR::EN => true,
            INTENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTENR {
        match value {
            true => INTENR::EN,
            false => INTENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == INTENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == INTENR::DIS
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED3W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 536870911;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INTTYPE`"]
pub enum INTTYPEW {
    #[doc = "Non-maskable interrupt"]
    NONMASKABLE,
    #[doc = "Maskable interrupt"]
    MASKABLE,
}
impl INTTYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTTYPEW::NONMASKABLE => true,
            INTTYPEW::MASKABLE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _INTTYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTTYPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Non-maskable interrupt"]
    #[inline]
    pub fn nonmaskable(self) -> &'a mut W {
        self.variant(INTTYPEW::NONMASKABLE)
    }
    #[doc = "Maskable interrupt"]
    #[inline]
    pub fn maskable(self) -> &'a mut W {
        self.variant(INTTYPEW::MASKABLE)
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
#[doc = "Values that can be written to the field `RESEN`"]
pub enum RESENW {
    #[doc = "Reset output Enabled"]
    EN,
    #[doc = "Reset output Disabled"]
    DIS,
}
impl RESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESENW::EN => true,
            RESENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESENW<'a> {
    w: &'a mut W,
}
impl<'a> _RESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset output Enabled"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(RESENW::EN)
    }
    #[doc = "Reset output Disabled"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(RESENW::DIS)
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
#[doc = "Values that can be written to the field `INTEN`"]
pub enum INTENW {
    #[doc = "Interrupt Enabled"]
    EN,
    #[doc = "Interrupt Disabled"]
    DIS,
}
impl INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTENW::EN => true,
            INTENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _INTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt Enabled"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(INTENW::EN)
    }
    #[doc = "Interrupt Disabled"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(INTENW::DIS)
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
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&self) -> RESERVED3R {
        let bits = {
            const MASK: u32 = 536870911;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED3R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] WDT Interrupt Type 0: WDT interrupt is a standard interrupt. 1: WDT interrupt is a non-maskable interrupt."]
    #[inline]
    pub fn inttype(&self) -> INTTYPER {
        INTTYPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - 1:1\\] WDT Reset Enable. Defines the function of the WDT reset source (see PRCM:WARMRESET.WDT_STAT if enabled) 0: Disabled. 1: Enable the Watchdog reset output."]
    #[inline]
    pub fn resen(&self) -> RESENR {
        RESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - 0:0\\] WDT Interrupt Enable 0: Interrupt event disabled. 1: Interrupt event enabled. Once set, this bit can only be cleared by a hardware reset."]
    #[inline]
    pub fn inten(&self) -> INTENR {
        INTENR::_from({
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
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&mut self) -> _RESERVED3W {
        _RESERVED3W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] WDT Interrupt Type 0: WDT interrupt is a standard interrupt. 1: WDT interrupt is a non-maskable interrupt."]
    #[inline]
    pub fn inttype(&mut self) -> _INTTYPEW {
        _INTTYPEW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] WDT Reset Enable. Defines the function of the WDT reset source (see PRCM:WARMRESET.WDT_STAT if enabled) 0: Disabled. 1: Enable the Watchdog reset output."]
    #[inline]
    pub fn resen(&mut self) -> _RESENW {
        _RESENW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] WDT Interrupt Enable 0: Interrupt event disabled. 1: Interrupt event enabled. Once set, this bit can only be cleared by a hardware reset."]
    #[inline]
    pub fn inten(&mut self) -> _INTENW {
        _INTENW { w: self }
    }
}
