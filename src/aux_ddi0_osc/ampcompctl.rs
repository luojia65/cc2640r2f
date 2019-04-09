#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AMPCOMPCTL {
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
pub struct SPARE31R {
    bits: bool,
}
impl SPARE31R {
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
pub struct AMPCOMP_REQ_MODER {
    bits: bool,
}
impl AMPCOMP_REQ_MODER {
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
#[doc = "Possible values of the field `AMPCOMP_FSM_UPDATE_RATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AMPCOMP_FSM_UPDATE_RATER {
    #[doc = "Internal. Only to be used through TI provided API."]
    _250KHZ,
    #[doc = "Internal. Only to be used through TI provided API."]
    _500KHZ,
    #[doc = "Internal. Only to be used through TI provided API."]
    _1MHZ,
    #[doc = "Internal. Only to be used through TI provided API."]
    _2MHZ,
}
impl AMPCOMP_FSM_UPDATE_RATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AMPCOMP_FSM_UPDATE_RATER::_250KHZ => 3,
            AMPCOMP_FSM_UPDATE_RATER::_500KHZ => 2,
            AMPCOMP_FSM_UPDATE_RATER::_1MHZ => 1,
            AMPCOMP_FSM_UPDATE_RATER::_2MHZ => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AMPCOMP_FSM_UPDATE_RATER {
        match value {
            3 => AMPCOMP_FSM_UPDATE_RATER::_250KHZ,
            2 => AMPCOMP_FSM_UPDATE_RATER::_500KHZ,
            1 => AMPCOMP_FSM_UPDATE_RATER::_1MHZ,
            0 => AMPCOMP_FSM_UPDATE_RATER::_2MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_250KHZ`"]
    #[inline]
    pub fn is_250khz(&self) -> bool {
        *self == AMPCOMP_FSM_UPDATE_RATER::_250KHZ
    }
    #[doc = "Checks if the value of the field is `_500KHZ`"]
    #[inline]
    pub fn is_500khz(&self) -> bool {
        *self == AMPCOMP_FSM_UPDATE_RATER::_500KHZ
    }
    #[doc = "Checks if the value of the field is `_1MHZ`"]
    #[inline]
    pub fn is_1mhz(&self) -> bool {
        *self == AMPCOMP_FSM_UPDATE_RATER::_1MHZ
    }
    #[doc = "Checks if the value of the field is `_2MHZ`"]
    #[inline]
    pub fn is_2mhz(&self) -> bool {
        *self == AMPCOMP_FSM_UPDATE_RATER::_2MHZ
    }
}
#[doc = r" Value of the field"]
pub struct AMPCOMP_SW_CTRLR {
    bits: bool,
}
impl AMPCOMP_SW_CTRLR {
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
pub struct AMPCOMP_SW_ENR {
    bits: bool,
}
impl AMPCOMP_SW_ENR {
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
pub struct RESERVED24R {
    bits: u8,
}
impl RESERVED24R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IBIAS_OFFSETR {
    bits: u8,
}
impl IBIAS_OFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IBIAS_INITR {
    bits: u8,
}
impl IBIAS_INITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LPM_IBIAS_WAIT_CNT_FINALR {
    bits: u8,
}
impl LPM_IBIAS_WAIT_CNT_FINALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CAP_STEPR {
    bits: u8,
}
impl CAP_STEPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IBIASCAP_HPTOLP_OL_CNTR {
    bits: u8,
}
impl IBIASCAP_HPTOLP_OL_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SPARE31W<'a> {
    w: &'a mut W,
}
impl<'a> _SPARE31W<'a> {
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
pub struct _AMPCOMP_REQ_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _AMPCOMP_REQ_MODEW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AMPCOMP_FSM_UPDATE_RATE`"]
pub enum AMPCOMP_FSM_UPDATE_RATEW {
    #[doc = "Internal. Only to be used through TI provided API."]
    _250KHZ,
    #[doc = "Internal. Only to be used through TI provided API."]
    _500KHZ,
    #[doc = "Internal. Only to be used through TI provided API."]
    _1MHZ,
    #[doc = "Internal. Only to be used through TI provided API."]
    _2MHZ,
}
impl AMPCOMP_FSM_UPDATE_RATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AMPCOMP_FSM_UPDATE_RATEW::_250KHZ => 3,
            AMPCOMP_FSM_UPDATE_RATEW::_500KHZ => 2,
            AMPCOMP_FSM_UPDATE_RATEW::_1MHZ => 1,
            AMPCOMP_FSM_UPDATE_RATEW::_2MHZ => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AMPCOMP_FSM_UPDATE_RATEW<'a> {
    w: &'a mut W,
}
impl<'a> _AMPCOMP_FSM_UPDATE_RATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AMPCOMP_FSM_UPDATE_RATEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn _250khz(self) -> &'a mut W {
        self.variant(AMPCOMP_FSM_UPDATE_RATEW::_250KHZ)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn _500khz(self) -> &'a mut W {
        self.variant(AMPCOMP_FSM_UPDATE_RATEW::_500KHZ)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn _1mhz(self) -> &'a mut W {
        self.variant(AMPCOMP_FSM_UPDATE_RATEW::_1MHZ)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn _2mhz(self) -> &'a mut W {
        self.variant(AMPCOMP_FSM_UPDATE_RATEW::_2MHZ)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AMPCOMP_SW_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _AMPCOMP_SW_CTRLW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AMPCOMP_SW_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _AMPCOMP_SW_ENW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED24W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED24W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IBIAS_OFFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _IBIAS_OFFSETW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IBIAS_INITW<'a> {
    w: &'a mut W,
}
impl<'a> _IBIAS_INITW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPM_IBIAS_WAIT_CNT_FINALW<'a> {
    w: &'a mut W,
}
impl<'a> _LPM_IBIAS_WAIT_CNT_FINALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CAP_STEPW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP_STEPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IBIASCAP_HPTOLP_OL_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _IBIASCAP_HPTOLP_OL_CNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bit 31 - 31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn spare31(&self) -> SPARE31R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SPARE31R { bits }
    }
    #[doc = "Bit 30 - 30:30\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ampcomp_req_mode(&self) -> AMPCOMP_REQ_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AMPCOMP_REQ_MODER { bits }
    }
    #[doc = "Bits 28:29 - 29:28\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ampcomp_fsm_update_rate(&self) -> AMPCOMP_FSM_UPDATE_RATER {
        AMPCOMP_FSM_UPDATE_RATER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 27 - 27:27\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ampcomp_sw_ctrl(&self) -> AMPCOMP_SW_CTRLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AMPCOMP_SW_CTRLR { bits }
    }
    #[doc = "Bit 26 - 26:26\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ampcomp_sw_en(&self) -> AMPCOMP_SW_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AMPCOMP_SW_ENR { bits }
    }
    #[doc = "Bits 24:25 - 25:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved24(&self) -> RESERVED24R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED24R { bits }
    }
    #[doc = "Bits 20:23 - 23:20\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ibias_offset(&self) -> IBIAS_OFFSETR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IBIAS_OFFSETR { bits }
    }
    #[doc = "Bits 16:19 - 19:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ibias_init(&self) -> IBIAS_INITR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IBIAS_INITR { bits }
    }
    #[doc = "Bits 8:15 - 15:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn lpm_ibias_wait_cnt_final(&self) -> LPM_IBIAS_WAIT_CNT_FINALR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPM_IBIAS_WAIT_CNT_FINALR { bits }
    }
    #[doc = "Bits 4:7 - 7:4\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn cap_step(&self) -> CAP_STEPR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CAP_STEPR { bits }
    }
    #[doc = "Bits 0:3 - 3:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ibiascap_hptolp_ol_cnt(&self) -> IBIASCAP_HPTOLP_OL_CNTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IBIASCAP_HPTOLP_OL_CNTR { bits }
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
    #[doc = "Bit 31 - 31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn spare31(&mut self) -> _SPARE31W {
        _SPARE31W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ampcomp_req_mode(&mut self) -> _AMPCOMP_REQ_MODEW {
        _AMPCOMP_REQ_MODEW { w: self }
    }
    #[doc = "Bits 28:29 - 29:28\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ampcomp_fsm_update_rate(&mut self) -> _AMPCOMP_FSM_UPDATE_RATEW {
        _AMPCOMP_FSM_UPDATE_RATEW { w: self }
    }
    #[doc = "Bit 27 - 27:27\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ampcomp_sw_ctrl(&mut self) -> _AMPCOMP_SW_CTRLW {
        _AMPCOMP_SW_CTRLW { w: self }
    }
    #[doc = "Bit 26 - 26:26\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ampcomp_sw_en(&mut self) -> _AMPCOMP_SW_ENW {
        _AMPCOMP_SW_ENW { w: self }
    }
    #[doc = "Bits 24:25 - 25:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved24(&mut self) -> _RESERVED24W {
        _RESERVED24W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ibias_offset(&mut self) -> _IBIAS_OFFSETW {
        _IBIAS_OFFSETW { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ibias_init(&mut self) -> _IBIAS_INITW {
        _IBIAS_INITW { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn lpm_ibias_wait_cnt_final(&mut self) -> _LPM_IBIAS_WAIT_CNT_FINALW {
        _LPM_IBIAS_WAIT_CNT_FINALW { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn cap_step(&mut self) -> _CAP_STEPW {
        _CAP_STEPW { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ibiascap_hptolp_ol_cnt(&mut self) -> _IBIASCAP_HPTOLP_OL_CNTW {
        _IBIASCAP_HPTOLP_OL_CNTW { w: self }
    }
}
