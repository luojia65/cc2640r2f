#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ICEPICK_DEVICE_ID {
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
pub struct PG_REVR {
    bits: u8,
}
impl PG_REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WAFER_IDR {
    bits: u16,
}
impl WAFER_IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MANUFACTURER_IDR {
    bits: u16,
}
impl MANUFACTURER_IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PG_REVW<'a> {
    w: &'a mut W,
}
impl<'a> _PG_REVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WAFER_IDW<'a> {
    w: &'a mut W,
}
impl<'a> _WAFER_IDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MANUFACTURER_IDW<'a> {
    w: &'a mut W,
}
impl<'a> _MANUFACTURER_IDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
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
    #[doc = "Bits 28:31 - 31:28\\] Field used to distinguish revisions of the device."]
    #[inline]
    pub fn pg_rev(&self) -> PG_REVR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PG_REVR { bits }
    }
    #[doc = "Bits 12:27 - 27:12\\] Field used to identify silicon die."]
    #[inline]
    pub fn wafer_id(&self) -> WAFER_IDR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        WAFER_IDR { bits }
    }
    #[doc = "Bits 0:11 - 11:0\\] Manufacturer code. 0x02F: Texas Instruments"]
    #[inline]
    pub fn manufacturer_id(&self) -> MANUFACTURER_IDR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MANUFACTURER_IDR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3147407407 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 28:31 - 31:28\\] Field used to distinguish revisions of the device."]
    #[inline]
    pub fn pg_rev(&mut self) -> _PG_REVW {
        _PG_REVW { w: self }
    }
    #[doc = "Bits 12:27 - 27:12\\] Field used to identify silicon die."]
    #[inline]
    pub fn wafer_id(&mut self) -> _WAFER_IDW {
        _WAFER_IDW { w: self }
    }
    #[doc = "Bits 0:11 - 11:0\\] Manufacturer code. 0x02F: Texas Instruments"]
    #[inline]
    pub fn manufacturer_id(&mut self) -> _MANUFACTURER_IDW {
        _MANUFACTURER_IDW { w: self }
    }
}
