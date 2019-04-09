#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHDW_ANA_TRIM {
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
pub struct BOD_BANDGAP_TRIM_CNFR {
    bits: u8,
}
impl BOD_BANDGAP_TRIM_CNFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VDDR_ENABLE_PG1R {
    bits: bool,
}
impl VDDR_ENABLE_PG1R {
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
pub struct VDDR_OK_HYSR {
    bits: bool,
}
impl VDDR_OK_HYSR {
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
pub struct IPTAT_TRIMR {
    bits: u8,
}
impl IPTAT_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VDDR_TRIMR {
    bits: u8,
}
impl VDDR_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIMBOD_INTMODER {
    bits: u8,
}
impl TRIMBOD_INTMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIMBOD_EXTMODER {
    bits: u8,
}
impl TRIMBOD_EXTMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIMTEMPR {
    bits: u8,
}
impl TRIMTEMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _BOD_BANDGAP_TRIM_CNFW<'a> {
    w: &'a mut W,
}
impl<'a> _BOD_BANDGAP_TRIM_CNFW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VDDR_ENABLE_PG1W<'a> {
    w: &'a mut W,
}
impl<'a> _VDDR_ENABLE_PG1W<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VDDR_OK_HYSW<'a> {
    w: &'a mut W,
}
impl<'a> _VDDR_OK_HYSW<'a> {
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
pub struct _IPTAT_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _IPTAT_TRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VDDR_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _VDDR_TRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRIMBOD_INTMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIMBOD_INTMODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRIMBOD_EXTMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIMBOD_EXTMODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRIMTEMPW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIMTEMPW<'a> {
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
    #[doc = "Bits 25:26 - 26:25\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn bod_bandgap_trim_cnf(&self) -> BOD_BANDGAP_TRIM_CNFR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BOD_BANDGAP_TRIM_CNFR { bits }
    }
    #[doc = "Bit 24 - 24:24\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vddr_enable_pg1(&self) -> VDDR_ENABLE_PG1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VDDR_ENABLE_PG1R { bits }
    }
    #[doc = "Bit 23 - 23:23\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vddr_ok_hys(&self) -> VDDR_OK_HYSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VDDR_OK_HYSR { bits }
    }
    #[doc = "Bits 21:22 - 22:21\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn iptat_trim(&self) -> IPTAT_TRIMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IPTAT_TRIMR { bits }
    }
    #[doc = "Bits 16:20 - 20:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vddr_trim(&self) -> VDDR_TRIMR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VDDR_TRIMR { bits }
    }
    #[doc = "Bits 11:15 - 15:11\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trimbod_intmode(&self) -> TRIMBOD_INTMODER {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIMBOD_INTMODER { bits }
    }
    #[doc = "Bits 6:10 - 10:6\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trimbod_extmode(&self) -> TRIMBOD_EXTMODER {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIMBOD_EXTMODER { bits }
    }
    #[doc = "Bits 0:5 - 5:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trimtemp(&self) -> TRIMTEMPR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIMTEMPR { bits }
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
    #[doc = "Bits 25:26 - 26:25\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn bod_bandgap_trim_cnf(&mut self) -> _BOD_BANDGAP_TRIM_CNFW {
        _BOD_BANDGAP_TRIM_CNFW { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vddr_enable_pg1(&mut self) -> _VDDR_ENABLE_PG1W {
        _VDDR_ENABLE_PG1W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vddr_ok_hys(&mut self) -> _VDDR_OK_HYSW {
        _VDDR_OK_HYSW { w: self }
    }
    #[doc = "Bits 21:22 - 22:21\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn iptat_trim(&mut self) -> _IPTAT_TRIMW {
        _IPTAT_TRIMW { w: self }
    }
    #[doc = "Bits 16:20 - 20:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vddr_trim(&mut self) -> _VDDR_TRIMW {
        _VDDR_TRIMW { w: self }
    }
    #[doc = "Bits 11:15 - 15:11\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trimbod_intmode(&mut self) -> _TRIMBOD_INTMODEW {
        _TRIMBOD_INTMODEW { w: self }
    }
    #[doc = "Bits 6:10 - 10:6\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trimbod_extmode(&mut self) -> _TRIMBOD_EXTMODEW {
        _TRIMBOD_EXTMODEW { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trimtemp(&mut self) -> _TRIMTEMPW {
        _TRIMTEMPW { w: self }
    }
}
