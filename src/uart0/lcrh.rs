#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LCRH {
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
pub struct SPSR {
    bits: bool,
}
impl SPSR {
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
#[doc = "Possible values of the field `WLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WLENR {
    #[doc = "Word Length 8 bits"]
    _8,
    #[doc = "Word Length 7 bits"]
    _7,
    #[doc = "Word Length 6 bits"]
    _6,
    #[doc = "Word Length 5 bits"]
    _5,
}
impl WLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WLENR::_8 => 3,
            WLENR::_7 => 2,
            WLENR::_6 => 1,
            WLENR::_5 => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WLENR {
        match value {
            3 => WLENR::_8,
            2 => WLENR::_7,
            1 => WLENR::_6,
            0 => WLENR::_5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == WLENR::_8
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == WLENR::_7
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == WLENR::_6
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == WLENR::_5
    }
}
#[doc = "Possible values of the field `FEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FENR {
    #[doc = "Transmit and receive FIFO buffers are enabled (FIFO mode)"]
    EN,
    #[doc = "FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers."]
    DIS,
}
impl FENR {
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
            FENR::EN => true,
            FENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FENR {
        match value {
            true => FENR::EN,
            false => FENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == FENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == FENR::DIS
    }
}
#[doc = r" Value of the field"]
pub struct STP2R {
    bits: bool,
}
impl STP2R {
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
#[doc = "Possible values of the field `EPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPSR {
    #[doc = "Even parity: The UART generates or checks for an even number of 1s in the data and parity bits."]
    EVEN,
    #[doc = "Odd parity: The UART generates or checks for an odd number of 1s in the data and parity bits."]
    ODD,
}
impl EPSR {
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
            EPSR::EVEN => true,
            EPSR::ODD => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPSR {
        match value {
            true => EPSR::EVEN,
            false => EPSR::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline]
    pub fn is_even(&self) -> bool {
        *self == EPSR::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline]
    pub fn is_odd(&self) -> bool {
        *self == EPSR::ODD
    }
}
#[doc = "Possible values of the field `PEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENR {
    #[doc = "Parity checking and generation is enabled."]
    EN,
    #[doc = "Parity is disabled and no parity bit is added to the data frame"]
    DIS,
}
impl PENR {
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
            PENR::EN => true,
            PENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PENR {
        match value {
            true => PENR::EN,
            false => PENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == PENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PENR::DIS
    }
}
#[doc = r" Value of the field"]
pub struct BRKR {
    bits: bool,
}
impl BRKR {
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
pub struct _SPSW<'a> {
    w: &'a mut W,
}
impl<'a> _SPSW<'a> {
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
#[doc = "Values that can be written to the field `WLEN`"]
pub enum WLENW {
    #[doc = "Word Length 8 bits"]
    _8,
    #[doc = "Word Length 7 bits"]
    _7,
    #[doc = "Word Length 6 bits"]
    _6,
    #[doc = "Word Length 5 bits"]
    _5,
}
impl WLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WLENW::_8 => 3,
            WLENW::_7 => 2,
            WLENW::_6 => 1,
            WLENW::_5 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WLENW<'a> {
    w: &'a mut W,
}
impl<'a> _WLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WLENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Word Length 8 bits"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(WLENW::_8)
    }
    #[doc = "Word Length 7 bits"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(WLENW::_7)
    }
    #[doc = "Word Length 6 bits"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(WLENW::_6)
    }
    #[doc = "Word Length 5 bits"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(WLENW::_5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FEN`"]
pub enum FENW {
    #[doc = "Transmit and receive FIFO buffers are enabled (FIFO mode)"]
    EN,
    #[doc = "FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers."]
    DIS,
}
impl FENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FENW::EN => true,
            FENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FENW<'a> {
    w: &'a mut W,
}
impl<'a> _FENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit and receive FIFO buffers are enabled (FIFO mode)"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(FENW::EN)
    }
    #[doc = "FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(FENW::DIS)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STP2W<'a> {
    w: &'a mut W,
}
impl<'a> _STP2W<'a> {
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
#[doc = "Values that can be written to the field `EPS`"]
pub enum EPSW {
    #[doc = "Even parity: The UART generates or checks for an even number of 1s in the data and parity bits."]
    EVEN,
    #[doc = "Odd parity: The UART generates or checks for an odd number of 1s in the data and parity bits."]
    ODD,
}
impl EPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPSW::EVEN => true,
            EPSW::ODD => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPSW<'a> {
    w: &'a mut W,
}
impl<'a> _EPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Even parity: The UART generates or checks for an even number of 1s in the data and parity bits."]
    #[inline]
    pub fn even(self) -> &'a mut W {
        self.variant(EPSW::EVEN)
    }
    #[doc = "Odd parity: The UART generates or checks for an odd number of 1s in the data and parity bits."]
    #[inline]
    pub fn odd(self) -> &'a mut W {
        self.variant(EPSW::ODD)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PEN`"]
pub enum PENW {
    #[doc = "Parity checking and generation is enabled."]
    EN,
    #[doc = "Parity is disabled and no parity bit is added to the data frame"]
    DIS,
}
impl PENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PENW::EN => true,
            PENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PENW<'a> {
    w: &'a mut W,
}
impl<'a> _PENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Parity checking and generation is enabled."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(PENW::EN)
    }
    #[doc = "Parity is disabled and no parity bit is added to the data frame"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PENW::DIS)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BRKW<'a> {
    w: &'a mut W,
}
impl<'a> _BRKW<'a> {
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
    #[doc = "Bit 7 - 7:7\\] UART Stick Parity Select: 0: Stick parity is disabled 1: The parity bit is transmitted and checked as invert of EPS field (i.e. the parity bit is transmitted and checked as 1 when EPS = 0). This bit has no effect when PEN disables parity checking and generation."]
    #[inline]
    pub fn sps(&self) -> SPSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SPSR { bits }
    }
    #[doc = "Bits 5:6 - 6:5\\] UART Word Length: These bits indicate the number of data bits transmitted or received in a frame."]
    #[inline]
    pub fn wlen(&self) -> WLENR {
        WLENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - 4:4\\] UART Enable FIFOs"]
    #[inline]
    pub fn fen(&self) -> FENR {
        FENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - 3:3\\] UART Two Stop Bits Select: If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
    #[inline]
    pub fn stp2(&self) -> STP2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STP2R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] UART Even Parity Select"]
    #[inline]
    pub fn eps(&self) -> EPSR {
        EPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - 1:1\\] UART Parity Enable This bit controls generation and checking of parity bit."]
    #[inline]
    pub fn pen(&self) -> PENR {
        PENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - 0:0\\] UART Send Break If this bit is set to 1, a low-level is continually output on the UARTTXD output pin, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
    #[inline]
    pub fn brk(&self) -> BRKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BRKR { bits }
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
    #[doc = "Bit 7 - 7:7\\] UART Stick Parity Select: 0: Stick parity is disabled 1: The parity bit is transmitted and checked as invert of EPS field (i.e. the parity bit is transmitted and checked as 1 when EPS = 0). This bit has no effect when PEN disables parity checking and generation."]
    #[inline]
    pub fn sps(&mut self) -> _SPSW {
        _SPSW { w: self }
    }
    #[doc = "Bits 5:6 - 6:5\\] UART Word Length: These bits indicate the number of data bits transmitted or received in a frame."]
    #[inline]
    pub fn wlen(&mut self) -> _WLENW {
        _WLENW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] UART Enable FIFOs"]
    #[inline]
    pub fn fen(&mut self) -> _FENW {
        _FENW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] UART Two Stop Bits Select: If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
    #[inline]
    pub fn stp2(&mut self) -> _STP2W {
        _STP2W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] UART Even Parity Select"]
    #[inline]
    pub fn eps(&mut self) -> _EPSW {
        _EPSW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] UART Parity Enable This bit controls generation and checking of parity bit."]
    #[inline]
    pub fn pen(&mut self) -> _PENW {
        _PENW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] UART Send Break If this bit is set to 1, a low-level is continually output on the UARTTXD output pin, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
    #[inline]
    pub fn brk(&mut self) -> _BRKW {
        _BRKW { w: self }
    }
}
