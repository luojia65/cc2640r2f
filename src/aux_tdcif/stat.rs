#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STAT {
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
pub struct RESERVED8R {
    bits: u32,
}
impl RESERVED8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SATR {
    bits: bool,
}
impl SATR {
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
pub struct DONER {
    bits: bool,
}
impl DONER {
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
#[doc = "Possible values of the field `STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATER {
    #[doc = "Current state is TDC_FORCESTOP.\nYou wrote ABORT to CTL.CMD to abort the TDC measurement."]
    FORCE_STOP,
    #[doc = "Current state is TDC_WAIT_STARTFALL. \nThe fast-counter circuit waits for a falling edge on the start event."]
    START_FALL,
    #[doc = "Current state is TDC_STATE_WAIT_CLRCNT_DONE. \nThe state machine waits for fast-counter circuit to finish reset."]
    WAIT_CLR_CNT_DONE,
    #[doc = "Current state is TDC_STATE_POR. \nThis is the reset state."]
    POR,
    #[doc = "Current state is TDC_STATE_GETRESULTS.\nThe state machine copies the counter value from the fast-counter circuit."]
    GET_RESULT,
    #[doc = "Current state is TDC_STATE_WAIT_STOPCNTDOWN.\nThe fast-counter circuit looks for the stop condition. It will ignore a number of stop events configured in TRIGCNTLOAD.CNT."]
    WAIT_STOP_CNTDWN,
    #[doc = "Current state is TDC_STATE_WAIT_STOP.\nThe state machine waits for the fast-counter circuit to stop."]
    WAIT_STOP,
    #[doc = "Current state is TDC_STATE_CLRCNT. The fast-counter circuit is reset."]
    CLR_CNT,
    #[doc = "Current state is TDC_STATE_IDLE. \nThis is the default state after reset and abortion. State will change when you write CTL.CMD to either RUN_SYNC_START or RUN."]
    IDLE,
    #[doc = "Current state is TDC_STATE_WAIT_STARTSTOPCNTEN.\nThe fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment."]
    WAIT_START_STOP_CNT_EN,
    #[doc = "Current state is TDC_STATE_WAIT_START. \nThe fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment."]
    WAIT_START,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STATER::FORCE_STOP => 46,
            STATER::START_FALL => 30,
            STATER::WAIT_CLR_CNT_DONE => 22,
            STATER::POR => 15,
            STATER::GET_RESULT => 14,
            STATER::WAIT_STOP_CNTDWN => 12,
            STATER::WAIT_STOP => 8,
            STATER::CLR_CNT => 7,
            STATER::IDLE => 6,
            STATER::WAIT_START_STOP_CNT_EN => 4,
            STATER::WAIT_START => 0,
            STATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STATER {
        match value {
            46 => STATER::FORCE_STOP,
            30 => STATER::START_FALL,
            22 => STATER::WAIT_CLR_CNT_DONE,
            15 => STATER::POR,
            14 => STATER::GET_RESULT,
            12 => STATER::WAIT_STOP_CNTDWN,
            8 => STATER::WAIT_STOP,
            7 => STATER::CLR_CNT,
            6 => STATER::IDLE,
            4 => STATER::WAIT_START_STOP_CNT_EN,
            0 => STATER::WAIT_START,
            i => STATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FORCE_STOP`"]
    #[inline]
    pub fn is_force_stop(&self) -> bool {
        *self == STATER::FORCE_STOP
    }
    #[doc = "Checks if the value of the field is `START_FALL`"]
    #[inline]
    pub fn is_start_fall(&self) -> bool {
        *self == STATER::START_FALL
    }
    #[doc = "Checks if the value of the field is `WAIT_CLR_CNT_DONE`"]
    #[inline]
    pub fn is_wait_clr_cnt_done(&self) -> bool {
        *self == STATER::WAIT_CLR_CNT_DONE
    }
    #[doc = "Checks if the value of the field is `POR`"]
    #[inline]
    pub fn is_por(&self) -> bool {
        *self == STATER::POR
    }
    #[doc = "Checks if the value of the field is `GET_RESULT`"]
    #[inline]
    pub fn is_get_result(&self) -> bool {
        *self == STATER::GET_RESULT
    }
    #[doc = "Checks if the value of the field is `WAIT_STOP_CNTDWN`"]
    #[inline]
    pub fn is_wait_stop_cntdwn(&self) -> bool {
        *self == STATER::WAIT_STOP_CNTDWN
    }
    #[doc = "Checks if the value of the field is `WAIT_STOP`"]
    #[inline]
    pub fn is_wait_stop(&self) -> bool {
        *self == STATER::WAIT_STOP
    }
    #[doc = "Checks if the value of the field is `CLR_CNT`"]
    #[inline]
    pub fn is_clr_cnt(&self) -> bool {
        *self == STATER::CLR_CNT
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == STATER::IDLE
    }
    #[doc = "Checks if the value of the field is `WAIT_START_STOP_CNT_EN`"]
    #[inline]
    pub fn is_wait_start_stop_cnt_en(&self) -> bool {
        *self == STATER::WAIT_START_STOP_CNT_EN
    }
    #[doc = "Checks if the value of the field is `WAIT_START`"]
    #[inline]
    pub fn is_wait_start(&self) -> bool {
        *self == STATER::WAIT_START
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED8W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED8W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SATW<'a> {
    w: &'a mut W,
}
impl<'a> _SATW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _DONEW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STATE`"]
pub enum STATEW {
    #[doc = "Current state is TDC_FORCESTOP.\nYou wrote ABORT to CTL.CMD to abort the TDC measurement."]
    FORCE_STOP,
    #[doc = "Current state is TDC_WAIT_STARTFALL. \nThe fast-counter circuit waits for a falling edge on the start event."]
    START_FALL,
    #[doc = "Current state is TDC_STATE_WAIT_CLRCNT_DONE. \nThe state machine waits for fast-counter circuit to finish reset."]
    WAIT_CLR_CNT_DONE,
    #[doc = "Current state is TDC_STATE_POR. \nThis is the reset state."]
    POR,
    #[doc = "Current state is TDC_STATE_GETRESULTS.\nThe state machine copies the counter value from the fast-counter circuit."]
    GET_RESULT,
    #[doc = "Current state is TDC_STATE_WAIT_STOPCNTDOWN.\nThe fast-counter circuit looks for the stop condition. It will ignore a number of stop events configured in TRIGCNTLOAD.CNT."]
    WAIT_STOP_CNTDWN,
    #[doc = "Current state is TDC_STATE_WAIT_STOP.\nThe state machine waits for the fast-counter circuit to stop."]
    WAIT_STOP,
    #[doc = "Current state is TDC_STATE_CLRCNT. The fast-counter circuit is reset."]
    CLR_CNT,
    #[doc = "Current state is TDC_STATE_IDLE. \nThis is the default state after reset and abortion. State will change when you write CTL.CMD to either RUN_SYNC_START or RUN."]
    IDLE,
    #[doc = "Current state is TDC_STATE_WAIT_STARTSTOPCNTEN.\nThe fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment."]
    WAIT_START_STOP_CNT_EN,
    #[doc = "Current state is TDC_STATE_WAIT_START. \nThe fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment."]
    WAIT_START,
}
impl STATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STATEW::FORCE_STOP => 46,
            STATEW::START_FALL => 30,
            STATEW::WAIT_CLR_CNT_DONE => 22,
            STATEW::POR => 15,
            STATEW::GET_RESULT => 14,
            STATEW::WAIT_STOP_CNTDWN => 12,
            STATEW::WAIT_STOP => 8,
            STATEW::CLR_CNT => 7,
            STATEW::IDLE => 6,
            STATEW::WAIT_START_STOP_CNT_EN => 4,
            STATEW::WAIT_START => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STATEW<'a> {
    w: &'a mut W,
}
impl<'a> _STATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STATEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Current state is TDC_FORCESTOP. You wrote ABORT to CTL.CMD to abort the TDC measurement."]
    #[inline]
    pub fn force_stop(self) -> &'a mut W {
        self.variant(STATEW::FORCE_STOP)
    }
    #[doc = "Current state is TDC_WAIT_STARTFALL. The fast-counter circuit waits for a falling edge on the start event."]
    #[inline]
    pub fn start_fall(self) -> &'a mut W {
        self.variant(STATEW::START_FALL)
    }
    #[doc = "Current state is TDC_STATE_WAIT_CLRCNT_DONE. The state machine waits for fast-counter circuit to finish reset."]
    #[inline]
    pub fn wait_clr_cnt_done(self) -> &'a mut W {
        self.variant(STATEW::WAIT_CLR_CNT_DONE)
    }
    #[doc = "Current state is TDC_STATE_POR. This is the reset state."]
    #[inline]
    pub fn por(self) -> &'a mut W {
        self.variant(STATEW::POR)
    }
    #[doc = "Current state is TDC_STATE_GETRESULTS. The state machine copies the counter value from the fast-counter circuit."]
    #[inline]
    pub fn get_result(self) -> &'a mut W {
        self.variant(STATEW::GET_RESULT)
    }
    #[doc = "Current state is TDC_STATE_WAIT_STOPCNTDOWN. The fast-counter circuit looks for the stop condition. It will ignore a number of stop events configured in TRIGCNTLOAD.CNT."]
    #[inline]
    pub fn wait_stop_cntdwn(self) -> &'a mut W {
        self.variant(STATEW::WAIT_STOP_CNTDWN)
    }
    #[doc = "Current state is TDC_STATE_WAIT_STOP. The state machine waits for the fast-counter circuit to stop."]
    #[inline]
    pub fn wait_stop(self) -> &'a mut W {
        self.variant(STATEW::WAIT_STOP)
    }
    #[doc = "Current state is TDC_STATE_CLRCNT. The fast-counter circuit is reset."]
    #[inline]
    pub fn clr_cnt(self) -> &'a mut W {
        self.variant(STATEW::CLR_CNT)
    }
    #[doc = "Current state is TDC_STATE_IDLE. This is the default state after reset and abortion. State will change when you write CTL.CMD to either RUN_SYNC_START or RUN."]
    #[inline]
    pub fn idle(self) -> &'a mut W {
        self.variant(STATEW::IDLE)
    }
    #[doc = "Current state is TDC_STATE_WAIT_STARTSTOPCNTEN. The fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment."]
    #[inline]
    pub fn wait_start_stop_cnt_en(self) -> &'a mut W {
        self.variant(STATEW::WAIT_START_STOP_CNT_EN)
    }
    #[doc = "Current state is TDC_STATE_WAIT_START. The fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment."]
    #[inline]
    pub fn wait_start(self) -> &'a mut W {
        self.variant(STATEW::WAIT_START)
    }
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
    #[doc = "Bits 8:31 - 31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved8(&self) -> RESERVED8R {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED8R { bits }
    }
    #[doc = "Bit 7 - 7:7\\] TDC measurement saturation flag. 0: Conversion has not saturated. 1: Conversion stopped due to saturation. This field is cleared when a new measurement is started or when CLR_RESULT is written to CTL.CMD."]
    #[inline]
    pub fn sat(&self) -> SATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SATR { bits }
    }
    #[doc = "Bit 6 - 6:6\\] TDC measurement complete flag. 0: TDC measurement has not yet completed. 1: TDC measurement has completed. This field clears when a new TDC measurement starts or when you write CLR_RESULT to CTL.CMD."]
    #[inline]
    pub fn done(&self) -> DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DONER { bits }
    }
    #[doc = "Bits 0:5 - 5:0\\] TDC state machine status."]
    #[inline]
    pub fn state(&self) -> STATER {
        STATER::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 6 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:31 - 31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved8(&mut self) -> _RESERVED8W {
        _RESERVED8W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] TDC measurement saturation flag. 0: Conversion has not saturated. 1: Conversion stopped due to saturation. This field is cleared when a new measurement is started or when CLR_RESULT is written to CTL.CMD."]
    #[inline]
    pub fn sat(&mut self) -> _SATW {
        _SATW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] TDC measurement complete flag. 0: TDC measurement has not yet completed. 1: TDC measurement has completed. This field clears when a new TDC measurement starts or when you write CLR_RESULT to CTL.CMD."]
    #[inline]
    pub fn done(&mut self) -> _DONEW {
        _DONEW { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\] TDC state machine status."]
    #[inline]
    pub fn state(&mut self) -> _STATEW {
        _STATEW { w: self }
    }
}
