#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ANA2_TRIM {
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
pub struct RCOSCHFCTRIMFRACT_ENR {
    bits: bool,
}
impl RCOSCHFCTRIMFRACT_ENR {
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
pub struct RCOSCHFCTRIMFRACTR {
    bits: u8,
}
impl RCOSCHFCTRIMFRACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
#[doc = r" Value of the field"]
pub struct SET_RCOSC_HF_FINE_RESISTORR {
    bits: u8,
}
impl SET_RCOSC_HF_FINE_RESISTORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ATESTLF_UDIGLDO_IBIAS_TRIMR {
    bits: bool,
}
impl ATESTLF_UDIGLDO_IBIAS_TRIMR {
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
pub struct NANOAMP_RES_TRIMR {
    bits: u8,
}
impl NANOAMP_RES_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED1R {
    bits: u8,
}
impl RESERVED1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DITHER_ENR {
    bits: bool,
}
impl DITHER_ENR {
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
pub struct DCDC_IPEAKR {
    bits: u8,
}
impl DCDC_IPEAKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DEAD_TIME_TRIMR {
    bits: u8,
}
impl DEAD_TIME_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCDC_LOW_EN_SELR {
    bits: u8,
}
impl DCDC_LOW_EN_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCDC_HIGH_EN_SELR {
    bits: u8,
}
impl DCDC_HIGH_EN_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RCOSCHFCTRIMFRACT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RCOSCHFCTRIMFRACT_ENW<'a> {
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
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RCOSCHFCTRIMFRACTW<'a> {
    w: &'a mut W,
}
impl<'a> _RCOSCHFCTRIMFRACTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 26;
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SET_RCOSC_HF_FINE_RESISTORW<'a> {
    w: &'a mut W,
}
impl<'a> _SET_RCOSC_HF_FINE_RESISTORW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ATESTLF_UDIGLDO_IBIAS_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _ATESTLF_UDIGLDO_IBIAS_TRIMW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NANOAMP_RES_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _NANOAMP_RES_TRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED1W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DITHER_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DITHER_ENW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCDC_IPEAKW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_IPEAKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DEAD_TIME_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _DEAD_TIME_TRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCDC_LOW_EN_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_LOW_EN_SELW<'a> {
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
pub struct _DCDC_HIGH_EN_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_HIGH_EN_SELW<'a> {
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
    #[doc = "Bit 31 - 31:31\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcoschfctrimfract_en(&self) -> RCOSCHFCTRIMFRACT_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RCOSCHFCTRIMFRACT_ENR { bits }
    }
    #[doc = "Bits 26:30 - 30:26\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcoschfctrimfract(&self) -> RCOSCHFCTRIMFRACTR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RCOSCHFCTRIMFRACTR { bits }
    }
    #[doc = "Bit 25 - 25:25\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved0(&self) -> RESERVED0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED0R { bits }
    }
    #[doc = "Bits 23:24 - 24:23\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn set_rcosc_hf_fine_resistor(&self) -> SET_RCOSC_HF_FINE_RESISTORR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SET_RCOSC_HF_FINE_RESISTORR { bits }
    }
    #[doc = "Bit 22 - 22:22\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn atestlf_udigldo_ibias_trim(&self) -> ATESTLF_UDIGLDO_IBIAS_TRIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ATESTLF_UDIGLDO_IBIAS_TRIMR { bits }
    }
    #[doc = "Bits 16:21 - 21:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn nanoamp_res_trim(&self) -> NANOAMP_RES_TRIMR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NANOAMP_RES_TRIMR { bits }
    }
    #[doc = "Bits 12:15 - 15:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved1(&self) -> RESERVED1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED1R { bits }
    }
    #[doc = "Bit 11 - 11:11\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dither_en(&self) -> DITHER_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DITHER_ENR { bits }
    }
    #[doc = "Bits 8:10 - 10:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dcdc_ipeak(&self) -> DCDC_IPEAKR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCDC_IPEAKR { bits }
    }
    #[doc = "Bits 6:7 - 7:6\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dead_time_trim(&self) -> DEAD_TIME_TRIMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DEAD_TIME_TRIMR { bits }
    }
    #[doc = "Bits 3:5 - 5:3\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dcdc_low_en_sel(&self) -> DCDC_LOW_EN_SELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCDC_LOW_EN_SELR { bits }
    }
    #[doc = "Bits 0:2 - 2:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dcdc_high_en_sel(&self) -> DCDC_HIGH_EN_SELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCDC_HIGH_EN_SELR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2185294975 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 31 - 31:31\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcoschfctrimfract_en(&mut self) -> _RCOSCHFCTRIMFRACT_ENW {
        _RCOSCHFCTRIMFRACT_ENW { w: self }
    }
    #[doc = "Bits 26:30 - 30:26\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcoschfctrimfract(&mut self) -> _RCOSCHFCTRIMFRACTW {
        _RCOSCHFCTRIMFRACTW { w: self }
    }
    #[doc = "Bit 25 - 25:25\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved0(&mut self) -> _RESERVED0W {
        _RESERVED0W { w: self }
    }
    #[doc = "Bits 23:24 - 24:23\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn set_rcosc_hf_fine_resistor(&mut self) -> _SET_RCOSC_HF_FINE_RESISTORW {
        _SET_RCOSC_HF_FINE_RESISTORW { w: self }
    }
    #[doc = "Bit 22 - 22:22\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn atestlf_udigldo_ibias_trim(&mut self) -> _ATESTLF_UDIGLDO_IBIAS_TRIMW {
        _ATESTLF_UDIGLDO_IBIAS_TRIMW { w: self }
    }
    #[doc = "Bits 16:21 - 21:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn nanoamp_res_trim(&mut self) -> _NANOAMP_RES_TRIMW {
        _NANOAMP_RES_TRIMW { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved1(&mut self) -> _RESERVED1W {
        _RESERVED1W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dither_en(&mut self) -> _DITHER_ENW {
        _DITHER_ENW { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dcdc_ipeak(&mut self) -> _DCDC_IPEAKW {
        _DCDC_IPEAKW { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dead_time_trim(&mut self) -> _DEAD_TIME_TRIMW {
        _DEAD_TIME_TRIMW { w: self }
    }
    #[doc = "Bits 3:5 - 5:3\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dcdc_low_en_sel(&mut self) -> _DCDC_LOW_EN_SELW {
        _DCDC_LOW_EN_SELW { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dcdc_high_en_sel(&mut self) -> _DCDC_HIGH_EN_SELW {
        _DCDC_HIGH_EN_SELW { w: self }
    }
}
