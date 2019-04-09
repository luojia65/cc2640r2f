#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LFOSCCTL {
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
pub struct RESERVED24R {
    bits: u8,
}
impl RESERVED24R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XOSCLF_REGULATOR_TRIMR {
    bits: u8,
}
impl XOSCLF_REGULATOR_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XOSCLF_CMIRRWR_RATIOR {
    bits: u8,
}
impl XOSCLF_CMIRRWR_RATIOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED10R {
    bits: u8,
}
impl RESERVED10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RCOSCLF_RTUNE_TRIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCOSCLF_RTUNE_TRIMR {
    #[doc = "Internal. Only to be used through TI provided API."]
    _6P0MEG,
    #[doc = "Internal. Only to be used through TI provided API."]
    _6P5MEG,
    #[doc = "Internal. Only to be used through TI provided API."]
    _7P0MEG,
    #[doc = "Internal. Only to be used through TI provided API."]
    _7P5MEG,
}
impl RCOSCLF_RTUNE_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RCOSCLF_RTUNE_TRIMR::_6P0MEG => 3,
            RCOSCLF_RTUNE_TRIMR::_6P5MEG => 2,
            RCOSCLF_RTUNE_TRIMR::_7P0MEG => 1,
            RCOSCLF_RTUNE_TRIMR::_7P5MEG => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RCOSCLF_RTUNE_TRIMR {
        match value {
            3 => RCOSCLF_RTUNE_TRIMR::_6P0MEG,
            2 => RCOSCLF_RTUNE_TRIMR::_6P5MEG,
            1 => RCOSCLF_RTUNE_TRIMR::_7P0MEG,
            0 => RCOSCLF_RTUNE_TRIMR::_7P5MEG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_6P0MEG`"]
    #[inline]
    pub fn is_6p0meg(&self) -> bool {
        *self == RCOSCLF_RTUNE_TRIMR::_6P0MEG
    }
    #[doc = "Checks if the value of the field is `_6P5MEG`"]
    #[inline]
    pub fn is_6p5meg(&self) -> bool {
        *self == RCOSCLF_RTUNE_TRIMR::_6P5MEG
    }
    #[doc = "Checks if the value of the field is `_7P0MEG`"]
    #[inline]
    pub fn is_7p0meg(&self) -> bool {
        *self == RCOSCLF_RTUNE_TRIMR::_7P0MEG
    }
    #[doc = "Checks if the value of the field is `_7P5MEG`"]
    #[inline]
    pub fn is_7p5meg(&self) -> bool {
        *self == RCOSCLF_RTUNE_TRIMR::_7P5MEG
    }
}
#[doc = r" Value of the field"]
pub struct RCOSCLF_CTUNE_TRIMR {
    bits: u8,
}
impl RCOSCLF_CTUNE_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED24W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED24W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XOSCLF_REGULATOR_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _XOSCLF_REGULATOR_TRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XOSCLF_CMIRRWR_RATIOW<'a> {
    w: &'a mut W,
}
impl<'a> _XOSCLF_CMIRRWR_RATIOW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED10W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED10W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCOSCLF_RTUNE_TRIM`"]
pub enum RCOSCLF_RTUNE_TRIMW {
    #[doc = "Internal. Only to be used through TI provided API."]
    _6P0MEG,
    #[doc = "Internal. Only to be used through TI provided API."]
    _6P5MEG,
    #[doc = "Internal. Only to be used through TI provided API."]
    _7P0MEG,
    #[doc = "Internal. Only to be used through TI provided API."]
    _7P5MEG,
}
impl RCOSCLF_RTUNE_TRIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RCOSCLF_RTUNE_TRIMW::_6P0MEG => 3,
            RCOSCLF_RTUNE_TRIMW::_6P5MEG => 2,
            RCOSCLF_RTUNE_TRIMW::_7P0MEG => 1,
            RCOSCLF_RTUNE_TRIMW::_7P5MEG => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCOSCLF_RTUNE_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _RCOSCLF_RTUNE_TRIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCOSCLF_RTUNE_TRIMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn _6p0meg(self) -> &'a mut W {
        self.variant(RCOSCLF_RTUNE_TRIMW::_6P0MEG)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn _6p5meg(self) -> &'a mut W {
        self.variant(RCOSCLF_RTUNE_TRIMW::_6P5MEG)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn _7p0meg(self) -> &'a mut W {
        self.variant(RCOSCLF_RTUNE_TRIMW::_7P0MEG)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn _7p5meg(self) -> &'a mut W {
        self.variant(RCOSCLF_RTUNE_TRIMW::_7P5MEG)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RCOSCLF_CTUNE_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _RCOSCLF_CTUNE_TRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 24:31 - 31:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved24(&self) -> RESERVED24R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED24R { bits }
    }
    #[doc = "Bits 22:23 - 23:22\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn xosclf_regulator_trim(&self) -> XOSCLF_REGULATOR_TRIMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XOSCLF_REGULATOR_TRIMR { bits }
    }
    #[doc = "Bits 18:21 - 21:18\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn xosclf_cmirrwr_ratio(&self) -> XOSCLF_CMIRRWR_RATIOR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XOSCLF_CMIRRWR_RATIOR { bits }
    }
    #[doc = "Bits 10:17 - 17:10\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved10(&self) -> RESERVED10R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED10R { bits }
    }
    #[doc = "Bits 8:9 - 9:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcosclf_rtune_trim(&self) -> RCOSCLF_RTUNE_TRIMR {
        RCOSCLF_RTUNE_TRIMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:7 - 7:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcosclf_ctune_trim(&self) -> RCOSCLF_CTUNE_TRIMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RCOSCLF_CTUNE_TRIMR { bits }
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
    #[doc = "Bits 24:31 - 31:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved24(&mut self) -> _RESERVED24W {
        _RESERVED24W { w: self }
    }
    #[doc = "Bits 22:23 - 23:22\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn xosclf_regulator_trim(&mut self) -> _XOSCLF_REGULATOR_TRIMW {
        _XOSCLF_REGULATOR_TRIMW { w: self }
    }
    #[doc = "Bits 18:21 - 21:18\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn xosclf_cmirrwr_ratio(&mut self) -> _XOSCLF_CMIRRWR_RATIOW {
        _XOSCLF_CMIRRWR_RATIOW { w: self }
    }
    #[doc = "Bits 10:17 - 17:10\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved10(&mut self) -> _RESERVED10W {
        _RESERVED10W { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcosclf_rtune_trim(&mut self) -> _RCOSCLF_RTUNE_TRIMW {
        _RCOSCLF_RTUNE_TRIMW { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcosclf_ctune_trim(&mut self) -> _RCOSCLF_CTUNE_TRIMW {
        _RCOSCLF_CTUNE_TRIMW { w: self }
    }
}
