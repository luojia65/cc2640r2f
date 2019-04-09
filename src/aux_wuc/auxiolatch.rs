#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AUXIOLATCH {
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
#[doc = "Possible values of the field `EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENR {
    #[doc = "Latches are transparent ( open )"]
    TRANSP,
    #[doc = "Latches are static ( closed )"]
    STATIC,
}
impl ENR {
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
            ENR::TRANSP => true,
            ENR::STATIC => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENR {
        match value {
            true => ENR::TRANSP,
            false => ENR::STATIC,
        }
    }
    #[doc = "Checks if the value of the field is `TRANSP`"]
    #[inline]
    pub fn is_transp(&self) -> bool {
        *self == ENR::TRANSP
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline]
    pub fn is_static_(&self) -> bool {
        *self == ENR::STATIC
    }
}
#[doc = "Values that can be written to the field `EN`"]
pub enum ENW {
    #[doc = "Latches are transparent ( open )"]
    TRANSP,
    #[doc = "Latches are static ( closed )"]
    STATIC,
}
impl ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENW::TRANSP => true,
            ENW::STATIC => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Latches are transparent ( open )"]
    #[inline]
    pub fn transp(self) -> &'a mut W {
        self.variant(ENW::TRANSP)
    }
    #[doc = "Latches are static ( closed )"]
    #[inline]
    pub fn static_(self) -> &'a mut W {
        self.variant(ENW::STATIC)
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
    #[doc = "Bit 0 - 0:0\\] Opens (1) or closes (0) the AUX_AIODIO0/AUX_AIODIO1 signal latching. At startup, set EN = TRANSP before configuring AUX_AIODIO0/AUX_AIODIO1 and subsequently selecting AUX mode in the AON_IOC. When powering off the AUX domain (using PWROFFREQ.REQ), set EN = STATIC in advance preserve the current state (mode and output value) of the I/O pins."]
    #[inline]
    pub fn en(&self) -> ENR {
        ENR::_from({
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
    #[doc = "Bit 0 - 0:0\\] Opens (1) or closes (0) the AUX_AIODIO0/AUX_AIODIO1 signal latching. At startup, set EN = TRANSP before configuring AUX_AIODIO0/AUX_AIODIO1 and subsequently selecting AUX mode in the AON_IOC. When powering off the AUX domain (using PWROFFREQ.REQ), set EN = STATIC in advance preserve the current state (mode and output value) of the I/O pins."]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
}
