#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSGPOCTL {
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
#[doc = "Possible values of the field `GPOCTL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPOCTL3R {
    #[doc = "RAT GPO line 3"]
    RATGPO3,
    #[doc = "RAT GPO line 2"]
    RATGPO2,
    #[doc = "RAT GPO line 1"]
    RATGPO1,
    #[doc = "RAT GPO line 0"]
    RATGPO0,
    #[doc = "RFE GPO line 3"]
    RFEGPO3,
    #[doc = "RFE GPO line 2"]
    RFEGPO2,
    #[doc = "RFE GPO line 1"]
    RFEGPO1,
    #[doc = "RFE GPO line 0"]
    RFEGPO0,
    #[doc = "MCE GPO line 3"]
    MCEGPO3,
    #[doc = "MCE GPO line 2"]
    MCEGPO2,
    #[doc = "MCE GPO line 1"]
    MCEGPO1,
    #[doc = "MCE GPO line 0"]
    MCEGPO0,
    #[doc = "CPE GPO line 3"]
    CPEGPO3,
    #[doc = "CPE GPO line 2"]
    CPEGPO2,
    #[doc = "CPE GPO line 1"]
    CPEGPO1,
    #[doc = "CPE GPO line 0"]
    CPEGPO0,
}
impl GPOCTL3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPOCTL3R::RATGPO3 => 15,
            GPOCTL3R::RATGPO2 => 14,
            GPOCTL3R::RATGPO1 => 13,
            GPOCTL3R::RATGPO0 => 12,
            GPOCTL3R::RFEGPO3 => 11,
            GPOCTL3R::RFEGPO2 => 10,
            GPOCTL3R::RFEGPO1 => 9,
            GPOCTL3R::RFEGPO0 => 8,
            GPOCTL3R::MCEGPO3 => 7,
            GPOCTL3R::MCEGPO2 => 6,
            GPOCTL3R::MCEGPO1 => 5,
            GPOCTL3R::MCEGPO0 => 4,
            GPOCTL3R::CPEGPO3 => 3,
            GPOCTL3R::CPEGPO2 => 2,
            GPOCTL3R::CPEGPO1 => 1,
            GPOCTL3R::CPEGPO0 => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPOCTL3R {
        match value {
            15 => GPOCTL3R::RATGPO3,
            14 => GPOCTL3R::RATGPO2,
            13 => GPOCTL3R::RATGPO1,
            12 => GPOCTL3R::RATGPO0,
            11 => GPOCTL3R::RFEGPO3,
            10 => GPOCTL3R::RFEGPO2,
            9 => GPOCTL3R::RFEGPO1,
            8 => GPOCTL3R::RFEGPO0,
            7 => GPOCTL3R::MCEGPO3,
            6 => GPOCTL3R::MCEGPO2,
            5 => GPOCTL3R::MCEGPO1,
            4 => GPOCTL3R::MCEGPO0,
            3 => GPOCTL3R::CPEGPO3,
            2 => GPOCTL3R::CPEGPO2,
            1 => GPOCTL3R::CPEGPO1,
            0 => GPOCTL3R::CPEGPO0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RATGPO3`"]
    #[inline]
    pub fn is_ratgpo3(&self) -> bool {
        *self == GPOCTL3R::RATGPO3
    }
    #[doc = "Checks if the value of the field is `RATGPO2`"]
    #[inline]
    pub fn is_ratgpo2(&self) -> bool {
        *self == GPOCTL3R::RATGPO2
    }
    #[doc = "Checks if the value of the field is `RATGPO1`"]
    #[inline]
    pub fn is_ratgpo1(&self) -> bool {
        *self == GPOCTL3R::RATGPO1
    }
    #[doc = "Checks if the value of the field is `RATGPO0`"]
    #[inline]
    pub fn is_ratgpo0(&self) -> bool {
        *self == GPOCTL3R::RATGPO0
    }
    #[doc = "Checks if the value of the field is `RFEGPO3`"]
    #[inline]
    pub fn is_rfegpo3(&self) -> bool {
        *self == GPOCTL3R::RFEGPO3
    }
    #[doc = "Checks if the value of the field is `RFEGPO2`"]
    #[inline]
    pub fn is_rfegpo2(&self) -> bool {
        *self == GPOCTL3R::RFEGPO2
    }
    #[doc = "Checks if the value of the field is `RFEGPO1`"]
    #[inline]
    pub fn is_rfegpo1(&self) -> bool {
        *self == GPOCTL3R::RFEGPO1
    }
    #[doc = "Checks if the value of the field is `RFEGPO0`"]
    #[inline]
    pub fn is_rfegpo0(&self) -> bool {
        *self == GPOCTL3R::RFEGPO0
    }
    #[doc = "Checks if the value of the field is `MCEGPO3`"]
    #[inline]
    pub fn is_mcegpo3(&self) -> bool {
        *self == GPOCTL3R::MCEGPO3
    }
    #[doc = "Checks if the value of the field is `MCEGPO2`"]
    #[inline]
    pub fn is_mcegpo2(&self) -> bool {
        *self == GPOCTL3R::MCEGPO2
    }
    #[doc = "Checks if the value of the field is `MCEGPO1`"]
    #[inline]
    pub fn is_mcegpo1(&self) -> bool {
        *self == GPOCTL3R::MCEGPO1
    }
    #[doc = "Checks if the value of the field is `MCEGPO0`"]
    #[inline]
    pub fn is_mcegpo0(&self) -> bool {
        *self == GPOCTL3R::MCEGPO0
    }
    #[doc = "Checks if the value of the field is `CPEGPO3`"]
    #[inline]
    pub fn is_cpegpo3(&self) -> bool {
        *self == GPOCTL3R::CPEGPO3
    }
    #[doc = "Checks if the value of the field is `CPEGPO2`"]
    #[inline]
    pub fn is_cpegpo2(&self) -> bool {
        *self == GPOCTL3R::CPEGPO2
    }
    #[doc = "Checks if the value of the field is `CPEGPO1`"]
    #[inline]
    pub fn is_cpegpo1(&self) -> bool {
        *self == GPOCTL3R::CPEGPO1
    }
    #[doc = "Checks if the value of the field is `CPEGPO0`"]
    #[inline]
    pub fn is_cpegpo0(&self) -> bool {
        *self == GPOCTL3R::CPEGPO0
    }
}
#[doc = "Possible values of the field `GPOCTL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPOCTL2R {
    #[doc = "RAT GPO line 3"]
    RATGPO3,
    #[doc = "RAT GPO line 2"]
    RATGPO2,
    #[doc = "RAT GPO line 1"]
    RATGPO1,
    #[doc = "RAT GPO line 0"]
    RATGPO0,
    #[doc = "RFE GPO line 3"]
    RFEGPO3,
    #[doc = "RFE GPO line 2"]
    RFEGPO2,
    #[doc = "RFE GPO line 1"]
    RFEGPO1,
    #[doc = "RFE GPO line 0"]
    RFEGPO0,
    #[doc = "MCE GPO line 3"]
    MCEGPO3,
    #[doc = "MCE GPO line 2"]
    MCEGPO2,
    #[doc = "MCE GPO line 1"]
    MCEGPO1,
    #[doc = "MCE GPO line 0"]
    MCEGPO0,
    #[doc = "CPE GPO line 3"]
    CPEGPO3,
    #[doc = "CPE GPO line 2"]
    CPEGPO2,
    #[doc = "CPE GPO line 1"]
    CPEGPO1,
    #[doc = "CPE GPO line 0"]
    CPEGPO0,
}
impl GPOCTL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPOCTL2R::RATGPO3 => 15,
            GPOCTL2R::RATGPO2 => 14,
            GPOCTL2R::RATGPO1 => 13,
            GPOCTL2R::RATGPO0 => 12,
            GPOCTL2R::RFEGPO3 => 11,
            GPOCTL2R::RFEGPO2 => 10,
            GPOCTL2R::RFEGPO1 => 9,
            GPOCTL2R::RFEGPO0 => 8,
            GPOCTL2R::MCEGPO3 => 7,
            GPOCTL2R::MCEGPO2 => 6,
            GPOCTL2R::MCEGPO1 => 5,
            GPOCTL2R::MCEGPO0 => 4,
            GPOCTL2R::CPEGPO3 => 3,
            GPOCTL2R::CPEGPO2 => 2,
            GPOCTL2R::CPEGPO1 => 1,
            GPOCTL2R::CPEGPO0 => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPOCTL2R {
        match value {
            15 => GPOCTL2R::RATGPO3,
            14 => GPOCTL2R::RATGPO2,
            13 => GPOCTL2R::RATGPO1,
            12 => GPOCTL2R::RATGPO0,
            11 => GPOCTL2R::RFEGPO3,
            10 => GPOCTL2R::RFEGPO2,
            9 => GPOCTL2R::RFEGPO1,
            8 => GPOCTL2R::RFEGPO0,
            7 => GPOCTL2R::MCEGPO3,
            6 => GPOCTL2R::MCEGPO2,
            5 => GPOCTL2R::MCEGPO1,
            4 => GPOCTL2R::MCEGPO0,
            3 => GPOCTL2R::CPEGPO3,
            2 => GPOCTL2R::CPEGPO2,
            1 => GPOCTL2R::CPEGPO1,
            0 => GPOCTL2R::CPEGPO0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RATGPO3`"]
    #[inline]
    pub fn is_ratgpo3(&self) -> bool {
        *self == GPOCTL2R::RATGPO3
    }
    #[doc = "Checks if the value of the field is `RATGPO2`"]
    #[inline]
    pub fn is_ratgpo2(&self) -> bool {
        *self == GPOCTL2R::RATGPO2
    }
    #[doc = "Checks if the value of the field is `RATGPO1`"]
    #[inline]
    pub fn is_ratgpo1(&self) -> bool {
        *self == GPOCTL2R::RATGPO1
    }
    #[doc = "Checks if the value of the field is `RATGPO0`"]
    #[inline]
    pub fn is_ratgpo0(&self) -> bool {
        *self == GPOCTL2R::RATGPO0
    }
    #[doc = "Checks if the value of the field is `RFEGPO3`"]
    #[inline]
    pub fn is_rfegpo3(&self) -> bool {
        *self == GPOCTL2R::RFEGPO3
    }
    #[doc = "Checks if the value of the field is `RFEGPO2`"]
    #[inline]
    pub fn is_rfegpo2(&self) -> bool {
        *self == GPOCTL2R::RFEGPO2
    }
    #[doc = "Checks if the value of the field is `RFEGPO1`"]
    #[inline]
    pub fn is_rfegpo1(&self) -> bool {
        *self == GPOCTL2R::RFEGPO1
    }
    #[doc = "Checks if the value of the field is `RFEGPO0`"]
    #[inline]
    pub fn is_rfegpo0(&self) -> bool {
        *self == GPOCTL2R::RFEGPO0
    }
    #[doc = "Checks if the value of the field is `MCEGPO3`"]
    #[inline]
    pub fn is_mcegpo3(&self) -> bool {
        *self == GPOCTL2R::MCEGPO3
    }
    #[doc = "Checks if the value of the field is `MCEGPO2`"]
    #[inline]
    pub fn is_mcegpo2(&self) -> bool {
        *self == GPOCTL2R::MCEGPO2
    }
    #[doc = "Checks if the value of the field is `MCEGPO1`"]
    #[inline]
    pub fn is_mcegpo1(&self) -> bool {
        *self == GPOCTL2R::MCEGPO1
    }
    #[doc = "Checks if the value of the field is `MCEGPO0`"]
    #[inline]
    pub fn is_mcegpo0(&self) -> bool {
        *self == GPOCTL2R::MCEGPO0
    }
    #[doc = "Checks if the value of the field is `CPEGPO3`"]
    #[inline]
    pub fn is_cpegpo3(&self) -> bool {
        *self == GPOCTL2R::CPEGPO3
    }
    #[doc = "Checks if the value of the field is `CPEGPO2`"]
    #[inline]
    pub fn is_cpegpo2(&self) -> bool {
        *self == GPOCTL2R::CPEGPO2
    }
    #[doc = "Checks if the value of the field is `CPEGPO1`"]
    #[inline]
    pub fn is_cpegpo1(&self) -> bool {
        *self == GPOCTL2R::CPEGPO1
    }
    #[doc = "Checks if the value of the field is `CPEGPO0`"]
    #[inline]
    pub fn is_cpegpo0(&self) -> bool {
        *self == GPOCTL2R::CPEGPO0
    }
}
#[doc = "Possible values of the field `GPOCTL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPOCTL1R {
    #[doc = "RAT GPO line 3"]
    RATGPO3,
    #[doc = "RAT GPO line 2"]
    RATGPO2,
    #[doc = "RAT GPO line 1"]
    RATGPO1,
    #[doc = "RAT GPO line 0"]
    RATGPO0,
    #[doc = "RFE GPO line 3"]
    RFEGPO3,
    #[doc = "RFE GPO line 2"]
    RFEGPO2,
    #[doc = "RFE GPO line 1"]
    RFEGPO1,
    #[doc = "RFE GPO line 0"]
    RFEGPO0,
    #[doc = "MCE GPO line 3"]
    MCEGPO3,
    #[doc = "MCE GPO line 2"]
    MCEGPO2,
    #[doc = "MCE GPO line 1"]
    MCEGPO1,
    #[doc = "MCE GPO line 0"]
    MCEGPO0,
    #[doc = "CPE GPO line 3"]
    CPEGPO3,
    #[doc = "CPE GPO line 2"]
    CPEGPO2,
    #[doc = "CPE GPO line 1"]
    CPEGPO1,
    #[doc = "CPE GPO line 0"]
    CPEGPO0,
}
impl GPOCTL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPOCTL1R::RATGPO3 => 15,
            GPOCTL1R::RATGPO2 => 14,
            GPOCTL1R::RATGPO1 => 13,
            GPOCTL1R::RATGPO0 => 12,
            GPOCTL1R::RFEGPO3 => 11,
            GPOCTL1R::RFEGPO2 => 10,
            GPOCTL1R::RFEGPO1 => 9,
            GPOCTL1R::RFEGPO0 => 8,
            GPOCTL1R::MCEGPO3 => 7,
            GPOCTL1R::MCEGPO2 => 6,
            GPOCTL1R::MCEGPO1 => 5,
            GPOCTL1R::MCEGPO0 => 4,
            GPOCTL1R::CPEGPO3 => 3,
            GPOCTL1R::CPEGPO2 => 2,
            GPOCTL1R::CPEGPO1 => 1,
            GPOCTL1R::CPEGPO0 => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPOCTL1R {
        match value {
            15 => GPOCTL1R::RATGPO3,
            14 => GPOCTL1R::RATGPO2,
            13 => GPOCTL1R::RATGPO1,
            12 => GPOCTL1R::RATGPO0,
            11 => GPOCTL1R::RFEGPO3,
            10 => GPOCTL1R::RFEGPO2,
            9 => GPOCTL1R::RFEGPO1,
            8 => GPOCTL1R::RFEGPO0,
            7 => GPOCTL1R::MCEGPO3,
            6 => GPOCTL1R::MCEGPO2,
            5 => GPOCTL1R::MCEGPO1,
            4 => GPOCTL1R::MCEGPO0,
            3 => GPOCTL1R::CPEGPO3,
            2 => GPOCTL1R::CPEGPO2,
            1 => GPOCTL1R::CPEGPO1,
            0 => GPOCTL1R::CPEGPO0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RATGPO3`"]
    #[inline]
    pub fn is_ratgpo3(&self) -> bool {
        *self == GPOCTL1R::RATGPO3
    }
    #[doc = "Checks if the value of the field is `RATGPO2`"]
    #[inline]
    pub fn is_ratgpo2(&self) -> bool {
        *self == GPOCTL1R::RATGPO2
    }
    #[doc = "Checks if the value of the field is `RATGPO1`"]
    #[inline]
    pub fn is_ratgpo1(&self) -> bool {
        *self == GPOCTL1R::RATGPO1
    }
    #[doc = "Checks if the value of the field is `RATGPO0`"]
    #[inline]
    pub fn is_ratgpo0(&self) -> bool {
        *self == GPOCTL1R::RATGPO0
    }
    #[doc = "Checks if the value of the field is `RFEGPO3`"]
    #[inline]
    pub fn is_rfegpo3(&self) -> bool {
        *self == GPOCTL1R::RFEGPO3
    }
    #[doc = "Checks if the value of the field is `RFEGPO2`"]
    #[inline]
    pub fn is_rfegpo2(&self) -> bool {
        *self == GPOCTL1R::RFEGPO2
    }
    #[doc = "Checks if the value of the field is `RFEGPO1`"]
    #[inline]
    pub fn is_rfegpo1(&self) -> bool {
        *self == GPOCTL1R::RFEGPO1
    }
    #[doc = "Checks if the value of the field is `RFEGPO0`"]
    #[inline]
    pub fn is_rfegpo0(&self) -> bool {
        *self == GPOCTL1R::RFEGPO0
    }
    #[doc = "Checks if the value of the field is `MCEGPO3`"]
    #[inline]
    pub fn is_mcegpo3(&self) -> bool {
        *self == GPOCTL1R::MCEGPO3
    }
    #[doc = "Checks if the value of the field is `MCEGPO2`"]
    #[inline]
    pub fn is_mcegpo2(&self) -> bool {
        *self == GPOCTL1R::MCEGPO2
    }
    #[doc = "Checks if the value of the field is `MCEGPO1`"]
    #[inline]
    pub fn is_mcegpo1(&self) -> bool {
        *self == GPOCTL1R::MCEGPO1
    }
    #[doc = "Checks if the value of the field is `MCEGPO0`"]
    #[inline]
    pub fn is_mcegpo0(&self) -> bool {
        *self == GPOCTL1R::MCEGPO0
    }
    #[doc = "Checks if the value of the field is `CPEGPO3`"]
    #[inline]
    pub fn is_cpegpo3(&self) -> bool {
        *self == GPOCTL1R::CPEGPO3
    }
    #[doc = "Checks if the value of the field is `CPEGPO2`"]
    #[inline]
    pub fn is_cpegpo2(&self) -> bool {
        *self == GPOCTL1R::CPEGPO2
    }
    #[doc = "Checks if the value of the field is `CPEGPO1`"]
    #[inline]
    pub fn is_cpegpo1(&self) -> bool {
        *self == GPOCTL1R::CPEGPO1
    }
    #[doc = "Checks if the value of the field is `CPEGPO0`"]
    #[inline]
    pub fn is_cpegpo0(&self) -> bool {
        *self == GPOCTL1R::CPEGPO0
    }
}
#[doc = "Possible values of the field `GPOCTL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPOCTL0R {
    #[doc = "RAT GPO line 3"]
    RATGPO3,
    #[doc = "RAT GPO line 2"]
    RATGPO2,
    #[doc = "RAT GPO line 1"]
    RATGPO1,
    #[doc = "RAT GPO line 0"]
    RATGPO0,
    #[doc = "RFE GPO line 3"]
    RFEGPO3,
    #[doc = "RFE GPO line 2"]
    RFEGPO2,
    #[doc = "RFE GPO line 1"]
    RFEGPO1,
    #[doc = "RFE GPO line 0"]
    RFEGPO0,
    #[doc = "MCE GPO line 3"]
    MCEGPO3,
    #[doc = "MCE GPO line 2"]
    MCEGPO2,
    #[doc = "MCE GPO line 1"]
    MCEGPO1,
    #[doc = "MCE GPO line 0"]
    MCEGPO0,
    #[doc = "CPE GPO line 3"]
    CPEGPO3,
    #[doc = "CPE GPO line 2"]
    CPEGPO2,
    #[doc = "CPE GPO line 1"]
    CPEGPO1,
    #[doc = "CPE GPO line 0"]
    CPEGPO0,
}
impl GPOCTL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPOCTL0R::RATGPO3 => 15,
            GPOCTL0R::RATGPO2 => 14,
            GPOCTL0R::RATGPO1 => 13,
            GPOCTL0R::RATGPO0 => 12,
            GPOCTL0R::RFEGPO3 => 11,
            GPOCTL0R::RFEGPO2 => 10,
            GPOCTL0R::RFEGPO1 => 9,
            GPOCTL0R::RFEGPO0 => 8,
            GPOCTL0R::MCEGPO3 => 7,
            GPOCTL0R::MCEGPO2 => 6,
            GPOCTL0R::MCEGPO1 => 5,
            GPOCTL0R::MCEGPO0 => 4,
            GPOCTL0R::CPEGPO3 => 3,
            GPOCTL0R::CPEGPO2 => 2,
            GPOCTL0R::CPEGPO1 => 1,
            GPOCTL0R::CPEGPO0 => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPOCTL0R {
        match value {
            15 => GPOCTL0R::RATGPO3,
            14 => GPOCTL0R::RATGPO2,
            13 => GPOCTL0R::RATGPO1,
            12 => GPOCTL0R::RATGPO0,
            11 => GPOCTL0R::RFEGPO3,
            10 => GPOCTL0R::RFEGPO2,
            9 => GPOCTL0R::RFEGPO1,
            8 => GPOCTL0R::RFEGPO0,
            7 => GPOCTL0R::MCEGPO3,
            6 => GPOCTL0R::MCEGPO2,
            5 => GPOCTL0R::MCEGPO1,
            4 => GPOCTL0R::MCEGPO0,
            3 => GPOCTL0R::CPEGPO3,
            2 => GPOCTL0R::CPEGPO2,
            1 => GPOCTL0R::CPEGPO1,
            0 => GPOCTL0R::CPEGPO0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RATGPO3`"]
    #[inline]
    pub fn is_ratgpo3(&self) -> bool {
        *self == GPOCTL0R::RATGPO3
    }
    #[doc = "Checks if the value of the field is `RATGPO2`"]
    #[inline]
    pub fn is_ratgpo2(&self) -> bool {
        *self == GPOCTL0R::RATGPO2
    }
    #[doc = "Checks if the value of the field is `RATGPO1`"]
    #[inline]
    pub fn is_ratgpo1(&self) -> bool {
        *self == GPOCTL0R::RATGPO1
    }
    #[doc = "Checks if the value of the field is `RATGPO0`"]
    #[inline]
    pub fn is_ratgpo0(&self) -> bool {
        *self == GPOCTL0R::RATGPO0
    }
    #[doc = "Checks if the value of the field is `RFEGPO3`"]
    #[inline]
    pub fn is_rfegpo3(&self) -> bool {
        *self == GPOCTL0R::RFEGPO3
    }
    #[doc = "Checks if the value of the field is `RFEGPO2`"]
    #[inline]
    pub fn is_rfegpo2(&self) -> bool {
        *self == GPOCTL0R::RFEGPO2
    }
    #[doc = "Checks if the value of the field is `RFEGPO1`"]
    #[inline]
    pub fn is_rfegpo1(&self) -> bool {
        *self == GPOCTL0R::RFEGPO1
    }
    #[doc = "Checks if the value of the field is `RFEGPO0`"]
    #[inline]
    pub fn is_rfegpo0(&self) -> bool {
        *self == GPOCTL0R::RFEGPO0
    }
    #[doc = "Checks if the value of the field is `MCEGPO3`"]
    #[inline]
    pub fn is_mcegpo3(&self) -> bool {
        *self == GPOCTL0R::MCEGPO3
    }
    #[doc = "Checks if the value of the field is `MCEGPO2`"]
    #[inline]
    pub fn is_mcegpo2(&self) -> bool {
        *self == GPOCTL0R::MCEGPO2
    }
    #[doc = "Checks if the value of the field is `MCEGPO1`"]
    #[inline]
    pub fn is_mcegpo1(&self) -> bool {
        *self == GPOCTL0R::MCEGPO1
    }
    #[doc = "Checks if the value of the field is `MCEGPO0`"]
    #[inline]
    pub fn is_mcegpo0(&self) -> bool {
        *self == GPOCTL0R::MCEGPO0
    }
    #[doc = "Checks if the value of the field is `CPEGPO3`"]
    #[inline]
    pub fn is_cpegpo3(&self) -> bool {
        *self == GPOCTL0R::CPEGPO3
    }
    #[doc = "Checks if the value of the field is `CPEGPO2`"]
    #[inline]
    pub fn is_cpegpo2(&self) -> bool {
        *self == GPOCTL0R::CPEGPO2
    }
    #[doc = "Checks if the value of the field is `CPEGPO1`"]
    #[inline]
    pub fn is_cpegpo1(&self) -> bool {
        *self == GPOCTL0R::CPEGPO1
    }
    #[doc = "Checks if the value of the field is `CPEGPO0`"]
    #[inline]
    pub fn is_cpegpo0(&self) -> bool {
        *self == GPOCTL0R::CPEGPO0
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
#[doc = "Values that can be written to the field `GPOCTL3`"]
pub enum GPOCTL3W {
    #[doc = "RAT GPO line 3"]
    RATGPO3,
    #[doc = "RAT GPO line 2"]
    RATGPO2,
    #[doc = "RAT GPO line 1"]
    RATGPO1,
    #[doc = "RAT GPO line 0"]
    RATGPO0,
    #[doc = "RFE GPO line 3"]
    RFEGPO3,
    #[doc = "RFE GPO line 2"]
    RFEGPO2,
    #[doc = "RFE GPO line 1"]
    RFEGPO1,
    #[doc = "RFE GPO line 0"]
    RFEGPO0,
    #[doc = "MCE GPO line 3"]
    MCEGPO3,
    #[doc = "MCE GPO line 2"]
    MCEGPO2,
    #[doc = "MCE GPO line 1"]
    MCEGPO1,
    #[doc = "MCE GPO line 0"]
    MCEGPO0,
    #[doc = "CPE GPO line 3"]
    CPEGPO3,
    #[doc = "CPE GPO line 2"]
    CPEGPO2,
    #[doc = "CPE GPO line 1"]
    CPEGPO1,
    #[doc = "CPE GPO line 0"]
    CPEGPO0,
}
impl GPOCTL3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPOCTL3W::RATGPO3 => 15,
            GPOCTL3W::RATGPO2 => 14,
            GPOCTL3W::RATGPO1 => 13,
            GPOCTL3W::RATGPO0 => 12,
            GPOCTL3W::RFEGPO3 => 11,
            GPOCTL3W::RFEGPO2 => 10,
            GPOCTL3W::RFEGPO1 => 9,
            GPOCTL3W::RFEGPO0 => 8,
            GPOCTL3W::MCEGPO3 => 7,
            GPOCTL3W::MCEGPO2 => 6,
            GPOCTL3W::MCEGPO1 => 5,
            GPOCTL3W::MCEGPO0 => 4,
            GPOCTL3W::CPEGPO3 => 3,
            GPOCTL3W::CPEGPO2 => 2,
            GPOCTL3W::CPEGPO1 => 1,
            GPOCTL3W::CPEGPO0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPOCTL3W<'a> {
    w: &'a mut W,
}
impl<'a> _GPOCTL3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPOCTL3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "RAT GPO line 3"]
    #[inline]
    pub fn ratgpo3(self) -> &'a mut W {
        self.variant(GPOCTL3W::RATGPO3)
    }
    #[doc = "RAT GPO line 2"]
    #[inline]
    pub fn ratgpo2(self) -> &'a mut W {
        self.variant(GPOCTL3W::RATGPO2)
    }
    #[doc = "RAT GPO line 1"]
    #[inline]
    pub fn ratgpo1(self) -> &'a mut W {
        self.variant(GPOCTL3W::RATGPO1)
    }
    #[doc = "RAT GPO line 0"]
    #[inline]
    pub fn ratgpo0(self) -> &'a mut W {
        self.variant(GPOCTL3W::RATGPO0)
    }
    #[doc = "RFE GPO line 3"]
    #[inline]
    pub fn rfegpo3(self) -> &'a mut W {
        self.variant(GPOCTL3W::RFEGPO3)
    }
    #[doc = "RFE GPO line 2"]
    #[inline]
    pub fn rfegpo2(self) -> &'a mut W {
        self.variant(GPOCTL3W::RFEGPO2)
    }
    #[doc = "RFE GPO line 1"]
    #[inline]
    pub fn rfegpo1(self) -> &'a mut W {
        self.variant(GPOCTL3W::RFEGPO1)
    }
    #[doc = "RFE GPO line 0"]
    #[inline]
    pub fn rfegpo0(self) -> &'a mut W {
        self.variant(GPOCTL3W::RFEGPO0)
    }
    #[doc = "MCE GPO line 3"]
    #[inline]
    pub fn mcegpo3(self) -> &'a mut W {
        self.variant(GPOCTL3W::MCEGPO3)
    }
    #[doc = "MCE GPO line 2"]
    #[inline]
    pub fn mcegpo2(self) -> &'a mut W {
        self.variant(GPOCTL3W::MCEGPO2)
    }
    #[doc = "MCE GPO line 1"]
    #[inline]
    pub fn mcegpo1(self) -> &'a mut W {
        self.variant(GPOCTL3W::MCEGPO1)
    }
    #[doc = "MCE GPO line 0"]
    #[inline]
    pub fn mcegpo0(self) -> &'a mut W {
        self.variant(GPOCTL3W::MCEGPO0)
    }
    #[doc = "CPE GPO line 3"]
    #[inline]
    pub fn cpegpo3(self) -> &'a mut W {
        self.variant(GPOCTL3W::CPEGPO3)
    }
    #[doc = "CPE GPO line 2"]
    #[inline]
    pub fn cpegpo2(self) -> &'a mut W {
        self.variant(GPOCTL3W::CPEGPO2)
    }
    #[doc = "CPE GPO line 1"]
    #[inline]
    pub fn cpegpo1(self) -> &'a mut W {
        self.variant(GPOCTL3W::CPEGPO1)
    }
    #[doc = "CPE GPO line 0"]
    #[inline]
    pub fn cpegpo0(self) -> &'a mut W {
        self.variant(GPOCTL3W::CPEGPO0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPOCTL2`"]
pub enum GPOCTL2W {
    #[doc = "RAT GPO line 3"]
    RATGPO3,
    #[doc = "RAT GPO line 2"]
    RATGPO2,
    #[doc = "RAT GPO line 1"]
    RATGPO1,
    #[doc = "RAT GPO line 0"]
    RATGPO0,
    #[doc = "RFE GPO line 3"]
    RFEGPO3,
    #[doc = "RFE GPO line 2"]
    RFEGPO2,
    #[doc = "RFE GPO line 1"]
    RFEGPO1,
    #[doc = "RFE GPO line 0"]
    RFEGPO0,
    #[doc = "MCE GPO line 3"]
    MCEGPO3,
    #[doc = "MCE GPO line 2"]
    MCEGPO2,
    #[doc = "MCE GPO line 1"]
    MCEGPO1,
    #[doc = "MCE GPO line 0"]
    MCEGPO0,
    #[doc = "CPE GPO line 3"]
    CPEGPO3,
    #[doc = "CPE GPO line 2"]
    CPEGPO2,
    #[doc = "CPE GPO line 1"]
    CPEGPO1,
    #[doc = "CPE GPO line 0"]
    CPEGPO0,
}
impl GPOCTL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPOCTL2W::RATGPO3 => 15,
            GPOCTL2W::RATGPO2 => 14,
            GPOCTL2W::RATGPO1 => 13,
            GPOCTL2W::RATGPO0 => 12,
            GPOCTL2W::RFEGPO3 => 11,
            GPOCTL2W::RFEGPO2 => 10,
            GPOCTL2W::RFEGPO1 => 9,
            GPOCTL2W::RFEGPO0 => 8,
            GPOCTL2W::MCEGPO3 => 7,
            GPOCTL2W::MCEGPO2 => 6,
            GPOCTL2W::MCEGPO1 => 5,
            GPOCTL2W::MCEGPO0 => 4,
            GPOCTL2W::CPEGPO3 => 3,
            GPOCTL2W::CPEGPO2 => 2,
            GPOCTL2W::CPEGPO1 => 1,
            GPOCTL2W::CPEGPO0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPOCTL2W<'a> {
    w: &'a mut W,
}
impl<'a> _GPOCTL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPOCTL2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "RAT GPO line 3"]
    #[inline]
    pub fn ratgpo3(self) -> &'a mut W {
        self.variant(GPOCTL2W::RATGPO3)
    }
    #[doc = "RAT GPO line 2"]
    #[inline]
    pub fn ratgpo2(self) -> &'a mut W {
        self.variant(GPOCTL2W::RATGPO2)
    }
    #[doc = "RAT GPO line 1"]
    #[inline]
    pub fn ratgpo1(self) -> &'a mut W {
        self.variant(GPOCTL2W::RATGPO1)
    }
    #[doc = "RAT GPO line 0"]
    #[inline]
    pub fn ratgpo0(self) -> &'a mut W {
        self.variant(GPOCTL2W::RATGPO0)
    }
    #[doc = "RFE GPO line 3"]
    #[inline]
    pub fn rfegpo3(self) -> &'a mut W {
        self.variant(GPOCTL2W::RFEGPO3)
    }
    #[doc = "RFE GPO line 2"]
    #[inline]
    pub fn rfegpo2(self) -> &'a mut W {
        self.variant(GPOCTL2W::RFEGPO2)
    }
    #[doc = "RFE GPO line 1"]
    #[inline]
    pub fn rfegpo1(self) -> &'a mut W {
        self.variant(GPOCTL2W::RFEGPO1)
    }
    #[doc = "RFE GPO line 0"]
    #[inline]
    pub fn rfegpo0(self) -> &'a mut W {
        self.variant(GPOCTL2W::RFEGPO0)
    }
    #[doc = "MCE GPO line 3"]
    #[inline]
    pub fn mcegpo3(self) -> &'a mut W {
        self.variant(GPOCTL2W::MCEGPO3)
    }
    #[doc = "MCE GPO line 2"]
    #[inline]
    pub fn mcegpo2(self) -> &'a mut W {
        self.variant(GPOCTL2W::MCEGPO2)
    }
    #[doc = "MCE GPO line 1"]
    #[inline]
    pub fn mcegpo1(self) -> &'a mut W {
        self.variant(GPOCTL2W::MCEGPO1)
    }
    #[doc = "MCE GPO line 0"]
    #[inline]
    pub fn mcegpo0(self) -> &'a mut W {
        self.variant(GPOCTL2W::MCEGPO0)
    }
    #[doc = "CPE GPO line 3"]
    #[inline]
    pub fn cpegpo3(self) -> &'a mut W {
        self.variant(GPOCTL2W::CPEGPO3)
    }
    #[doc = "CPE GPO line 2"]
    #[inline]
    pub fn cpegpo2(self) -> &'a mut W {
        self.variant(GPOCTL2W::CPEGPO2)
    }
    #[doc = "CPE GPO line 1"]
    #[inline]
    pub fn cpegpo1(self) -> &'a mut W {
        self.variant(GPOCTL2W::CPEGPO1)
    }
    #[doc = "CPE GPO line 0"]
    #[inline]
    pub fn cpegpo0(self) -> &'a mut W {
        self.variant(GPOCTL2W::CPEGPO0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPOCTL1`"]
pub enum GPOCTL1W {
    #[doc = "RAT GPO line 3"]
    RATGPO3,
    #[doc = "RAT GPO line 2"]
    RATGPO2,
    #[doc = "RAT GPO line 1"]
    RATGPO1,
    #[doc = "RAT GPO line 0"]
    RATGPO0,
    #[doc = "RFE GPO line 3"]
    RFEGPO3,
    #[doc = "RFE GPO line 2"]
    RFEGPO2,
    #[doc = "RFE GPO line 1"]
    RFEGPO1,
    #[doc = "RFE GPO line 0"]
    RFEGPO0,
    #[doc = "MCE GPO line 3"]
    MCEGPO3,
    #[doc = "MCE GPO line 2"]
    MCEGPO2,
    #[doc = "MCE GPO line 1"]
    MCEGPO1,
    #[doc = "MCE GPO line 0"]
    MCEGPO0,
    #[doc = "CPE GPO line 3"]
    CPEGPO3,
    #[doc = "CPE GPO line 2"]
    CPEGPO2,
    #[doc = "CPE GPO line 1"]
    CPEGPO1,
    #[doc = "CPE GPO line 0"]
    CPEGPO0,
}
impl GPOCTL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPOCTL1W::RATGPO3 => 15,
            GPOCTL1W::RATGPO2 => 14,
            GPOCTL1W::RATGPO1 => 13,
            GPOCTL1W::RATGPO0 => 12,
            GPOCTL1W::RFEGPO3 => 11,
            GPOCTL1W::RFEGPO2 => 10,
            GPOCTL1W::RFEGPO1 => 9,
            GPOCTL1W::RFEGPO0 => 8,
            GPOCTL1W::MCEGPO3 => 7,
            GPOCTL1W::MCEGPO2 => 6,
            GPOCTL1W::MCEGPO1 => 5,
            GPOCTL1W::MCEGPO0 => 4,
            GPOCTL1W::CPEGPO3 => 3,
            GPOCTL1W::CPEGPO2 => 2,
            GPOCTL1W::CPEGPO1 => 1,
            GPOCTL1W::CPEGPO0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPOCTL1W<'a> {
    w: &'a mut W,
}
impl<'a> _GPOCTL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPOCTL1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "RAT GPO line 3"]
    #[inline]
    pub fn ratgpo3(self) -> &'a mut W {
        self.variant(GPOCTL1W::RATGPO3)
    }
    #[doc = "RAT GPO line 2"]
    #[inline]
    pub fn ratgpo2(self) -> &'a mut W {
        self.variant(GPOCTL1W::RATGPO2)
    }
    #[doc = "RAT GPO line 1"]
    #[inline]
    pub fn ratgpo1(self) -> &'a mut W {
        self.variant(GPOCTL1W::RATGPO1)
    }
    #[doc = "RAT GPO line 0"]
    #[inline]
    pub fn ratgpo0(self) -> &'a mut W {
        self.variant(GPOCTL1W::RATGPO0)
    }
    #[doc = "RFE GPO line 3"]
    #[inline]
    pub fn rfegpo3(self) -> &'a mut W {
        self.variant(GPOCTL1W::RFEGPO3)
    }
    #[doc = "RFE GPO line 2"]
    #[inline]
    pub fn rfegpo2(self) -> &'a mut W {
        self.variant(GPOCTL1W::RFEGPO2)
    }
    #[doc = "RFE GPO line 1"]
    #[inline]
    pub fn rfegpo1(self) -> &'a mut W {
        self.variant(GPOCTL1W::RFEGPO1)
    }
    #[doc = "RFE GPO line 0"]
    #[inline]
    pub fn rfegpo0(self) -> &'a mut W {
        self.variant(GPOCTL1W::RFEGPO0)
    }
    #[doc = "MCE GPO line 3"]
    #[inline]
    pub fn mcegpo3(self) -> &'a mut W {
        self.variant(GPOCTL1W::MCEGPO3)
    }
    #[doc = "MCE GPO line 2"]
    #[inline]
    pub fn mcegpo2(self) -> &'a mut W {
        self.variant(GPOCTL1W::MCEGPO2)
    }
    #[doc = "MCE GPO line 1"]
    #[inline]
    pub fn mcegpo1(self) -> &'a mut W {
        self.variant(GPOCTL1W::MCEGPO1)
    }
    #[doc = "MCE GPO line 0"]
    #[inline]
    pub fn mcegpo0(self) -> &'a mut W {
        self.variant(GPOCTL1W::MCEGPO0)
    }
    #[doc = "CPE GPO line 3"]
    #[inline]
    pub fn cpegpo3(self) -> &'a mut W {
        self.variant(GPOCTL1W::CPEGPO3)
    }
    #[doc = "CPE GPO line 2"]
    #[inline]
    pub fn cpegpo2(self) -> &'a mut W {
        self.variant(GPOCTL1W::CPEGPO2)
    }
    #[doc = "CPE GPO line 1"]
    #[inline]
    pub fn cpegpo1(self) -> &'a mut W {
        self.variant(GPOCTL1W::CPEGPO1)
    }
    #[doc = "CPE GPO line 0"]
    #[inline]
    pub fn cpegpo0(self) -> &'a mut W {
        self.variant(GPOCTL1W::CPEGPO0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPOCTL0`"]
pub enum GPOCTL0W {
    #[doc = "RAT GPO line 3"]
    RATGPO3,
    #[doc = "RAT GPO line 2"]
    RATGPO2,
    #[doc = "RAT GPO line 1"]
    RATGPO1,
    #[doc = "RAT GPO line 0"]
    RATGPO0,
    #[doc = "RFE GPO line 3"]
    RFEGPO3,
    #[doc = "RFE GPO line 2"]
    RFEGPO2,
    #[doc = "RFE GPO line 1"]
    RFEGPO1,
    #[doc = "RFE GPO line 0"]
    RFEGPO0,
    #[doc = "MCE GPO line 3"]
    MCEGPO3,
    #[doc = "MCE GPO line 2"]
    MCEGPO2,
    #[doc = "MCE GPO line 1"]
    MCEGPO1,
    #[doc = "MCE GPO line 0"]
    MCEGPO0,
    #[doc = "CPE GPO line 3"]
    CPEGPO3,
    #[doc = "CPE GPO line 2"]
    CPEGPO2,
    #[doc = "CPE GPO line 1"]
    CPEGPO1,
    #[doc = "CPE GPO line 0"]
    CPEGPO0,
}
impl GPOCTL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPOCTL0W::RATGPO3 => 15,
            GPOCTL0W::RATGPO2 => 14,
            GPOCTL0W::RATGPO1 => 13,
            GPOCTL0W::RATGPO0 => 12,
            GPOCTL0W::RFEGPO3 => 11,
            GPOCTL0W::RFEGPO2 => 10,
            GPOCTL0W::RFEGPO1 => 9,
            GPOCTL0W::RFEGPO0 => 8,
            GPOCTL0W::MCEGPO3 => 7,
            GPOCTL0W::MCEGPO2 => 6,
            GPOCTL0W::MCEGPO1 => 5,
            GPOCTL0W::MCEGPO0 => 4,
            GPOCTL0W::CPEGPO3 => 3,
            GPOCTL0W::CPEGPO2 => 2,
            GPOCTL0W::CPEGPO1 => 1,
            GPOCTL0W::CPEGPO0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPOCTL0W<'a> {
    w: &'a mut W,
}
impl<'a> _GPOCTL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPOCTL0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "RAT GPO line 3"]
    #[inline]
    pub fn ratgpo3(self) -> &'a mut W {
        self.variant(GPOCTL0W::RATGPO3)
    }
    #[doc = "RAT GPO line 2"]
    #[inline]
    pub fn ratgpo2(self) -> &'a mut W {
        self.variant(GPOCTL0W::RATGPO2)
    }
    #[doc = "RAT GPO line 1"]
    #[inline]
    pub fn ratgpo1(self) -> &'a mut W {
        self.variant(GPOCTL0W::RATGPO1)
    }
    #[doc = "RAT GPO line 0"]
    #[inline]
    pub fn ratgpo0(self) -> &'a mut W {
        self.variant(GPOCTL0W::RATGPO0)
    }
    #[doc = "RFE GPO line 3"]
    #[inline]
    pub fn rfegpo3(self) -> &'a mut W {
        self.variant(GPOCTL0W::RFEGPO3)
    }
    #[doc = "RFE GPO line 2"]
    #[inline]
    pub fn rfegpo2(self) -> &'a mut W {
        self.variant(GPOCTL0W::RFEGPO2)
    }
    #[doc = "RFE GPO line 1"]
    #[inline]
    pub fn rfegpo1(self) -> &'a mut W {
        self.variant(GPOCTL0W::RFEGPO1)
    }
    #[doc = "RFE GPO line 0"]
    #[inline]
    pub fn rfegpo0(self) -> &'a mut W {
        self.variant(GPOCTL0W::RFEGPO0)
    }
    #[doc = "MCE GPO line 3"]
    #[inline]
    pub fn mcegpo3(self) -> &'a mut W {
        self.variant(GPOCTL0W::MCEGPO3)
    }
    #[doc = "MCE GPO line 2"]
    #[inline]
    pub fn mcegpo2(self) -> &'a mut W {
        self.variant(GPOCTL0W::MCEGPO2)
    }
    #[doc = "MCE GPO line 1"]
    #[inline]
    pub fn mcegpo1(self) -> &'a mut W {
        self.variant(GPOCTL0W::MCEGPO1)
    }
    #[doc = "MCE GPO line 0"]
    #[inline]
    pub fn mcegpo0(self) -> &'a mut W {
        self.variant(GPOCTL0W::MCEGPO0)
    }
    #[doc = "CPE GPO line 3"]
    #[inline]
    pub fn cpegpo3(self) -> &'a mut W {
        self.variant(GPOCTL0W::CPEGPO3)
    }
    #[doc = "CPE GPO line 2"]
    #[inline]
    pub fn cpegpo2(self) -> &'a mut W {
        self.variant(GPOCTL0W::CPEGPO2)
    }
    #[doc = "CPE GPO line 1"]
    #[inline]
    pub fn cpegpo1(self) -> &'a mut W {
        self.variant(GPOCTL0W::CPEGPO1)
    }
    #[doc = "CPE GPO line 0"]
    #[inline]
    pub fn cpegpo0(self) -> &'a mut W {
        self.variant(GPOCTL0W::CPEGPO0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 12:15 - 15:12\\] RF Core GPO control bit 3. Selects which signal to output on the RF Core GPO line 3."]
    #[inline]
    pub fn gpoctl3(&self) -> GPOCTL3R {
        GPOCTL3R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - 11:8\\] RF Core GPO control bit 2. Selects which signal to output on the RF Core GPO line 2."]
    #[inline]
    pub fn gpoctl2(&self) -> GPOCTL2R {
        GPOCTL2R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - 7:4\\] RF Core GPO control bit 1. Selects which signal to output on the RF Core GPO line 1."]
    #[inline]
    pub fn gpoctl1(&self) -> GPOCTL1R {
        GPOCTL1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:3 - 3:0\\] RF Core GPO control bit 0. Selects which signal to output on the RF Core GPO line 0."]
    #[inline]
    pub fn gpoctl0(&self) -> GPOCTL0R {
        GPOCTL0R::_from({
            const MASK: u8 = 15;
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
    #[doc = "Bits 12:15 - 15:12\\] RF Core GPO control bit 3. Selects which signal to output on the RF Core GPO line 3."]
    #[inline]
    pub fn gpoctl3(&mut self) -> _GPOCTL3W {
        _GPOCTL3W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\] RF Core GPO control bit 2. Selects which signal to output on the RF Core GPO line 2."]
    #[inline]
    pub fn gpoctl2(&mut self) -> _GPOCTL2W {
        _GPOCTL2W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\] RF Core GPO control bit 1. Selects which signal to output on the RF Core GPO line 1."]
    #[inline]
    pub fn gpoctl1(&mut self) -> _GPOCTL1W {
        _GPOCTL1W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] RF Core GPO control bit 0. Selects which signal to output on the RF Core GPO line 0."]
    #[inline]
    pub fn gpoctl0(&mut self) -> _GPOCTL0W {
        _GPOCTL0W { w: self }
    }
}
