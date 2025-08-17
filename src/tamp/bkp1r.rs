#[doc = "Register `BKP1R` reader"]
pub type R = crate::R<BKP1R_SPEC>;
#[doc = "Register `BKP1R` writer"]
pub type W = crate::W<BKP1R_SPEC>;
#[doc = "Field `BKP` reader - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set."]
pub type BKP_R = crate::FieldReader<u32>;
#[doc = "Field `BKP` writer - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set."]
pub type BKP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set."]
    #[inline(always)]
    pub fn BKP(&self) -> BKP_R {
        BKP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set."]
    #[inline(always)]
    pub fn BKP(&mut self) -> BKP_W<'_, BKP1R_SPEC> {
        BKP_W::new(self, 0)
    }
}
#[doc = "TAMP backup 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BKP1R_SPEC;
impl crate::RegisterSpec for BKP1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp1r::R`](R) reader structure"]
impl crate::Readable for BKP1R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bkp1r::W`](W) writer structure"]
impl crate::Writable for BKP1R_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets BKP1R to value 0"]
impl crate::Resettable for BKP1R_SPEC {}
