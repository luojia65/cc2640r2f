#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTL {
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
pub struct RESERVED2R {
    bits: u32,
}
impl RESERVED2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `CMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDR {
    #[doc = "Force TDC state machine back to IDLE state.\n\nNever write this command while AUX_TDC:STAT.STATE equals CLR_CNT or WAIT_CLR_CNT_DONE."]
    ABORT,
    #[doc = "Asynchronous counter start.\n\nThe counter starts to count when the start event is high. To achieve precise edge-to-edge measurements you must ensure that the start event is low for at least 420 ns after you write this command. "]
    RUN,
    #[doc = "Synchronous counter start.\n\nThe counter looks for the opposite edge of the selected start event before it starts to count when the selected edge occurs. This guarantees an edge-triggered start and is recommended for frequency measurements."]
    RUN_SYNC_START,
    #[doc = "Clear STAT.SAT, STAT.DONE, and RESULT.VALUE. \n\nThis is not needed as prerequisite for a measurement. Reliable clear is only guaranteed from IDLE state."]
    CLR_RESULT,
}
impl CMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMDR::ABORT => 3,
            CMDR::RUN => 2,
            CMDR::RUN_SYNC_START => 1,
            CMDR::CLR_RESULT => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMDR {
        match value {
            3 => CMDR::ABORT,
            2 => CMDR::RUN,
            1 => CMDR::RUN_SYNC_START,
            0 => CMDR::CLR_RESULT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ABORT`"]
    #[inline]
    pub fn is_abort(&self) -> bool {
        *self == CMDR::ABORT
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == CMDR::RUN
    }
    #[doc = "Checks if the value of the field is `RUN_SYNC_START`"]
    #[inline]
    pub fn is_run_sync_start(&self) -> bool {
        *self == CMDR::RUN_SYNC_START
    }
    #[doc = "Checks if the value of the field is `CLR_RESULT`"]
    #[inline]
    pub fn is_clr_result(&self) -> bool {
        *self == CMDR::CLR_RESULT
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED2W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 1073741823;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMD`"]
pub enum CMDW {
    #[doc = "Force TDC state machine back to IDLE state.\n\nNever write this command while AUX_TDC:STAT.STATE equals CLR_CNT or WAIT_CLR_CNT_DONE."]
    ABORT,
    #[doc = "Asynchronous counter start.\n\nThe counter starts to count when the start event is high. To achieve precise edge-to-edge measurements you must ensure that the start event is low for at least 420 ns after you write this command. "]
    RUN,
    #[doc = "Synchronous counter start.\n\nThe counter looks for the opposite edge of the selected start event before it starts to count when the selected edge occurs. This guarantees an edge-triggered start and is recommended for frequency measurements."]
    RUN_SYNC_START,
    #[doc = "Clear STAT.SAT, STAT.DONE, and RESULT.VALUE. \n\nThis is not needed as prerequisite for a measurement. Reliable clear is only guaranteed from IDLE state."]
    CLR_RESULT,
}
impl CMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDW::ABORT => 3,
            CMDW::RUN => 2,
            CMDW::RUN_SYNC_START => 1,
            CMDW::CLR_RESULT => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Force TDC state machine back to IDLE state. Never write this command while AUX_TDC:STAT.STATE equals CLR_CNT or WAIT_CLR_CNT_DONE."]
    #[inline]
    pub fn abort(self) -> &'a mut W {
        self.variant(CMDW::ABORT)
    }
    #[doc = "Asynchronous counter start. The counter starts to count when the start event is high. To achieve precise edge-to-edge measurements you must ensure that the start event is low for at least 420 ns after you write this command."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(CMDW::RUN)
    }
    #[doc = "Synchronous counter start. The counter looks for the opposite edge of the selected start event before it starts to count when the selected edge occurs. This guarantees an edge-triggered start and is recommended for frequency measurements."]
    #[inline]
    pub fn run_sync_start(self) -> &'a mut W {
        self.variant(CMDW::RUN_SYNC_START)
    }
    #[doc = "Clear STAT.SAT, STAT.DONE, and RESULT.VALUE. This is not needed as prerequisite for a measurement. Reliable clear is only guaranteed from IDLE state."]
    #[inline]
    pub fn clr_result(self) -> &'a mut W {
        self.variant(CMDW::CLR_RESULT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 2:31 - 31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: u32 = 1073741823;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED2R { bits }
    }
    #[doc = "Bits 0:1 - 1:0\\] TDC commands."]
    #[inline]
    pub fn cmd(&self) -> CMDR {
        CMDR::_from({
            const MASK: u8 = 3;
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
    #[doc = "Bits 2:31 - 31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&mut self) -> _RESERVED2W {
        _RESERVED2W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] TDC commands."]
    #[inline]
    pub fn cmd(&mut self) -> _CMDW {
        _CMDW { w: self }
    }
}
