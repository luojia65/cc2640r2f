#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IFLS {
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
#[doc = "Possible values of the field `RXSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSELR {
    #[doc = "Receive FIFO becomes >= 7/8 full"]
    _7_8,
    #[doc = "Receive FIFO becomes >= 3/4 full"]
    _6_8,
    #[doc = "Receive FIFO becomes >= 1/2 full"]
    _4_8,
    #[doc = "Receive FIFO becomes >= 1/4 full"]
    _2_8,
    #[doc = "Receive FIFO becomes >= 1/8 full"]
    _1_8,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RXSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXSELR::_7_8 => 4,
            RXSELR::_6_8 => 3,
            RXSELR::_4_8 => 2,
            RXSELR::_2_8 => 1,
            RXSELR::_1_8 => 0,
            RXSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXSELR {
        match value {
            4 => RXSELR::_7_8,
            3 => RXSELR::_6_8,
            2 => RXSELR::_4_8,
            1 => RXSELR::_2_8,
            0 => RXSELR::_1_8,
            i => RXSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_7_8`"]
    #[inline]
    pub fn is_7_8(&self) -> bool {
        *self == RXSELR::_7_8
    }
    #[doc = "Checks if the value of the field is `_6_8`"]
    #[inline]
    pub fn is_6_8(&self) -> bool {
        *self == RXSELR::_6_8
    }
    #[doc = "Checks if the value of the field is `_4_8`"]
    #[inline]
    pub fn is_4_8(&self) -> bool {
        *self == RXSELR::_4_8
    }
    #[doc = "Checks if the value of the field is `_2_8`"]
    #[inline]
    pub fn is_2_8(&self) -> bool {
        *self == RXSELR::_2_8
    }
    #[doc = "Checks if the value of the field is `_1_8`"]
    #[inline]
    pub fn is_1_8(&self) -> bool {
        *self == RXSELR::_1_8
    }
}
#[doc = "Possible values of the field `TXSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSELR {
    #[doc = "Transmit FIFO becomes <= 7/8 full"]
    _7_8,
    #[doc = "Transmit FIFO becomes <= 3/4 full"]
    _6_8,
    #[doc = "Transmit FIFO becomes <= 1/2 full"]
    _4_8,
    #[doc = "Transmit FIFO becomes <= 1/4 full"]
    _2_8,
    #[doc = "Transmit FIFO becomes <= 1/8 full"]
    _1_8,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXSELR::_7_8 => 4,
            TXSELR::_6_8 => 3,
            TXSELR::_4_8 => 2,
            TXSELR::_2_8 => 1,
            TXSELR::_1_8 => 0,
            TXSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXSELR {
        match value {
            4 => TXSELR::_7_8,
            3 => TXSELR::_6_8,
            2 => TXSELR::_4_8,
            1 => TXSELR::_2_8,
            0 => TXSELR::_1_8,
            i => TXSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_7_8`"]
    #[inline]
    pub fn is_7_8(&self) -> bool {
        *self == TXSELR::_7_8
    }
    #[doc = "Checks if the value of the field is `_6_8`"]
    #[inline]
    pub fn is_6_8(&self) -> bool {
        *self == TXSELR::_6_8
    }
    #[doc = "Checks if the value of the field is `_4_8`"]
    #[inline]
    pub fn is_4_8(&self) -> bool {
        *self == TXSELR::_4_8
    }
    #[doc = "Checks if the value of the field is `_2_8`"]
    #[inline]
    pub fn is_2_8(&self) -> bool {
        *self == TXSELR::_2_8
    }
    #[doc = "Checks if the value of the field is `_1_8`"]
    #[inline]
    pub fn is_1_8(&self) -> bool {
        *self == TXSELR::_1_8
    }
}
#[doc = "Values that can be written to the field `RXSEL`"]
pub enum RXSELW {
    #[doc = "Receive FIFO becomes >= 7/8 full"]
    _7_8,
    #[doc = "Receive FIFO becomes >= 3/4 full"]
    _6_8,
    #[doc = "Receive FIFO becomes >= 1/2 full"]
    _4_8,
    #[doc = "Receive FIFO becomes >= 1/4 full"]
    _2_8,
    #[doc = "Receive FIFO becomes >= 1/8 full"]
    _1_8,
}
impl RXSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXSELW::_7_8 => 4,
            RXSELW::_6_8 => 3,
            RXSELW::_4_8 => 2,
            RXSELW::_2_8 => 1,
            RXSELW::_1_8 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RXSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Receive FIFO becomes >= 7/8 full"]
    #[inline]
    pub fn _7_8(self) -> &'a mut W {
        self.variant(RXSELW::_7_8)
    }
    #[doc = "Receive FIFO becomes >= 3/4 full"]
    #[inline]
    pub fn _6_8(self) -> &'a mut W {
        self.variant(RXSELW::_6_8)
    }
    #[doc = "Receive FIFO becomes >= 1/2 full"]
    #[inline]
    pub fn _4_8(self) -> &'a mut W {
        self.variant(RXSELW::_4_8)
    }
    #[doc = "Receive FIFO becomes >= 1/4 full"]
    #[inline]
    pub fn _2_8(self) -> &'a mut W {
        self.variant(RXSELW::_2_8)
    }
    #[doc = "Receive FIFO becomes >= 1/8 full"]
    #[inline]
    pub fn _1_8(self) -> &'a mut W {
        self.variant(RXSELW::_1_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXSEL`"]
pub enum TXSELW {
    #[doc = "Transmit FIFO becomes <= 7/8 full"]
    _7_8,
    #[doc = "Transmit FIFO becomes <= 3/4 full"]
    _6_8,
    #[doc = "Transmit FIFO becomes <= 1/2 full"]
    _4_8,
    #[doc = "Transmit FIFO becomes <= 1/4 full"]
    _2_8,
    #[doc = "Transmit FIFO becomes <= 1/8 full"]
    _1_8,
}
impl TXSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXSELW::_7_8 => 4,
            TXSELW::_6_8 => 3,
            TXSELW::_4_8 => 2,
            TXSELW::_2_8 => 1,
            TXSELW::_1_8 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Transmit FIFO becomes <= 7/8 full"]
    #[inline]
    pub fn _7_8(self) -> &'a mut W {
        self.variant(TXSELW::_7_8)
    }
    #[doc = "Transmit FIFO becomes <= 3/4 full"]
    #[inline]
    pub fn _6_8(self) -> &'a mut W {
        self.variant(TXSELW::_6_8)
    }
    #[doc = "Transmit FIFO becomes <= 1/2 full"]
    #[inline]
    pub fn _4_8(self) -> &'a mut W {
        self.variant(TXSELW::_4_8)
    }
    #[doc = "Transmit FIFO becomes <= 1/4 full"]
    #[inline]
    pub fn _2_8(self) -> &'a mut W {
        self.variant(TXSELW::_2_8)
    }
    #[doc = "Transmit FIFO becomes <= 1/8 full"]
    #[inline]
    pub fn _1_8(self) -> &'a mut W {
        self.variant(TXSELW::_1_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 3:5 - 5:3\\] Receive interrupt FIFO level select: This field sets the trigger points for the receive interrupt. Values 0b101-0b111 are reserved."]
    #[inline]
    pub fn rxsel(&self) -> RXSELR {
        RXSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:2 - 2:0\\] Transmit interrupt FIFO level select: This field sets the trigger points for the transmit interrupt. Values 0b101-0b111 are reserved."]
    #[inline]
    pub fn txsel(&self) -> TXSELR {
        TXSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 18 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 3:5 - 5:3\\] Receive interrupt FIFO level select: This field sets the trigger points for the receive interrupt. Values 0b101-0b111 are reserved."]
    #[inline]
    pub fn rxsel(&mut self) -> _RXSELW {
        _RXSELW { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] Transmit interrupt FIFO level select: This field sets the trigger points for the transmit interrupt. Values 0b101-0b111 are reserved."]
    #[inline]
    pub fn txsel(&mut self) -> _TXSELW {
        _TXSELW { w: self }
    }
}
