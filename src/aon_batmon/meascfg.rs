#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MEASCFG {
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
pub struct RESERVED2R {
    bits: u32,
}
impl RESERVED2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `PER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERR {
    #[doc = "Internal. Only to be used through TI provided API."]
    _32CYC,
    #[doc = "Internal. Only to be used through TI provided API."]
    _16CYC,
    #[doc = "Internal. Only to be used through TI provided API."]
    _8CYC,
    #[doc = "Internal. Only to be used through TI provided API."]
    CONT,
}
impl PERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PERR::_32CYC => 3,
            PERR::_16CYC => 2,
            PERR::_8CYC => 1,
            PERR::CONT => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PERR {
        match value {
            3 => PERR::_32CYC,
            2 => PERR::_16CYC,
            1 => PERR::_8CYC,
            0 => PERR::CONT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_32CYC`"]
    #[inline]
    pub fn is_32cyc(&self) -> bool {
        *self == PERR::_32CYC
    }
    #[doc = "Checks if the value of the field is `_16CYC`"]
    #[inline]
    pub fn is_16cyc(&self) -> bool {
        *self == PERR::_16CYC
    }
    #[doc = "Checks if the value of the field is `_8CYC`"]
    #[inline]
    pub fn is_8cyc(&self) -> bool {
        *self == PERR::_8CYC
    }
    #[doc = "Checks if the value of the field is `CONT`"]
    #[inline]
    pub fn is_cont(&self) -> bool {
        *self == PERR::CONT
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED2W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 1073741823;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PER`"]
pub enum PERW {
    #[doc = "Internal. Only to be used through TI provided API."]
    _32CYC,
    #[doc = "Internal. Only to be used through TI provided API."]
    _16CYC,
    #[doc = "Internal. Only to be used through TI provided API."]
    _8CYC,
    #[doc = "Internal. Only to be used through TI provided API."]
    CONT,
}
impl PERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PERW::_32CYC => 3,
            PERW::_16CYC => 2,
            PERW::_8CYC => 1,
            PERW::CONT => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PERW<'a> {
    w: &'a mut W,
}
impl<'a> _PERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PERW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn _32cyc(self) -> &'a mut W {
        self.variant(PERW::_32CYC)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn _16cyc(self) -> &'a mut W {
        self.variant(PERW::_16CYC)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn _8cyc(self) -> &'a mut W {
        self.variant(PERW::_8CYC)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn cont(self) -> &'a mut W {
        self.variant(PERW::CONT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 2:31 - 31:2\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: u32 = 1073741823;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED2R { bits }
    }
    #[doc = "Bits 0:1 - 1:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn per(&self) -> PERR {
        PERR::_from({
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
    #[doc = "Bits 2:31 - 31:2\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved2(&mut self) -> _RESERVED2W {
        _RESERVED2W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn per(&mut self) -> _PERW {
        _PERW { w: self }
    }
}
