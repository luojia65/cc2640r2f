#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WUEVCLR {
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
pub struct AON_RTC_CH2R {
    bits: bool,
}
impl AON_RTC_CH2R {
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
pub struct AON_SWR {
    bits: bool,
}
impl AON_SWR {
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
pub struct AON_PROG_WUR {
    bits: bool,
}
impl AON_PROG_WUR {
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
pub struct _AON_RTC_CH2W<'a> {
    w: &'a mut W,
}
impl<'a> _AON_RTC_CH2W<'a> {
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
pub struct _AON_SWW<'a> {
    w: &'a mut W,
}
impl<'a> _AON_SWW<'a> {
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
pub struct _AON_PROG_WUW<'a> {
    w: &'a mut W,
}
impl<'a> _AON_PROG_WUW<'a> {
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
    #[doc = "Bit 2 - 2:2\\] Set to clear the WUEVFLAGS.AON_RTC_CH2 wake-up event. Note that if RTC channel 2 is also set as source for AON_PROG_WU this field can also clear WUEVFLAGS.AON_PROG_WU This bit must remain set until WUEVFLAGS.AON_RTC_CH2 returns to 0."]
    #[inline]
    pub fn aon_rtc_ch2(&self) -> AON_RTC_CH2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AON_RTC_CH2R { bits }
    }
    #[doc = "Bit 1 - 1:1\\] Set to clear the WUEVFLAGS.AON_SW wake-up event. This bit must remain set until WUEVFLAGS.AON_SW returns to 0."]
    #[inline]
    pub fn aon_sw(&self) -> AON_SWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AON_SWR { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Set to clear the WUEVFLAGS.AON_PROG_WU wake-up event. Note only if an IO event is selected as wake-up event, is it possible to use this field to clear the source. Other sources cannot be cleared using this field. The IO pin needs to be assigned to AUX in the IOC and the input enable for the pin needs to be set in AIODIO0 or AIODIO1 for this clearing to take effect. This bit must remain set until WUEVFLAGS.AON_PROG_WU returns to 0."]
    #[inline]
    pub fn aon_prog_wu(&self) -> AON_PROG_WUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AON_PROG_WUR { bits }
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
    #[doc = "Bit 2 - 2:2\\] Set to clear the WUEVFLAGS.AON_RTC_CH2 wake-up event. Note that if RTC channel 2 is also set as source for AON_PROG_WU this field can also clear WUEVFLAGS.AON_PROG_WU This bit must remain set until WUEVFLAGS.AON_RTC_CH2 returns to 0."]
    #[inline]
    pub fn aon_rtc_ch2(&mut self) -> _AON_RTC_CH2W {
        _AON_RTC_CH2W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Set to clear the WUEVFLAGS.AON_SW wake-up event. This bit must remain set until WUEVFLAGS.AON_SW returns to 0."]
    #[inline]
    pub fn aon_sw(&mut self) -> _AON_SWW {
        _AON_SWW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Set to clear the WUEVFLAGS.AON_PROG_WU wake-up event. Note only if an IO event is selected as wake-up event, is it possible to use this field to clear the source. Other sources cannot be cleared using this field. The IO pin needs to be assigned to AUX in the IOC and the input enable for the pin needs to be set in AIODIO0 or AIODIO1 for this clearing to take effect. This bit must remain set until WUEVFLAGS.AON_PROG_WU returns to 0."]
    #[inline]
    pub fn aon_prog_wu(&mut self) -> _AON_PROG_WUW {
        _AON_PROG_WUW { w: self }
    }
}
