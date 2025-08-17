#[doc = "Register `2ICR` reader"]
pub type R = crate::R<_2ICR_SPEC>;
#[doc = "Register `2ICR` writer"]
pub type W = crate::W<_2ICR_SPEC>;
#[doc = "Field `CSEDC` reader - Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the RAMCFG_MxISR register. Reading this flag returns the SEDC value."]
pub type CSEDC_R = crate::BitReader;
#[doc = "Field `CSEDC` writer - Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the RAMCFG_MxISR register. Reading this flag returns the SEDC value."]
pub type CSEDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDED` reader - Clear ECC double error detected Writing 1 to this flag clears the DED bit in the RAMCFG_MxISR register. Reading this flag returns the DED value."]
pub type CDED_R = crate::BitReader;
#[doc = "Field `CDED` writer - Clear ECC double error detected Writing 1 to this flag clears the DED bit in the RAMCFG_MxISR register. Reading this flag returns the DED value."]
pub type CDED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the RAMCFG_MxISR register. Reading this flag returns the SEDC value."]
    #[inline(always)]
    pub fn CSEDC(&self) -> CSEDC_R {
        CSEDC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear ECC double error detected Writing 1 to this flag clears the DED bit in the RAMCFG_MxISR register. Reading this flag returns the DED value."]
    #[inline(always)]
    pub fn CDED(&self) -> CDED_R {
        CDED_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the RAMCFG_MxISR register. Reading this flag returns the SEDC value."]
    #[inline(always)]
    pub fn CSEDC(&mut self) -> CSEDC_W<'_, _2ICR_SPEC> {
        CSEDC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear ECC double error detected Writing 1 to this flag clears the DED bit in the RAMCFG_MxISR register. Reading this flag returns the DED value."]
    #[inline(always)]
    pub fn CDED(&mut self) -> CDED_W<'_, _2ICR_SPEC> {
        CDED_W::new(self, 1)
    }
}
#[doc = "RAMCFG memory 2 interrupt clear register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`_2icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_2icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _2ICR_SPEC;
impl crate::RegisterSpec for _2ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_2icr::R`](R) reader structure"]
impl crate::Readable for _2ICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_2icr::W`](W) writer structure"]
impl crate::Writable for _2ICR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets 2ICR to value 0"]
impl crate::Resettable for _2ICR_SPEC {}
