#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RFCMODESEL {
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
#[doc = "Possible values of the field `CURR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CURRR {
    #[doc = "Select Mode 7"]
    MODE7,
    #[doc = "Select Mode 6"]
    MODE6,
    #[doc = "Select Mode 5"]
    MODE5,
    #[doc = "Select Mode 4"]
    MODE4,
    #[doc = "Select Mode 3"]
    MODE3,
    #[doc = "Select Mode 2"]
    MODE2,
    #[doc = "Select Mode 1"]
    MODE1,
    #[doc = "Select Mode 0"]
    MODE0,
}
impl CURRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CURRR::MODE7 => 7,
            CURRR::MODE6 => 6,
            CURRR::MODE5 => 5,
            CURRR::MODE4 => 4,
            CURRR::MODE3 => 3,
            CURRR::MODE2 => 2,
            CURRR::MODE1 => 1,
            CURRR::MODE0 => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CURRR {
        match value {
            7 => CURRR::MODE7,
            6 => CURRR::MODE6,
            5 => CURRR::MODE5,
            4 => CURRR::MODE4,
            3 => CURRR::MODE3,
            2 => CURRR::MODE2,
            1 => CURRR::MODE1,
            0 => CURRR::MODE0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MODE7`"]
    #[inline]
    pub fn is_mode7(&self) -> bool {
        *self == CURRR::MODE7
    }
    #[doc = "Checks if the value of the field is `MODE6`"]
    #[inline]
    pub fn is_mode6(&self) -> bool {
        *self == CURRR::MODE6
    }
    #[doc = "Checks if the value of the field is `MODE5`"]
    #[inline]
    pub fn is_mode5(&self) -> bool {
        *self == CURRR::MODE5
    }
    #[doc = "Checks if the value of the field is `MODE4`"]
    #[inline]
    pub fn is_mode4(&self) -> bool {
        *self == CURRR::MODE4
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline]
    pub fn is_mode3(&self) -> bool {
        *self == CURRR::MODE3
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline]
    pub fn is_mode2(&self) -> bool {
        *self == CURRR::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline]
    pub fn is_mode1(&self) -> bool {
        *self == CURRR::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline]
    pub fn is_mode0(&self) -> bool {
        *self == CURRR::MODE0
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
#[doc = "Values that can be written to the field `CURR`"]
pub enum CURRW {
    #[doc = "Select Mode 7"]
    MODE7,
    #[doc = "Select Mode 6"]
    MODE6,
    #[doc = "Select Mode 5"]
    MODE5,
    #[doc = "Select Mode 4"]
    MODE4,
    #[doc = "Select Mode 3"]
    MODE3,
    #[doc = "Select Mode 2"]
    MODE2,
    #[doc = "Select Mode 1"]
    MODE1,
    #[doc = "Select Mode 0"]
    MODE0,
}
impl CURRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CURRW::MODE7 => 7,
            CURRW::MODE6 => 6,
            CURRW::MODE5 => 5,
            CURRW::MODE4 => 4,
            CURRW::MODE3 => 3,
            CURRW::MODE2 => 2,
            CURRW::MODE1 => 1,
            CURRW::MODE0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CURRW<'a> {
    w: &'a mut W,
}
impl<'a> _CURRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CURRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Select Mode 7"]
    #[inline]
    pub fn mode7(self) -> &'a mut W {
        self.variant(CURRW::MODE7)
    }
    #[doc = "Select Mode 6"]
    #[inline]
    pub fn mode6(self) -> &'a mut W {
        self.variant(CURRW::MODE6)
    }
    #[doc = "Select Mode 5"]
    #[inline]
    pub fn mode5(self) -> &'a mut W {
        self.variant(CURRW::MODE5)
    }
    #[doc = "Select Mode 4"]
    #[inline]
    pub fn mode4(self) -> &'a mut W {
        self.variant(CURRW::MODE4)
    }
    #[doc = "Select Mode 3"]
    #[inline]
    pub fn mode3(self) -> &'a mut W {
        self.variant(CURRW::MODE3)
    }
    #[doc = "Select Mode 2"]
    #[inline]
    pub fn mode2(self) -> &'a mut W {
        self.variant(CURRW::MODE2)
    }
    #[doc = "Select Mode 1"]
    #[inline]
    pub fn mode1(self) -> &'a mut W {
        self.variant(CURRW::MODE1)
    }
    #[doc = "Select Mode 0"]
    #[inline]
    pub fn mode0(self) -> &'a mut W {
        self.variant(CURRW::MODE0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:2 - 2:0\\] Selects the set of commands that the RFC will accept. Only modes permitted by RFCMODEHWOPT.AVAIL are writeable. See the technical reference manual for details."]
    #[inline]
    pub fn curr(&self) -> CURRR {
        CURRR::_from({
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
    #[doc = "Bits 0:2 - 2:0\\] Selects the set of commands that the RFC will accept. Only modes permitted by RFCMODEHWOPT.AVAIL are writeable. See the technical reference manual for details."]
    #[inline]
    pub fn curr(&mut self) -> _CURRW {
        _CURRW { w: self }
    }
}
