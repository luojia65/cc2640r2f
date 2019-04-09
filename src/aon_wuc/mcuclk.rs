#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCUCLK {
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
#[doc = r" Value of the field"]
pub struct RCOSC_HF_CAL_DONER {
    bits: bool,
}
impl RCOSC_HF_CAL_DONER {
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
#[doc = "Possible values of the field `PWR_DWN_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_DWN_SRCR {
    #[doc = "Use SCLK_LF in Powerdown"]
    SCLK_LF,
    #[doc = "No clock in Powerdown"]
    NONE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWR_DWN_SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWR_DWN_SRCR::SCLK_LF => 1,
            PWR_DWN_SRCR::NONE => 0,
            PWR_DWN_SRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWR_DWN_SRCR {
        match value {
            1 => PWR_DWN_SRCR::SCLK_LF,
            0 => PWR_DWN_SRCR::NONE,
            i => PWR_DWN_SRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_LF`"]
    #[inline]
    pub fn is_sclk_lf(&self) -> bool {
        *self == PWR_DWN_SRCR::SCLK_LF
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == PWR_DWN_SRCR::NONE
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
#[doc = r" Proxy"]
pub struct _RCOSC_HF_CAL_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _RCOSC_HF_CAL_DONEW<'a> {
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
#[doc = "Values that can be written to the field `PWR_DWN_SRC`"]
pub enum PWR_DWN_SRCW {
    #[doc = "Use SCLK_LF in Powerdown"]
    SCLK_LF,
    #[doc = "No clock in Powerdown"]
    NONE,
}
impl PWR_DWN_SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWR_DWN_SRCW::SCLK_LF => 1,
            PWR_DWN_SRCW::NONE => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWR_DWN_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_DWN_SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_DWN_SRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Use SCLK_LF in Powerdown"]
    #[inline]
    pub fn sclk_lf(self) -> &'a mut W {
        self.variant(PWR_DWN_SRCW::SCLK_LF)
    }
    #[doc = "No clock in Powerdown"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(PWR_DWN_SRCW::NONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bit 2 - 2:2\\] MCU bootcode will set this bit when RCOSC_HF is calibrated. The FLASH can not be used until this bit is set. 1: RCOSC_HF is calibrated to 48 MHz, allowing FLASH to power up. 0: RCOSC_HF is not yet calibrated, ie FLASH must not assume that the SCLK_HF is safe"]
    #[inline]
    pub fn rcosc_hf_cal_done(&self) -> RCOSC_HF_CAL_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RCOSC_HF_CAL_DONER { bits }
    }
    #[doc = "Bits 0:1 - 1:0\\] Controls the clock source for the entire MCU domain while MCU is requesting powerdown. When MCU requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when MCU is no longer requesting powerdown and system is back in active mode."]
    #[inline]
    pub fn pwr_dwn_src(&self) -> PWR_DWN_SRCR {
        PWR_DWN_SRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 2 - 2:2\\] MCU bootcode will set this bit when RCOSC_HF is calibrated. The FLASH can not be used until this bit is set. 1: RCOSC_HF is calibrated to 48 MHz, allowing FLASH to power up. 0: RCOSC_HF is not yet calibrated, ie FLASH must not assume that the SCLK_HF is safe"]
    #[inline]
    pub fn rcosc_hf_cal_done(&mut self) -> _RCOSC_HF_CAL_DONEW {
        _RCOSC_HF_CAL_DONEW { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Controls the clock source for the entire MCU domain while MCU is requesting powerdown. When MCU requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when MCU is no longer requesting powerdown and system is back in active mode."]
    #[inline]
    pub fn pwr_dwn_src(&mut self) -> _PWR_DWN_SRCW {
        _PWR_DWN_SRCW { w: self }
    }
}
