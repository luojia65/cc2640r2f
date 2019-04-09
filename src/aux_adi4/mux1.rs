#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::MUX1 {
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
#[doc = "Possible values of the field `COMPA_IN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPA_INR {
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
impl COMPA_INR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COMPA_INR::AUXIO0 => 128,
            COMPA_INR::AUXIO1 => 64,
            COMPA_INR::AUXIO2 => 32,
            COMPA_INR::AUXIO3 => 16,
            COMPA_INR::AUXIO4 => 8,
            COMPA_INR::AUXIO5 => 4,
            COMPA_INR::AUXIO6 => 2,
            COMPA_INR::AUXIO7 => 1,
            COMPA_INR::NC => 0,
            COMPA_INR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COMPA_INR {
        match value {
            128 => COMPA_INR::AUXIO0,
            64 => COMPA_INR::AUXIO1,
            32 => COMPA_INR::AUXIO2,
            16 => COMPA_INR::AUXIO3,
            8 => COMPA_INR::AUXIO4,
            4 => COMPA_INR::AUXIO5,
            2 => COMPA_INR::AUXIO6,
            1 => COMPA_INR::AUXIO7,
            0 => COMPA_INR::NC,
            i => COMPA_INR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUXIO0`"]
    #[inline]
    pub fn is_auxio0(&self) -> bool {
        *self == COMPA_INR::AUXIO0
    }
    #[doc = "Checks if the value of the field is `AUXIO1`"]
    #[inline]
    pub fn is_auxio1(&self) -> bool {
        *self == COMPA_INR::AUXIO1
    }
    #[doc = "Checks if the value of the field is `AUXIO2`"]
    #[inline]
    pub fn is_auxio2(&self) -> bool {
        *self == COMPA_INR::AUXIO2
    }
    #[doc = "Checks if the value of the field is `AUXIO3`"]
    #[inline]
    pub fn is_auxio3(&self) -> bool {
        *self == COMPA_INR::AUXIO3
    }
    #[doc = "Checks if the value of the field is `AUXIO4`"]
    #[inline]
    pub fn is_auxio4(&self) -> bool {
        *self == COMPA_INR::AUXIO4
    }
    #[doc = "Checks if the value of the field is `AUXIO5`"]
    #[inline]
    pub fn is_auxio5(&self) -> bool {
        *self == COMPA_INR::AUXIO5
    }
    #[doc = "Checks if the value of the field is `AUXIO6`"]
    #[inline]
    pub fn is_auxio6(&self) -> bool {
        *self == COMPA_INR::AUXIO6
    }
    #[doc = "Checks if the value of the field is `AUXIO7`"]
    #[inline]
    pub fn is_auxio7(&self) -> bool {
        *self == COMPA_INR::AUXIO7
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline]
    pub fn is_nc(&self) -> bool {
        *self == COMPA_INR::NC
    }
}
#[doc = "Values that can be written to the field `COMPA_IN`"]
pub enum COMPA_INW {
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
impl COMPA_INW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COMPA_INW::AUXIO0 => 128,
            COMPA_INW::AUXIO1 => 64,
            COMPA_INW::AUXIO2 => 32,
            COMPA_INW::AUXIO3 => 16,
            COMPA_INW::AUXIO4 => 8,
            COMPA_INW::AUXIO5 => 4,
            COMPA_INW::AUXIO6 => 2,
            COMPA_INW::AUXIO7 => 1,
            COMPA_INW::NC => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPA_INW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPA_INW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPA_INW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio0(self) -> &'a mut W {
        self.variant(COMPA_INW::AUXIO0)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio1(self) -> &'a mut W {
        self.variant(COMPA_INW::AUXIO1)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio2(self) -> &'a mut W {
        self.variant(COMPA_INW::AUXIO2)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio3(self) -> &'a mut W {
        self.variant(COMPA_INW::AUXIO3)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(COMPA_INW::AUXIO4)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(COMPA_INW::AUXIO5)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(COMPA_INW::AUXIO6)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(COMPA_INW::AUXIO7)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn nc(self) -> &'a mut W {
        self.variant(COMPA_INW::NC)
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
    pub fn compa_in(&self) -> COMPA_INR {
        COMPA_INR::_from({
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
    pub fn compa_in(&mut self) -> _COMPA_INW {
        _COMPA_INW { w: self }
    }
}
