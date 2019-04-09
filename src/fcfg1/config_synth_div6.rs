#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG_SYNTH_DIV6 {
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
pub struct RFC_MDM_DEMIQMC0R {
    bits: u16,
}
impl RFC_MDM_DEMIQMC0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LDOVCO_TRIM_OUTPUTR {
    bits: u8,
}
impl LDOVCO_TRIM_OUTPUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SLDO_TRIM_OUTPUTR {
    bits: u8,
}
impl SLDO_TRIM_OUTPUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RFC_MDM_DEMIQMC0W<'a> {
    w: &'a mut W,
}
impl<'a> _RFC_MDM_DEMIQMC0W<'a> {
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
pub struct _LDOVCO_TRIM_OUTPUTW<'a> {
    w: &'a mut W,
}
impl<'a> _LDOVCO_TRIM_OUTPUTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SLDO_TRIM_OUTPUTW<'a> {
    w: &'a mut W,
}
impl<'a> _SLDO_TRIM_OUTPUTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
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
    #[doc = "Bits 12:27 - 27:12\\] Trim value for RF Core. Value is read by RF Core ROM FW during RF Core initialization."]
    #[inline]
    pub fn rfc_mdm_demiqmc0(&self) -> RFC_MDM_DEMIQMC0R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RFC_MDM_DEMIQMC0R { bits }
    }
    #[doc = "Bits 6:11 - 11:6\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ldovco_trim_output(&self) -> LDOVCO_TRIM_OUTPUTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LDOVCO_TRIM_OUTPUTR { bits }
    }
    #[doc = "Bits 0:5 - 5:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sldo_trim_output(&self) -> SLDO_TRIM_OUTPUTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SLDO_TRIM_OUTPUTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4294967295 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 12:27 - 27:12\\] Trim value for RF Core. Value is read by RF Core ROM FW during RF Core initialization."]
    #[inline]
    pub fn rfc_mdm_demiqmc0(&mut self) -> _RFC_MDM_DEMIQMC0W {
        _RFC_MDM_DEMIQMC0W { w: self }
    }
    #[doc = "Bits 6:11 - 11:6\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ldovco_trim_output(&mut self) -> _LDOVCO_TRIM_OUTPUTW {
        _LDOVCO_TRIM_OUTPUTW { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sldo_trim_output(&mut self) -> _SLDO_TRIM_OUTPUTW {
        _SLDO_TRIM_OUTPUTW { w: self }
    }
}
