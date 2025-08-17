#[doc = "Register `BRR` reader"]
pub type R = crate::R<BRR_SPEC>;
#[doc = "Register `BRR` writer"]
pub type W = crate::W<BRR_SPEC>;
#[doc = "Field `BRR` reader - LPUART baud rate division (LPUARTDIV)"]
pub type BRR_R = crate::FieldReader<u32>;
#[doc = "Field `BRR` writer - LPUART baud rate division (LPUARTDIV)"]
pub type BRR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - LPUART baud rate division (LPUARTDIV)"]
    #[inline(always)]
    pub fn BRR(&self) -> BRR_R {
        BRR_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - LPUART baud rate division (LPUARTDIV)"]
    #[inline(always)]
    pub fn BRR(&mut self) -> BRR_W<'_, BRR_SPEC> {
        BRR_W::new(self, 0)
    }
}
#[doc = "LPUART baud rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRR_SPEC;
impl crate::RegisterSpec for BRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr::R`](R) reader structure"]
impl crate::Readable for BRR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BRR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BRR_SPEC {}
