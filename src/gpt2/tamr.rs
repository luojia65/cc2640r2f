#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TAMR {
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
#[doc = "Possible values of the field `TACINTD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TACINTDR {
    #[doc = "Time-out interrupt are disabled"]
    DIS_TO_INTR,
    #[doc = "Time-out interrupt function as normal"]
    EN_TO_INTR,
}
impl TACINTDR {
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
            TACINTDR::DIS_TO_INTR => true,
            TACINTDR::EN_TO_INTR => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TACINTDR {
        match value {
            true => TACINTDR::DIS_TO_INTR,
            false => TACINTDR::EN_TO_INTR,
        }
    }
    #[doc = "Checks if the value of the field is `DIS_TO_INTR`"]
    #[inline]
    pub fn is_dis_to_intr(&self) -> bool {
        *self == TACINTDR::DIS_TO_INTR
    }
    #[doc = "Checks if the value of the field is `EN_TO_INTR`"]
    #[inline]
    pub fn is_en_to_intr(&self) -> bool {
        *self == TACINTDR::EN_TO_INTR
    }
}
#[doc = "Possible values of the field `TAPLO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAPLOR {
    #[doc = "CCP output pin is set to 1 on time-out"]
    CCP_ON_TO,
    #[doc = "Legacy operation"]
    LEGACY,
}
impl TAPLOR {
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
            TAPLOR::CCP_ON_TO => true,
            TAPLOR::LEGACY => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TAPLOR {
        match value {
            true => TAPLOR::CCP_ON_TO,
            false => TAPLOR::LEGACY,
        }
    }
    #[doc = "Checks if the value of the field is `CCP_ON_TO`"]
    #[inline]
    pub fn is_ccp_on_to(&self) -> bool {
        *self == TAPLOR::CCP_ON_TO
    }
    #[doc = "Checks if the value of the field is `LEGACY`"]
    #[inline]
    pub fn is_legacy(&self) -> bool {
        *self == TAPLOR::LEGACY
    }
}
#[doc = "Possible values of the field `TAMRSU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMRSUR {
    #[doc = "Update TAMATCHR and TAPR, if used, on the next time-out. "]
    TOUPDATE,
    #[doc = "Update TAMATCHR and TAPR, if used, on the next cycle."]
    CYCLEUPDATE,
}
impl TAMRSUR {
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
            TAMRSUR::TOUPDATE => true,
            TAMRSUR::CYCLEUPDATE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TAMRSUR {
        match value {
            true => TAMRSUR::TOUPDATE,
            false => TAMRSUR::CYCLEUPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `TOUPDATE`"]
    #[inline]
    pub fn is_toupdate(&self) -> bool {
        *self == TAMRSUR::TOUPDATE
    }
    #[doc = "Checks if the value of the field is `CYCLEUPDATE`"]
    #[inline]
    pub fn is_cycleupdate(&self) -> bool {
        *self == TAMRSUR::CYCLEUPDATE
    }
}
#[doc = "Possible values of the field `TAPWMIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAPWMIER {
    #[doc = "Interrupt is enabled.  This bit is only valid in PWM mode."]
    EN,
    #[doc = "Interrupt is disabled.  "]
    DIS,
}
impl TAPWMIER {
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
            TAPWMIER::EN => true,
            TAPWMIER::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TAPWMIER {
        match value {
            true => TAPWMIER::EN,
            false => TAPWMIER::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TAPWMIER::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TAPWMIER::DIS
    }
}
#[doc = "Possible values of the field `TAILD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAILDR {
    #[doc = "Update the TAR register with the value in the TAILR register on the next timeout. If the prescaler is used, update the TAPS register with the value in the TAPR register on the next timeout."]
    TOUPDATE,
    #[doc = "Update the TAR register with the value in the TAILR register on the next clock cycle. If the pre-scaler is used, update the TAPS register with the value in the TAPR register on the next clock cycle. "]
    CYCLEUPDATE,
}
impl TAILDR {
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
            TAILDR::TOUPDATE => true,
            TAILDR::CYCLEUPDATE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TAILDR {
        match value {
            true => TAILDR::TOUPDATE,
            false => TAILDR::CYCLEUPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `TOUPDATE`"]
    #[inline]
    pub fn is_toupdate(&self) -> bool {
        *self == TAILDR::TOUPDATE
    }
    #[doc = "Checks if the value of the field is `CYCLEUPDATE`"]
    #[inline]
    pub fn is_cycleupdate(&self) -> bool {
        *self == TAILDR::CYCLEUPDATE
    }
}
#[doc = "Possible values of the field `TASNAPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASNAPSR {
    #[doc = "If Timer A is configured in the periodic mode, the actual free-running value of Timer A is loaded at the time-out event into the GPT Timer A (TAR) register."]
    EN,
    #[doc = "Snap-shot mode is disabled.  "]
    DIS,
}
impl TASNAPSR {
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
            TASNAPSR::EN => true,
            TASNAPSR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TASNAPSR {
        match value {
            true => TASNAPSR::EN,
            false => TASNAPSR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TASNAPSR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TASNAPSR::DIS
    }
}
#[doc = "Possible values of the field `TAWOT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAWOTR {
    #[doc = "If Timer A is enabled (CTL.TAEN = 1), Timer A does not begin counting until it receives a trigger from the timer in the previous position in the daisy chain. This bit must be clear for GPT Module 0, Timer A. This function is valid for one-shot, periodic, and PWM modes"]
    WAIT,
    #[doc = "Timer A begins counting as soon as it is enabled."]
    NOWAIT,
}
impl TAWOTR {
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
            TAWOTR::WAIT => true,
            TAWOTR::NOWAIT => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TAWOTR {
        match value {
            true => TAWOTR::WAIT,
            false => TAWOTR::NOWAIT,
        }
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline]
    pub fn is_wait(&self) -> bool {
        *self == TAWOTR::WAIT
    }
    #[doc = "Checks if the value of the field is `NOWAIT`"]
    #[inline]
    pub fn is_nowait(&self) -> bool {
        *self == TAWOTR::NOWAIT
    }
}
#[doc = "Possible values of the field `TAMIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMIER {
    #[doc = "An interrupt is generated when the match value in TAMATCHR is reached in the one-shot and periodic modes."]
    EN,
    #[doc = "The match interrupt is disabled for match events. Additionally, output triggers on match events are prevented."]
    DIS,
}
impl TAMIER {
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
            TAMIER::EN => true,
            TAMIER::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TAMIER {
        match value {
            true => TAMIER::EN,
            false => TAMIER::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TAMIER::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TAMIER::DIS
    }
}
#[doc = "Possible values of the field `TACDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TACDIRR {
    #[doc = "The timer counts up. When counting up, the timer starts from a value of 0x0."]
    UP,
    #[doc = "The timer counts down. "]
    DOWN,
}
impl TACDIRR {
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
            TACDIRR::UP => true,
            TACDIRR::DOWN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TACDIRR {
        match value {
            true => TACDIRR::UP,
            false => TACDIRR::DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline]
    pub fn is_up(&self) -> bool {
        *self == TACDIRR::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline]
    pub fn is_down(&self) -> bool {
        *self == TACDIRR::DOWN
    }
}
#[doc = "Possible values of the field `TAAMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAAMSR {
    #[doc = "PWM mode is enabled"]
    PWM,
    #[doc = "Capture/Compare mode is enabled."]
    CAP_COMP,
}
impl TAAMSR {
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
            TAAMSR::PWM => true,
            TAAMSR::CAP_COMP => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TAAMSR {
        match value {
            true => TAAMSR::PWM,
            false => TAAMSR::CAP_COMP,
        }
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline]
    pub fn is_pwm(&self) -> bool {
        *self == TAAMSR::PWM
    }
    #[doc = "Checks if the value of the field is `CAP_COMP`"]
    #[inline]
    pub fn is_cap_comp(&self) -> bool {
        *self == TAAMSR::CAP_COMP
    }
}
#[doc = "Possible values of the field `TACM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TACMR {
    #[doc = "Edge-Time mode"]
    EDGTIME,
    #[doc = "Edge-Count mode"]
    EDGCNT,
}
impl TACMR {
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
            TACMR::EDGTIME => true,
            TACMR::EDGCNT => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TACMR {
        match value {
            true => TACMR::EDGTIME,
            false => TACMR::EDGCNT,
        }
    }
    #[doc = "Checks if the value of the field is `EDGTIME`"]
    #[inline]
    pub fn is_edgtime(&self) -> bool {
        *self == TACMR::EDGTIME
    }
    #[doc = "Checks if the value of the field is `EDGCNT`"]
    #[inline]
    pub fn is_edgcnt(&self) -> bool {
        *self == TACMR::EDGCNT
    }
}
#[doc = "Possible values of the field `TAMR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMRR {
    #[doc = "Capture mode"]
    CAPTURE,
    #[doc = "Periodic Timer mode "]
    PERIODIC,
    #[doc = "One-Shot Timer mode"]
    ONE_SHOT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TAMRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TAMRR::CAPTURE => 3,
            TAMRR::PERIODIC => 2,
            TAMRR::ONE_SHOT => 1,
            TAMRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TAMRR {
        match value {
            3 => TAMRR::CAPTURE,
            2 => TAMRR::PERIODIC,
            1 => TAMRR::ONE_SHOT,
            i => TAMRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline]
    pub fn is_capture(&self) -> bool {
        *self == TAMRR::CAPTURE
    }
    #[doc = "Checks if the value of the field is `PERIODIC`"]
    #[inline]
    pub fn is_periodic(&self) -> bool {
        *self == TAMRR::PERIODIC
    }
    #[doc = "Checks if the value of the field is `ONE_SHOT`"]
    #[inline]
    pub fn is_one_shot(&self) -> bool {
        *self == TAMRR::ONE_SHOT
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
#[doc = "Values that can be written to the field `TACINTD`"]
pub enum TACINTDW {
    #[doc = "Time-out interrupt are disabled"]
    DIS_TO_INTR,
    #[doc = "Time-out interrupt function as normal"]
    EN_TO_INTR,
}
impl TACINTDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TACINTDW::DIS_TO_INTR => true,
            TACINTDW::EN_TO_INTR => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TACINTDW<'a> {
    w: &'a mut W,
}
impl<'a> _TACINTDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TACINTDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Time-out interrupt are disabled"]
    #[inline]
    pub fn dis_to_intr(self) -> &'a mut W {
        self.variant(TACINTDW::DIS_TO_INTR)
    }
    #[doc = "Time-out interrupt function as normal"]
    #[inline]
    pub fn en_to_intr(self) -> &'a mut W {
        self.variant(TACINTDW::EN_TO_INTR)
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
#[doc = "Values that can be written to the field `TAPLO`"]
pub enum TAPLOW {
    #[doc = "CCP output pin is set to 1 on time-out"]
    CCP_ON_TO,
    #[doc = "Legacy operation"]
    LEGACY,
}
impl TAPLOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TAPLOW::CCP_ON_TO => true,
            TAPLOW::LEGACY => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TAPLOW<'a> {
    w: &'a mut W,
}
impl<'a> _TAPLOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAPLOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCP output pin is set to 1 on time-out"]
    #[inline]
    pub fn ccp_on_to(self) -> &'a mut W {
        self.variant(TAPLOW::CCP_ON_TO)
    }
    #[doc = "Legacy operation"]
    #[inline]
    pub fn legacy(self) -> &'a mut W {
        self.variant(TAPLOW::LEGACY)
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
#[doc = "Values that can be written to the field `TAMRSU`"]
pub enum TAMRSUW {
    #[doc = "Update TAMATCHR and TAPR, if used, on the next time-out. "]
    TOUPDATE,
    #[doc = "Update TAMATCHR and TAPR, if used, on the next cycle."]
    CYCLEUPDATE,
}
impl TAMRSUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TAMRSUW::TOUPDATE => true,
            TAMRSUW::CYCLEUPDATE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TAMRSUW<'a> {
    w: &'a mut W,
}
impl<'a> _TAMRSUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAMRSUW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Update TAMATCHR and TAPR, if used, on the next time-out."]
    #[inline]
    pub fn toupdate(self) -> &'a mut W {
        self.variant(TAMRSUW::TOUPDATE)
    }
    #[doc = "Update TAMATCHR and TAPR, if used, on the next cycle."]
    #[inline]
    pub fn cycleupdate(self) -> &'a mut W {
        self.variant(TAMRSUW::CYCLEUPDATE)
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
#[doc = "Values that can be written to the field `TAPWMIE`"]
pub enum TAPWMIEW {
    #[doc = "Interrupt is enabled.  This bit is only valid in PWM mode."]
    EN,
    #[doc = "Interrupt is disabled.  "]
    DIS,
}
impl TAPWMIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TAPWMIEW::EN => true,
            TAPWMIEW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TAPWMIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TAPWMIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAPWMIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is enabled. This bit is only valid in PWM mode."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TAPWMIEW::EN)
    }
    #[doc = "Interrupt is disabled."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TAPWMIEW::DIS)
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
#[doc = "Values that can be written to the field `TAILD`"]
pub enum TAILDW {
    #[doc = "Update the TAR register with the value in the TAILR register on the next timeout. If the prescaler is used, update the TAPS register with the value in the TAPR register on the next timeout."]
    TOUPDATE,
    #[doc = "Update the TAR register with the value in the TAILR register on the next clock cycle. If the pre-scaler is used, update the TAPS register with the value in the TAPR register on the next clock cycle. "]
    CYCLEUPDATE,
}
impl TAILDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TAILDW::TOUPDATE => true,
            TAILDW::CYCLEUPDATE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TAILDW<'a> {
    w: &'a mut W,
}
impl<'a> _TAILDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAILDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Update the TAR register with the value in the TAILR register on the next timeout. If the prescaler is used, update the TAPS register with the value in the TAPR register on the next timeout."]
    #[inline]
    pub fn toupdate(self) -> &'a mut W {
        self.variant(TAILDW::TOUPDATE)
    }
    #[doc = "Update the TAR register with the value in the TAILR register on the next clock cycle. If the pre-scaler is used, update the TAPS register with the value in the TAPR register on the next clock cycle."]
    #[inline]
    pub fn cycleupdate(self) -> &'a mut W {
        self.variant(TAILDW::CYCLEUPDATE)
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
#[doc = "Values that can be written to the field `TASNAPS`"]
pub enum TASNAPSW {
    #[doc = "If Timer A is configured in the periodic mode, the actual free-running value of Timer A is loaded at the time-out event into the GPT Timer A (TAR) register."]
    EN,
    #[doc = "Snap-shot mode is disabled.  "]
    DIS,
}
impl TASNAPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TASNAPSW::EN => true,
            TASNAPSW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TASNAPSW<'a> {
    w: &'a mut W,
}
impl<'a> _TASNAPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TASNAPSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "If Timer A is configured in the periodic mode, the actual free-running value of Timer A is loaded at the time-out event into the GPT Timer A (TAR) register."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TASNAPSW::EN)
    }
    #[doc = "Snap-shot mode is disabled."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TASNAPSW::DIS)
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
#[doc = "Values that can be written to the field `TAWOT`"]
pub enum TAWOTW {
    #[doc = "If Timer A is enabled (CTL.TAEN = 1), Timer A does not begin counting until it receives a trigger from the timer in the previous position in the daisy chain. This bit must be clear for GPT Module 0, Timer A. This function is valid for one-shot, periodic, and PWM modes"]
    WAIT,
    #[doc = "Timer A begins counting as soon as it is enabled."]
    NOWAIT,
}
impl TAWOTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TAWOTW::WAIT => true,
            TAWOTW::NOWAIT => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TAWOTW<'a> {
    w: &'a mut W,
}
impl<'a> _TAWOTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAWOTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "If Timer A is enabled (CTL.TAEN = 1), Timer A does not begin counting until it receives a trigger from the timer in the previous position in the daisy chain. This bit must be clear for GPT Module 0, Timer A. This function is valid for one-shot, periodic, and PWM modes"]
    #[inline]
    pub fn wait(self) -> &'a mut W {
        self.variant(TAWOTW::WAIT)
    }
    #[doc = "Timer A begins counting as soon as it is enabled."]
    #[inline]
    pub fn nowait(self) -> &'a mut W {
        self.variant(TAWOTW::NOWAIT)
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
#[doc = "Values that can be written to the field `TAMIE`"]
pub enum TAMIEW {
    #[doc = "An interrupt is generated when the match value in TAMATCHR is reached in the one-shot and periodic modes."]
    EN,
    #[doc = "The match interrupt is disabled for match events. Additionally, output triggers on match events are prevented."]
    DIS,
}
impl TAMIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TAMIEW::EN => true,
            TAMIEW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TAMIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TAMIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAMIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An interrupt is generated when the match value in TAMATCHR is reached in the one-shot and periodic modes."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TAMIEW::EN)
    }
    #[doc = "The match interrupt is disabled for match events. Additionally, output triggers on match events are prevented."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TAMIEW::DIS)
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
#[doc = "Values that can be written to the field `TACDIR`"]
pub enum TACDIRW {
    #[doc = "The timer counts up. When counting up, the timer starts from a value of 0x0."]
    UP,
    #[doc = "The timer counts down. "]
    DOWN,
}
impl TACDIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TACDIRW::UP => true,
            TACDIRW::DOWN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TACDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _TACDIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TACDIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The timer counts up. When counting up, the timer starts from a value of 0x0."]
    #[inline]
    pub fn up(self) -> &'a mut W {
        self.variant(TACDIRW::UP)
    }
    #[doc = "The timer counts down."]
    #[inline]
    pub fn down(self) -> &'a mut W {
        self.variant(TACDIRW::DOWN)
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
#[doc = "Values that can be written to the field `TAAMS`"]
pub enum TAAMSW {
    #[doc = "PWM mode is enabled"]
    PWM,
    #[doc = "Capture/Compare mode is enabled."]
    CAP_COMP,
}
impl TAAMSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TAAMSW::PWM => true,
            TAAMSW::CAP_COMP => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TAAMSW<'a> {
    w: &'a mut W,
}
impl<'a> _TAAMSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAAMSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PWM mode is enabled"]
    #[inline]
    pub fn pwm(self) -> &'a mut W {
        self.variant(TAAMSW::PWM)
    }
    #[doc = "Capture/Compare mode is enabled."]
    #[inline]
    pub fn cap_comp(self) -> &'a mut W {
        self.variant(TAAMSW::CAP_COMP)
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
#[doc = "Values that can be written to the field `TACM`"]
pub enum TACMW {
    #[doc = "Edge-Time mode"]
    EDGTIME,
    #[doc = "Edge-Count mode"]
    EDGCNT,
}
impl TACMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TACMW::EDGTIME => true,
            TACMW::EDGCNT => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TACMW<'a> {
    w: &'a mut W,
}
impl<'a> _TACMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TACMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Edge-Time mode"]
    #[inline]
    pub fn edgtime(self) -> &'a mut W {
        self.variant(TACMW::EDGTIME)
    }
    #[doc = "Edge-Count mode"]
    #[inline]
    pub fn edgcnt(self) -> &'a mut W {
        self.variant(TACMW::EDGCNT)
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
#[doc = "Values that can be written to the field `TAMR`"]
pub enum TAMRW {
    #[doc = "Capture mode"]
    CAPTURE,
    #[doc = "Periodic Timer mode "]
    PERIODIC,
    #[doc = "One-Shot Timer mode"]
    ONE_SHOT,
}
impl TAMRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TAMRW::CAPTURE => 3,
            TAMRW::PERIODIC => 2,
            TAMRW::ONE_SHOT => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TAMRW<'a> {
    w: &'a mut W,
}
impl<'a> _TAMRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAMRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Capture mode"]
    #[inline]
    pub fn capture(self) -> &'a mut W {
        self.variant(TAMRW::CAPTURE)
    }
    #[doc = "Periodic Timer mode"]
    #[inline]
    pub fn periodic(self) -> &'a mut W {
        self.variant(TAMRW::PERIODIC)
    }
    #[doc = "One-Shot Timer mode"]
    #[inline]
    pub fn one_shot(self) -> &'a mut W {
        self.variant(TAMRW::ONE_SHOT)
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
    #[doc = "Bit 12 - 12:12\\] One-Shot/Periodic Interrupt Disable"]
    #[inline]
    pub fn tacintd(&self) -> TACINTDR {
        TACINTDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - 11:11\\] GPTM Timer A PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TAILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TAILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode."]
    #[inline]
    pub fn taplo(&self) -> TAPLOR {
        TAPLOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - 10:10\\] Timer A Match Register Update mode This bit defines when the TAMATCHR and TAPR registers are updated. If the timer is disabled (CTL.TAEN = 0) when this bit is set, TAMATCHR and TAPR are updated when the timer is enabled. If the timer is stalled (CTL.TASTALL = 1) when this bit is set, TAMATCHR and TAPR are updated according to the configuration of this bit."]
    #[inline]
    pub fn tamrsu(&self) -> TAMRSUR {
        TAMRSUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - 9:9\\] GPTM Timer A PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TAEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TAOTE bit and the DMAEV.CAEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode."]
    #[inline]
    pub fn tapwmie(&self) -> TAPWMIER {
        TAPWMIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - 8:8\\] GPT Timer A PWM Interval Load Write"]
    #[inline]
    pub fn taild(&self) -> TAILDR {
        TAILDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - 7:7\\] GPT Timer A Snap-Shot Mode"]
    #[inline]
    pub fn tasnaps(&self) -> TASNAPSR {
        TASNAPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - 6:6\\] GPT Timer A Wait-On-Trigger"]
    #[inline]
    pub fn tawot(&self) -> TAWOTR {
        TAWOTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - 5:5\\] GPT Timer A Match Interrupt Enable"]
    #[inline]
    pub fn tamie(&self) -> TAMIER {
        TAMIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - 4:4\\] GPT Timer A Count Direction"]
    #[inline]
    pub fn tacdir(&self) -> TACDIRR {
        TACDIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - 3:3\\] GPT Timer A Alternate Mode Note: To enable PWM mode, you must also clear TACM and then configure TAMR field to 0x2."]
    #[inline]
    pub fn taams(&self) -> TAAMSR {
        TAAMSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - 2:2\\] GPT Timer A Capture Mode"]
    #[inline]
    pub fn tacm(&self) -> TACMR {
        TACMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:1 - 1:0\\] GPT Timer A Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register"]
    #[inline]
    pub fn tamr(&self) -> TAMRR {
        TAMRR::_from({
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
    #[doc = "Bit 12 - 12:12\\] One-Shot/Periodic Interrupt Disable"]
    #[inline]
    pub fn tacintd(&mut self) -> _TACINTDW {
        _TACINTDW { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] GPTM Timer A PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TAILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TAILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode."]
    #[inline]
    pub fn taplo(&mut self) -> _TAPLOW {
        _TAPLOW { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Timer A Match Register Update mode This bit defines when the TAMATCHR and TAPR registers are updated. If the timer is disabled (CTL.TAEN = 0) when this bit is set, TAMATCHR and TAPR are updated when the timer is enabled. If the timer is stalled (CTL.TASTALL = 1) when this bit is set, TAMATCHR and TAPR are updated according to the configuration of this bit."]
    #[inline]
    pub fn tamrsu(&mut self) -> _TAMRSUW {
        _TAMRSUW { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] GPTM Timer A PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TAEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TAOTE bit and the DMAEV.CAEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode."]
    #[inline]
    pub fn tapwmie(&mut self) -> _TAPWMIEW {
        _TAPWMIEW { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] GPT Timer A PWM Interval Load Write"]
    #[inline]
    pub fn taild(&mut self) -> _TAILDW {
        _TAILDW { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] GPT Timer A Snap-Shot Mode"]
    #[inline]
    pub fn tasnaps(&mut self) -> _TASNAPSW {
        _TASNAPSW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] GPT Timer A Wait-On-Trigger"]
    #[inline]
    pub fn tawot(&mut self) -> _TAWOTW {
        _TAWOTW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] GPT Timer A Match Interrupt Enable"]
    #[inline]
    pub fn tamie(&mut self) -> _TAMIEW {
        _TAMIEW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] GPT Timer A Count Direction"]
    #[inline]
    pub fn tacdir(&mut self) -> _TACDIRW {
        _TACDIRW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] GPT Timer A Alternate Mode Note: To enable PWM mode, you must also clear TACM and then configure TAMR field to 0x2."]
    #[inline]
    pub fn taams(&mut self) -> _TAAMSW {
        _TAAMSW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] GPT Timer A Capture Mode"]
    #[inline]
    pub fn tacm(&mut self) -> _TACMW {
        _TACMW { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] GPT Timer A Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register"]
    #[inline]
    pub fn tamr(&mut self) -> _TAMRW {
        _TAMRW { w: self }
    }
}
