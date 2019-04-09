#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTL {
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
pub struct RESERVED19R {
    bits: u16,
}
impl RESERVED19R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `COMB_EV_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMB_EV_MASKR {
    #[doc = "Use Channel 2 delayed event in combined event"]
    CH2,
    #[doc = "Use Channel 1 delayed event in combined event"]
    CH1,
    #[doc = "Use Channel 0 delayed event in combined event"]
    CH0,
    #[doc = "No event is selected for combined event."]
    NONE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl COMB_EV_MASKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COMB_EV_MASKR::CH2 => 4,
            COMB_EV_MASKR::CH1 => 2,
            COMB_EV_MASKR::CH0 => 1,
            COMB_EV_MASKR::NONE => 0,
            COMB_EV_MASKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COMB_EV_MASKR {
        match value {
            4 => COMB_EV_MASKR::CH2,
            2 => COMB_EV_MASKR::CH1,
            1 => COMB_EV_MASKR::CH0,
            0 => COMB_EV_MASKR::NONE,
            i => COMB_EV_MASKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline]
    pub fn is_ch2(&self) -> bool {
        *self == COMB_EV_MASKR::CH2
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline]
    pub fn is_ch1(&self) -> bool {
        *self == COMB_EV_MASKR::CH1
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline]
    pub fn is_ch0(&self) -> bool {
        *self == COMB_EV_MASKR::CH0
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == COMB_EV_MASKR::NONE
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED12R {
    bits: u8,
}
impl RESERVED12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `EV_DELAY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EV_DELAYR {
    #[doc = "Delay by 144 clock cycles"]
    D144,
    #[doc = "Delay by 128 clock cycles"]
    D128,
    #[doc = "Delay by 112 clock cycles"]
    D112,
    #[doc = "Delay by 96 clock cycles"]
    D96,
    #[doc = "Delay by 80 clock cycles"]
    D80,
    #[doc = "Delay by 64 clock cycles"]
    D64,
    #[doc = "Delay by 48 clock cycles"]
    D48,
    #[doc = "Delay by 32 clock cycles"]
    D32,
    #[doc = "Delay by 16 clock cycles"]
    D16,
    #[doc = "Delay by 8 clock cycles"]
    D8,
    #[doc = "Delay by 4 clock cycles"]
    D4,
    #[doc = "Delay by 2 clock cycles"]
    D2,
    #[doc = "Delay by 1 clock cycles"]
    D1,
    #[doc = "No delay on delayed event"]
    D0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EV_DELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EV_DELAYR::D144 => 13,
            EV_DELAYR::D128 => 12,
            EV_DELAYR::D112 => 11,
            EV_DELAYR::D96 => 10,
            EV_DELAYR::D80 => 9,
            EV_DELAYR::D64 => 8,
            EV_DELAYR::D48 => 7,
            EV_DELAYR::D32 => 6,
            EV_DELAYR::D16 => 5,
            EV_DELAYR::D8 => 4,
            EV_DELAYR::D4 => 3,
            EV_DELAYR::D2 => 2,
            EV_DELAYR::D1 => 1,
            EV_DELAYR::D0 => 0,
            EV_DELAYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EV_DELAYR {
        match value {
            13 => EV_DELAYR::D144,
            12 => EV_DELAYR::D128,
            11 => EV_DELAYR::D112,
            10 => EV_DELAYR::D96,
            9 => EV_DELAYR::D80,
            8 => EV_DELAYR::D64,
            7 => EV_DELAYR::D48,
            6 => EV_DELAYR::D32,
            5 => EV_DELAYR::D16,
            4 => EV_DELAYR::D8,
            3 => EV_DELAYR::D4,
            2 => EV_DELAYR::D2,
            1 => EV_DELAYR::D1,
            0 => EV_DELAYR::D0,
            i => EV_DELAYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `D144`"]
    #[inline]
    pub fn is_d144(&self) -> bool {
        *self == EV_DELAYR::D144
    }
    #[doc = "Checks if the value of the field is `D128`"]
    #[inline]
    pub fn is_d128(&self) -> bool {
        *self == EV_DELAYR::D128
    }
    #[doc = "Checks if the value of the field is `D112`"]
    #[inline]
    pub fn is_d112(&self) -> bool {
        *self == EV_DELAYR::D112
    }
    #[doc = "Checks if the value of the field is `D96`"]
    #[inline]
    pub fn is_d96(&self) -> bool {
        *self == EV_DELAYR::D96
    }
    #[doc = "Checks if the value of the field is `D80`"]
    #[inline]
    pub fn is_d80(&self) -> bool {
        *self == EV_DELAYR::D80
    }
    #[doc = "Checks if the value of the field is `D64`"]
    #[inline]
    pub fn is_d64(&self) -> bool {
        *self == EV_DELAYR::D64
    }
    #[doc = "Checks if the value of the field is `D48`"]
    #[inline]
    pub fn is_d48(&self) -> bool {
        *self == EV_DELAYR::D48
    }
    #[doc = "Checks if the value of the field is `D32`"]
    #[inline]
    pub fn is_d32(&self) -> bool {
        *self == EV_DELAYR::D32
    }
    #[doc = "Checks if the value of the field is `D16`"]
    #[inline]
    pub fn is_d16(&self) -> bool {
        *self == EV_DELAYR::D16
    }
    #[doc = "Checks if the value of the field is `D8`"]
    #[inline]
    pub fn is_d8(&self) -> bool {
        *self == EV_DELAYR::D8
    }
    #[doc = "Checks if the value of the field is `D4`"]
    #[inline]
    pub fn is_d4(&self) -> bool {
        *self == EV_DELAYR::D4
    }
    #[doc = "Checks if the value of the field is `D2`"]
    #[inline]
    pub fn is_d2(&self) -> bool {
        *self == EV_DELAYR::D2
    }
    #[doc = "Checks if the value of the field is `D1`"]
    #[inline]
    pub fn is_d1(&self) -> bool {
        *self == EV_DELAYR::D1
    }
    #[doc = "Checks if the value of the field is `D0`"]
    #[inline]
    pub fn is_d0(&self) -> bool {
        *self == EV_DELAYR::D0
    }
}
#[doc = r" Value of the field"]
pub struct RESETR {
    bits: bool,
}
impl RESETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
#[doc = r" Value of the field"]
pub struct RTC_4KHZ_ENR {
    bits: bool,
}
impl RTC_4KHZ_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct RTC_UPD_ENR {
    bits: bool,
}
impl RTC_UPD_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct ENR {
    bits: bool,
}
impl ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Proxy"]
pub struct _RESERVED19W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED19W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 8191;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COMB_EV_MASK`"]
pub enum COMB_EV_MASKW {
    #[doc = "Use Channel 2 delayed event in combined event"]
    CH2,
    #[doc = "Use Channel 1 delayed event in combined event"]
    CH1,
    #[doc = "Use Channel 0 delayed event in combined event"]
    CH0,
    #[doc = "No event is selected for combined event."]
    NONE,
}
impl COMB_EV_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COMB_EV_MASKW::CH2 => 4,
            COMB_EV_MASKW::CH1 => 2,
            COMB_EV_MASKW::CH0 => 1,
            COMB_EV_MASKW::NONE => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMB_EV_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _COMB_EV_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMB_EV_MASKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Use Channel 2 delayed event in combined event"]
    #[inline]
    pub fn ch2(self) -> &'a mut W {
        self.variant(COMB_EV_MASKW::CH2)
    }
    #[doc = "Use Channel 1 delayed event in combined event"]
    #[inline]
    pub fn ch1(self) -> &'a mut W {
        self.variant(COMB_EV_MASKW::CH1)
    }
    #[doc = "Use Channel 0 delayed event in combined event"]
    #[inline]
    pub fn ch0(self) -> &'a mut W {
        self.variant(COMB_EV_MASKW::CH0)
    }
    #[doc = "No event is selected for combined event."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(COMB_EV_MASKW::NONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED12W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED12W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EV_DELAY`"]
pub enum EV_DELAYW {
    #[doc = "Delay by 144 clock cycles"]
    D144,
    #[doc = "Delay by 128 clock cycles"]
    D128,
    #[doc = "Delay by 112 clock cycles"]
    D112,
    #[doc = "Delay by 96 clock cycles"]
    D96,
    #[doc = "Delay by 80 clock cycles"]
    D80,
    #[doc = "Delay by 64 clock cycles"]
    D64,
    #[doc = "Delay by 48 clock cycles"]
    D48,
    #[doc = "Delay by 32 clock cycles"]
    D32,
    #[doc = "Delay by 16 clock cycles"]
    D16,
    #[doc = "Delay by 8 clock cycles"]
    D8,
    #[doc = "Delay by 4 clock cycles"]
    D4,
    #[doc = "Delay by 2 clock cycles"]
    D2,
    #[doc = "Delay by 1 clock cycles"]
    D1,
    #[doc = "No delay on delayed event"]
    D0,
}
impl EV_DELAYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EV_DELAYW::D144 => 13,
            EV_DELAYW::D128 => 12,
            EV_DELAYW::D112 => 11,
            EV_DELAYW::D96 => 10,
            EV_DELAYW::D80 => 9,
            EV_DELAYW::D64 => 8,
            EV_DELAYW::D48 => 7,
            EV_DELAYW::D32 => 6,
            EV_DELAYW::D16 => 5,
            EV_DELAYW::D8 => 4,
            EV_DELAYW::D4 => 3,
            EV_DELAYW::D2 => 2,
            EV_DELAYW::D1 => 1,
            EV_DELAYW::D0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EV_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _EV_DELAYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EV_DELAYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Delay by 144 clock cycles"]
    #[inline]
    pub fn d144(self) -> &'a mut W {
        self.variant(EV_DELAYW::D144)
    }
    #[doc = "Delay by 128 clock cycles"]
    #[inline]
    pub fn d128(self) -> &'a mut W {
        self.variant(EV_DELAYW::D128)
    }
    #[doc = "Delay by 112 clock cycles"]
    #[inline]
    pub fn d112(self) -> &'a mut W {
        self.variant(EV_DELAYW::D112)
    }
    #[doc = "Delay by 96 clock cycles"]
    #[inline]
    pub fn d96(self) -> &'a mut W {
        self.variant(EV_DELAYW::D96)
    }
    #[doc = "Delay by 80 clock cycles"]
    #[inline]
    pub fn d80(self) -> &'a mut W {
        self.variant(EV_DELAYW::D80)
    }
    #[doc = "Delay by 64 clock cycles"]
    #[inline]
    pub fn d64(self) -> &'a mut W {
        self.variant(EV_DELAYW::D64)
    }
    #[doc = "Delay by 48 clock cycles"]
    #[inline]
    pub fn d48(self) -> &'a mut W {
        self.variant(EV_DELAYW::D48)
    }
    #[doc = "Delay by 32 clock cycles"]
    #[inline]
    pub fn d32(self) -> &'a mut W {
        self.variant(EV_DELAYW::D32)
    }
    #[doc = "Delay by 16 clock cycles"]
    #[inline]
    pub fn d16(self) -> &'a mut W {
        self.variant(EV_DELAYW::D16)
    }
    #[doc = "Delay by 8 clock cycles"]
    #[inline]
    pub fn d8(self) -> &'a mut W {
        self.variant(EV_DELAYW::D8)
    }
    #[doc = "Delay by 4 clock cycles"]
    #[inline]
    pub fn d4(self) -> &'a mut W {
        self.variant(EV_DELAYW::D4)
    }
    #[doc = "Delay by 2 clock cycles"]
    #[inline]
    pub fn d2(self) -> &'a mut W {
        self.variant(EV_DELAYW::D2)
    }
    #[doc = "Delay by 1 clock cycles"]
    #[inline]
    pub fn d1(self) -> &'a mut W {
        self.variant(EV_DELAYW::D1)
    }
    #[doc = "No delay on delayed event"]
    #[inline]
    pub fn d0(self) -> &'a mut W {
        self.variant(EV_DELAYW::D0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _RESETW<'a> {
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
        const OFFSET: u8 = 7;
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
        const MASK: u8 = 15;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RTC_4KHZ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_4KHZ_ENW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RTC_UPD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_UPD_ENW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
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
    #[doc = "Bits 19:31 - 31:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved19(&self) -> RESERVED19R {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED19R { bits }
    }
    #[doc = "Bits 16:18 - 18:16\\] Eventmask selecting which delayed events that form the combined event."]
    #[inline]
    pub fn comb_ev_mask(&self) -> COMB_EV_MASKR {
        COMB_EV_MASKR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - 15:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved12(&self) -> RESERVED12R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED12R { bits }
    }
    #[doc = "Bits 8:11 - 11:8\\] Number of SCLK_LF clock cycles waited before generating delayed events. (Common setting for all RTC cannels) the delayed event is delayed"]
    #[inline]
    pub fn ev_delay(&self) -> EV_DELAYR {
        EV_DELAYR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - 7:7\\] RTC Counter reset. Writing 1 to this bit will reset the RTC counter. This bit is cleared when reset takes effect"]
    #[inline]
    pub fn reset(&self) -> RESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESETR { bits }
    }
    #[doc = "Bits 3:6 - 6:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&self) -> RESERVED3R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED3R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] RTC_4KHZ is a 4 KHz reference output, tapped from SUBSEC.VALUE bit 19 which is used by AUX timer. 0: RTC_4KHZ signal is forced to 0 1: RTC_4KHZ is enabled ( provied that RTC is enabled EN)"]
    #[inline]
    pub fn rtc_4khz_en(&self) -> RTC_4KHZ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RTC_4KHZ_ENR { bits }
    }
    #[doc = "Bit 1 - 1:1\\] RTC_UPD is a 16 KHz signal used to sync up the radio timer. The 16 Khz is SCLK_LF divided by 2 0: RTC_UPD signal is forced to 0 1: RTC_UPD signal is toggling @16 kHz"]
    #[inline]
    pub fn rtc_upd_en(&self) -> RTC_UPD_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RTC_UPD_ENR { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Enable RTC counter 0: Halted (frozen) 1: Running"]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENR { bits }
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
    #[doc = "Bits 19:31 - 31:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved19(&mut self) -> _RESERVED19W {
        _RESERVED19W { w: self }
    }
    #[doc = "Bits 16:18 - 18:16\\] Eventmask selecting which delayed events that form the combined event."]
    #[inline]
    pub fn comb_ev_mask(&mut self) -> _COMB_EV_MASKW {
        _COMB_EV_MASKW { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved12(&mut self) -> _RESERVED12W {
        _RESERVED12W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\] Number of SCLK_LF clock cycles waited before generating delayed events. (Common setting for all RTC cannels) the delayed event is delayed"]
    #[inline]
    pub fn ev_delay(&mut self) -> _EV_DELAYW {
        _EV_DELAYW { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] RTC Counter reset. Writing 1 to this bit will reset the RTC counter. This bit is cleared when reset takes effect"]
    #[inline]
    pub fn reset(&mut self) -> _RESETW {
        _RESETW { w: self }
    }
    #[doc = "Bits 3:6 - 6:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&mut self) -> _RESERVED3W {
        _RESERVED3W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] RTC_4KHZ is a 4 KHz reference output, tapped from SUBSEC.VALUE bit 19 which is used by AUX timer. 0: RTC_4KHZ signal is forced to 0 1: RTC_4KHZ is enabled ( provied that RTC is enabled EN)"]
    #[inline]
    pub fn rtc_4khz_en(&mut self) -> _RTC_4KHZ_ENW {
        _RTC_4KHZ_ENW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] RTC_UPD is a 16 KHz signal used to sync up the radio timer. The 16 Khz is SCLK_LF divided by 2 0: RTC_UPD signal is forced to 0 1: RTC_UPD signal is toggling @16 kHz"]
    #[inline]
    pub fn rtc_upd_en(&mut self) -> _RTC_UPD_ENW {
        _RTC_UPD_ENW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Enable RTC counter 0: Halted (frozen) 1: Running"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
}
