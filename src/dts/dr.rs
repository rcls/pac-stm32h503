#[doc = "Register `DR` reader"]
pub type R = crate::R<DR_SPEC>;
#[doc = "Register `DR` writer"]
pub type W = crate::W<DR_SPEC>;
#[doc = "Field `TS1_MFREQ` reader - Value of the counter output value for temperature sensor 1"]
pub type TS1_MFREQ_R = crate::FieldReader<u16>;
#[doc = "Field `TS1_MFREQ` writer - Value of the counter output value for temperature sensor 1"]
pub type TS1_MFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Value of the counter output value for temperature sensor 1"]
    #[inline(always)]
    pub fn TS1_MFREQ(&self) -> TS1_MFREQ_R {
        TS1_MFREQ_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Value of the counter output value for temperature sensor 1"]
    #[inline(always)]
    pub fn TS1_MFREQ(&mut self) -> TS1_MFREQ_W<'_, DR_SPEC> {
        TS1_MFREQ_W::new(self, 0)
    }
}
#[doc = "Temperature sensor data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::Writable for DR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DR_SPEC {}
