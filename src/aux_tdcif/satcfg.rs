#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SATCFG {
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
pub struct RESERVED4R {
    bits: u32,
}
impl RESERVED4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `LIMIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LIMITR {
    #[doc = "Result bit 24: TDC conversion saturates and stops when RESULT.VALUE\\[24\\] is set."]
    R24,
    #[doc = "Result bit 23: TDC conversion saturates and stops when RESULT.VALUE\\[23\\] is set."]
    R23,
    #[doc = "Result bit 22: TDC conversion saturates and stops when RESULT.VALUE\\[22\\] is set."]
    R22,
    #[doc = "Result bit 21: TDC conversion saturates and stops when RESULT.VALUE\\[21\\] is set. "]
    R21,
    #[doc = "Result bit 20: TDC conversion saturates and stops when RESULT.VALUE\\[20\\] is set."]
    R20,
    #[doc = "Result bit 19: TDC conversion saturates and stops when RESULT.VALUE\\[19\\] is set. "]
    R19,
    #[doc = "Result bit 18: TDC conversion saturates and stops when RESULT.VALUE\\[18\\] is set."]
    R18,
    #[doc = "Result bit 17: TDC conversion saturates and stops when RESULT.VALUE\\[17\\] is set. "]
    R17,
    #[doc = "Result bit 16: TDC conversion saturates and stops when RESULT.VALUE\\[16\\] is set."]
    R16,
    #[doc = "Result bit 15: TDC conversion saturates and stops when RESULT.VALUE\\[15\\] is set."]
    R15,
    #[doc = "Result bit 14: TDC conversion saturates and stops when RESULT.VALUE\\[14\\] is set."]
    R14,
    #[doc = "Result bit 13: TDC conversion saturates and stops when RESULT.VALUE\\[13\\] is set."]
    R13,
    #[doc = "Result bit 12: TDC conversion saturates and stops when RESULT.VALUE\\[12\\] is set. "]
    R12,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LIMITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LIMITR::R24 => 15,
            LIMITR::R23 => 14,
            LIMITR::R22 => 13,
            LIMITR::R21 => 12,
            LIMITR::R20 => 11,
            LIMITR::R19 => 10,
            LIMITR::R18 => 9,
            LIMITR::R17 => 8,
            LIMITR::R16 => 7,
            LIMITR::R15 => 6,
            LIMITR::R14 => 5,
            LIMITR::R13 => 4,
            LIMITR::R12 => 3,
            LIMITR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LIMITR {
        match value {
            15 => LIMITR::R24,
            14 => LIMITR::R23,
            13 => LIMITR::R22,
            12 => LIMITR::R21,
            11 => LIMITR::R20,
            10 => LIMITR::R19,
            9 => LIMITR::R18,
            8 => LIMITR::R17,
            7 => LIMITR::R16,
            6 => LIMITR::R15,
            5 => LIMITR::R14,
            4 => LIMITR::R13,
            3 => LIMITR::R12,
            i => LIMITR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `R24`"]
    #[inline]
    pub fn is_r24(&self) -> bool {
        *self == LIMITR::R24
    }
    #[doc = "Checks if the value of the field is `R23`"]
    #[inline]
    pub fn is_r23(&self) -> bool {
        *self == LIMITR::R23
    }
    #[doc = "Checks if the value of the field is `R22`"]
    #[inline]
    pub fn is_r22(&self) -> bool {
        *self == LIMITR::R22
    }
    #[doc = "Checks if the value of the field is `R21`"]
    #[inline]
    pub fn is_r21(&self) -> bool {
        *self == LIMITR::R21
    }
    #[doc = "Checks if the value of the field is `R20`"]
    #[inline]
    pub fn is_r20(&self) -> bool {
        *self == LIMITR::R20
    }
    #[doc = "Checks if the value of the field is `R19`"]
    #[inline]
    pub fn is_r19(&self) -> bool {
        *self == LIMITR::R19
    }
    #[doc = "Checks if the value of the field is `R18`"]
    #[inline]
    pub fn is_r18(&self) -> bool {
        *self == LIMITR::R18
    }
    #[doc = "Checks if the value of the field is `R17`"]
    #[inline]
    pub fn is_r17(&self) -> bool {
        *self == LIMITR::R17
    }
    #[doc = "Checks if the value of the field is `R16`"]
    #[inline]
    pub fn is_r16(&self) -> bool {
        *self == LIMITR::R16
    }
    #[doc = "Checks if the value of the field is `R15`"]
    #[inline]
    pub fn is_r15(&self) -> bool {
        *self == LIMITR::R15
    }
    #[doc = "Checks if the value of the field is `R14`"]
    #[inline]
    pub fn is_r14(&self) -> bool {
        *self == LIMITR::R14
    }
    #[doc = "Checks if the value of the field is `R13`"]
    #[inline]
    pub fn is_r13(&self) -> bool {
        *self == LIMITR::R13
    }
    #[doc = "Checks if the value of the field is `R12`"]
    #[inline]
    pub fn is_r12(&self) -> bool {
        *self == LIMITR::R12
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED4W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 268435455;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LIMIT`"]
pub enum LIMITW {
    #[doc = "Result bit 24: TDC conversion saturates and stops when RESULT.VALUE\\[24\\] is set."]
    R24,
    #[doc = "Result bit 23: TDC conversion saturates and stops when RESULT.VALUE\\[23\\] is set."]
    R23,
    #[doc = "Result bit 22: TDC conversion saturates and stops when RESULT.VALUE\\[22\\] is set."]
    R22,
    #[doc = "Result bit 21: TDC conversion saturates and stops when RESULT.VALUE\\[21\\] is set. "]
    R21,
    #[doc = "Result bit 20: TDC conversion saturates and stops when RESULT.VALUE\\[20\\] is set."]
    R20,
    #[doc = "Result bit 19: TDC conversion saturates and stops when RESULT.VALUE\\[19\\] is set. "]
    R19,
    #[doc = "Result bit 18: TDC conversion saturates and stops when RESULT.VALUE\\[18\\] is set."]
    R18,
    #[doc = "Result bit 17: TDC conversion saturates and stops when RESULT.VALUE\\[17\\] is set. "]
    R17,
    #[doc = "Result bit 16: TDC conversion saturates and stops when RESULT.VALUE\\[16\\] is set."]
    R16,
    #[doc = "Result bit 15: TDC conversion saturates and stops when RESULT.VALUE\\[15\\] is set."]
    R15,
    #[doc = "Result bit 14: TDC conversion saturates and stops when RESULT.VALUE\\[14\\] is set."]
    R14,
    #[doc = "Result bit 13: TDC conversion saturates and stops when RESULT.VALUE\\[13\\] is set."]
    R13,
    #[doc = "Result bit 12: TDC conversion saturates and stops when RESULT.VALUE\\[12\\] is set. "]
    R12,
}
impl LIMITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LIMITW::R24 => 15,
            LIMITW::R23 => 14,
            LIMITW::R22 => 13,
            LIMITW::R21 => 12,
            LIMITW::R20 => 11,
            LIMITW::R19 => 10,
            LIMITW::R18 => 9,
            LIMITW::R17 => 8,
            LIMITW::R16 => 7,
            LIMITW::R15 => 6,
            LIMITW::R14 => 5,
            LIMITW::R13 => 4,
            LIMITW::R12 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LIMITW<'a> {
    w: &'a mut W,
}
impl<'a> _LIMITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LIMITW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Result bit 24: TDC conversion saturates and stops when RESULT.VALUE\\[24\\] is set."]
    #[inline]
    pub fn r24(self) -> &'a mut W {
        self.variant(LIMITW::R24)
    }
    #[doc = "Result bit 23: TDC conversion saturates and stops when RESULT.VALUE\\[23\\] is set."]
    #[inline]
    pub fn r23(self) -> &'a mut W {
        self.variant(LIMITW::R23)
    }
    #[doc = "Result bit 22: TDC conversion saturates and stops when RESULT.VALUE\\[22\\] is set."]
    #[inline]
    pub fn r22(self) -> &'a mut W {
        self.variant(LIMITW::R22)
    }
    #[doc = "Result bit 21: TDC conversion saturates and stops when RESULT.VALUE\\[21\\] is set."]
    #[inline]
    pub fn r21(self) -> &'a mut W {
        self.variant(LIMITW::R21)
    }
    #[doc = "Result bit 20: TDC conversion saturates and stops when RESULT.VALUE\\[20\\] is set."]
    #[inline]
    pub fn r20(self) -> &'a mut W {
        self.variant(LIMITW::R20)
    }
    #[doc = "Result bit 19: TDC conversion saturates and stops when RESULT.VALUE\\[19\\] is set."]
    #[inline]
    pub fn r19(self) -> &'a mut W {
        self.variant(LIMITW::R19)
    }
    #[doc = "Result bit 18: TDC conversion saturates and stops when RESULT.VALUE\\[18\\] is set."]
    #[inline]
    pub fn r18(self) -> &'a mut W {
        self.variant(LIMITW::R18)
    }
    #[doc = "Result bit 17: TDC conversion saturates and stops when RESULT.VALUE\\[17\\] is set."]
    #[inline]
    pub fn r17(self) -> &'a mut W {
        self.variant(LIMITW::R17)
    }
    #[doc = "Result bit 16: TDC conversion saturates and stops when RESULT.VALUE\\[16\\] is set."]
    #[inline]
    pub fn r16(self) -> &'a mut W {
        self.variant(LIMITW::R16)
    }
    #[doc = "Result bit 15: TDC conversion saturates and stops when RESULT.VALUE\\[15\\] is set."]
    #[inline]
    pub fn r15(self) -> &'a mut W {
        self.variant(LIMITW::R15)
    }
    #[doc = "Result bit 14: TDC conversion saturates and stops when RESULT.VALUE\\[14\\] is set."]
    #[inline]
    pub fn r14(self) -> &'a mut W {
        self.variant(LIMITW::R14)
    }
    #[doc = "Result bit 13: TDC conversion saturates and stops when RESULT.VALUE\\[13\\] is set."]
    #[inline]
    pub fn r13(self) -> &'a mut W {
        self.variant(LIMITW::R13)
    }
    #[doc = "Result bit 12: TDC conversion saturates and stops when RESULT.VALUE\\[12\\] is set."]
    #[inline]
    pub fn r12(self) -> &'a mut W {
        self.variant(LIMITW::R12)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 4:31 - 31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&self) -> RESERVED4R {
        let bits = {
            const MASK: u32 = 268435455;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED4R { bits }
    }
    #[doc = "Bits 0:3 - 3:0\\] Saturation limit. The flag STAT.SAT is set when the TDC counter saturates. Values not enumerated are not supported"]
    #[inline]
    pub fn limit(&self) -> LIMITR {
        LIMITR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 15 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 4:31 - 31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&mut self) -> _RESERVED4W {
        _RESERVED4W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Saturation limit. The flag STAT.SAT is set when the TDC counter saturates. Values not enumerated are not supported"]
    #[inline]
    pub fn limit(&mut self) -> _LIMITW {
        _LIMITW { w: self }
    }
}
