#[doc = "Register `ECCCORR` reader"]
pub type R = crate::R<ECCCORR_SPEC>;
#[doc = "Register `ECCCORR` writer"]
pub type W = crate::W<ECCCORR_SPEC>;
#[doc = "Field `ADDR_ECC` reader - ECC error address When an ECC error occurs (for single correction) during a read operation, the ADDR_ECC contains the address that generated the error. ADDR_ECC is reset when the flag error is reset. The embedded Flash memory programs the address in this register only when no ECC error flags are set. This means that only the first address that generated an ECC error is saved. The address in ADDR_ECC is relative to the Flash memory area where the error occurred (user Flash memory, system Flash memory, data area, read-only/OTP area)."]
pub type ADDR_ECC_R = crate::FieldReader<u16>;
#[doc = "Field `BK_ECC` reader - ECC bank flag for corrected ECC error It indicates which bank is concerned by ECC error"]
pub type BK_ECC_R = crate::BitReader;
#[doc = "Field `SYSF_ECC` reader - ECC flag for corrected ECC error in system FLASH It indicates if system Flash memory is concerned by ECC error."]
pub type SYSF_ECC_R = crate::BitReader;
#[doc = "Field `OTP_ECC` reader - OTP ECC error bit This bit is set to 1 when one single ECC correction occurred during the last successful read operation from the read-only/ OTP area. The address of the ECC error is available in ADDR_ECC bitfield."]
pub type OTP_ECC_R = crate::BitReader;
#[doc = "ECC single correction error interrupt enable bit When ECCCIE bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCCIE_A {
    #[doc = "0: no interrupt generated when an ECC single correction error occurs"]
    B_0x0 = 0,
    #[doc = "1: non-secure interrupt generated when an ECC single correction error occurs"]
    B_0x1 = 1,
}
impl From<ECCCIE_A> for bool {
    #[inline(always)]
    fn from(variant: ECCCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCCIE` reader - ECC single correction error interrupt enable bit When ECCCIE bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation."]
pub type ECCCIE_R = crate::BitReader<ECCCIE_A>;
impl ECCCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECCCIE_A {
        match self.bits {
            false => ECCCIE_A::B_0x0,
            true => ECCCIE_A::B_0x1,
        }
    }
    #[doc = "no interrupt generated when an ECC single correction error occurs"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ECCCIE_A::B_0x0
    }
    #[doc = "non-secure interrupt generated when an ECC single correction error occurs"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ECCCIE_A::B_0x1
    }
}
#[doc = "Field `ECCCIE` writer - ECC single correction error interrupt enable bit When ECCCIE bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation."]
pub type ECCCIE_W<'a, REG> = crate::BitWriter<'a, REG, ECCCIE_A>;
impl<'a, REG> ECCCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no interrupt generated when an ECC single correction error occurs"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ECCCIE_A::B_0x0)
    }
    #[doc = "non-secure interrupt generated when an ECC single correction error occurs"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ECCCIE_A::B_0x1)
    }
}
#[doc = "Field `ECCC` reader - ECC correction set by hardware when single ECC error has been detected and corrected. Cleared by writing 1."]
pub type ECCC_R = crate::BitReader;
#[doc = "Field `ECCC` writer - ECC correction set by hardware when single ECC error has been detected and corrected. Cleared by writing 1."]
pub type ECCC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - ECC error address When an ECC error occurs (for single correction) during a read operation, the ADDR_ECC contains the address that generated the error. ADDR_ECC is reset when the flag error is reset. The embedded Flash memory programs the address in this register only when no ECC error flags are set. This means that only the first address that generated an ECC error is saved. The address in ADDR_ECC is relative to the Flash memory area where the error occurred (user Flash memory, system Flash memory, data area, read-only/OTP area)."]
    #[inline(always)]
    pub fn ADDR_ECC(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 22 - ECC bank flag for corrected ECC error It indicates which bank is concerned by ECC error"]
    #[inline(always)]
    pub fn BK_ECC(&self) -> BK_ECC_R {
        BK_ECC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ECC flag for corrected ECC error in system FLASH It indicates if system Flash memory is concerned by ECC error."]
    #[inline(always)]
    pub fn SYSF_ECC(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - OTP ECC error bit This bit is set to 1 when one single ECC correction occurred during the last successful read operation from the read-only/ OTP area. The address of the ECC error is available in ADDR_ECC bitfield."]
    #[inline(always)]
    pub fn OTP_ECC(&self) -> OTP_ECC_R {
        OTP_ECC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ECC single correction error interrupt enable bit When ECCCIE bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation."]
    #[inline(always)]
    pub fn ECCCIE(&self) -> ECCCIE_R {
        ECCCIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - ECC correction set by hardware when single ECC error has been detected and corrected. Cleared by writing 1."]
    #[inline(always)]
    pub fn ECCC(&self) -> ECCC_R {
        ECCC_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - ECC single correction error interrupt enable bit When ECCCIE bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation."]
    #[inline(always)]
    pub fn ECCCIE(&mut self) -> ECCCIE_W<'_, ECCCORR_SPEC> {
        ECCCIE_W::new(self, 25)
    }
    #[doc = "Bit 30 - ECC correction set by hardware when single ECC error has been detected and corrected. Cleared by writing 1."]
    #[inline(always)]
    pub fn ECCC(&mut self) -> ECCC_W<'_, ECCCORR_SPEC> {
        ECCC_W::new(self, 30)
    }
}
#[doc = "FLASH Flash ECC correction register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecccorr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecccorr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCCORR_SPEC;
impl crate::RegisterSpec for ECCCORR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecccorr::R`](R) reader structure"]
impl crate::Readable for ECCCORR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ecccorr::W`](W) writer structure"]
impl crate::Writable for ECCCORR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ECCCORR to value 0"]
impl crate::Resettable for ECCCORR_SPEC {}
