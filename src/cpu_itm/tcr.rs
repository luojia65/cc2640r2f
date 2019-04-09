#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TCR {
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
pub struct RESERVED24R {
    bits: u8,
}
impl RESERVED24R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
pub struct ATBIDR {
    bits: u8,
}
impl ATBIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED10R {
    bits: u8,
}
impl RESERVED10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TSPRESCALE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSPRESCALER {
    #[doc = "Divide by 64"]
    DIV64,
    #[doc = "Divide by 16"]
    DIV16,
    #[doc = "Divide by 4"]
    DIV4,
    #[doc = "No prescaling"]
    NOPRESCALING,
}
impl TSPRESCALER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSPRESCALER::DIV64 => 3,
            TSPRESCALER::DIV16 => 2,
            TSPRESCALER::DIV4 => 1,
            TSPRESCALER::NOPRESCALING => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSPRESCALER {
        match value {
            3 => TSPRESCALER::DIV64,
            2 => TSPRESCALER::DIV16,
            1 => TSPRESCALER::DIV4,
            0 => TSPRESCALER::NOPRESCALING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == TSPRESCALER::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == TSPRESCALER::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == TSPRESCALER::DIV4
    }
    #[doc = "Checks if the value of the field is `NOPRESCALING`"]
    #[inline]
    pub fn is_noprescaling(&self) -> bool {
        *self == TSPRESCALER::NOPRESCALING
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED5R {
    bits: u8,
}
impl RESERVED5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SWOENAR {
    bits: bool,
}
impl SWOENAR {
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
pub struct DWTENAR {
    bits: bool,
}
impl DWTENAR {
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
pub struct SYNCENAR {
    bits: bool,
}
impl SYNCENAR {
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
pub struct TSENAR {
    bits: bool,
}
impl TSENAR {
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
pub struct ITMENAR {
    bits: bool,
}
impl ITMENAR {
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
pub struct _RESERVED24W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED24W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ATBIDW<'a> {
    w: &'a mut W,
}
impl<'a> _ATBIDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED10W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED10W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TSPRESCALE`"]
pub enum TSPRESCALEW {
    #[doc = "Divide by 64"]
    DIV64,
    #[doc = "Divide by 16"]
    DIV16,
    #[doc = "Divide by 4"]
    DIV4,
    #[doc = "No prescaling"]
    NOPRESCALING,
}
impl TSPRESCALEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSPRESCALEW::DIV64 => 3,
            TSPRESCALEW::DIV16 => 2,
            TSPRESCALEW::DIV4 => 1,
            TSPRESCALEW::NOPRESCALING => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSPRESCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _TSPRESCALEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSPRESCALEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide by 64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(TSPRESCALEW::DIV64)
    }
    #[doc = "Divide by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(TSPRESCALEW::DIV16)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(TSPRESCALEW::DIV4)
    }
    #[doc = "No prescaling"]
    #[inline]
    pub fn noprescaling(self) -> &'a mut W {
        self.variant(TSPRESCALEW::NOPRESCALING)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED5W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED5W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SWOENAW<'a> {
    w: &'a mut W,
}
impl<'a> _SWOENAW<'a> {
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
pub struct _DWTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _DWTENAW<'a> {
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
pub struct _SYNCENAW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCENAW<'a> {
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
pub struct _TSENAW<'a> {
    w: &'a mut W,
}
impl<'a> _TSENAW<'a> {
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
pub struct _ITMENAW<'a> {
    w: &'a mut W,
}
impl<'a> _ITMENAW<'a> {
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
    #[doc = "Bits 24:31 - 31:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved24(&self) -> RESERVED24R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED24R { bits }
    }
    #[doc = "Bit 23 - 23:23\\] Set when ITM events present and being drained."]
    #[inline]
    pub fn busy(&self) -> BUSYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BUSYR { bits }
    }
    #[doc = "Bits 16:22 - 22:16\\] Trace Bus ID for CoreSight system. Optional identifier for multi-source trace stream formatting. If multi-source trace is in use, this field must be written with a non-zero value."]
    #[inline]
    pub fn atbid(&self) -> ATBIDR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ATBIDR { bits }
    }
    #[doc = "Bits 10:15 - 15:10\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved10(&self) -> RESERVED10R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED10R { bits }
    }
    #[doc = "Bits 8:9 - 9:8\\] Timestamp prescaler"]
    #[inline]
    pub fn tsprescale(&self) -> TSPRESCALER {
        TSPRESCALER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:7 - 7:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved5(&self) -> RESERVED5R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED5R { bits }
    }
    #[doc = "Bit 4 - 4:4\\] Enables asynchronous clocking of the timestamp counter (when TSENA = 1). If TSENA = 0, writing this bit to 1 does not enable asynchronous clocking of the timestamp counter. 0x0: Mode disabled. Timestamp counter uses system clock from the core and counts continuously. 0x1: Timestamp counter uses lineout (data related) clock from TPIU interface. The timestamp counter is held in reset while the output line is idle."]
    #[inline]
    pub fn swoena(&self) -> SWOENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWOENAR { bits }
    }
    #[doc = "Bit 3 - 3:3\\] Enables the DWT stimulus (hardware event packet emission to the TPIU from the DWT)"]
    #[inline]
    pub fn dwtena(&self) -> DWTENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DWTENAR { bits }
    }
    #[doc = "Bit 2 - 2:2\\] Enables synchronization packet transmission for a synchronous TPIU. CPU_DWT:CTRL.SYNCTAP must be configured for the correct synchronization speed."]
    #[inline]
    pub fn syncena(&self) -> SYNCENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYNCENAR { bits }
    }
    #[doc = "Bit 1 - 1:1\\] Enables differential timestamps. Differential timestamps are emitted when a packet is written to the FIFO with a non-zero timestamp counter, and when the timestamp counter overflows. Timestamps are emitted during idle times after a fixed number of two million cycles. This provides a time reference for packets and inter-packet gaps. If SWOENA (bit \\[4\\]) is set, timestamps are triggered by activity on the internal trace bus only. In this case there is no regular timestamp output when the ITM is idle."]
    #[inline]
    pub fn tsena(&self) -> TSENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSENAR { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Enables ITM. This is the master enable, and must be set before ITM Stimulus and Trace Enable registers can be written."]
    #[inline]
    pub fn itmena(&self) -> ITMENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ITMENAR { bits }
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
    #[doc = "Bits 24:31 - 31:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved24(&mut self) -> _RESERVED24W {
        _RESERVED24W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\] Set when ITM events present and being drained."]
    #[inline]
    pub fn busy(&mut self) -> _BUSYW {
        _BUSYW { w: self }
    }
    #[doc = "Bits 16:22 - 22:16\\] Trace Bus ID for CoreSight system. Optional identifier for multi-source trace stream formatting. If multi-source trace is in use, this field must be written with a non-zero value."]
    #[inline]
    pub fn atbid(&mut self) -> _ATBIDW {
        _ATBIDW { w: self }
    }
    #[doc = "Bits 10:15 - 15:10\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved10(&mut self) -> _RESERVED10W {
        _RESERVED10W { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\] Timestamp prescaler"]
    #[inline]
    pub fn tsprescale(&mut self) -> _TSPRESCALEW {
        _TSPRESCALEW { w: self }
    }
    #[doc = "Bits 5:7 - 7:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved5(&mut self) -> _RESERVED5W {
        _RESERVED5W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Enables asynchronous clocking of the timestamp counter (when TSENA = 1). If TSENA = 0, writing this bit to 1 does not enable asynchronous clocking of the timestamp counter. 0x0: Mode disabled. Timestamp counter uses system clock from the core and counts continuously. 0x1: Timestamp counter uses lineout (data related) clock from TPIU interface. The timestamp counter is held in reset while the output line is idle."]
    #[inline]
    pub fn swoena(&mut self) -> _SWOENAW {
        _SWOENAW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Enables the DWT stimulus (hardware event packet emission to the TPIU from the DWT)"]
    #[inline]
    pub fn dwtena(&mut self) -> _DWTENAW {
        _DWTENAW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Enables synchronization packet transmission for a synchronous TPIU. CPU_DWT:CTRL.SYNCTAP must be configured for the correct synchronization speed."]
    #[inline]
    pub fn syncena(&mut self) -> _SYNCENAW {
        _SYNCENAW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Enables differential timestamps. Differential timestamps are emitted when a packet is written to the FIFO with a non-zero timestamp counter, and when the timestamp counter overflows. Timestamps are emitted during idle times after a fixed number of two million cycles. This provides a time reference for packets and inter-packet gaps. If SWOENA (bit \\[4\\]) is set, timestamps are triggered by activity on the internal trace bus only. In this case there is no regular timestamp output when the ITM is idle."]
    #[inline]
    pub fn tsena(&mut self) -> _TSENAW {
        _TSENAW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Enables ITM. This is the master enable, and must be set before ITM Stimulus and Trace Enable registers can be written."]
    #[inline]
    pub fn itmena(&mut self) -> _ITMENAW {
        _ITMENAW { w: self }
    }
}
