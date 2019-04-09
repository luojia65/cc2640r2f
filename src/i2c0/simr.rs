#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SIMR {
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
#[doc = "Possible values of the field `STOPIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPIMR {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl STOPIMR {
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
            STOPIMR::EN => true,
            STOPIMR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPIMR {
        match value {
            true => STOPIMR::EN,
            false => STOPIMR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == STOPIMR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == STOPIMR::DIS
    }
}
#[doc = "Possible values of the field `STARTIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTIMR {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl STARTIMR {
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
            STARTIMR::EN => true,
            STARTIMR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STARTIMR {
        match value {
            true => STARTIMR::EN,
            false => STARTIMR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == STARTIMR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == STARTIMR::DIS
    }
}
#[doc = r" Value of the field"]
pub struct DATAIMR {
    bits: bool,
}
impl DATAIMR {
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
#[doc = "Values that can be written to the field `STOPIM`"]
pub enum STOPIMW {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl STOPIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPIMW::EN => true,
            STOPIMW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPIMW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(STOPIMW::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(STOPIMW::DIS)
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
#[doc = "Values that can be written to the field `STARTIM`"]
pub enum STARTIMW {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl STARTIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STARTIMW::EN => true,
            STARTIMW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(STARTIMW::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(STARTIMW::DIS)
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
pub struct _DATAIMW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAIMW<'a> {
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
    #[doc = "Bit 2 - 2:2\\] Stop condition interrupt mask 0: The SRIS.STOPRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STOPRIS interrupt is enabled and sent to the interrupt controller."]
    #[inline]
    pub fn stopim(&self) -> STOPIMR {
        STOPIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - 1:1\\] Start condition interrupt mask 0: The SRIS.STARTRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STARTRIS interrupt is enabled and sent to the interrupt controller."]
    #[inline]
    pub fn startim(&self) -> STARTIMR {
        STARTIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - 0:0\\] Data interrupt mask 0: The SRIS.DATARIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.DATARIS interrupt is enabled and sent to the interrupt controller."]
    #[inline]
    pub fn dataim(&self) -> DATAIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATAIMR { bits }
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
    #[doc = "Bit 2 - 2:2\\] Stop condition interrupt mask 0: The SRIS.STOPRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STOPRIS interrupt is enabled and sent to the interrupt controller."]
    #[inline]
    pub fn stopim(&mut self) -> _STOPIMW {
        _STOPIMW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Start condition interrupt mask 0: The SRIS.STARTRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STARTRIS interrupt is enabled and sent to the interrupt controller."]
    #[inline]
    pub fn startim(&mut self) -> _STARTIMW {
        _STARTIMW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Data interrupt mask 0: The SRIS.DATARIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.DATARIS interrupt is enabled and sent to the interrupt controller."]
    #[inline]
    pub fn dataim(&mut self) -> _DATAIMW {
        _DATAIMW { w: self }
    }
}
