#[doc = "Register `ICFR` reader"]
pub type R = crate::R<ICFR_SPEC>;
#[doc = "Register `ICFR` writer"]
pub type W = crate::W<ICFR_SPEC>;
#[doc = "Field `CC1IF` reader - Clear COMP Channel1 interrupt flag Writing 1 clears the C1IF flag in the COMP_SR register."]
pub type CC1IF_R = crate::BitReader;
#[doc = "Field `CC1IF` writer - Clear COMP Channel1 interrupt flag Writing 1 clears the C1IF flag in the COMP_SR register."]
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Clear COMP Channel1 interrupt flag Writing 1 clears the C1IF flag in the COMP_SR register."]
    #[inline(always)]
    pub fn CC1IF(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Clear COMP Channel1 interrupt flag Writing 1 clears the C1IF flag in the COMP_SR register."]
    #[inline(always)]
    pub fn CC1IF(&mut self) -> CC1IF_W<'_, ICFR_SPEC> {
        CC1IF_W::new(self, 16)
    }
}
#[doc = "Comparator interrupt clear flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`icfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICFR_SPEC;
impl crate::RegisterSpec for ICFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icfr::R`](R) reader structure"]
impl crate::Readable for ICFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icfr::W`](W) writer structure"]
impl crate::Writable for ICFR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ICFR to value 0"]
impl crate::Resettable for ICFR_SPEC {}
