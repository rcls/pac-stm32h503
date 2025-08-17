#[doc = "Register `OPAMP1_HSOTR` reader"]
pub type R = crate::R<OPAMP1_HSOTR_SPEC>;
#[doc = "Register `OPAMP1_HSOTR` writer"]
pub type W = crate::W<OPAMP1_HSOTR_SPEC>;
#[doc = "Field `TRIMHSOFFSETN` reader - High-speed mode trim for NMOS differential pairs"]
pub type TRIMHSOFFSETN_R = crate::FieldReader;
#[doc = "Field `TRIMHSOFFSETN` writer - High-speed mode trim for NMOS differential pairs"]
pub type TRIMHSOFFSETN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIMHSOFFSETP` reader - High-speed mode trim for PMOS differential pairs"]
pub type TRIMHSOFFSETP_R = crate::FieldReader;
#[doc = "Field `TRIMHSOFFSETP` writer - High-speed mode trim for PMOS differential pairs"]
pub type TRIMHSOFFSETP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - High-speed mode trim for NMOS differential pairs"]
    #[inline(always)]
    pub fn TRIMHSOFFSETN(&self) -> TRIMHSOFFSETN_R {
        TRIMHSOFFSETN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - High-speed mode trim for PMOS differential pairs"]
    #[inline(always)]
    pub fn TRIMHSOFFSETP(&self) -> TRIMHSOFFSETP_R {
        TRIMHSOFFSETP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - High-speed mode trim for NMOS differential pairs"]
    #[inline(always)]
    pub fn TRIMHSOFFSETN(&mut self) -> TRIMHSOFFSETN_W<'_, OPAMP1_HSOTR_SPEC> {
        TRIMHSOFFSETN_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - High-speed mode trim for PMOS differential pairs"]
    #[inline(always)]
    pub fn TRIMHSOFFSETP(&mut self) -> TRIMHSOFFSETP_W<'_, OPAMP1_HSOTR_SPEC> {
        TRIMHSOFFSETP_W::new(self, 8)
    }
}
#[doc = "OPAMP1 trimming register in high-speed mode\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp1_hsotr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_hsotr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPAMP1_HSOTR_SPEC;
impl crate::RegisterSpec for OPAMP1_HSOTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp1_hsotr::R`](R) reader structure"]
impl crate::Readable for OPAMP1_HSOTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`opamp1_hsotr::W`](W) writer structure"]
impl crate::Writable for OPAMP1_HSOTR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OPAMP1_HSOTR to value 0"]
impl crate::Resettable for OPAMP1_HSOTR_SPEC {}
