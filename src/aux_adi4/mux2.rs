#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::MUX2 {
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
#[doc = "Possible values of the field `ADCCOMPB_IN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCCOMPB_INR {
    #[doc = "Internal. Only to be used through TI provided API."]
    VDDS,
    #[doc = "Internal. Only to be used through TI provided API."]
    VSS,
    #[doc = "Internal. Only to be used through TI provided API."]
    DCOUPL,
    #[doc = "Internal. Only to be used through TI provided API."]
    ATEST1,
    #[doc = "Internal. Only to be used through TI provided API."]
    ATEST0,
    #[doc = "Internal. Only to be used through TI provided API."]
    NC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ADCCOMPB_INR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCCOMPB_INR::VDDS => 16,
            ADCCOMPB_INR::VSS => 8,
            ADCCOMPB_INR::DCOUPL => 4,
            ADCCOMPB_INR::ATEST1 => 2,
            ADCCOMPB_INR::ATEST0 => 1,
            ADCCOMPB_INR::NC => 0,
            ADCCOMPB_INR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCCOMPB_INR {
        match value {
            16 => ADCCOMPB_INR::VDDS,
            8 => ADCCOMPB_INR::VSS,
            4 => ADCCOMPB_INR::DCOUPL,
            2 => ADCCOMPB_INR::ATEST1,
            1 => ADCCOMPB_INR::ATEST0,
            0 => ADCCOMPB_INR::NC,
            i => ADCCOMPB_INR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VDDS`"]
    #[inline]
    pub fn is_vdds(&self) -> bool {
        *self == ADCCOMPB_INR::VDDS
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline]
    pub fn is_vss(&self) -> bool {
        *self == ADCCOMPB_INR::VSS
    }
    #[doc = "Checks if the value of the field is `DCOUPL`"]
    #[inline]
    pub fn is_dcoupl(&self) -> bool {
        *self == ADCCOMPB_INR::DCOUPL
    }
    #[doc = "Checks if the value of the field is `ATEST1`"]
    #[inline]
    pub fn is_atest1(&self) -> bool {
        *self == ADCCOMPB_INR::ATEST1
    }
    #[doc = "Checks if the value of the field is `ATEST0`"]
    #[inline]
    pub fn is_atest0(&self) -> bool {
        *self == ADCCOMPB_INR::ATEST0
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline]
    pub fn is_nc(&self) -> bool {
        *self == ADCCOMPB_INR::NC
    }
}
#[doc = "Possible values of the field `COMPB_REF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPB_REFR {
    #[doc = "Internal. Only to be used through TI provided API."]
    VDDS,
    #[doc = "Internal. Only to be used through TI provided API."]
    VSS,
    #[doc = "Internal. Only to be used through TI provided API."]
    DCOUPL,
    #[doc = "Internal. Only to be used through TI provided API."]
    NC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl COMPB_REFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COMPB_REFR::VDDS => 4,
            COMPB_REFR::VSS => 2,
            COMPB_REFR::DCOUPL => 1,
            COMPB_REFR::NC => 0,
            COMPB_REFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COMPB_REFR {
        match value {
            4 => COMPB_REFR::VDDS,
            2 => COMPB_REFR::VSS,
            1 => COMPB_REFR::DCOUPL,
            0 => COMPB_REFR::NC,
            i => COMPB_REFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VDDS`"]
    #[inline]
    pub fn is_vdds(&self) -> bool {
        *self == COMPB_REFR::VDDS
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline]
    pub fn is_vss(&self) -> bool {
        *self == COMPB_REFR::VSS
    }
    #[doc = "Checks if the value of the field is `DCOUPL`"]
    #[inline]
    pub fn is_dcoupl(&self) -> bool {
        *self == COMPB_REFR::DCOUPL
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline]
    pub fn is_nc(&self) -> bool {
        *self == COMPB_REFR::NC
    }
}
#[doc = "Values that can be written to the field `ADCCOMPB_IN`"]
pub enum ADCCOMPB_INW {
    #[doc = "Internal. Only to be used through TI provided API."]
    VDDS,
    #[doc = "Internal. Only to be used through TI provided API."]
    VSS,
    #[doc = "Internal. Only to be used through TI provided API."]
    DCOUPL,
    #[doc = "Internal. Only to be used through TI provided API."]
    ATEST1,
    #[doc = "Internal. Only to be used through TI provided API."]
    ATEST0,
    #[doc = "Internal. Only to be used through TI provided API."]
    NC,
}
impl ADCCOMPB_INW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCCOMPB_INW::VDDS => 16,
            ADCCOMPB_INW::VSS => 8,
            ADCCOMPB_INW::DCOUPL => 4,
            ADCCOMPB_INW::ATEST1 => 2,
            ADCCOMPB_INW::ATEST0 => 1,
            ADCCOMPB_INW::NC => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCCOMPB_INW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCCOMPB_INW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCCOMPB_INW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vdds(self) -> &'a mut W {
        self.variant(ADCCOMPB_INW::VDDS)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vss(self) -> &'a mut W {
        self.variant(ADCCOMPB_INW::VSS)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dcoupl(self) -> &'a mut W {
        self.variant(ADCCOMPB_INW::DCOUPL)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn atest1(self) -> &'a mut W {
        self.variant(ADCCOMPB_INW::ATEST1)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn atest0(self) -> &'a mut W {
        self.variant(ADCCOMPB_INW::ATEST0)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn nc(self) -> &'a mut W {
        self.variant(ADCCOMPB_INW::NC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COMPB_REF`"]
pub enum COMPB_REFW {
    #[doc = "Internal. Only to be used through TI provided API."]
    VDDS,
    #[doc = "Internal. Only to be used through TI provided API."]
    VSS,
    #[doc = "Internal. Only to be used through TI provided API."]
    DCOUPL,
    #[doc = "Internal. Only to be used through TI provided API."]
    NC,
}
impl COMPB_REFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COMPB_REFW::VDDS => 4,
            COMPB_REFW::VSS => 2,
            COMPB_REFW::DCOUPL => 1,
            COMPB_REFW::NC => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPB_REFW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPB_REFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPB_REFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vdds(self) -> &'a mut W {
        self.variant(COMPB_REFW::VDDS)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vss(self) -> &'a mut W {
        self.variant(COMPB_REFW::VSS)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dcoupl(self) -> &'a mut W {
        self.variant(COMPB_REFW::DCOUPL)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn nc(self) -> &'a mut W {
        self.variant(COMPB_REFW::NC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 3:7 - 7:3\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn adccompb_in(&self) -> ADCCOMPB_INR {
        ADCCOMPB_INR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 0:2 - 2:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn compb_ref(&self) -> COMPB_REFR {
        COMPB_REFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 3:7 - 7:3\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn adccompb_in(&mut self) -> _ADCCOMPB_INW {
        _ADCCOMPB_INW { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn compb_ref(&mut self) -> _COMPB_REFW {
        _COMPB_REFW { w: self }
    }
}
