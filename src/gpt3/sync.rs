#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYNC {
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
pub struct RESERVED8R {
    bits: u32,
}
impl RESERVED8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `SYNC3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC3R {
    #[doc = "A timeout event for both Timer A and Timer B of GPT3 is triggered"]
    BOTH,
    #[doc = "A timeout event for Timer B of GPT3 is triggered"]
    TIMERB,
    #[doc = "A timeout event for Timer A of GPT3 is triggered"]
    TIMERA,
    #[doc = "No Sync. GPT3 is not affected. "]
    NOSYNC,
}
impl SYNC3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNC3R::BOTH => 3,
            SYNC3R::TIMERB => 2,
            SYNC3R::TIMERA => 1,
            SYNC3R::NOSYNC => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNC3R {
        match value {
            3 => SYNC3R::BOTH,
            2 => SYNC3R::TIMERB,
            1 => SYNC3R::TIMERA,
            0 => SYNC3R::NOSYNC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == SYNC3R::BOTH
    }
    #[doc = "Checks if the value of the field is `TIMERB`"]
    #[inline]
    pub fn is_timerb(&self) -> bool {
        *self == SYNC3R::TIMERB
    }
    #[doc = "Checks if the value of the field is `TIMERA`"]
    #[inline]
    pub fn is_timera(&self) -> bool {
        *self == SYNC3R::TIMERA
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline]
    pub fn is_nosync(&self) -> bool {
        *self == SYNC3R::NOSYNC
    }
}
#[doc = "Possible values of the field `SYNC2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC2R {
    #[doc = "A timeout event for both Timer A and Timer B of GPT2 is triggered"]
    BOTH,
    #[doc = "A timeout event for Timer B of GPT2 is triggered"]
    TIMERB,
    #[doc = "A timeout event for Timer A of GPT2 is triggered"]
    TIMERA,
    #[doc = "No Sync. GPT2 is not affected. "]
    NOSYNC,
}
impl SYNC2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNC2R::BOTH => 3,
            SYNC2R::TIMERB => 2,
            SYNC2R::TIMERA => 1,
            SYNC2R::NOSYNC => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNC2R {
        match value {
            3 => SYNC2R::BOTH,
            2 => SYNC2R::TIMERB,
            1 => SYNC2R::TIMERA,
            0 => SYNC2R::NOSYNC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == SYNC2R::BOTH
    }
    #[doc = "Checks if the value of the field is `TIMERB`"]
    #[inline]
    pub fn is_timerb(&self) -> bool {
        *self == SYNC2R::TIMERB
    }
    #[doc = "Checks if the value of the field is `TIMERA`"]
    #[inline]
    pub fn is_timera(&self) -> bool {
        *self == SYNC2R::TIMERA
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline]
    pub fn is_nosync(&self) -> bool {
        *self == SYNC2R::NOSYNC
    }
}
#[doc = "Possible values of the field `SYNC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC1R {
    #[doc = "A timeout event for both Timer A and Timer B of GPT1 is triggered"]
    BOTH,
    #[doc = "A timeout event for Timer B of GPT1 is triggered"]
    TIMERB,
    #[doc = "A timeout event for Timer A of GPT1 is triggered"]
    TIMERA,
    #[doc = "No Sync. GPT1 is not affected. "]
    NOSYNC,
}
impl SYNC1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNC1R::BOTH => 3,
            SYNC1R::TIMERB => 2,
            SYNC1R::TIMERA => 1,
            SYNC1R::NOSYNC => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNC1R {
        match value {
            3 => SYNC1R::BOTH,
            2 => SYNC1R::TIMERB,
            1 => SYNC1R::TIMERA,
            0 => SYNC1R::NOSYNC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == SYNC1R::BOTH
    }
    #[doc = "Checks if the value of the field is `TIMERB`"]
    #[inline]
    pub fn is_timerb(&self) -> bool {
        *self == SYNC1R::TIMERB
    }
    #[doc = "Checks if the value of the field is `TIMERA`"]
    #[inline]
    pub fn is_timera(&self) -> bool {
        *self == SYNC1R::TIMERA
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline]
    pub fn is_nosync(&self) -> bool {
        *self == SYNC1R::NOSYNC
    }
}
#[doc = "Possible values of the field `SYNC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC0R {
    #[doc = "A timeout event for both Timer A and Timer B of GPT0 is triggered"]
    BOTH,
    #[doc = "A timeout event for Timer B of GPT0 is triggered"]
    TIMERB,
    #[doc = "A timeout event for Timer A of GPT0 is triggered"]
    TIMERA,
    #[doc = "No Sync. GPT0 is not affected. "]
    NOSYNC,
}
impl SYNC0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNC0R::BOTH => 3,
            SYNC0R::TIMERB => 2,
            SYNC0R::TIMERA => 1,
            SYNC0R::NOSYNC => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNC0R {
        match value {
            3 => SYNC0R::BOTH,
            2 => SYNC0R::TIMERB,
            1 => SYNC0R::TIMERA,
            0 => SYNC0R::NOSYNC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == SYNC0R::BOTH
    }
    #[doc = "Checks if the value of the field is `TIMERB`"]
    #[inline]
    pub fn is_timerb(&self) -> bool {
        *self == SYNC0R::TIMERB
    }
    #[doc = "Checks if the value of the field is `TIMERA`"]
    #[inline]
    pub fn is_timera(&self) -> bool {
        *self == SYNC0R::TIMERA
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline]
    pub fn is_nosync(&self) -> bool {
        *self == SYNC0R::NOSYNC
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED8W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED8W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNC3`"]
pub enum SYNC3W {
    #[doc = "A timeout event for both Timer A and Timer B of GPT3 is triggered"]
    BOTH,
    #[doc = "A timeout event for Timer B of GPT3 is triggered"]
    TIMERB,
    #[doc = "A timeout event for Timer A of GPT3 is triggered"]
    TIMERA,
    #[doc = "No Sync. GPT3 is not affected. "]
    NOSYNC,
}
impl SYNC3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNC3W::BOTH => 3,
            SYNC3W::TIMERB => 2,
            SYNC3W::TIMERA => 1,
            SYNC3W::NOSYNC => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNC3W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNC3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNC3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPT3 is triggered"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(SYNC3W::BOTH)
    }
    #[doc = "A timeout event for Timer B of GPT3 is triggered"]
    #[inline]
    pub fn timerb(self) -> &'a mut W {
        self.variant(SYNC3W::TIMERB)
    }
    #[doc = "A timeout event for Timer A of GPT3 is triggered"]
    #[inline]
    pub fn timera(self) -> &'a mut W {
        self.variant(SYNC3W::TIMERA)
    }
    #[doc = "No Sync. GPT3 is not affected."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(SYNC3W::NOSYNC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNC2`"]
pub enum SYNC2W {
    #[doc = "A timeout event for both Timer A and Timer B of GPT2 is triggered"]
    BOTH,
    #[doc = "A timeout event for Timer B of GPT2 is triggered"]
    TIMERB,
    #[doc = "A timeout event for Timer A of GPT2 is triggered"]
    TIMERA,
    #[doc = "No Sync. GPT2 is not affected. "]
    NOSYNC,
}
impl SYNC2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNC2W::BOTH => 3,
            SYNC2W::TIMERB => 2,
            SYNC2W::TIMERA => 1,
            SYNC2W::NOSYNC => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNC2W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNC2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNC2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPT2 is triggered"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(SYNC2W::BOTH)
    }
    #[doc = "A timeout event for Timer B of GPT2 is triggered"]
    #[inline]
    pub fn timerb(self) -> &'a mut W {
        self.variant(SYNC2W::TIMERB)
    }
    #[doc = "A timeout event for Timer A of GPT2 is triggered"]
    #[inline]
    pub fn timera(self) -> &'a mut W {
        self.variant(SYNC2W::TIMERA)
    }
    #[doc = "No Sync. GPT2 is not affected."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(SYNC2W::NOSYNC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNC1`"]
pub enum SYNC1W {
    #[doc = "A timeout event for both Timer A and Timer B of GPT1 is triggered"]
    BOTH,
    #[doc = "A timeout event for Timer B of GPT1 is triggered"]
    TIMERB,
    #[doc = "A timeout event for Timer A of GPT1 is triggered"]
    TIMERA,
    #[doc = "No Sync. GPT1 is not affected. "]
    NOSYNC,
}
impl SYNC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNC1W::BOTH => 3,
            SYNC1W::TIMERB => 2,
            SYNC1W::TIMERA => 1,
            SYNC1W::NOSYNC => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNC1W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNC1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPT1 is triggered"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(SYNC1W::BOTH)
    }
    #[doc = "A timeout event for Timer B of GPT1 is triggered"]
    #[inline]
    pub fn timerb(self) -> &'a mut W {
        self.variant(SYNC1W::TIMERB)
    }
    #[doc = "A timeout event for Timer A of GPT1 is triggered"]
    #[inline]
    pub fn timera(self) -> &'a mut W {
        self.variant(SYNC1W::TIMERA)
    }
    #[doc = "No Sync. GPT1 is not affected."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(SYNC1W::NOSYNC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNC0`"]
pub enum SYNC0W {
    #[doc = "A timeout event for both Timer A and Timer B of GPT0 is triggered"]
    BOTH,
    #[doc = "A timeout event for Timer B of GPT0 is triggered"]
    TIMERB,
    #[doc = "A timeout event for Timer A of GPT0 is triggered"]
    TIMERA,
    #[doc = "No Sync. GPT0 is not affected. "]
    NOSYNC,
}
impl SYNC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNC0W::BOTH => 3,
            SYNC0W::TIMERB => 2,
            SYNC0W::TIMERA => 1,
            SYNC0W::NOSYNC => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNC0W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNC0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPT0 is triggered"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(SYNC0W::BOTH)
    }
    #[doc = "A timeout event for Timer B of GPT0 is triggered"]
    #[inline]
    pub fn timerb(self) -> &'a mut W {
        self.variant(SYNC0W::TIMERB)
    }
    #[doc = "A timeout event for Timer A of GPT0 is triggered"]
    #[inline]
    pub fn timera(self) -> &'a mut W {
        self.variant(SYNC0W::TIMERA)
    }
    #[doc = "No Sync. GPT0 is not affected."]
    #[inline]
    pub fn nosync(self) -> &'a mut W {
        self.variant(SYNC0W::NOSYNC)
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
    #[doc = "Bits 8:31 - 31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved8(&self) -> RESERVED8R {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED8R { bits }
    }
    #[doc = "Bits 6:7 - 7:6\\] Synchronize GPT Timer 3."]
    #[inline]
    pub fn sync3(&self) -> SYNC3R {
        SYNC3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - 5:4\\] Synchronize GPT Timer 2."]
    #[inline]
    pub fn sync2(&self) -> SYNC2R {
        SYNC2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - 3:2\\] Synchronize GPT Timer 1"]
    #[inline]
    pub fn sync1(&self) -> SYNC1R {
        SYNC1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:1 - 1:0\\] Synchronize GPT Timer 0"]
    #[inline]
    pub fn sync0(&self) -> SYNC0R {
        SYNC0R::_from({
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
    #[doc = "Bits 8:31 - 31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved8(&mut self) -> _RESERVED8W {
        _RESERVED8W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Synchronize GPT Timer 3."]
    #[inline]
    pub fn sync3(&mut self) -> _SYNC3W {
        _SYNC3W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\] Synchronize GPT Timer 2."]
    #[inline]
    pub fn sync2(&mut self) -> _SYNC2W {
        _SYNC2W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\] Synchronize GPT Timer 1"]
    #[inline]
    pub fn sync1(&mut self) -> _SYNC1W {
        _SYNC1W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Synchronize GPT Timer 0"]
    #[inline]
    pub fn sync0(&mut self) -> _SYNC0W {
        _SYNC0W { w: self }
    }
}
