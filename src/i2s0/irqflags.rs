#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IRQFLAGS {
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
pub struct RESERVED6R {
    bits: u32,
}
impl RESERVED6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AIF_DMA_INR {
    bits: bool,
}
impl AIF_DMA_INR {
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
pub struct AIF_DMA_OUTR {
    bits: bool,
}
impl AIF_DMA_OUTR {
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
pub struct WCLK_TIMEOUTR {
    bits: bool,
}
impl WCLK_TIMEOUTR {
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
pub struct BUS_ERRR {
    bits: bool,
}
impl BUS_ERRR {
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
pub struct WCLK_ERRR {
    bits: bool,
}
impl WCLK_ERRR {
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
pub struct PTR_ERRR {
    bits: bool,
}
impl PTR_ERRR {
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
pub struct _RESERVED6W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED6W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 67108863;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AIF_DMA_INW<'a> {
    w: &'a mut W,
}
impl<'a> _AIF_DMA_INW<'a> {
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
pub struct _AIF_DMA_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _AIF_DMA_OUTW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WCLK_TIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _WCLK_TIMEOUTW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BUS_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _BUS_ERRW<'a> {
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
pub struct _WCLK_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _WCLK_ERRW<'a> {
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
pub struct _PTR_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _PTR_ERRW<'a> {
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
    #[doc = "Bits 6:31 - 31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&self) -> RESERVED6R {
        let bits = {
            const MASK: u32 = 67108863;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED6R { bits }
    }
    #[doc = "Bit 5 - 5:5\\] Set when condition for this bit field event occurs (auto cleared when input pointer is updated - AIFINPTRNEXT), see description of AIFINPTRNEXT register for details."]
    #[inline]
    pub fn aif_dma_in(&self) -> AIF_DMA_INR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AIF_DMA_INR { bits }
    }
    #[doc = "Bit 4 - 4:4\\] Set when condition for this bit field event occurs (auto cleared when output pointer is updated - AIFOUTPTRNEXT), see description of AIFOUTPTRNEXT register for details"]
    #[inline]
    pub fn aif_dma_out(&self) -> AIF_DMA_OUTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AIF_DMA_OUTR { bits }
    }
    #[doc = "Bit 3 - 3:3\\] Set when the sample stamp generator does not detect a positive WCLK edge for 65535 clk periods. This signalizes that the internal or external BCLK and WCLK generator source has been disabled. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.WCLK_TIMEOUT)."]
    #[inline]
    pub fn wclk_timeout(&self) -> WCLK_TIMEOUTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WCLK_TIMEOUTR { bits }
    }
    #[doc = "Bit 2 - 2:2\\] Set when a DMA operation is not completed in time (that is audio output buffer underflow, or audio input buffer overflow). This error requires a complete restart since word synchronization has been lost. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.BUS_ERR). Note that DMA initiated transactions to illegal addresses will not trigger an interrupt. The response to such transactions is undefined."]
    #[inline]
    pub fn bus_err(&self) -> BUS_ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BUS_ERRR { bits }
    }
    #[doc = "Bit 1 - 1:1\\] Set when: - An unexpected WCLK edge occurs during the data delay period of a phase. Note unexpected WCLK edges during the word and idle periods of the phase are not detected. - In dual-phase mode, when two WCLK edges are less than 4 BCLK cycles apart. - In single-phase mode, when a WCLK pulse occurs before the last channel. This error requires a complete restart since word synchronization has been lost. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.WCLK_ERR)."]
    #[inline]
    pub fn wclk_err(&self) -> WCLK_ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WCLK_ERRR { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Set when AIFINPTRNEXT or AIFOUTPTRNEXT has not been loaded with the next block address in time. This error requires a complete restart since word synchronization has been lost. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.PTR_ERR)."]
    #[inline]
    pub fn ptr_err(&self) -> PTR_ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PTR_ERRR { bits }
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
    #[doc = "Bits 6:31 - 31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&mut self) -> _RESERVED6W {
        _RESERVED6W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Set when condition for this bit field event occurs (auto cleared when input pointer is updated - AIFINPTRNEXT), see description of AIFINPTRNEXT register for details."]
    #[inline]
    pub fn aif_dma_in(&mut self) -> _AIF_DMA_INW {
        _AIF_DMA_INW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Set when condition for this bit field event occurs (auto cleared when output pointer is updated - AIFOUTPTRNEXT), see description of AIFOUTPTRNEXT register for details"]
    #[inline]
    pub fn aif_dma_out(&mut self) -> _AIF_DMA_OUTW {
        _AIF_DMA_OUTW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Set when the sample stamp generator does not detect a positive WCLK edge for 65535 clk periods. This signalizes that the internal or external BCLK and WCLK generator source has been disabled. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.WCLK_TIMEOUT)."]
    #[inline]
    pub fn wclk_timeout(&mut self) -> _WCLK_TIMEOUTW {
        _WCLK_TIMEOUTW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Set when a DMA operation is not completed in time (that is audio output buffer underflow, or audio input buffer overflow). This error requires a complete restart since word synchronization has been lost. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.BUS_ERR). Note that DMA initiated transactions to illegal addresses will not trigger an interrupt. The response to such transactions is undefined."]
    #[inline]
    pub fn bus_err(&mut self) -> _BUS_ERRW {
        _BUS_ERRW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Set when: - An unexpected WCLK edge occurs during the data delay period of a phase. Note unexpected WCLK edges during the word and idle periods of the phase are not detected. - In dual-phase mode, when two WCLK edges are less than 4 BCLK cycles apart. - In single-phase mode, when a WCLK pulse occurs before the last channel. This error requires a complete restart since word synchronization has been lost. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.WCLK_ERR)."]
    #[inline]
    pub fn wclk_err(&mut self) -> _WCLK_ERRW {
        _WCLK_ERRW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Set when AIFINPTRNEXT or AIFOUTPTRNEXT has not been loaded with the next block address in time. This error requires a complete restart since word synchronization has been lost. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.PTR_ERR)."]
    #[inline]
    pub fn ptr_err(&mut self) -> _PTR_ERRW {
        _PTR_ERRW { w: self }
    }
}
