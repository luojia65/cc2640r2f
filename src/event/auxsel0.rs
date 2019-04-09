#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AUXSEL0 {
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
#[doc = "Possible values of the field `EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVR {
    #[doc = "Always asserted"]
    ALWAYS_ACTIVE,
    #[doc = "GPT1B interrupt event, controlled by GPT1:TBMR"]
    GPT1B,
    #[doc = "GPT1A interrupt event, controlled by GPT1:TAMR"]
    GPT1A,
    #[doc = "GPT0B interrupt event, controlled by GPT0:TBMR"]
    GPT0B,
    #[doc = "GPT0A interrupt event, controlled by GPT0:TAMR"]
    GPT0A,
    #[doc = "GPT3B interrupt event, controlled by GPT3:TBMR"]
    GPT3B,
    #[doc = "GPT3A interrupt event, controlled by GPT3:TAMR"]
    GPT3A,
    #[doc = "GPT2B interrupt event, controlled by GPT2:TBMR"]
    GPT2B,
    #[doc = "GPT2A interrupt event, controlled by GPT2:TAMR"]
    GPT2A,
    #[doc = "Always inactive"]
    NONE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EVR::ALWAYS_ACTIVE => 121,
            EVR::GPT1B => 19,
            EVR::GPT1A => 18,
            EVR::GPT0B => 17,
            EVR::GPT0A => 16,
            EVR::GPT3B => 15,
            EVR::GPT3A => 14,
            EVR::GPT2B => 13,
            EVR::GPT2A => 12,
            EVR::NONE => 0,
            EVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EVR {
        match value {
            121 => EVR::ALWAYS_ACTIVE,
            19 => EVR::GPT1B,
            18 => EVR::GPT1A,
            17 => EVR::GPT0B,
            16 => EVR::GPT0A,
            15 => EVR::GPT3B,
            14 => EVR::GPT3A,
            13 => EVR::GPT2B,
            12 => EVR::GPT2A,
            0 => EVR::NONE,
            i => EVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ACTIVE`"]
    #[inline]
    pub fn is_always_active(&self) -> bool {
        *self == EVR::ALWAYS_ACTIVE
    }
    #[doc = "Checks if the value of the field is `GPT1B`"]
    #[inline]
    pub fn is_gpt1b(&self) -> bool {
        *self == EVR::GPT1B
    }
    #[doc = "Checks if the value of the field is `GPT1A`"]
    #[inline]
    pub fn is_gpt1a(&self) -> bool {
        *self == EVR::GPT1A
    }
    #[doc = "Checks if the value of the field is `GPT0B`"]
    #[inline]
    pub fn is_gpt0b(&self) -> bool {
        *self == EVR::GPT0B
    }
    #[doc = "Checks if the value of the field is `GPT0A`"]
    #[inline]
    pub fn is_gpt0a(&self) -> bool {
        *self == EVR::GPT0A
    }
    #[doc = "Checks if the value of the field is `GPT3B`"]
    #[inline]
    pub fn is_gpt3b(&self) -> bool {
        *self == EVR::GPT3B
    }
    #[doc = "Checks if the value of the field is `GPT3A`"]
    #[inline]
    pub fn is_gpt3a(&self) -> bool {
        *self == EVR::GPT3A
    }
    #[doc = "Checks if the value of the field is `GPT2B`"]
    #[inline]
    pub fn is_gpt2b(&self) -> bool {
        *self == EVR::GPT2B
    }
    #[doc = "Checks if the value of the field is `GPT2A`"]
    #[inline]
    pub fn is_gpt2a(&self) -> bool {
        *self == EVR::GPT2A
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == EVR::NONE
    }
}
#[doc = "Values that can be written to the field `EV`"]
pub enum EVW {
    #[doc = "Always asserted"]
    ALWAYS_ACTIVE,
    #[doc = "GPT1B interrupt event, controlled by GPT1:TBMR"]
    GPT1B,
    #[doc = "GPT1A interrupt event, controlled by GPT1:TAMR"]
    GPT1A,
    #[doc = "GPT0B interrupt event, controlled by GPT0:TBMR"]
    GPT0B,
    #[doc = "GPT0A interrupt event, controlled by GPT0:TAMR"]
    GPT0A,
    #[doc = "GPT3B interrupt event, controlled by GPT3:TBMR"]
    GPT3B,
    #[doc = "GPT3A interrupt event, controlled by GPT3:TAMR"]
    GPT3A,
    #[doc = "GPT2B interrupt event, controlled by GPT2:TBMR"]
    GPT2B,
    #[doc = "GPT2A interrupt event, controlled by GPT2:TAMR"]
    GPT2A,
    #[doc = "Always inactive"]
    NONE,
}
impl EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EVW::ALWAYS_ACTIVE => 121,
            EVW::GPT1B => 19,
            EVW::GPT1A => 18,
            EVW::GPT0B => 17,
            EVW::GPT0A => 16,
            EVW::GPT3B => 15,
            EVW::GPT3A => 14,
            EVW::GPT2B => 13,
            EVW::GPT2A => 12,
            EVW::NONE => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EVW<'a> {
    w: &'a mut W,
}
impl<'a> _EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Always asserted"]
    #[inline]
    pub fn always_active(self) -> &'a mut W {
        self.variant(EVW::ALWAYS_ACTIVE)
    }
    #[doc = "GPT1B interrupt event, controlled by GPT1:TBMR"]
    #[inline]
    pub fn gpt1b(self) -> &'a mut W {
        self.variant(EVW::GPT1B)
    }
    #[doc = "GPT1A interrupt event, controlled by GPT1:TAMR"]
    #[inline]
    pub fn gpt1a(self) -> &'a mut W {
        self.variant(EVW::GPT1A)
    }
    #[doc = "GPT0B interrupt event, controlled by GPT0:TBMR"]
    #[inline]
    pub fn gpt0b(self) -> &'a mut W {
        self.variant(EVW::GPT0B)
    }
    #[doc = "GPT0A interrupt event, controlled by GPT0:TAMR"]
    #[inline]
    pub fn gpt0a(self) -> &'a mut W {
        self.variant(EVW::GPT0A)
    }
    #[doc = "GPT3B interrupt event, controlled by GPT3:TBMR"]
    #[inline]
    pub fn gpt3b(self) -> &'a mut W {
        self.variant(EVW::GPT3B)
    }
    #[doc = "GPT3A interrupt event, controlled by GPT3:TAMR"]
    #[inline]
    pub fn gpt3a(self) -> &'a mut W {
        self.variant(EVW::GPT3A)
    }
    #[doc = "GPT2B interrupt event, controlled by GPT2:TBMR"]
    #[inline]
    pub fn gpt2b(self) -> &'a mut W {
        self.variant(EVW::GPT2B)
    }
    #[doc = "GPT2A interrupt event, controlled by GPT2:TAMR"]
    #[inline]
    pub fn gpt2a(self) -> &'a mut W {
        self.variant(EVW::GPT2A)
    }
    #[doc = "Always inactive"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(EVW::NONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
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
    #[doc = "Bits 0:6 - 6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline]
    pub fn ev(&self) -> EVR {
        EVR::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - 6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline]
    pub fn ev(&mut self) -> _EVW {
        _EVW { w: self }
    }
}
