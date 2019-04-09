#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FSM_MODE {
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
pub struct RESERVED20R {
    bits: u16,
}
impl RESERVED20R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RDV_SUBMODER {
    bits: u8,
}
impl RDV_SUBMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PGM_SUBMODER {
    bits: u8,
}
impl PGM_SUBMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ERA_SUBMODER {
    bits: u8,
}
impl ERA_SUBMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SUBMODER {
    bits: u8,
}
impl SUBMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SAV_PGM_CMDR {
    bits: u8,
}
impl SAV_PGM_CMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SAV_ERA_MODER {
    bits: u8,
}
impl SAV_ERA_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MODER {
    bits: u8,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMDR {
    bits: u8,
}
impl CMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED20W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED20W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RDV_SUBMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _RDV_SUBMODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PGM_SUBMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PGM_SUBMODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ERA_SUBMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ERA_SUBMODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SUBMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SUBMODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SAV_PGM_CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _SAV_PGM_CMDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SAV_ERA_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SAV_ERA_MODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
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
#[doc = r" Proxy"]
pub struct _CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDW<'a> {
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
    #[doc = "Bits 20:31 - 31:20\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved20(&self) -> RESERVED20R {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED20R { bits }
    }
    #[doc = "Bits 18:19 - 19:18\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rdv_submode(&self) -> RDV_SUBMODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RDV_SUBMODER { bits }
    }
    #[doc = "Bits 16:17 - 17:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn pgm_submode(&self) -> PGM_SUBMODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PGM_SUBMODER { bits }
    }
    #[doc = "Bits 14:15 - 15:14\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn era_submode(&self) -> ERA_SUBMODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ERA_SUBMODER { bits }
    }
    #[doc = "Bits 12:13 - 13:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn submode(&self) -> SUBMODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SUBMODER { bits }
    }
    #[doc = "Bits 9:11 - 11:9\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sav_pgm_cmd(&self) -> SAV_PGM_CMDR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SAV_PGM_CMDR { bits }
    }
    #[doc = "Bits 6:8 - 8:6\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sav_era_mode(&self) -> SAV_ERA_MODER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SAV_ERA_MODER { bits }
    }
    #[doc = "Bits 3:5 - 5:3\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn mode(&self) -> MODER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MODER { bits }
    }
    #[doc = "Bits 0:2 - 2:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn cmd(&self) -> CMDR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMDR { bits }
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
    #[doc = "Bits 20:31 - 31:20\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved20(&mut self) -> _RESERVED20W {
        _RESERVED20W { w: self }
    }
    #[doc = "Bits 18:19 - 19:18\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rdv_submode(&mut self) -> _RDV_SUBMODEW {
        _RDV_SUBMODEW { w: self }
    }
    #[doc = "Bits 16:17 - 17:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn pgm_submode(&mut self) -> _PGM_SUBMODEW {
        _PGM_SUBMODEW { w: self }
    }
    #[doc = "Bits 14:15 - 15:14\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn era_submode(&mut self) -> _ERA_SUBMODEW {
        _ERA_SUBMODEW { w: self }
    }
    #[doc = "Bits 12:13 - 13:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn submode(&mut self) -> _SUBMODEW {
        _SUBMODEW { w: self }
    }
    #[doc = "Bits 9:11 - 11:9\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sav_pgm_cmd(&mut self) -> _SAV_PGM_CMDW {
        _SAV_PGM_CMDW { w: self }
    }
    #[doc = "Bits 6:8 - 8:6\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sav_era_mode(&mut self) -> _SAV_ERA_MODEW {
        _SAV_ERA_MODEW { w: self }
    }
    #[doc = "Bits 3:5 - 5:3\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn cmd(&mut self) -> _CMDW {
        _CMDW { w: self }
    }
}
