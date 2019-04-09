#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FR {
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
pub struct RESERVED1R {
    bits: u32,
}
impl RESERVED1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXFER {
    bits: bool,
}
impl TXFER {
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
pub struct RXFFR {
    bits: bool,
}
impl RXFFR {
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
pub struct TXFFR {
    bits: bool,
}
impl TXFFR {
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
pub struct RXFER {
    bits: bool,
}
impl RXFER {
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
pub struct BUSYR {
    bits: bool,
}
impl BUSYR {
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
pub struct RESERVED0R {
    bits: u8,
}
impl RESERVED0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTSR {
    bits: bool,
}
impl CTSR {
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
pub struct _RESERVED1W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED1W<'a> {
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
#[doc = r" Proxy"]
pub struct _TXFEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFEW<'a> {
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
pub struct _RXFFW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFFW<'a> {
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
pub struct _TXFFW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFFW<'a> {
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
pub struct _RXFEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFEW<'a> {
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
pub struct _BUSYW<'a> {
    w: &'a mut W,
}
impl<'a> _BUSYW<'a> {
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
pub struct _RESERVED0W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTSW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSW<'a> {
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
    #[doc = "Bits 8:31 - 31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved1(&self) -> RESERVED1R {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED1R { bits }
    }
    #[doc = "Bit 7 - 7:7\\] UART Transmit FIFO Empty: The meaning of this bit depends on the state of LCRH.FEN . - If the FIFO is disabled, this bit is set when the transmit holding register is empty. - If the FIFO is enabled, this bit is set when the transmit FIFO is empty. This bit does not indicate if there is data in the transmit shift register."]
    #[inline]
    pub fn txfe(&self) -> TXFER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXFER { bits }
    }
    #[doc = "Bit 6 - 6:6\\] UART Receive FIFO Full: The meaning of this bit depends on the state of LCRH.FEN. - If the FIFO is disabled, this bit is set when the receive holding register is full. - If the FIFO is enabled, this bit is set when the receive FIFO is full."]
    #[inline]
    pub fn rxff(&self) -> RXFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXFFR { bits }
    }
    #[doc = "Bit 5 - 5:5\\] UART Transmit FIFO Full: Transmit FIFO full. The meaning of this bit depends on the state of LCRH.FEN. - If the FIFO is disabled, this bit is set when the transmit holding register is full. - If the FIFO is enabled, this bit is set when the transmit FIFO is full."]
    #[inline]
    pub fn txff(&self) -> TXFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXFFR { bits }
    }
    #[doc = "Bit 4 - 4:4\\] UART Receive FIFO Empty: Receive FIFO empty. The meaning of this bit depends on the state of LCRH.FEN. - If the FIFO is disabled, this bit is set when the receive holding register is empty. - If the FIFO is enabled, this bit is set when the receive FIFO is empty."]
    #[inline]
    pub fn rxfe(&self) -> RXFER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXFER { bits }
    }
    #[doc = "Bit 3 - 3:3\\] UART Busy: If this bit is set to 1, the UART is busy transmitting data. This bit remains set until the complete byte, including all the stop bits, has been sent from the shift register. This bit is set as soon as the transmit FIFO becomes non-empty, regardless of whether the UART is enabled or not."]
    #[inline]
    pub fn busy(&self) -> BUSYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BUSYR { bits }
    }
    #[doc = "Bits 1:2 - 2:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved0(&self) -> RESERVED0R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED0R { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Clear To Send: This bit is the complement of the active-low UART CTS input pin. That is, the bit is 1 when CTS input pin is LOW."]
    #[inline]
    pub fn cts(&self) -> CTSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTSR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 144 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:31 - 31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved1(&mut self) -> _RESERVED1W {
        _RESERVED1W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] UART Transmit FIFO Empty: The meaning of this bit depends on the state of LCRH.FEN . - If the FIFO is disabled, this bit is set when the transmit holding register is empty. - If the FIFO is enabled, this bit is set when the transmit FIFO is empty. This bit does not indicate if there is data in the transmit shift register."]
    #[inline]
    pub fn txfe(&mut self) -> _TXFEW {
        _TXFEW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] UART Receive FIFO Full: The meaning of this bit depends on the state of LCRH.FEN. - If the FIFO is disabled, this bit is set when the receive holding register is full. - If the FIFO is enabled, this bit is set when the receive FIFO is full."]
    #[inline]
    pub fn rxff(&mut self) -> _RXFFW {
        _RXFFW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] UART Transmit FIFO Full: Transmit FIFO full. The meaning of this bit depends on the state of LCRH.FEN. - If the FIFO is disabled, this bit is set when the transmit holding register is full. - If the FIFO is enabled, this bit is set when the transmit FIFO is full."]
    #[inline]
    pub fn txff(&mut self) -> _TXFFW {
        _TXFFW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] UART Receive FIFO Empty: Receive FIFO empty. The meaning of this bit depends on the state of LCRH.FEN. - If the FIFO is disabled, this bit is set when the receive holding register is empty. - If the FIFO is enabled, this bit is set when the receive FIFO is empty."]
    #[inline]
    pub fn rxfe(&mut self) -> _RXFEW {
        _RXFEW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] UART Busy: If this bit is set to 1, the UART is busy transmitting data. This bit remains set until the complete byte, including all the stop bits, has been sent from the shift register. This bit is set as soon as the transmit FIFO becomes non-empty, regardless of whether the UART is enabled or not."]
    #[inline]
    pub fn busy(&mut self) -> _BUSYW {
        _BUSYW { w: self }
    }
    #[doc = "Bits 1:2 - 2:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved0(&mut self) -> _RESERVED0W {
        _RESERVED0W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Clear To Send: This bit is the complement of the active-low UART CTS input pin. That is, the bit is 1 when CTS input pin is LOW."]
    #[inline]
    pub fn cts(&mut self) -> _CTSW {
        _CTSW { w: self }
    }
}
