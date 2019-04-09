#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SETBURST {
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
pub struct CHNLSR {
    bits: u32,
}
impl CHNLSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CHNLSW<'a> {
    w: &'a mut W,
}
impl<'a> _CHNLSW<'a> {
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
    #[doc = "Bits 0:31 - 31:0\\] Returns the useburst status, or disables individual channels from generating single uDMA requests. The value R is the arbitration rate and stored in the controller data structure. Read as: Bit \\[Ch\\] = 0: uDMA channel Ch responds to both burst and single requests on channel C. The controller performs 2^R, or single, bus transfers. Bit \\[Ch\\] = 1: uDMA channel Ch does not respond to single transfer requests. The controller only responds to burst transfer requests and performs 2^R transfers. Write as: Bit \\[Ch\\] = 0: No effect. Use the CLEARBURST.CHNLS to set bit \\[Ch\\] to 0. Bit \\[Ch\\] = 1: Disables single transfer requests on channel Ch. The controller performs 2^R transfers for burst requests. Writing to a bit where a uDMA channel is not implemented has no effect"]
    #[inline]
    pub fn chnls(&self) -> CHNLSR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        CHNLSR { bits }
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
    #[doc = "Bits 0:31 - 31:0\\] Returns the useburst status, or disables individual channels from generating single uDMA requests. The value R is the arbitration rate and stored in the controller data structure. Read as: Bit \\[Ch\\] = 0: uDMA channel Ch responds to both burst and single requests on channel C. The controller performs 2^R, or single, bus transfers. Bit \\[Ch\\] = 1: uDMA channel Ch does not respond to single transfer requests. The controller only responds to burst transfer requests and performs 2^R transfers. Write as: Bit \\[Ch\\] = 0: No effect. Use the CLEARBURST.CHNLS to set bit \\[Ch\\] to 0. Bit \\[Ch\\] = 1: Disables single transfer requests on channel Ch. The controller performs 2^R transfers for burst requests. Writing to a bit where a uDMA channel is not implemented has no effect"]
    #[inline]
    pub fn chnls(&mut self) -> _CHNLSW {
        _CHNLSW { w: self }
    }
}
