#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TRACECLKMUX {
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
#[doc = "Possible values of the field `TRACECLK_N_SWV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRACECLK_N_SWVR {
    #[doc = "Internal. Only to be used through TI provided API."]
    TRACECLK,
    #[doc = "Internal. Only to be used through TI provided API."]
    SWV,
}
impl TRACECLK_N_SWVR {
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
            TRACECLK_N_SWVR::TRACECLK => true,
            TRACECLK_N_SWVR::SWV => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRACECLK_N_SWVR {
        match value {
            true => TRACECLK_N_SWVR::TRACECLK,
            false => TRACECLK_N_SWVR::SWV,
        }
    }
    #[doc = "Checks if the value of the field is `TRACECLK`"]
    #[inline]
    pub fn is_traceclk(&self) -> bool {
        *self == TRACECLK_N_SWVR::TRACECLK
    }
    #[doc = "Checks if the value of the field is `SWV`"]
    #[inline]
    pub fn is_swv(&self) -> bool {
        *self == TRACECLK_N_SWVR::SWV
    }
}
#[doc = "Values that can be written to the field `TRACECLK_N_SWV`"]
pub enum TRACECLK_N_SWVW {
    #[doc = "Internal. Only to be used through TI provided API."]
    TRACECLK,
    #[doc = "Internal. Only to be used through TI provided API."]
    SWV,
}
impl TRACECLK_N_SWVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRACECLK_N_SWVW::TRACECLK => true,
            TRACECLK_N_SWVW::SWV => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRACECLK_N_SWVW<'a> {
    w: &'a mut W,
}
impl<'a> _TRACECLK_N_SWVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRACECLK_N_SWVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn traceclk(self) -> &'a mut W {
        self.variant(TRACECLK_N_SWVW::TRACECLK)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn swv(self) -> &'a mut W {
        self.variant(TRACECLK_N_SWVW::SWV)
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
    #[doc = "Bit 0 - 0:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn traceclk_n_swv(&self) -> TRACECLK_N_SWVR {
        TRACECLK_N_SWVR::_from({
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
    #[doc = "Bit 0 - 0:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn traceclk_n_swv(&mut self) -> _TRACECLK_N_SWVW {
        _TRACECLK_N_SWVW { w: self }
    }
}
