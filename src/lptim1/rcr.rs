#[doc = "Register `RCR` reader"]
pub type R = crate::R<RCR_SPEC>;
#[doc = "Register `RCR` writer"]
pub type W = crate::W<RCR_SPEC>;
#[doc = "Field `REP` reader - Repetition register value REP is the repetition value for the LPTIM."]
pub type REP_R = crate::FieldReader;
#[doc = "Field `REP` writer - Repetition register value REP is the repetition value for the LPTIM."]
pub type REP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Repetition register value REP is the repetition value for the LPTIM."]
    #[inline(always)]
    pub fn REP(&self) -> REP_R {
        REP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Repetition register value REP is the repetition value for the LPTIM."]
    #[inline(always)]
    pub fn REP(&mut self) -> REP_W<'_, RCR_SPEC> {
        REP_W::new(self, 0)
    }
}
#[doc = "LPTIM repetition register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCR_SPEC;
impl crate::RegisterSpec for RCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcr::R`](R) reader structure"]
impl crate::Readable for RCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcr::W`](W) writer structure"]
impl crate::Writable for RCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RCR to value 0"]
impl crate::Resettable for RCR_SPEC {}
