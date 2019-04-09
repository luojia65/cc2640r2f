#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RIS {
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
pub struct RESERVED14R {
    bits: u32,
}
impl RESERVED14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMABRISR {
    bits: bool,
}
impl DMABRISR {
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
pub struct RESERVED12R {
    bits: bool,
}
impl RESERVED12R {
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
pub struct TBMRISR {
    bits: bool,
}
impl TBMRISR {
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
pub struct CBERISR {
    bits: bool,
}
impl CBERISR {
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
pub struct CBMRISR {
    bits: bool,
}
impl CBMRISR {
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
pub struct TBTORISR {
    bits: bool,
}
impl TBTORISR {
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
pub struct RESERVED6R {
    bits: u8,
}
impl RESERVED6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMAARISR {
    bits: bool,
}
impl DMAARISR {
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
pub struct TAMRISR {
    bits: bool,
}
impl TAMRISR {
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
pub struct RESERVED3R {
    bits: bool,
}
impl RESERVED3R {
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
pub struct CAERISR {
    bits: bool,
}
impl CAERISR {
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
pub struct CAMRISR {
    bits: bool,
}
impl CAMRISR {
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
pub struct TATORISR {
    bits: bool,
}
impl TATORISR {
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
pub struct _RESERVED14W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED14W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 262143;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMABRISW<'a> {
    w: &'a mut W,
}
impl<'a> _DMABRISW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED12W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED12W<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TBMRISW<'a> {
    w: &'a mut W,
}
impl<'a> _TBMRISW<'a> {
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
pub struct _CBERISW<'a> {
    w: &'a mut W,
}
impl<'a> _CBERISW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CBMRISW<'a> {
    w: &'a mut W,
}
impl<'a> _CBMRISW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TBTORISW<'a> {
    w: &'a mut W,
}
impl<'a> _TBTORISW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED6W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED6W<'a> {
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
pub struct _DMAARISW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAARISW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TAMRISW<'a> {
    w: &'a mut W,
}
impl<'a> _TAMRISW<'a> {
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
pub struct _RESERVED3W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED3W<'a> {
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
pub struct _CAERISW<'a> {
    w: &'a mut W,
}
impl<'a> _CAERISW<'a> {
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
pub struct _CAMRISW<'a> {
    w: &'a mut W,
}
impl<'a> _CAMRISW<'a> {
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
pub struct _TATORISW<'a> {
    w: &'a mut W,
}
impl<'a> _TATORISW<'a> {
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
    #[doc = "Bits 14:31 - 31:14\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved14(&self) -> RESERVED14R {
        let bits = {
            const MASK: u32 = 262143;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED14R { bits }
    }
    #[doc = "Bit 13 - 13:13\\] GPT Timer B DMA Done Raw Interrupt Status 0: Transfer has not completed 1: Transfer has completed"]
    #[inline]
    pub fn dmabris(&self) -> DMABRISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMABRISR { bits }
    }
    #[doc = "Bit 12 - 12:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved12(&self) -> RESERVED12R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED12R { bits }
    }
    #[doc = "Bit 11 - 11:11\\] GPT Timer B Match Raw Interrupt 0: The match value has not been reached 1: The match value is reached. TBMR.TBMIE is set, and the match values in TBMATCHR and optionally TBPMR have been reached when configured in one-shot or periodic mode."]
    #[inline]
    pub fn tbmris(&self) -> TBMRISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TBMRISR { bits }
    }
    #[doc = "Bit 10 - 10:10\\] GPT Timer B Capture Mode Event Raw Interrupt 0: The event has not occured. 1: The event has occured. This interrupt asserts when the subtimer is configured in Input Edge-Time mode"]
    #[inline]
    pub fn cberis(&self) -> CBERISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CBERISR { bits }
    }
    #[doc = "Bit 9 - 9:9\\] GPT Timer B Capture Mode Match Raw Interrupt 0: The capture mode match for Timer B has not occurred. 1: A capture mode match has occurred for Timer B. This interrupt asserts when the values in the TBR and TBPR match the values in the TBMATCHR and TBPMR when configured in Input Edge-Time mode. This bit is cleared by writing a 1 to the ICLR.CBMCINT bit."]
    #[inline]
    pub fn cbmris(&self) -> CBMRISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CBMRISR { bits }
    }
    #[doc = "Bit 8 - 8:8\\] GPT Timer B Time-out Raw Interrupt 0: Timer B has not timed out 1: Timer B has timed out. This interrupt is asserted when a one-shot or periodic mode timer reaches its count limit. The count limit is 0 or the value loaded into TBILR, depending on the count direction."]
    #[inline]
    pub fn tbtoris(&self) -> TBTORISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TBTORISR { bits }
    }
    #[doc = "Bits 6:7 - 7:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&self) -> RESERVED6R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED6R { bits }
    }
    #[doc = "Bit 5 - 5:5\\] GPT Timer A DMA Done Raw Interrupt Status 0: Transfer has not completed 1: Transfer has completed"]
    #[inline]
    pub fn dmaaris(&self) -> DMAARISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMAARISR { bits }
    }
    #[doc = "Bit 4 - 4:4\\] GPT Timer A Match Raw Interrupt 0: The match value has not been reached 1: The match value is reached. TAMR.TAMIE is set, and the match values in TAMATCHR and optionally TAPMR have been reached when configured in one-shot or periodic mode."]
    #[inline]
    pub fn tamris(&self) -> TAMRISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAMRISR { bits }
    }
    #[doc = "Bit 3 - 3:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&self) -> RESERVED3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED3R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] GPT Timer A Capture Mode Event Raw Interrupt 0: The event has not occured. 1: The event has occured. This interrupt asserts when the subtimer is configured in Input Edge-Time mode"]
    #[inline]
    pub fn caeris(&self) -> CAERISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAERISR { bits }
    }
    #[doc = "Bit 1 - 1:1\\] GPT Timer A Capture Mode Match Raw Interrupt 0: The capture mode match for Timer A has not occurred. 1: A capture mode match has occurred for Timer A. This interrupt asserts when the values in the TAR and TAPR match the values in the TAMATCHR and TAPMR when configured in Input Edge-Time mode. This bit is cleared by writing a 1 to the ICLR.CAMCINT bit."]
    #[inline]
    pub fn camris(&self) -> CAMRISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAMRISR { bits }
    }
    #[doc = "Bit 0 - 0:0\\] GPT Timer A Time-out Raw Interrupt 0: Timer A has not timed out 1: Timer A has timed out. This interrupt is asserted when a one-shot or periodic mode timer reaches its count limit. The count limit is 0 or the value loaded into TAILR, depending on the count direction."]
    #[inline]
    pub fn tatoris(&self) -> TATORISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TATORISR { bits }
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
    #[doc = "Bits 14:31 - 31:14\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved14(&mut self) -> _RESERVED14W {
        _RESERVED14W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] GPT Timer B DMA Done Raw Interrupt Status 0: Transfer has not completed 1: Transfer has completed"]
    #[inline]
    pub fn dmabris(&mut self) -> _DMABRISW {
        _DMABRISW { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved12(&mut self) -> _RESERVED12W {
        _RESERVED12W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] GPT Timer B Match Raw Interrupt 0: The match value has not been reached 1: The match value is reached. TBMR.TBMIE is set, and the match values in TBMATCHR and optionally TBPMR have been reached when configured in one-shot or periodic mode."]
    #[inline]
    pub fn tbmris(&mut self) -> _TBMRISW {
        _TBMRISW { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] GPT Timer B Capture Mode Event Raw Interrupt 0: The event has not occured. 1: The event has occured. This interrupt asserts when the subtimer is configured in Input Edge-Time mode"]
    #[inline]
    pub fn cberis(&mut self) -> _CBERISW {
        _CBERISW { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] GPT Timer B Capture Mode Match Raw Interrupt 0: The capture mode match for Timer B has not occurred. 1: A capture mode match has occurred for Timer B. This interrupt asserts when the values in the TBR and TBPR match the values in the TBMATCHR and TBPMR when configured in Input Edge-Time mode. This bit is cleared by writing a 1 to the ICLR.CBMCINT bit."]
    #[inline]
    pub fn cbmris(&mut self) -> _CBMRISW {
        _CBMRISW { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] GPT Timer B Time-out Raw Interrupt 0: Timer B has not timed out 1: Timer B has timed out. This interrupt is asserted when a one-shot or periodic mode timer reaches its count limit. The count limit is 0 or the value loaded into TBILR, depending on the count direction."]
    #[inline]
    pub fn tbtoris(&mut self) -> _TBTORISW {
        _TBTORISW { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&mut self) -> _RESERVED6W {
        _RESERVED6W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] GPT Timer A DMA Done Raw Interrupt Status 0: Transfer has not completed 1: Transfer has completed"]
    #[inline]
    pub fn dmaaris(&mut self) -> _DMAARISW {
        _DMAARISW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] GPT Timer A Match Raw Interrupt 0: The match value has not been reached 1: The match value is reached. TAMR.TAMIE is set, and the match values in TAMATCHR and optionally TAPMR have been reached when configured in one-shot or periodic mode."]
    #[inline]
    pub fn tamris(&mut self) -> _TAMRISW {
        _TAMRISW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&mut self) -> _RESERVED3W {
        _RESERVED3W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] GPT Timer A Capture Mode Event Raw Interrupt 0: The event has not occured. 1: The event has occured. This interrupt asserts when the subtimer is configured in Input Edge-Time mode"]
    #[inline]
    pub fn caeris(&mut self) -> _CAERISW {
        _CAERISW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] GPT Timer A Capture Mode Match Raw Interrupt 0: The capture mode match for Timer A has not occurred. 1: A capture mode match has occurred for Timer A. This interrupt asserts when the values in the TAR and TAPR match the values in the TAMATCHR and TAPMR when configured in Input Edge-Time mode. This bit is cleared by writing a 1 to the ICLR.CAMCINT bit."]
    #[inline]
    pub fn camris(&mut self) -> _CAMRISW {
        _CAMRISW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] GPT Timer A Time-out Raw Interrupt 0: Timer A has not timed out 1: Timer A has timed out. This interrupt is asserted when a one-shot or periodic mode timer reaches its count limit. The count limit is 0 or the value loaded into TAILR, depending on the count direction."]
    #[inline]
    pub fn tatoris(&mut self) -> _TATORISW {
        _TATORISW { w: self }
    }
}
