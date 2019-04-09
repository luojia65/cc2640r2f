#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::MUX4 {
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
#[doc = "Possible values of the field `COMPA_REF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPA_REFR {
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO0,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO1,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO2,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO3,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO4,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO5,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO6,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO7,
    #[doc = "Internal. Only to be used through TI provided API."]
    NC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl COMPA_REFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COMPA_REFR::AUXIO0 => 128,
            COMPA_REFR::AUXIO1 => 64,
            COMPA_REFR::AUXIO2 => 32,
            COMPA_REFR::AUXIO3 => 16,
            COMPA_REFR::AUXIO4 => 8,
            COMPA_REFR::AUXIO5 => 4,
            COMPA_REFR::AUXIO6 => 2,
            COMPA_REFR::AUXIO7 => 1,
            COMPA_REFR::NC => 0,
            COMPA_REFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COMPA_REFR {
        match value {
            128 => COMPA_REFR::AUXIO0,
            64 => COMPA_REFR::AUXIO1,
            32 => COMPA_REFR::AUXIO2,
            16 => COMPA_REFR::AUXIO3,
            8 => COMPA_REFR::AUXIO4,
            4 => COMPA_REFR::AUXIO5,
            2 => COMPA_REFR::AUXIO6,
            1 => COMPA_REFR::AUXIO7,
            0 => COMPA_REFR::NC,
            i => COMPA_REFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUXIO0`"]
    #[inline]
    pub fn is_auxio0(&self) -> bool {
        *self == COMPA_REFR::AUXIO0
    }
    #[doc = "Checks if the value of the field is `AUXIO1`"]
    #[inline]
    pub fn is_auxio1(&self) -> bool {
        *self == COMPA_REFR::AUXIO1
    }
    #[doc = "Checks if the value of the field is `AUXIO2`"]
    #[inline]
    pub fn is_auxio2(&self) -> bool {
        *self == COMPA_REFR::AUXIO2
    }
    #[doc = "Checks if the value of the field is `AUXIO3`"]
    #[inline]
    pub fn is_auxio3(&self) -> bool {
        *self == COMPA_REFR::AUXIO3
    }
    #[doc = "Checks if the value of the field is `AUXIO4`"]
    #[inline]
    pub fn is_auxio4(&self) -> bool {
        *self == COMPA_REFR::AUXIO4
    }
    #[doc = "Checks if the value of the field is `AUXIO5`"]
    #[inline]
    pub fn is_auxio5(&self) -> bool {
        *self == COMPA_REFR::AUXIO5
    }
    #[doc = "Checks if the value of the field is `AUXIO6`"]
    #[inline]
    pub fn is_auxio6(&self) -> bool {
        *self == COMPA_REFR::AUXIO6
    }
    #[doc = "Checks if the value of the field is `AUXIO7`"]
    #[inline]
    pub fn is_auxio7(&self) -> bool {
        *self == COMPA_REFR::AUXIO7
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline]
    pub fn is_nc(&self) -> bool {
        *self == COMPA_REFR::NC
    }
}
#[doc = "Values that can be written to the field `COMPA_REF`"]
pub enum COMPA_REFW {
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO0,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO1,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO2,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO3,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO4,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO5,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO6,
    #[doc = "Internal. Only to be used through TI provided API."]
    AUXIO7,
    #[doc = "Internal. Only to be used through TI provided API."]
    NC,
}
impl COMPA_REFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COMPA_REFW::AUXIO0 => 128,
            COMPA_REFW::AUXIO1 => 64,
            COMPA_REFW::AUXIO2 => 32,
            COMPA_REFW::AUXIO3 => 16,
            COMPA_REFW::AUXIO4 => 8,
            COMPA_REFW::AUXIO5 => 4,
            COMPA_REFW::AUXIO6 => 2,
            COMPA_REFW::AUXIO7 => 1,
            COMPA_REFW::NC => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPA_REFW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPA_REFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPA_REFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio0(self) -> &'a mut W {
        self.variant(COMPA_REFW::AUXIO0)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio1(self) -> &'a mut W {
        self.variant(COMPA_REFW::AUXIO1)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio2(self) -> &'a mut W {
        self.variant(COMPA_REFW::AUXIO2)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio3(self) -> &'a mut W {
        self.variant(COMPA_REFW::AUXIO3)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(COMPA_REFW::AUXIO4)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(COMPA_REFW::AUXIO5)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(COMPA_REFW::AUXIO6)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(COMPA_REFW::AUXIO7)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn nc(self) -> &'a mut W {
        self.variant(COMPA_REFW::NC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - 7:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn compa_ref(&self) -> COMPA_REFR {
        COMPA_REFR::_from({
            const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - 7:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn compa_ref(&mut self) -> _COMPA_REFW {
        _COMPA_REFW { w: self }
    }
}
