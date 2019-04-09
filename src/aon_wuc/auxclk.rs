#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AUXCLK {
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
pub struct RESERVED13R {
    bits: u32,
}
impl RESERVED13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
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
#[doc = "Possible values of the field `SCLK_HF_DIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLK_HF_DIVR {
    #[doc = "Divide by 256"]
    DIV256,
    #[doc = "Divide by 128"]
    DIV128,
    #[doc = "Divide by 64"]
    DIV64,
    #[doc = "Divide by 32"]
    DIV32,
    #[doc = "Divide by 16"]
    DIV16,
    #[doc = "Divide by 8"]
    DIV8,
    #[doc = "Divide by 4"]
    DIV4,
    #[doc = "Divide by 2"]
    DIV2,
}
impl SCLK_HF_DIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCLK_HF_DIVR::DIV256 => 7,
            SCLK_HF_DIVR::DIV128 => 6,
            SCLK_HF_DIVR::DIV64 => 5,
            SCLK_HF_DIVR::DIV32 => 4,
            SCLK_HF_DIVR::DIV16 => 3,
            SCLK_HF_DIVR::DIV8 => 2,
            SCLK_HF_DIVR::DIV4 => 1,
            SCLK_HF_DIVR::DIV2 => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCLK_HF_DIVR {
        match value {
            7 => SCLK_HF_DIVR::DIV256,
            6 => SCLK_HF_DIVR::DIV128,
            5 => SCLK_HF_DIVR::DIV64,
            4 => SCLK_HF_DIVR::DIV32,
            3 => SCLK_HF_DIVR::DIV16,
            2 => SCLK_HF_DIVR::DIV8,
            1 => SCLK_HF_DIVR::DIV4,
            0 => SCLK_HF_DIVR::DIV2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == SCLK_HF_DIVR::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == SCLK_HF_DIVR::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == SCLK_HF_DIVR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == SCLK_HF_DIVR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == SCLK_HF_DIVR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == SCLK_HF_DIVR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == SCLK_HF_DIVR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == SCLK_HF_DIVR::DIV2
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED3R {
    bits: u8,
}
impl RESERVED3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCR {
    #[doc = "LF Clock (SCLK_LF)"]
    SCLK_LF,
    #[doc = "HF Clock (SCLK_HF)"]
    SCLK_HF,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRCR::SCLK_LF => 4,
            SRCR::SCLK_HF => 1,
            SRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRCR {
        match value {
            4 => SRCR::SCLK_LF,
            1 => SRCR::SCLK_HF,
            i => SRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_LF`"]
    #[inline]
    pub fn is_sclk_lf(&self) -> bool {
        *self == SRCR::SCLK_LF
    }
    #[doc = "Checks if the value of the field is `SCLK_HF`"]
    #[inline]
    pub fn is_sclk_hf(&self) -> bool {
        *self == SRCR::SCLK_HF
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED13W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED13W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 13;
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCLK_HF_DIV`"]
pub enum SCLK_HF_DIVW {
    #[doc = "Divide by 256"]
    DIV256,
    #[doc = "Divide by 128"]
    DIV128,
    #[doc = "Divide by 64"]
    DIV64,
    #[doc = "Divide by 32"]
    DIV32,
    #[doc = "Divide by 16"]
    DIV16,
    #[doc = "Divide by 8"]
    DIV8,
    #[doc = "Divide by 4"]
    DIV4,
    #[doc = "Divide by 2"]
    DIV2,
}
impl SCLK_HF_DIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SCLK_HF_DIVW::DIV256 => 7,
            SCLK_HF_DIVW::DIV128 => 6,
            SCLK_HF_DIVW::DIV64 => 5,
            SCLK_HF_DIVW::DIV32 => 4,
            SCLK_HF_DIVW::DIV16 => 3,
            SCLK_HF_DIVW::DIV8 => 2,
            SCLK_HF_DIVW::DIV4 => 1,
            SCLK_HF_DIVW::DIV2 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCLK_HF_DIVW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLK_HF_DIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCLK_HF_DIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide by 256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(SCLK_HF_DIVW::DIV256)
    }
    #[doc = "Divide by 128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(SCLK_HF_DIVW::DIV128)
    }
    #[doc = "Divide by 64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(SCLK_HF_DIVW::DIV64)
    }
    #[doc = "Divide by 32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(SCLK_HF_DIVW::DIV32)
    }
    #[doc = "Divide by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(SCLK_HF_DIVW::DIV16)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(SCLK_HF_DIVW::DIV8)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(SCLK_HF_DIVW::DIV4)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(SCLK_HF_DIVW::DIV2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED3W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRC`"]
pub enum SRCW {
    #[doc = "LF Clock (SCLK_LF)"]
    SCLK_LF,
    #[doc = "HF Clock (SCLK_HF)"]
    SCLK_HF,
}
impl SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRCW::SCLK_LF => 4,
            SRCW::SCLK_HF => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LF Clock (SCLK_LF)"]
    #[inline]
    pub fn sclk_lf(self) -> &'a mut W {
        self.variant(SRCW::SCLK_LF)
    }
    #[doc = "HF Clock (SCLK_HF)"]
    #[inline]
    pub fn sclk_hf(self) -> &'a mut W {
        self.variant(SRCW::SCLK_HF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 13:31 - 31:13\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved13(&self) -> RESERVED13R {
        let bits = {
            const MASK: u32 = 524287;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED13R { bits }
    }
    #[doc = "Bits 11:12 - 12:11\\] When AUX requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when AUX system is back in active mode"]
    #[inline]
    pub fn pwr_dwn_src(&self) -> PWR_DWN_SRCR {
        PWR_DWN_SRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - 10:8\\] Select the AUX clock divider for SCLK_HF NB: It is not supported to change the AUX clock divider while SCLK_HF is active source for AUX"]
    #[inline]
    pub fn sclk_hf_div(&self) -> SCLK_HF_DIVR {
        SCLK_HF_DIVR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:7 - 7:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&self) -> RESERVED3R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED3R { bits }
    }
    #[doc = "Bits 0:2 - 2:0\\] Selects the clock source for AUX: NB: Switching the clock source is guaranteed to be glitchless"]
    #[inline]
    pub fn src(&self) -> SRCR {
        SRCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 13:31 - 31:13\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved13(&mut self) -> _RESERVED13W {
        _RESERVED13W { w: self }
    }
    #[doc = "Bits 11:12 - 12:11\\] When AUX requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when AUX system is back in active mode"]
    #[inline]
    pub fn pwr_dwn_src(&mut self) -> _PWR_DWN_SRCW {
        _PWR_DWN_SRCW { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\] Select the AUX clock divider for SCLK_HF NB: It is not supported to change the AUX clock divider while SCLK_HF is active source for AUX"]
    #[inline]
    pub fn sclk_hf_div(&mut self) -> _SCLK_HF_DIVW {
        _SCLK_HF_DIVW { w: self }
    }
    #[doc = "Bits 3:7 - 7:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&mut self) -> _RESERVED3W {
        _RESERVED3W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] Selects the clock source for AUX: NB: Switching the clock source is guaranteed to be glitchless"]
    #[inline]
    pub fn src(&mut self) -> _SRCW {
        _SRCW { w: self }
    }
}
