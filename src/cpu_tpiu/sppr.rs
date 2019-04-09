#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SPPR {
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
#[doc = "Possible values of the field `PROTOCOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROTOCOLR {
    #[doc = "SerialWire Output (NRZ)"]
    SWO_NRZ,
    #[doc = "SerialWire Output (Manchester). This is the reset value."]
    SWO_MANCHESTER,
    #[doc = "TracePort mode"]
    TRACEPORT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PROTOCOLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PROTOCOLR::SWO_NRZ => 2,
            PROTOCOLR::SWO_MANCHESTER => 1,
            PROTOCOLR::TRACEPORT => 0,
            PROTOCOLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PROTOCOLR {
        match value {
            2 => PROTOCOLR::SWO_NRZ,
            1 => PROTOCOLR::SWO_MANCHESTER,
            0 => PROTOCOLR::TRACEPORT,
            i => PROTOCOLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SWO_NRZ`"]
    #[inline]
    pub fn is_swo_nrz(&self) -> bool {
        *self == PROTOCOLR::SWO_NRZ
    }
    #[doc = "Checks if the value of the field is `SWO_MANCHESTER`"]
    #[inline]
    pub fn is_swo_manchester(&self) -> bool {
        *self == PROTOCOLR::SWO_MANCHESTER
    }
    #[doc = "Checks if the value of the field is `TRACEPORT`"]
    #[inline]
    pub fn is_traceport(&self) -> bool {
        *self == PROTOCOLR::TRACEPORT
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
        const MASK: u32 = 1073741823;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PROTOCOL`"]
pub enum PROTOCOLW {
    #[doc = "SerialWire Output (NRZ)"]
    SWO_NRZ,
    #[doc = "SerialWire Output (Manchester). This is the reset value."]
    SWO_MANCHESTER,
    #[doc = "TracePort mode"]
    TRACEPORT,
}
impl PROTOCOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PROTOCOLW::SWO_NRZ => 2,
            PROTOCOLW::SWO_MANCHESTER => 1,
            PROTOCOLW::TRACEPORT => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PROTOCOLW<'a> {
    w: &'a mut W,
}
impl<'a> _PROTOCOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PROTOCOLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SerialWire Output (NRZ)"]
    #[inline]
    pub fn swo_nrz(self) -> &'a mut W {
        self.variant(PROTOCOLW::SWO_NRZ)
    }
    #[doc = "SerialWire Output (Manchester). This is the reset value."]
    #[inline]
    pub fn swo_manchester(self) -> &'a mut W {
        self.variant(PROTOCOLW::SWO_MANCHESTER)
    }
    #[doc = "TracePort mode"]
    #[inline]
    pub fn traceport(self) -> &'a mut W {
        self.variant(PROTOCOLW::TRACEPORT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 2:31 - 31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: u32 = 1073741823;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED2R { bits }
    }
    #[doc = "Bits 0:1 - 1:0\\] Trace output protocol"]
    #[inline]
    pub fn protocol(&self) -> PROTOCOLR {
        PROTOCOLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 2:31 - 31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&mut self) -> _RESERVED2W {
        _RESERVED2W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Trace output protocol"]
    #[inline]
    pub fn protocol(&mut self) -> _PROTOCOLW {
        _PROTOCOLW { w: self }
    }
}
