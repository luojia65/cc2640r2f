#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IRQSTAT {
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
pub struct DMA_BUS_ERRR {
    bits: bool,
}
impl DMA_BUS_ERRR {
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
pub struct KEY_ST_WR_ERRR {
    bits: bool,
}
impl KEY_ST_WR_ERRR {
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
pub struct KEY_ST_RD_ERRR {
    bits: bool,
}
impl KEY_ST_RD_ERRR {
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
pub struct RESERVED2R {
    bits: u32,
}
impl RESERVED2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMA_IN_DONER {
    bits: bool,
}
impl DMA_IN_DONER {
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
pub struct RESULT_AVAILR {
    bits: bool,
}
impl RESULT_AVAILR {
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
pub struct _DMA_BUS_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_BUS_ERRW<'a> {
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
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _KEY_ST_WR_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _KEY_ST_WR_ERRW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _KEY_ST_RD_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _KEY_ST_RD_ERRW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED2W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 134217727;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMA_IN_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_IN_DONEW<'a> {
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
pub struct _RESULT_AVAILW<'a> {
    w: &'a mut W,
}
impl<'a> _RESULT_AVAILW<'a> {
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
    #[doc = "Bit 31 - 31:31\\] This bit is set when a DMA bus error is detected during a DMA operation. The value of this register is held until it is cleared via IRQCLR.DMA_BUS_ERR Note: This error is asserted if an error is detected on the AHB master interface during a DMA operation. Note: This is not an interrupt source."]
    #[inline]
    pub fn dma_bus_err(&self) -> DMA_BUS_ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA_BUS_ERRR { bits }
    }
    #[doc = "Bit 30 - 30:30\\] This bit is set when a write error is detected during the DMA write operation to the key store memory. The value of this register is held until it is cleared via IRQCLR.KEY_ST_WR_ERR Note: This error is asserted if a DMA operation does not cover a full key area or more areas are written than expected. Note: This is not an interrupt source."]
    #[inline]
    pub fn key_st_wr_err(&self) -> KEY_ST_WR_ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        KEY_ST_WR_ERRR { bits }
    }
    #[doc = "Bit 29 - 29:29\\] This bit will be set when a read error is detected during the read of a key from the key store, while copying it to the AES engine. The value of this register is held until it is cleared via IRQCLR.KEY_ST_RD_ERR. Note: This error is asserted if a key location is selected in the key store that is not available. Note: This is not an interrupt source."]
    #[inline]
    pub fn key_st_rd_err(&self) -> KEY_ST_RD_ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        KEY_ST_RD_ERRR { bits }
    }
    #[doc = "Bits 2:28 - 28:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: u32 = 134217727;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED2R { bits }
    }
    #[doc = "Bit 1 - 1:1\\] This bit returns the status of DMA data in done interrupt."]
    #[inline]
    pub fn dma_in_done(&self) -> DMA_IN_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA_IN_DONER { bits }
    }
    #[doc = "Bit 0 - 0:0\\] This bit is set high when the Crypto peripheral has a result available."]
    #[inline]
    pub fn result_avail(&self) -> RESULT_AVAILR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESULT_AVAILR { bits }
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
    #[doc = "Bit 31 - 31:31\\] This bit is set when a DMA bus error is detected during a DMA operation. The value of this register is held until it is cleared via IRQCLR.DMA_BUS_ERR Note: This error is asserted if an error is detected on the AHB master interface during a DMA operation. Note: This is not an interrupt source."]
    #[inline]
    pub fn dma_bus_err(&mut self) -> _DMA_BUS_ERRW {
        _DMA_BUS_ERRW { w: self }
    }
    #[doc = "Bit 30 - 30:30\\] This bit is set when a write error is detected during the DMA write operation to the key store memory. The value of this register is held until it is cleared via IRQCLR.KEY_ST_WR_ERR Note: This error is asserted if a DMA operation does not cover a full key area or more areas are written than expected. Note: This is not an interrupt source."]
    #[inline]
    pub fn key_st_wr_err(&mut self) -> _KEY_ST_WR_ERRW {
        _KEY_ST_WR_ERRW { w: self }
    }
    #[doc = "Bit 29 - 29:29\\] This bit will be set when a read error is detected during the read of a key from the key store, while copying it to the AES engine. The value of this register is held until it is cleared via IRQCLR.KEY_ST_RD_ERR. Note: This error is asserted if a key location is selected in the key store that is not available. Note: This is not an interrupt source."]
    #[inline]
    pub fn key_st_rd_err(&mut self) -> _KEY_ST_RD_ERRW {
        _KEY_ST_RD_ERRW { w: self }
    }
    #[doc = "Bits 2:28 - 28:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&mut self) -> _RESERVED2W {
        _RESERVED2W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] This bit returns the status of DMA data in done interrupt."]
    #[inline]
    pub fn dma_in_done(&mut self) -> _DMA_IN_DONEW {
        _DMA_IN_DONEW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] This bit is set high when the Crypto peripheral has a result available."]
    #[inline]
    pub fn result_avail(&mut self) -> _RESULT_AVAILW {
        _RESULT_AVAILW { w: self }
    }
}
