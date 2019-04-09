#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AIFFMTCFG {
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
pub struct RESERVED16R {
    bits: u16,
}
impl RESERVED16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATA_DELAYR {
    bits: u8,
}
impl DATA_DELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `MEM_LEN_24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEM_LEN_24R {
    #[doc = "24-bit (one 8 bit and one 16 bit locked access per sample)"]
    _24BIT,
    #[doc = "16-bit (one 16 bit access per sample)"]
    _16BIT,
}
impl MEM_LEN_24R {
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
            MEM_LEN_24R::_24BIT => true,
            MEM_LEN_24R::_16BIT => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MEM_LEN_24R {
        match value {
            true => MEM_LEN_24R::_24BIT,
            false => MEM_LEN_24R::_16BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_24BIT`"]
    #[inline]
    pub fn is_24bit(&self) -> bool {
        *self == MEM_LEN_24R::_24BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline]
    pub fn is_16bit(&self) -> bool {
        *self == MEM_LEN_24R::_16BIT
    }
}
#[doc = "Possible values of the field `SMPL_EDGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPL_EDGER {
    #[doc = "Data is sampled on the positive edge and clocked out on the negative edge."]
    POS,
    #[doc = "Data is sampled on the negative edge and clocked out on the positive edge. "]
    NEG,
}
impl SMPL_EDGER {
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
            SMPL_EDGER::POS => true,
            SMPL_EDGER::NEG => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMPL_EDGER {
        match value {
            true => SMPL_EDGER::POS,
            false => SMPL_EDGER::NEG,
        }
    }
    #[doc = "Checks if the value of the field is `POS`"]
    #[inline]
    pub fn is_pos(&self) -> bool {
        *self == SMPL_EDGER::POS
    }
    #[doc = "Checks if the value of the field is `NEG`"]
    #[inline]
    pub fn is_neg(&self) -> bool {
        *self == SMPL_EDGER::NEG
    }
}
#[doc = r" Value of the field"]
pub struct DUAL_PHASER {
    bits: bool,
}
impl DUAL_PHASER {
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
pub struct WORD_LENR {
    bits: u8,
}
impl WORD_LENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED16W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED16W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATA_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_DELAYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MEM_LEN_24`"]
pub enum MEM_LEN_24W {
    #[doc = "24-bit (one 8 bit and one 16 bit locked access per sample)"]
    _24BIT,
    #[doc = "16-bit (one 16 bit access per sample)"]
    _16BIT,
}
impl MEM_LEN_24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MEM_LEN_24W::_24BIT => true,
            MEM_LEN_24W::_16BIT => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MEM_LEN_24W<'a> {
    w: &'a mut W,
}
impl<'a> _MEM_LEN_24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MEM_LEN_24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "24-bit (one 8 bit and one 16 bit locked access per sample)"]
    #[inline]
    pub fn _24bit(self) -> &'a mut W {
        self.variant(MEM_LEN_24W::_24BIT)
    }
    #[doc = "16-bit (one 16 bit access per sample)"]
    #[inline]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(MEM_LEN_24W::_16BIT)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMPL_EDGE`"]
pub enum SMPL_EDGEW {
    #[doc = "Data is sampled on the positive edge and clocked out on the negative edge."]
    POS,
    #[doc = "Data is sampled on the negative edge and clocked out on the positive edge. "]
    NEG,
}
impl SMPL_EDGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMPL_EDGEW::POS => true,
            SMPL_EDGEW::NEG => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMPL_EDGEW<'a> {
    w: &'a mut W,
}
impl<'a> _SMPL_EDGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMPL_EDGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data is sampled on the positive edge and clocked out on the negative edge."]
    #[inline]
    pub fn pos(self) -> &'a mut W {
        self.variant(SMPL_EDGEW::POS)
    }
    #[doc = "Data is sampled on the negative edge and clocked out on the positive edge."]
    #[inline]
    pub fn neg(self) -> &'a mut W {
        self.variant(SMPL_EDGEW::NEG)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DUAL_PHASEW<'a> {
    w: &'a mut W,
}
impl<'a> _DUAL_PHASEW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WORD_LENW<'a> {
    w: &'a mut W,
}
impl<'a> _WORD_LENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
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
    #[doc = "Bits 16:31 - 31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved16(&self) -> RESERVED16R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED16R { bits }
    }
    #[doc = "Bits 8:15 - 15:8\\] The number of BCLK periods between a WCLK edge and MSB of the first word in a phase: 0x00: LJF and DSP format 0x01: I2S and DSP format 0x02: RJF format ... 0xFF: RJF format Note: When 0, MSB of the next word will be output in the idle period between LSB of the previous word and the start of the next word. Otherwise logical 0 will be output until the data delay has expired."]
    #[inline]
    pub fn data_delay(&self) -> DATA_DELAYR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATA_DELAYR { bits }
    }
    #[doc = "Bit 7 - 7:7\\] The size of each word stored to or loaded from memory:"]
    #[inline]
    pub fn mem_len_24(&self) -> MEM_LEN_24R {
        MEM_LEN_24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - 6:6\\] On the serial audio interface, data (and wclk) is sampled and clocked out on opposite edges of BCLK."]
    #[inline]
    pub fn smpl_edge(&self) -> SMPL_EDGER {
        SMPL_EDGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - 5:5\\] Selects dual- or single-phase format. 0: Single-phase: DSP format 1: Dual-phase: I2S, LJF and RJF formats"]
    #[inline]
    pub fn dual_phase(&self) -> DUAL_PHASER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DUAL_PHASER { bits }
    }
    #[doc = "Bits 0:4 - 4:0\\] Number of bits per word (8-24): In single-phase format, this is the exact number of bits per word. In dual-phase format, this is the maximum number of bits per word. Values below 8 and above 24 give undefined behavior. Data written to memory is always aligned to 16 or 24 bits as defined by MEM_LEN_24. Bit widths that differ from this alignment will either be truncated or zero padded."]
    #[inline]
    pub fn word_len(&self) -> WORD_LENR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WORD_LENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 368 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 16:31 - 31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved16(&mut self) -> _RESERVED16W {
        _RESERVED16W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] The number of BCLK periods between a WCLK edge and MSB of the first word in a phase: 0x00: LJF and DSP format 0x01: I2S and DSP format 0x02: RJF format ... 0xFF: RJF format Note: When 0, MSB of the next word will be output in the idle period between LSB of the previous word and the start of the next word. Otherwise logical 0 will be output until the data delay has expired."]
    #[inline]
    pub fn data_delay(&mut self) -> _DATA_DELAYW {
        _DATA_DELAYW { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] The size of each word stored to or loaded from memory:"]
    #[inline]
    pub fn mem_len_24(&mut self) -> _MEM_LEN_24W {
        _MEM_LEN_24W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] On the serial audio interface, data (and wclk) is sampled and clocked out on opposite edges of BCLK."]
    #[inline]
    pub fn smpl_edge(&mut self) -> _SMPL_EDGEW {
        _SMPL_EDGEW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Selects dual- or single-phase format. 0: Single-phase: DSP format 1: Dual-phase: I2S, LJF and RJF formats"]
    #[inline]
    pub fn dual_phase(&mut self) -> _DUAL_PHASEW {
        _DUAL_PHASEW { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\] Number of bits per word (8-24): In single-phase format, this is the exact number of bits per word. In dual-phase format, this is the maximum number of bits per word. Values below 8 and above 24 give undefined behavior. Data written to memory is always aligned to 16 or 24 bits as defined by MEM_LEN_24. Bit widths that differ from this alignment will either be truncated or zero padded."]
    #[inline]
    pub fn word_len(&mut self) -> _WORD_LENW {
        _WORD_LENW { w: self }
    }
}
