#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
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
pub struct RESERVED3R {
    bits: u32,
}
impl RESERVED3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `CFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGR {
    #[doc = "16-bit timer configuration.  \nConfigure for two 16-bit timers.\nAlso see TAMR.TAMR and TBMR.TBMR."]
    _16BIT_TIMER,
    #[doc = "32-bit timer configuration"]
    _32BIT_TIMER,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFGR::_16BIT_TIMER => 4,
            CFGR::_32BIT_TIMER => 0,
            CFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFGR {
        match value {
            4 => CFGR::_16BIT_TIMER,
            0 => CFGR::_32BIT_TIMER,
            i => CFGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_16BIT_TIMER`"]
    #[inline]
    pub fn is_16bit_timer(&self) -> bool {
        *self == CFGR::_16BIT_TIMER
    }
    #[doc = "Checks if the value of the field is `_32BIT_TIMER`"]
    #[inline]
    pub fn is_32bit_timer(&self) -> bool {
        *self == CFGR::_32BIT_TIMER
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED3W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 536870911;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFG`"]
pub enum CFGW {
    #[doc = "16-bit timer configuration.  \nConfigure for two 16-bit timers.\nAlso see TAMR.TAMR and TBMR.TBMR."]
    _16BIT_TIMER,
    #[doc = "32-bit timer configuration"]
    _32BIT_TIMER,
}
impl CFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFGW::_16BIT_TIMER => 4,
            CFGW::_32BIT_TIMER => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _CFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "16-bit timer configuration. Configure for two 16-bit timers. Also see TAMR.TAMR and TBMR.TBMR."]
    #[inline]
    pub fn _16bit_timer(self) -> &'a mut W {
        self.variant(CFGW::_16BIT_TIMER)
    }
    #[doc = "32-bit timer configuration"]
    #[inline]
    pub fn _32bit_timer(self) -> &'a mut W {
        self.variant(CFGW::_32BIT_TIMER)
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
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&self) -> RESERVED3R {
        let bits = {
            const MASK: u32 = 536870911;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED3R { bits }
    }
    #[doc = "Bits 0:2 - 2:0\\] GPT Configuration 0x2- 0x3 - Reserved 0x5- 0x7 - Reserved"]
    #[inline]
    pub fn cfg(&self) -> CFGR {
        CFGR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&mut self) -> _RESERVED3W {
        _RESERVED3W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] GPT Configuration 0x2- 0x3 - Reserved 0x5- 0x7 - Reserved"]
    #[inline]
    pub fn cfg(&mut self) -> _CFGW {
        _CFGW { w: self }
    }
}
