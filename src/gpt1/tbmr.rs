#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TBMR {
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
pub struct RESERVED16R {
    bits: u16,
}
impl RESERVED16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `TCACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCACTR {
    #[doc = "Clear CCP output pin immediately and set on Time-Out"]
    CLRSET_ON_TO,
    #[doc = "Set CCP output pin immediately and clear on Time-Out"]
    SETCLR_ON_TO,
    #[doc = "Clear CCP output pin immediately and toggle on Time-Out"]
    CLRTOG_ON_TO,
    #[doc = "Set CCP output pin immediately and toggle on Time-Out"]
    SETTOG_ON_TO,
    #[doc = "Set CCP output pin on Time-Out "]
    SET_ON_TO,
    #[doc = "Clear CCP output pin on Time-Out"]
    CLR_ON_TO,
    #[doc = "Toggle State on Time-Out"]
    TOG_ON_TO,
    #[doc = "Disable compare operations"]
    DIS_CMP,
}
impl TCACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TCACTR::CLRSET_ON_TO => 7,
            TCACTR::SETCLR_ON_TO => 6,
            TCACTR::CLRTOG_ON_TO => 5,
            TCACTR::SETTOG_ON_TO => 4,
            TCACTR::SET_ON_TO => 3,
            TCACTR::CLR_ON_TO => 2,
            TCACTR::TOG_ON_TO => 1,
            TCACTR::DIS_CMP => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TCACTR {
        match value {
            7 => TCACTR::CLRSET_ON_TO,
            6 => TCACTR::SETCLR_ON_TO,
            5 => TCACTR::CLRTOG_ON_TO,
            4 => TCACTR::SETTOG_ON_TO,
            3 => TCACTR::SET_ON_TO,
            2 => TCACTR::CLR_ON_TO,
            1 => TCACTR::TOG_ON_TO,
            0 => TCACTR::DIS_CMP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLRSET_ON_TO`"]
    #[inline]
    pub fn is_clrset_on_to(&self) -> bool {
        *self == TCACTR::CLRSET_ON_TO
    }
    #[doc = "Checks if the value of the field is `SETCLR_ON_TO`"]
    #[inline]
    pub fn is_setclr_on_to(&self) -> bool {
        *self == TCACTR::SETCLR_ON_TO
    }
    #[doc = "Checks if the value of the field is `CLRTOG_ON_TO`"]
    #[inline]
    pub fn is_clrtog_on_to(&self) -> bool {
        *self == TCACTR::CLRTOG_ON_TO
    }
    #[doc = "Checks if the value of the field is `SETTOG_ON_TO`"]
    #[inline]
    pub fn is_settog_on_to(&self) -> bool {
        *self == TCACTR::SETTOG_ON_TO
    }
    #[doc = "Checks if the value of the field is `SET_ON_TO`"]
    #[inline]
    pub fn is_set_on_to(&self) -> bool {
        *self == TCACTR::SET_ON_TO
    }
    #[doc = "Checks if the value of the field is `CLR_ON_TO`"]
    #[inline]
    pub fn is_clr_on_to(&self) -> bool {
        *self == TCACTR::CLR_ON_TO
    }
    #[doc = "Checks if the value of the field is `TOG_ON_TO`"]
    #[inline]
    pub fn is_tog_on_to(&self) -> bool {
        *self == TCACTR::TOG_ON_TO
    }
    #[doc = "Checks if the value of the field is `DIS_CMP`"]
    #[inline]
    pub fn is_dis_cmp(&self) -> bool {
        *self == TCACTR::DIS_CMP
    }
}
#[doc = "Possible values of the field `TBCINTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBCINTDR {
    #[doc = "Mask Time-Out Interrupt"]
    DIS_TO_INTR,
    #[doc = "Normal Time-Out Interrupt "]
    EN_TO_INTR,
}
impl TBCINTDR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TBCINTDR::DIS_TO_INTR => true,
            TBCINTDR::EN_TO_INTR => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBCINTDR {
        match value {
            true => TBCINTDR::DIS_TO_INTR,
            false => TBCINTDR::EN_TO_INTR,
        }
    }
    #[doc = "Checks if the value of the field is `DIS_TO_INTR`"]
    #[inline]
    pub fn is_dis_to_intr(&self) -> bool {
        *self == TBCINTDR::DIS_TO_INTR
    }
    #[doc = "Checks if the value of the field is `EN_TO_INTR`"]
    #[inline]
    pub fn is_en_to_intr(&self) -> bool {
        *self == TBCINTDR::EN_TO_INTR
    }
}
#[doc = "Possible values of the field `TBPLO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBPLOR {
    #[doc = "CCP output pin is set to 1 on time-out"]
    CCP_ON_TO,
    #[doc = "Legacy operation"]
    LEGACY,
}
impl TBPLOR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TBPLOR::CCP_ON_TO => true,
            TBPLOR::LEGACY => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBPLOR {
        match value {
            true => TBPLOR::CCP_ON_TO,
            false => TBPLOR::LEGACY,
        }
    }
    #[doc = "Checks if the value of the field is `CCP_ON_TO`"]
    #[inline]
    pub fn is_ccp_on_to(&self) -> bool {
        *self == TBPLOR::CCP_ON_TO
    }
    #[doc = "Checks if the value of the field is `LEGACY`"]
    #[inline]
    pub fn is_legacy(&self) -> bool {
        *self == TBPLOR::LEGACY
    }
}
#[doc = "Possible values of the field `TBMRSU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBMRSUR {
    #[doc = "Update TBMATCHR and TBPR, if used, on the next time-out. "]
    TOUPDATE,
    #[doc = "Update TBMATCHR and TBPR, if used, on the next cycle. "]
    CYCLEUPDATE,
}
impl TBMRSUR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TBMRSUR::TOUPDATE => true,
            TBMRSUR::CYCLEUPDATE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBMRSUR {
        match value {
            true => TBMRSUR::TOUPDATE,
            false => TBMRSUR::CYCLEUPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `TOUPDATE`"]
    #[inline]
    pub fn is_toupdate(&self) -> bool {
        *self == TBMRSUR::TOUPDATE
    }
    #[doc = "Checks if the value of the field is `CYCLEUPDATE`"]
    #[inline]
    pub fn is_cycleupdate(&self) -> bool {
        *self == TBMRSUR::CYCLEUPDATE
    }
}
#[doc = "Possible values of the field `TBPWMIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBPWMIER {
    #[doc = "Interrupt is enabled.  This bit is only valid in PWM mode."]
    EN,
    #[doc = "Interrupt is disabled.  "]
    DIS,
}
impl TBPWMIER {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TBPWMIER::EN => true,
            TBPWMIER::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBPWMIER {
        match value {
            true => TBPWMIER::EN,
            false => TBPWMIER::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TBPWMIER::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TBPWMIER::DIS
    }
}
#[doc = "Possible values of the field `TBILD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBILDR {
    #[doc = "Update the TBR register with the value in the TBILR register on the next timeout. If the prescaler is used, update the TBPS register with the value in the TBPR register on the next timeout."]
    TOUPDATE,
    #[doc = "Update the TBR register with the value in the TBILR register on the next clock cycle. If the pre-scaler is used, update the TBPS register with the value in the TBPR register on the next clock cycle. "]
    CYCLEUPDATE,
}
impl TBILDR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TBILDR::TOUPDATE => true,
            TBILDR::CYCLEUPDATE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBILDR {
        match value {
            true => TBILDR::TOUPDATE,
            false => TBILDR::CYCLEUPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `TOUPDATE`"]
    #[inline]
    pub fn is_toupdate(&self) -> bool {
        *self == TBILDR::TOUPDATE
    }
    #[doc = "Checks if the value of the field is `CYCLEUPDATE`"]
    #[inline]
    pub fn is_cycleupdate(&self) -> bool {
        *self == TBILDR::CYCLEUPDATE
    }
}
#[doc = "Possible values of the field `TBSNAPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBSNAPSR {
    #[doc = "If Timer B is configured in the periodic mode"]
    EN,
    #[doc = "Snap-shot mode is disabled. "]
    DIS,
}
impl TBSNAPSR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TBSNAPSR::EN => true,
            TBSNAPSR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBSNAPSR {
        match value {
            true => TBSNAPSR::EN,
            false => TBSNAPSR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TBSNAPSR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TBSNAPSR::DIS
    }
}
#[doc = "Possible values of the field `TBWOT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBWOTR {
    #[doc = "If Timer B is enabled (CTL.TBEN is set), Timer B does not begin counting until it receives a trigger from the timer in the previous position in the daisy chain. This function is valid for one-shot, periodic, and PWM modes"]
    WAIT,
    #[doc = "Timer B begins counting as soon as it is enabled. "]
    NOWAIT,
}
impl TBWOTR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TBWOTR::WAIT => true,
            TBWOTR::NOWAIT => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBWOTR {
        match value {
            true => TBWOTR::WAIT,
            false => TBWOTR::NOWAIT,
        }
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline]
    pub fn is_wait(&self) -> bool {
        *self == TBWOTR::WAIT
    }
    #[doc = "Checks if the value of the field is `NOWAIT`"]
    #[inline]
    pub fn is_nowait(&self) -> bool {
        *self == TBWOTR::NOWAIT
    }
}
#[doc = "Possible values of the field `TBMIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBMIER {
    #[doc = "An interrupt is generated when the match value in the TBMATCHR register is reached in the one-shot and periodic modes. "]
    EN,
    #[doc = "The match interrupt is disabled for match events. Additionally, output triggers on match events are prevented."]
    DIS,
}
impl TBMIER {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TBMIER::EN => true,
            TBMIER::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBMIER {
        match value {
            true => TBMIER::EN,
            false => TBMIER::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TBMIER::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TBMIER::DIS
    }
}
#[doc = "Possible values of the field `TBCDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBCDIRR {
    #[doc = "The timer counts up. When counting up, the timer starts from a value of 0x0."]
    UP,
    #[doc = "The timer counts down. "]
    DOWN,
}
impl TBCDIRR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TBCDIRR::UP => true,
            TBCDIRR::DOWN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBCDIRR {
        match value {
            true => TBCDIRR::UP,
            false => TBCDIRR::DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline]
    pub fn is_up(&self) -> bool {
        *self == TBCDIRR::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline]
    pub fn is_down(&self) -> bool {
        *self == TBCDIRR::DOWN
    }
}
#[doc = "Possible values of the field `TBAMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBAMSR {
    #[doc = "PWM mode is enabled"]
    PWM,
    #[doc = "Capture/Compare mode is enabled."]
    CAP_COMP,
}
impl TBAMSR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TBAMSR::PWM => true,
            TBAMSR::CAP_COMP => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBAMSR {
        match value {
            true => TBAMSR::PWM,
            false => TBAMSR::CAP_COMP,
        }
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline]
    pub fn is_pwm(&self) -> bool {
        *self == TBAMSR::PWM
    }
    #[doc = "Checks if the value of the field is `CAP_COMP`"]
    #[inline]
    pub fn is_cap_comp(&self) -> bool {
        *self == TBAMSR::CAP_COMP
    }
}
#[doc = "Possible values of the field `TBCM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBCMR {
    #[doc = "Edge-Time mode"]
    EDGTIME,
    #[doc = "Edge-Count mode"]
    EDGCNT,
}
impl TBCMR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TBCMR::EDGTIME => true,
            TBCMR::EDGCNT => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBCMR {
        match value {
            true => TBCMR::EDGTIME,
            false => TBCMR::EDGCNT,
        }
    }
    #[doc = "Checks if the value of the field is `EDGTIME`"]
    #[inline]
    pub fn is_edgtime(&self) -> bool {
        *self == TBCMR::EDGTIME
    }
    #[doc = "Checks if the value of the field is `EDGCNT`"]
    #[inline]
    pub fn is_edgcnt(&self) -> bool {
        *self == TBCMR::EDGCNT
    }
}
#[doc = "Possible values of the field `TBMR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBMRR {
    #[doc = "Capture mode"]
    CAPTURE,
    #[doc = "Periodic Timer mode "]
    PERIODIC,
    #[doc = "One-Shot Timer mode"]
    ONE_SHOT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TBMRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TBMRR::CAPTURE => 3,
            TBMRR::PERIODIC => 2,
            TBMRR::ONE_SHOT => 1,
            TBMRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TBMRR {
        match value {
            3 => TBMRR::CAPTURE,
            2 => TBMRR::PERIODIC,
            1 => TBMRR::ONE_SHOT,
            i => TBMRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline]
    pub fn is_capture(&self) -> bool {
        *self == TBMRR::CAPTURE
    }
    #[doc = "Checks if the value of the field is `PERIODIC`"]
    #[inline]
    pub fn is_periodic(&self) -> bool {
        *self == TBMRR::PERIODIC
    }
    #[doc = "Checks if the value of the field is `ONE_SHOT`"]
    #[inline]
    pub fn is_one_shot(&self) -> bool {
        *self == TBMRR::ONE_SHOT
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED16W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED16W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TCACT`"]
pub enum TCACTW {
    #[doc = "Clear CCP output pin immediately and set on Time-Out"]
    CLRSET_ON_TO,
    #[doc = "Set CCP output pin immediately and clear on Time-Out"]
    SETCLR_ON_TO,
    #[doc = "Clear CCP output pin immediately and toggle on Time-Out"]
    CLRTOG_ON_TO,
    #[doc = "Set CCP output pin immediately and toggle on Time-Out"]
    SETTOG_ON_TO,
    #[doc = "Set CCP output pin on Time-Out "]
    SET_ON_TO,
    #[doc = "Clear CCP output pin on Time-Out"]
    CLR_ON_TO,
    #[doc = "Toggle State on Time-Out"]
    TOG_ON_TO,
    #[doc = "Disable compare operations"]
    DIS_CMP,
}
impl TCACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TCACTW::CLRSET_ON_TO => 7,
            TCACTW::SETCLR_ON_TO => 6,
            TCACTW::CLRTOG_ON_TO => 5,
            TCACTW::SETTOG_ON_TO => 4,
            TCACTW::SET_ON_TO => 3,
            TCACTW::CLR_ON_TO => 2,
            TCACTW::TOG_ON_TO => 1,
            TCACTW::DIS_CMP => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCACTW<'a> {
    w: &'a mut W,
}
impl<'a> _TCACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCACTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Clear CCP output pin immediately and set on Time-Out"]
    #[inline]
    pub fn clrset_on_to(self) -> &'a mut W {
        self.variant(TCACTW::CLRSET_ON_TO)
    }
    #[doc = "Set CCP output pin immediately and clear on Time-Out"]
    #[inline]
    pub fn setclr_on_to(self) -> &'a mut W {
        self.variant(TCACTW::SETCLR_ON_TO)
    }
    #[doc = "Clear CCP output pin immediately and toggle on Time-Out"]
    #[inline]
    pub fn clrtog_on_to(self) -> &'a mut W {
        self.variant(TCACTW::CLRTOG_ON_TO)
    }
    #[doc = "Set CCP output pin immediately and toggle on Time-Out"]
    #[inline]
    pub fn settog_on_to(self) -> &'a mut W {
        self.variant(TCACTW::SETTOG_ON_TO)
    }
    #[doc = "Set CCP output pin on Time-Out"]
    #[inline]
    pub fn set_on_to(self) -> &'a mut W {
        self.variant(TCACTW::SET_ON_TO)
    }
    #[doc = "Clear CCP output pin on Time-Out"]
    #[inline]
    pub fn clr_on_to(self) -> &'a mut W {
        self.variant(TCACTW::CLR_ON_TO)
    }
    #[doc = "Toggle State on Time-Out"]
    #[inline]
    pub fn tog_on_to(self) -> &'a mut W {
        self.variant(TCACTW::TOG_ON_TO)
    }
    #[doc = "Disable compare operations"]
    #[inline]
    pub fn dis_cmp(self) -> &'a mut W {
        self.variant(TCACTW::DIS_CMP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TBCINTD`"]
pub enum TBCINTDW {
    #[doc = "Mask Time-Out Interrupt"]
    DIS_TO_INTR,
    #[doc = "Normal Time-Out Interrupt "]
    EN_TO_INTR,
}
impl TBCINTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBCINTDW::DIS_TO_INTR => true,
            TBCINTDW::EN_TO_INTR => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBCINTDW<'a> {
    w: &'a mut W,
}
impl<'a> _TBCINTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBCINTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Mask Time-Out Interrupt"]
    #[inline]
    pub fn dis_to_intr(self) -> &'a mut W {
        self.variant(TBCINTDW::DIS_TO_INTR)
    }
    #[doc = "Normal Time-Out Interrupt"]
    #[inline]
    pub fn en_to_intr(self) -> &'a mut W {
        self.variant(TBCINTDW::EN_TO_INTR)
    }
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
#[doc = "Values that can be written to the field `TBPLO`"]
pub enum TBPLOW {
    #[doc = "CCP output pin is set to 1 on time-out"]
    CCP_ON_TO,
    #[doc = "Legacy operation"]
    LEGACY,
}
impl TBPLOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBPLOW::CCP_ON_TO => true,
            TBPLOW::LEGACY => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBPLOW<'a> {
    w: &'a mut W,
}
impl<'a> _TBPLOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBPLOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCP output pin is set to 1 on time-out"]
    #[inline]
    pub fn ccp_on_to(self) -> &'a mut W {
        self.variant(TBPLOW::CCP_ON_TO)
    }
    #[doc = "Legacy operation"]
    #[inline]
    pub fn legacy(self) -> &'a mut W {
        self.variant(TBPLOW::LEGACY)
    }
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
#[doc = "Values that can be written to the field `TBMRSU`"]
pub enum TBMRSUW {
    #[doc = "Update TBMATCHR and TBPR, if used, on the next time-out. "]
    TOUPDATE,
    #[doc = "Update TBMATCHR and TBPR, if used, on the next cycle. "]
    CYCLEUPDATE,
}
impl TBMRSUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBMRSUW::TOUPDATE => true,
            TBMRSUW::CYCLEUPDATE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBMRSUW<'a> {
    w: &'a mut W,
}
impl<'a> _TBMRSUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBMRSUW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Update TBMATCHR and TBPR, if used, on the next time-out."]
    #[inline]
    pub fn toupdate(self) -> &'a mut W {
        self.variant(TBMRSUW::TOUPDATE)
    }
    #[doc = "Update TBMATCHR and TBPR, if used, on the next cycle."]
    #[inline]
    pub fn cycleupdate(self) -> &'a mut W {
        self.variant(TBMRSUW::CYCLEUPDATE)
    }
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
#[doc = "Values that can be written to the field `TBPWMIE`"]
pub enum TBPWMIEW {
    #[doc = "Interrupt is enabled.  This bit is only valid in PWM mode."]
    EN,
    #[doc = "Interrupt is disabled.  "]
    DIS,
}
impl TBPWMIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBPWMIEW::EN => true,
            TBPWMIEW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBPWMIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TBPWMIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBPWMIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is enabled. This bit is only valid in PWM mode."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TBPWMIEW::EN)
    }
    #[doc = "Interrupt is disabled."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TBPWMIEW::DIS)
    }
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
#[doc = "Values that can be written to the field `TBILD`"]
pub enum TBILDW {
    #[doc = "Update the TBR register with the value in the TBILR register on the next timeout. If the prescaler is used, update the TBPS register with the value in the TBPR register on the next timeout."]
    TOUPDATE,
    #[doc = "Update the TBR register with the value in the TBILR register on the next clock cycle. If the pre-scaler is used, update the TBPS register with the value in the TBPR register on the next clock cycle. "]
    CYCLEUPDATE,
}
impl TBILDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBILDW::TOUPDATE => true,
            TBILDW::CYCLEUPDATE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBILDW<'a> {
    w: &'a mut W,
}
impl<'a> _TBILDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBILDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Update the TBR register with the value in the TBILR register on the next timeout. If the prescaler is used, update the TBPS register with the value in the TBPR register on the next timeout."]
    #[inline]
    pub fn toupdate(self) -> &'a mut W {
        self.variant(TBILDW::TOUPDATE)
    }
    #[doc = "Update the TBR register with the value in the TBILR register on the next clock cycle. If the pre-scaler is used, update the TBPS register with the value in the TBPR register on the next clock cycle."]
    #[inline]
    pub fn cycleupdate(self) -> &'a mut W {
        self.variant(TBILDW::CYCLEUPDATE)
    }
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
#[doc = "Values that can be written to the field `TBSNAPS`"]
pub enum TBSNAPSW {
    #[doc = "If Timer B is configured in the periodic mode"]
    EN,
    #[doc = "Snap-shot mode is disabled. "]
    DIS,
}
impl TBSNAPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBSNAPSW::EN => true,
            TBSNAPSW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBSNAPSW<'a> {
    w: &'a mut W,
}
impl<'a> _TBSNAPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBSNAPSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "If Timer B is configured in the periodic mode"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TBSNAPSW::EN)
    }
    #[doc = "Snap-shot mode is disabled."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TBSNAPSW::DIS)
    }
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
#[doc = "Values that can be written to the field `TBWOT`"]
pub enum TBWOTW {
    #[doc = "If Timer B is enabled (CTL.TBEN is set), Timer B does not begin counting until it receives a trigger from the timer in the previous position in the daisy chain. This function is valid for one-shot, periodic, and PWM modes"]
    WAIT,
    #[doc = "Timer B begins counting as soon as it is enabled. "]
    NOWAIT,
}
impl TBWOTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBWOTW::WAIT => true,
            TBWOTW::NOWAIT => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBWOTW<'a> {
    w: &'a mut W,
}
impl<'a> _TBWOTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBWOTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "If Timer B is enabled (CTL.TBEN is set), Timer B does not begin counting until it receives a trigger from the timer in the previous position in the daisy chain. This function is valid for one-shot, periodic, and PWM modes"]
    #[inline]
    pub fn wait(self) -> &'a mut W {
        self.variant(TBWOTW::WAIT)
    }
    #[doc = "Timer B begins counting as soon as it is enabled."]
    #[inline]
    pub fn nowait(self) -> &'a mut W {
        self.variant(TBWOTW::NOWAIT)
    }
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
#[doc = "Values that can be written to the field `TBMIE`"]
pub enum TBMIEW {
    #[doc = "An interrupt is generated when the match value in the TBMATCHR register is reached in the one-shot and periodic modes. "]
    EN,
    #[doc = "The match interrupt is disabled for match events. Additionally, output triggers on match events are prevented."]
    DIS,
}
impl TBMIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBMIEW::EN => true,
            TBMIEW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBMIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TBMIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBMIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An interrupt is generated when the match value in the TBMATCHR register is reached in the one-shot and periodic modes."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TBMIEW::EN)
    }
    #[doc = "The match interrupt is disabled for match events. Additionally, output triggers on match events are prevented."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TBMIEW::DIS)
    }
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
#[doc = "Values that can be written to the field `TBCDIR`"]
pub enum TBCDIRW {
    #[doc = "The timer counts up. When counting up, the timer starts from a value of 0x0."]
    UP,
    #[doc = "The timer counts down. "]
    DOWN,
}
impl TBCDIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBCDIRW::UP => true,
            TBCDIRW::DOWN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBCDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _TBCDIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBCDIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The timer counts up. When counting up, the timer starts from a value of 0x0."]
    #[inline]
    pub fn up(self) -> &'a mut W {
        self.variant(TBCDIRW::UP)
    }
    #[doc = "The timer counts down."]
    #[inline]
    pub fn down(self) -> &'a mut W {
        self.variant(TBCDIRW::DOWN)
    }
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
#[doc = "Values that can be written to the field `TBAMS`"]
pub enum TBAMSW {
    #[doc = "PWM mode is enabled"]
    PWM,
    #[doc = "Capture/Compare mode is enabled."]
    CAP_COMP,
}
impl TBAMSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBAMSW::PWM => true,
            TBAMSW::CAP_COMP => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBAMSW<'a> {
    w: &'a mut W,
}
impl<'a> _TBAMSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBAMSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PWM mode is enabled"]
    #[inline]
    pub fn pwm(self) -> &'a mut W {
        self.variant(TBAMSW::PWM)
    }
    #[doc = "Capture/Compare mode is enabled."]
    #[inline]
    pub fn cap_comp(self) -> &'a mut W {
        self.variant(TBAMSW::CAP_COMP)
    }
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
#[doc = "Values that can be written to the field `TBCM`"]
pub enum TBCMW {
    #[doc = "Edge-Time mode"]
    EDGTIME,
    #[doc = "Edge-Count mode"]
    EDGCNT,
}
impl TBCMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBCMW::EDGTIME => true,
            TBCMW::EDGCNT => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBCMW<'a> {
    w: &'a mut W,
}
impl<'a> _TBCMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBCMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Edge-Time mode"]
    #[inline]
    pub fn edgtime(self) -> &'a mut W {
        self.variant(TBCMW::EDGTIME)
    }
    #[doc = "Edge-Count mode"]
    #[inline]
    pub fn edgcnt(self) -> &'a mut W {
        self.variant(TBCMW::EDGCNT)
    }
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
#[doc = "Values that can be written to the field `TBMR`"]
pub enum TBMRW {
    #[doc = "Capture mode"]
    CAPTURE,
    #[doc = "Periodic Timer mode "]
    PERIODIC,
    #[doc = "One-Shot Timer mode"]
    ONE_SHOT,
}
impl TBMRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TBMRW::CAPTURE => 3,
            TBMRW::PERIODIC => 2,
            TBMRW::ONE_SHOT => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBMRW<'a> {
    w: &'a mut W,
}
impl<'a> _TBMRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBMRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Capture mode"]
    #[inline]
    pub fn capture(self) -> &'a mut W {
        self.variant(TBMRW::CAPTURE)
    }
    #[doc = "Periodic Timer mode"]
    #[inline]
    pub fn periodic(self) -> &'a mut W {
        self.variant(TBMRW::PERIODIC)
    }
    #[doc = "One-Shot Timer mode"]
    #[inline]
    pub fn one_shot(self) -> &'a mut W {
        self.variant(TBMRW::ONE_SHOT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 16:31 - 31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved16(&self) -> RESERVED16R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED16R { bits }
    }
    #[doc = "Bits 13:15 - 15:13\\] Timer Compare Action Select"]
    #[inline]
    pub fn tcact(&self) -> TCACTR {
        TCACTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - 12:12\\] One-Shot/Periodic Interrupt Mode"]
    #[inline]
    pub fn tbcintd(&self) -> TBCINTDR {
        TBCINTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - 11:11\\] GPTM Timer B PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TBILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TBILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode."]
    #[inline]
    pub fn tbplo(&self) -> TBPLOR {
        TBPLOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - 10:10\\] Timer B Match Register Update mode This bit defines when the TBMATCHR and TBPR registers are updated If the timer is disabled (CTL.TBEN is clear) when this bit is set, TBMATCHR and TBPR are updated when the timer is enabled. If the timer is stalled (CTL.TBSTALL is set) when this bit is set, TBMATCHR and TBPR are updated according to the configuration of this bit."]
    #[inline]
    pub fn tbmrsu(&self) -> TBMRSUR {
        TBMRSUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - 9:9\\] GPTM Timer B PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TBEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TBOTE bit and the DMAEV.CBEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode."]
    #[inline]
    pub fn tbpwmie(&self) -> TBPWMIER {
        TBPWMIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - 8:8\\] GPT Timer B PWM Interval Load Write"]
    #[inline]
    pub fn tbild(&self) -> TBILDR {
        TBILDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - 7:7\\] GPT Timer B Snap-Shot Mode"]
    #[inline]
    pub fn tbsnaps(&self) -> TBSNAPSR {
        TBSNAPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - 6:6\\] GPT Timer B Wait-On-Trigger"]
    #[inline]
    pub fn tbwot(&self) -> TBWOTR {
        TBWOTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - 5:5\\] GPT Timer B Match Interrupt Enable."]
    #[inline]
    pub fn tbmie(&self) -> TBMIER {
        TBMIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - 4:4\\] GPT Timer B Count Direction"]
    #[inline]
    pub fn tbcdir(&self) -> TBCDIRR {
        TBCDIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - 3:3\\] GPT Timer B Alternate Mode Note: To enable PWM mode, you must also clear TBCM bit and configure TBMR field to 0x2."]
    #[inline]
    pub fn tbams(&self) -> TBAMSR {
        TBAMSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - 2:2\\] GPT Timer B Capture Mode"]
    #[inline]
    pub fn tbcm(&self) -> TBCMR {
        TBCMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:1 - 1:0\\] GPT Timer B Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register"]
    #[inline]
    pub fn tbmr(&self) -> TBMRR {
        TBMRR::_from({
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
    #[doc = "Bits 16:31 - 31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved16(&mut self) -> _RESERVED16W {
        _RESERVED16W { w: self }
    }
    #[doc = "Bits 13:15 - 15:13\\] Timer Compare Action Select"]
    #[inline]
    pub fn tcact(&mut self) -> _TCACTW {
        _TCACTW { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] One-Shot/Periodic Interrupt Mode"]
    #[inline]
    pub fn tbcintd(&mut self) -> _TBCINTDW {
        _TBCINTDW { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] GPTM Timer B PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TBILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TBILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode."]
    #[inline]
    pub fn tbplo(&mut self) -> _TBPLOW {
        _TBPLOW { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Timer B Match Register Update mode This bit defines when the TBMATCHR and TBPR registers are updated If the timer is disabled (CTL.TBEN is clear) when this bit is set, TBMATCHR and TBPR are updated when the timer is enabled. If the timer is stalled (CTL.TBSTALL is set) when this bit is set, TBMATCHR and TBPR are updated according to the configuration of this bit."]
    #[inline]
    pub fn tbmrsu(&mut self) -> _TBMRSUW {
        _TBMRSUW { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] GPTM Timer B PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TBEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TBOTE bit and the DMAEV.CBEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode."]
    #[inline]
    pub fn tbpwmie(&mut self) -> _TBPWMIEW {
        _TBPWMIEW { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] GPT Timer B PWM Interval Load Write"]
    #[inline]
    pub fn tbild(&mut self) -> _TBILDW {
        _TBILDW { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] GPT Timer B Snap-Shot Mode"]
    #[inline]
    pub fn tbsnaps(&mut self) -> _TBSNAPSW {
        _TBSNAPSW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] GPT Timer B Wait-On-Trigger"]
    #[inline]
    pub fn tbwot(&mut self) -> _TBWOTW {
        _TBWOTW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] GPT Timer B Match Interrupt Enable."]
    #[inline]
    pub fn tbmie(&mut self) -> _TBMIEW {
        _TBMIEW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] GPT Timer B Count Direction"]
    #[inline]
    pub fn tbcdir(&mut self) -> _TBCDIRW {
        _TBCDIRW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] GPT Timer B Alternate Mode Note: To enable PWM mode, you must also clear TBCM bit and configure TBMR field to 0x2."]
    #[inline]
    pub fn tbams(&mut self) -> _TBAMSW {
        _TBAMSW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] GPT Timer B Capture Mode"]
    #[inline]
    pub fn tbcm(&mut self) -> _TBCMW {
        _TBCMW { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] GPT Timer B Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register"]
    #[inline]
    pub fn tbmr(&mut self) -> _TBMRW {
        _TBMRW { w: self }
    }
}
