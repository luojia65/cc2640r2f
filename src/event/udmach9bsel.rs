#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::UDMACH9BSEL {
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
    #[doc = "GPT3B DMA trigger event. Configured by GPT3:DMAEV"]
    GPT3B_DMABREQ,
    #[doc = "GPT3A DMA trigger event. Configured by GPT3:DMAEV"]
    GPT3A_DMABREQ,
    #[doc = "GPT2B DMA trigger event. Configured by GPT2:DMAEV"]
    GPT2B_DMABREQ,
    #[doc = "GPT2A DMA trigger event. Configured by GPT2:DMAEV"]
    GPT2A_DMABREQ,
    #[doc = "GPT1B DMA trigger event. Configured by GPT1:DMAEV"]
    GPT1B_DMABREQ,
    #[doc = "GPT1A DMA trigger event. Configured by GPT1:DMAEV"]
    GPT1A_DMABREQ,
    #[doc = "GPT0B DMA trigger event. Configured by GPT0:DMAEV"]
    GPT0B_DMABREQ,
    #[doc = "GPT0A DMA trigger event. Configured by GPT0:DMAEV"]
    GPT0A_DMABREQ,
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
            EVR::GPT3B_DMABREQ => 84,
            EVR::GPT3A_DMABREQ => 83,
            EVR::GPT2B_DMABREQ => 82,
            EVR::GPT2A_DMABREQ => 81,
            EVR::GPT1B_DMABREQ => 80,
            EVR::GPT1A_DMABREQ => 79,
            EVR::GPT0B_DMABREQ => 78,
            EVR::GPT0A_DMABREQ => 77,
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
            84 => EVR::GPT3B_DMABREQ,
            83 => EVR::GPT3A_DMABREQ,
            82 => EVR::GPT2B_DMABREQ,
            81 => EVR::GPT2A_DMABREQ,
            80 => EVR::GPT1B_DMABREQ,
            79 => EVR::GPT1A_DMABREQ,
            78 => EVR::GPT0B_DMABREQ,
            77 => EVR::GPT0A_DMABREQ,
            0 => EVR::NONE,
            i => EVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ACTIVE`"]
    #[inline]
    pub fn is_always_active(&self) -> bool {
        *self == EVR::ALWAYS_ACTIVE
    }
    #[doc = "Checks if the value of the field is `GPT3B_DMABREQ`"]
    #[inline]
    pub fn is_gpt3b_dmabreq(&self) -> bool {
        *self == EVR::GPT3B_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT3A_DMABREQ`"]
    #[inline]
    pub fn is_gpt3a_dmabreq(&self) -> bool {
        *self == EVR::GPT3A_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT2B_DMABREQ`"]
    #[inline]
    pub fn is_gpt2b_dmabreq(&self) -> bool {
        *self == EVR::GPT2B_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT2A_DMABREQ`"]
    #[inline]
    pub fn is_gpt2a_dmabreq(&self) -> bool {
        *self == EVR::GPT2A_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT1B_DMABREQ`"]
    #[inline]
    pub fn is_gpt1b_dmabreq(&self) -> bool {
        *self == EVR::GPT1B_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT1A_DMABREQ`"]
    #[inline]
    pub fn is_gpt1a_dmabreq(&self) -> bool {
        *self == EVR::GPT1A_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT0B_DMABREQ`"]
    #[inline]
    pub fn is_gpt0b_dmabreq(&self) -> bool {
        *self == EVR::GPT0B_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT0A_DMABREQ`"]
    #[inline]
    pub fn is_gpt0a_dmabreq(&self) -> bool {
        *self == EVR::GPT0A_DMABREQ
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
    #[doc = "GPT3B DMA trigger event. Configured by GPT3:DMAEV"]
    GPT3B_DMABREQ,
    #[doc = "GPT3A DMA trigger event. Configured by GPT3:DMAEV"]
    GPT3A_DMABREQ,
    #[doc = "GPT2B DMA trigger event. Configured by GPT2:DMAEV"]
    GPT2B_DMABREQ,
    #[doc = "GPT2A DMA trigger event. Configured by GPT2:DMAEV"]
    GPT2A_DMABREQ,
    #[doc = "GPT1B DMA trigger event. Configured by GPT1:DMAEV"]
    GPT1B_DMABREQ,
    #[doc = "GPT1A DMA trigger event. Configured by GPT1:DMAEV"]
    GPT1A_DMABREQ,
    #[doc = "GPT0B DMA trigger event. Configured by GPT0:DMAEV"]
    GPT0B_DMABREQ,
    #[doc = "GPT0A DMA trigger event. Configured by GPT0:DMAEV"]
    GPT0A_DMABREQ,
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
            EVW::GPT3B_DMABREQ => 84,
            EVW::GPT3A_DMABREQ => 83,
            EVW::GPT2B_DMABREQ => 82,
            EVW::GPT2A_DMABREQ => 81,
            EVW::GPT1B_DMABREQ => 80,
            EVW::GPT1A_DMABREQ => 79,
            EVW::GPT0B_DMABREQ => 78,
            EVW::GPT0A_DMABREQ => 77,
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
    #[doc = "GPT3B DMA trigger event. Configured by GPT3:DMAEV"]
    #[inline]
    pub fn gpt3b_dmabreq(self) -> &'a mut W {
        self.variant(EVW::GPT3B_DMABREQ)
    }
    #[doc = "GPT3A DMA trigger event. Configured by GPT3:DMAEV"]
    #[inline]
    pub fn gpt3a_dmabreq(self) -> &'a mut W {
        self.variant(EVW::GPT3A_DMABREQ)
    }
    #[doc = "GPT2B DMA trigger event. Configured by GPT2:DMAEV"]
    #[inline]
    pub fn gpt2b_dmabreq(self) -> &'a mut W {
        self.variant(EVW::GPT2B_DMABREQ)
    }
    #[doc = "GPT2A DMA trigger event. Configured by GPT2:DMAEV"]
    #[inline]
    pub fn gpt2a_dmabreq(self) -> &'a mut W {
        self.variant(EVW::GPT2A_DMABREQ)
    }
    #[doc = "GPT1B DMA trigger event. Configured by GPT1:DMAEV"]
    #[inline]
    pub fn gpt1b_dmabreq(self) -> &'a mut W {
        self.variant(EVW::GPT1B_DMABREQ)
    }
    #[doc = "GPT1A DMA trigger event. Configured by GPT1:DMAEV"]
    #[inline]
    pub fn gpt1a_dmabreq(self) -> &'a mut W {
        self.variant(EVW::GPT1A_DMABREQ)
    }
    #[doc = "GPT0B DMA trigger event. Configured by GPT0:DMAEV"]
    #[inline]
    pub fn gpt0b_dmabreq(self) -> &'a mut W {
        self.variant(EVW::GPT0B_DMABREQ)
    }
    #[doc = "GPT0A DMA trigger event. Configured by GPT0:DMAEV"]
    #[inline]
    pub fn gpt0a_dmabreq(self) -> &'a mut W {
        self.variant(EVW::GPT0A_DMABREQ)
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
        W { bits: 77 }
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
