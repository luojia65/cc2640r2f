#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RIS {
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
pub struct RESERVED11R {
    bits: u32,
}
impl RESERVED11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OERISR {
    bits: bool,
}
impl OERISR {
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
pub struct BERISR {
    bits: bool,
}
impl BERISR {
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
pub struct PERISR {
    bits: bool,
}
impl PERISR {
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
pub struct FERISR {
    bits: bool,
}
impl FERISR {
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
pub struct RTRISR {
    bits: bool,
}
impl RTRISR {
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
pub struct TXRISR {
    bits: bool,
}
impl TXRISR {
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
pub struct RXRISR {
    bits: bool,
}
impl RXRISR {
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
    bits: u8,
}
impl RESERVED2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTSRMISR {
    bits: bool,
}
impl CTSRMISR {
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
    bits: bool,
}
impl RESERVED0R {
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
pub struct _RESERVED11W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED11W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 2097151;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OERISW<'a> {
    w: &'a mut W,
}
impl<'a> _OERISW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BERISW<'a> {
    w: &'a mut W,
}
impl<'a> _BERISW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PERISW<'a> {
    w: &'a mut W,
}
impl<'a> _PERISW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FERISW<'a> {
    w: &'a mut W,
}
impl<'a> _FERISW<'a> {
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
pub struct _RTRISW<'a> {
    w: &'a mut W,
}
impl<'a> _RTRISW<'a> {
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
pub struct _TXRISW<'a> {
    w: &'a mut W,
}
impl<'a> _TXRISW<'a> {
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
pub struct _RXRISW<'a> {
    w: &'a mut W,
}
impl<'a> _RXRISW<'a> {
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
pub struct _RESERVED2W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTSRMISW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSRMISW<'a> {
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
pub struct _RESERVED0W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED0W<'a> {
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
    #[doc = "Bits 11:31 - 31:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved11(&self) -> RESERVED11R {
        let bits = {
            const MASK: u32 = 2097151;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED11R { bits }
    }
    #[doc = "Bit 10 - 10:10\\] Overrun error interrupt status: This field returns the raw interrupt state of UART's overrun error interrupt. Overrun error occurs if data is received and the receive FIFO is full."]
    #[inline]
    pub fn oeris(&self) -> OERISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OERISR { bits }
    }
    #[doc = "Bit 9 - 9:9\\] Break error interrupt status: This field returns the raw interrupt state of UART's break error interrupt. Break error is set when a break condition is detected, indicating that the received data input (UARTRXD input pin) was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits)."]
    #[inline]
    pub fn beris(&self) -> BERISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BERISR { bits }
    }
    #[doc = "Bit 8 - 8:8\\] Parity error interrupt status: This field returns the raw interrupt state of UART's parity error interrupt. Parity error is set if the parity of the received data character does not match the parity that the LCRH.EPS and LCRH.SPS select."]
    #[inline]
    pub fn peris(&self) -> PERISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PERISR { bits }
    }
    #[doc = "Bit 7 - 7:7\\] Framing error interrupt status: This field returns the raw interrupt state of UART's framing error interrupt. Framing error is set if the received character does not have a valid stop bit (a valid stop bit is 1)."]
    #[inline]
    pub fn feris(&self) -> FERISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FERISR { bits }
    }
    #[doc = "Bit 6 - 6:6\\] Receive timeout interrupt status: This field returns the raw interrupt state of UART's receive timeout interrupt. The receive timeout interrupt is asserted when the receive FIFO is not empty, and no more data is received during a 32-bit period. The receive timeout interrupt is cleared either when the FIFO becomes empty through reading all the data, or when a 1 is written to ICR.RTIC. The raw interrupt for receive timeout cannot be set unless the mask is set (IMSC.RTIM = 1). This is because the mask acts as an enable for power saving. That is, the same status can be read from MIS.RTMIS and RTRIS."]
    #[inline]
    pub fn rtris(&self) -> RTRISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RTRISR { bits }
    }
    #[doc = "Bit 5 - 5:5\\] Transmit interrupt status: This field returns the raw interrupt state of UART's transmit interrupt. When FIFOs are enabled (LCRH.FEN = 1), the transmit interrupt is asserted if the number of bytes in transmit FIFO is equal to or lower than the programmed trigger level (IFLS.TXSEL). The transmit interrupt is cleared by writing data to the transmit FIFO until it becomes greater than the trigger level, or by clearing the interrupt through ICR.TXIC. When FIFOs are disabled (LCRH.FEN = 0), that is they have a depth of one location, the transmit interrupt is asserted if there is no data present in the transmitters single location. It is cleared by performing a single write to the transmit FIFO, or by clearing the interrupt through ICR.TXIC."]
    #[inline]
    pub fn txris(&self) -> TXRISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXRISR { bits }
    }
    #[doc = "Bit 4 - 4:4\\] Receive interrupt status: This field returns the raw interrupt state of UART's receive interrupt. When FIFOs are enabled (LCRH.FEN = 1), the receive interrupt is asserted if the receive FIFO reaches the programmed trigger level (IFLS.RXSEL). The receive interrupt is cleared by reading data from the receive FIFO until it becomes less than the trigger level, or by clearing the interrupt through ICR.RXIC. When FIFOs are disabled (LCRH.FEN = 0), that is they have a depth of one location, the receive interrupt is asserted if data is received thereby filling the location. The receive interrupt is cleared by performing a single read of the receive FIFO, or by clearing the interrupt through ICR.RXIC."]
    #[inline]
    pub fn rxris(&self) -> RXRISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXRISR { bits }
    }
    #[doc = "Bits 2:3 - 3:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED2R { bits }
    }
    #[doc = "Bit 1 - 1:1\\] Clear to Send (CTS) modem interrupt status: This field returns the raw interrupt state of UART's clear to send interrupt."]
    #[inline]
    pub fn ctsrmis(&self) -> CTSRMISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTSRMISR { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved0(&self) -> RESERVED0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED0R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 13 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 11:31 - 31:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved11(&mut self) -> _RESERVED11W {
        _RESERVED11W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Overrun error interrupt status: This field returns the raw interrupt state of UART's overrun error interrupt. Overrun error occurs if data is received and the receive FIFO is full."]
    #[inline]
    pub fn oeris(&mut self) -> _OERISW {
        _OERISW { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Break error interrupt status: This field returns the raw interrupt state of UART's break error interrupt. Break error is set when a break condition is detected, indicating that the received data input (UARTRXD input pin) was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits)."]
    #[inline]
    pub fn beris(&mut self) -> _BERISW {
        _BERISW { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Parity error interrupt status: This field returns the raw interrupt state of UART's parity error interrupt. Parity error is set if the parity of the received data character does not match the parity that the LCRH.EPS and LCRH.SPS select."]
    #[inline]
    pub fn peris(&mut self) -> _PERISW {
        _PERISW { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Framing error interrupt status: This field returns the raw interrupt state of UART's framing error interrupt. Framing error is set if the received character does not have a valid stop bit (a valid stop bit is 1)."]
    #[inline]
    pub fn feris(&mut self) -> _FERISW {
        _FERISW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Receive timeout interrupt status: This field returns the raw interrupt state of UART's receive timeout interrupt. The receive timeout interrupt is asserted when the receive FIFO is not empty, and no more data is received during a 32-bit period. The receive timeout interrupt is cleared either when the FIFO becomes empty through reading all the data, or when a 1 is written to ICR.RTIC. The raw interrupt for receive timeout cannot be set unless the mask is set (IMSC.RTIM = 1). This is because the mask acts as an enable for power saving. That is, the same status can be read from MIS.RTMIS and RTRIS."]
    #[inline]
    pub fn rtris(&mut self) -> _RTRISW {
        _RTRISW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Transmit interrupt status: This field returns the raw interrupt state of UART's transmit interrupt. When FIFOs are enabled (LCRH.FEN = 1), the transmit interrupt is asserted if the number of bytes in transmit FIFO is equal to or lower than the programmed trigger level (IFLS.TXSEL). The transmit interrupt is cleared by writing data to the transmit FIFO until it becomes greater than the trigger level, or by clearing the interrupt through ICR.TXIC. When FIFOs are disabled (LCRH.FEN = 0), that is they have a depth of one location, the transmit interrupt is asserted if there is no data present in the transmitters single location. It is cleared by performing a single write to the transmit FIFO, or by clearing the interrupt through ICR.TXIC."]
    #[inline]
    pub fn txris(&mut self) -> _TXRISW {
        _TXRISW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Receive interrupt status: This field returns the raw interrupt state of UART's receive interrupt. When FIFOs are enabled (LCRH.FEN = 1), the receive interrupt is asserted if the receive FIFO reaches the programmed trigger level (IFLS.RXSEL). The receive interrupt is cleared by reading data from the receive FIFO until it becomes less than the trigger level, or by clearing the interrupt through ICR.RXIC. When FIFOs are disabled (LCRH.FEN = 0), that is they have a depth of one location, the receive interrupt is asserted if data is received thereby filling the location. The receive interrupt is cleared by performing a single read of the receive FIFO, or by clearing the interrupt through ICR.RXIC."]
    #[inline]
    pub fn rxris(&mut self) -> _RXRISW {
        _RXRISW { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&mut self) -> _RESERVED2W {
        _RESERVED2W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Clear to Send (CTS) modem interrupt status: This field returns the raw interrupt state of UART's clear to send interrupt."]
    #[inline]
    pub fn ctsrmis(&mut self) -> _CTSRMISW {
        _CTSRMISW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved0(&mut self) -> _RESERVED0W {
        _RESERVED0W { w: self }
    }
}
