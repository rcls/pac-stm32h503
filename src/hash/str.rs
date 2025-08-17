#[doc = "Register `STR` reader"]
pub type R = crate::R<STR_SPEC>;
#[doc = "Register `STR` writer"]
pub type W = crate::W<STR_SPEC>;
#[doc = "Number of valid bits in the last word When the last word of the message bit string is written to HASH_DIN register, the hash processor takes only the valid bits, specified as below, after internal data swapping: ... The above mechanism is valid only if DCAL = 0. If NBLW bits are written while DCAL is set to 1, the NBLW bitfield remains unchanged. In other words it is not possible to configure NBLW and set DCAL at the same time. Reading NBLW bits returns the last value written to NBLW.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NBLW_A {
    #[doc = "0: All the 32 bits of the last data written are valid message bits, that is M\\[31:0\\]"]
    B_0x00 = 0,
    #[doc = "1: Only one bit of the last data written (after swapping) is valid, that is M\\[0\\]"]
    B_0x01 = 1,
    #[doc = "2: Only two bits of the last data written (after swapping) are valid, that is M\\[1:0\\]"]
    B_0x02 = 2,
    #[doc = "3: Only three bits of the last data written (after swapping) are valid that is M\\[2:0\\]"]
    B_0x03 = 3,
    #[doc = "31: Only 31 bits of the last data written (after swapping) are valid that is M\\[30:0\\]"]
    B_0x1F = 31,
}
impl From<NBLW_A> for u8 {
    #[inline(always)]
    fn from(variant: NBLW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NBLW_A {
    type Ux = u8;
}
impl crate::IsEnum for NBLW_A {}
#[doc = "Field `NBLW` reader - Number of valid bits in the last word When the last word of the message bit string is written to HASH_DIN register, the hash processor takes only the valid bits, specified as below, after internal data swapping: ... The above mechanism is valid only if DCAL = 0. If NBLW bits are written while DCAL is set to 1, the NBLW bitfield remains unchanged. In other words it is not possible to configure NBLW and set DCAL at the same time. Reading NBLW bits returns the last value written to NBLW."]
pub type NBLW_R = crate::FieldReader<NBLW_A>;
impl NBLW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NBLW_A> {
        match self.bits {
            0 => Some(NBLW_A::B_0x00),
            1 => Some(NBLW_A::B_0x01),
            2 => Some(NBLW_A::B_0x02),
            3 => Some(NBLW_A::B_0x03),
            31 => Some(NBLW_A::B_0x1F),
            _ => None,
        }
    }
    #[doc = "All the 32 bits of the last data written are valid message bits, that is M\\[31:0\\]"]
    #[inline(always)]
    pub fn is_B_0x00(&self) -> bool {
        *self == NBLW_A::B_0x00
    }
    #[doc = "Only one bit of the last data written (after swapping) is valid, that is M\\[0\\]"]
    #[inline(always)]
    pub fn is_B_0x01(&self) -> bool {
        *self == NBLW_A::B_0x01
    }
    #[doc = "Only two bits of the last data written (after swapping) are valid, that is M\\[1:0\\]"]
    #[inline(always)]
    pub fn is_B_0x02(&self) -> bool {
        *self == NBLW_A::B_0x02
    }
    #[doc = "Only three bits of the last data written (after swapping) are valid that is M\\[2:0\\]"]
    #[inline(always)]
    pub fn is_B_0x03(&self) -> bool {
        *self == NBLW_A::B_0x03
    }
    #[doc = "Only 31 bits of the last data written (after swapping) are valid that is M\\[30:0\\]"]
    #[inline(always)]
    pub fn is_B_0x1F(&self) -> bool {
        *self == NBLW_A::B_0x1F
    }
}
#[doc = "Field `NBLW` writer - Number of valid bits in the last word When the last word of the message bit string is written to HASH_DIN register, the hash processor takes only the valid bits, specified as below, after internal data swapping: ... The above mechanism is valid only if DCAL = 0. If NBLW bits are written while DCAL is set to 1, the NBLW bitfield remains unchanged. In other words it is not possible to configure NBLW and set DCAL at the same time. Reading NBLW bits returns the last value written to NBLW."]
pub type NBLW_W<'a, REG> = crate::FieldWriter<'a, REG, 5, NBLW_A>;
impl<'a, REG> NBLW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All the 32 bits of the last data written are valid message bits, that is M\\[31:0\\]"]
    #[inline(always)]
    pub fn B_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(NBLW_A::B_0x00)
    }
    #[doc = "Only one bit of the last data written (after swapping) is valid, that is M\\[0\\]"]
    #[inline(always)]
    pub fn B_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(NBLW_A::B_0x01)
    }
    #[doc = "Only two bits of the last data written (after swapping) are valid, that is M\\[1:0\\]"]
    #[inline(always)]
    pub fn B_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(NBLW_A::B_0x02)
    }
    #[doc = "Only three bits of the last data written (after swapping) are valid that is M\\[2:0\\]"]
    #[inline(always)]
    pub fn B_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(NBLW_A::B_0x03)
    }
    #[doc = "Only 31 bits of the last data written (after swapping) are valid that is M\\[30:0\\]"]
    #[inline(always)]
    pub fn B_0x1F(self) -> &'a mut crate::W<REG> {
        self.variant(NBLW_A::B_0x1F)
    }
}
#[doc = "Field `DCAL` reader - Digest calculation Writing this bit to 1 starts the message padding using the previously written value of NBLW, and starts the calculation of the final message digest with all the data words written to the input FIFO since the INIT bit was last written to 1. Reading this bit returns 0."]
pub type DCAL_R = crate::BitReader;
#[doc = "Field `DCAL` writer - Digest calculation Writing this bit to 1 starts the message padding using the previously written value of NBLW, and starts the calculation of the final message digest with all the data words written to the input FIFO since the INIT bit was last written to 1. Reading this bit returns 0."]
pub type DCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Number of valid bits in the last word When the last word of the message bit string is written to HASH_DIN register, the hash processor takes only the valid bits, specified as below, after internal data swapping: ... The above mechanism is valid only if DCAL = 0. If NBLW bits are written while DCAL is set to 1, the NBLW bitfield remains unchanged. In other words it is not possible to configure NBLW and set DCAL at the same time. Reading NBLW bits returns the last value written to NBLW."]
    #[inline(always)]
    pub fn NBLW(&self) -> NBLW_R {
        NBLW_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Digest calculation Writing this bit to 1 starts the message padding using the previously written value of NBLW, and starts the calculation of the final message digest with all the data words written to the input FIFO since the INIT bit was last written to 1. Reading this bit returns 0."]
    #[inline(always)]
    pub fn DCAL(&self) -> DCAL_R {
        DCAL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of valid bits in the last word When the last word of the message bit string is written to HASH_DIN register, the hash processor takes only the valid bits, specified as below, after internal data swapping: ... The above mechanism is valid only if DCAL = 0. If NBLW bits are written while DCAL is set to 1, the NBLW bitfield remains unchanged. In other words it is not possible to configure NBLW and set DCAL at the same time. Reading NBLW bits returns the last value written to NBLW."]
    #[inline(always)]
    pub fn NBLW(&mut self) -> NBLW_W<'_, STR_SPEC> {
        NBLW_W::new(self, 0)
    }
    #[doc = "Bit 8 - Digest calculation Writing this bit to 1 starts the message padding using the previously written value of NBLW, and starts the calculation of the final message digest with all the data words written to the input FIFO since the INIT bit was last written to 1. Reading this bit returns 0."]
    #[inline(always)]
    pub fn DCAL(&mut self) -> DCAL_W<'_, STR_SPEC> {
        DCAL_W::new(self, 8)
    }
}
#[doc = "HASH start register\n\nYou can [`read`](crate::Reg::read) this register and get [`str::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`str::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STR_SPEC;
impl crate::RegisterSpec for STR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`str::R`](R) reader structure"]
impl crate::Readable for STR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`str::W`](W) writer structure"]
impl crate::Writable for STR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets STR to value 0"]
impl crate::Resettable for STR_SPEC {}
