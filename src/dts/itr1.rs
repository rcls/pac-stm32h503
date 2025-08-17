#[doc = "Register `ITR1` reader"]
pub type R = crate::R<ITR1_SPEC>;
#[doc = "Register `ITR1` writer"]
pub type W = crate::W<ITR1_SPEC>;
#[doc = "Field `TS1_LITTHD` reader - Low interrupt threshold for temperature sensor 1 These bits are set and cleared by software. They indicate the lowest value than can be reached before raising an interrupt signal."]
pub type TS1_LITTHD_R = crate::FieldReader<u16>;
#[doc = "Field `TS1_LITTHD` writer - Low interrupt threshold for temperature sensor 1 These bits are set and cleared by software. They indicate the lowest value than can be reached before raising an interrupt signal."]
pub type TS1_LITTHD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TS1_HITTHD` reader - High interrupt threshold for temperature sensor 1 These bits are set and cleared by software. They indicate the highest value than can be reached before raising an interrupt signal."]
pub type TS1_HITTHD_R = crate::FieldReader<u16>;
#[doc = "Field `TS1_HITTHD` writer - High interrupt threshold for temperature sensor 1 These bits are set and cleared by software. They indicate the highest value than can be reached before raising an interrupt signal."]
pub type TS1_HITTHD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Low interrupt threshold for temperature sensor 1 These bits are set and cleared by software. They indicate the lowest value than can be reached before raising an interrupt signal."]
    #[inline(always)]
    pub fn TS1_LITTHD(&self) -> TS1_LITTHD_R {
        TS1_LITTHD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High interrupt threshold for temperature sensor 1 These bits are set and cleared by software. They indicate the highest value than can be reached before raising an interrupt signal."]
    #[inline(always)]
    pub fn TS1_HITTHD(&self) -> TS1_HITTHD_R {
        TS1_HITTHD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low interrupt threshold for temperature sensor 1 These bits are set and cleared by software. They indicate the lowest value than can be reached before raising an interrupt signal."]
    #[inline(always)]
    pub fn TS1_LITTHD(&mut self) -> TS1_LITTHD_W<'_, ITR1_SPEC> {
        TS1_LITTHD_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - High interrupt threshold for temperature sensor 1 These bits are set and cleared by software. They indicate the highest value than can be reached before raising an interrupt signal."]
    #[inline(always)]
    pub fn TS1_HITTHD(&mut self) -> TS1_HITTHD_W<'_, ITR1_SPEC> {
        TS1_HITTHD_W::new(self, 16)
    }
}
#[doc = "Temperature sensor interrupt threshold register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`itr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITR1_SPEC;
impl crate::RegisterSpec for ITR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itr1::R`](R) reader structure"]
impl crate::Readable for ITR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`itr1::W`](W) writer structure"]
impl crate::Writable for ITR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ITR1 to value 0"]
impl crate::Resettable for ITR1_SPEC {}
