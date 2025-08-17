#[doc = "Register `OPAMP1_OTR` reader"]
pub type R = crate::R<OPAMP1_OTR_SPEC>;
#[doc = "Register `OPAMP1_OTR` writer"]
pub type W = crate::W<OPAMP1_OTR_SPEC>;
#[doc = "Field `TRIMOFFSETN` reader - Trim for NMOS differential pairs"]
pub type TRIMOFFSETN_R = crate::FieldReader;
#[doc = "Field `TRIMOFFSETN` writer - Trim for NMOS differential pairs"]
pub type TRIMOFFSETN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIMOFFSETP` reader - Trim for PMOS differential pairs"]
pub type TRIMOFFSETP_R = crate::FieldReader;
#[doc = "Field `TRIMOFFSETP` writer - Trim for PMOS differential pairs"]
pub type TRIMOFFSETP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Trim for NMOS differential pairs"]
    #[inline(always)]
    pub fn TRIMOFFSETN(&self) -> TRIMOFFSETN_R {
        TRIMOFFSETN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Trim for PMOS differential pairs"]
    #[inline(always)]
    pub fn TRIMOFFSETP(&self) -> TRIMOFFSETP_R {
        TRIMOFFSETP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trim for NMOS differential pairs"]
    #[inline(always)]
    pub fn TRIMOFFSETN(&mut self) -> TRIMOFFSETN_W<'_, OPAMP1_OTR_SPEC> {
        TRIMOFFSETN_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Trim for PMOS differential pairs"]
    #[inline(always)]
    pub fn TRIMOFFSETP(&mut self) -> TRIMOFFSETP_W<'_, OPAMP1_OTR_SPEC> {
        TRIMOFFSETP_W::new(self, 8)
    }
}
#[doc = "OPAMP1 trimming register in normal mode\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp1_otr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_otr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPAMP1_OTR_SPEC;
impl crate::RegisterSpec for OPAMP1_OTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp1_otr::R`](R) reader structure"]
impl crate::Readable for OPAMP1_OTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`opamp1_otr::W`](W) writer structure"]
impl crate::Writable for OPAMP1_OTR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OPAMP1_OTR to value 0"]
impl crate::Resettable for OPAMP1_OTR_SPEC {}
