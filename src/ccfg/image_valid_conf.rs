#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IMAGE_VALID_CONF {
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
pub struct IMAGE_VALIDR {
    bits: u32,
}
impl IMAGE_VALIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _IMAGE_VALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _IMAGE_VALIDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
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
    #[doc = "Bits 0:31 - 31:0\\] This field must have a value of 0x00000000 in order for enabling the boot sequence to transfer control to a flash image. A non-zero value forces the boot sequence to call the boot loader. For CC2640R2: This field must have the address value of the start of the flash vector table in order for enabling the boot sequence to transfer control to a flash image. Any illegal vector table start address value forces the boot sequence to call the boot loader. Note that if any other legal vector table start address value than 0x0 is selected the PRCM:WARMRESET.WR_TO_PINRESET must be set to 1."]
    #[inline]
    pub fn image_valid(&self) -> IMAGE_VALIDR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        IMAGE_VALIDR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4294967295 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:31 - 31:0\\] This field must have a value of 0x00000000 in order for enabling the boot sequence to transfer control to a flash image. A non-zero value forces the boot sequence to call the boot loader. For CC2640R2: This field must have the address value of the start of the flash vector table in order for enabling the boot sequence to transfer control to a flash image. Any illegal vector table start address value forces the boot sequence to call the boot loader. Note that if any other legal vector table start address value than 0x0 is selected the PRCM:WARMRESET.WR_TO_PINRESET must be set to 1."]
    #[inline]
    pub fn image_valid(&mut self) -> _IMAGE_VALIDW {
        _IMAGE_VALIDW { w: self }
    }
}
