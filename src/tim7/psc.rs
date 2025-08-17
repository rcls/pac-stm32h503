#[doc = "Register `PSC` reader"]
pub type R = crate::R<PSC_SPEC>;
#[doc = "Register `PSC` writer"]
pub type W = crate::W<PSC_SPEC>;
#[doc = "Field `PSC` reader - Prescaler value The counter clock frequency ftim_cnt_ck is equal to ftim_psc_ck / (PSC\\[15:0\\] + 1). PSC contains the value to be loaded into the active prescaler register at each update event. (including when the counter is cleared through UG bit of TIMx_EGR register."]
pub type PSC_R = crate::FieldReader<u16>;
#[doc = "Field `PSC` writer - Prescaler value The counter clock frequency ftim_cnt_ck is equal to ftim_psc_ck / (PSC\\[15:0\\] + 1). PSC contains the value to be loaded into the active prescaler register at each update event. (including when the counter is cleared through UG bit of TIMx_EGR register."]
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Prescaler value The counter clock frequency ftim_cnt_ck is equal to ftim_psc_ck / (PSC\\[15:0\\] + 1). PSC contains the value to be loaded into the active prescaler register at each update event. (including when the counter is cleared through UG bit of TIMx_EGR register."]
    #[inline(always)]
    pub fn PSC(&self) -> PSC_R {
        PSC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Prescaler value The counter clock frequency ftim_cnt_ck is equal to ftim_psc_ck / (PSC\\[15:0\\] + 1). PSC contains the value to be loaded into the active prescaler register at each update event. (including when the counter is cleared through UG bit of TIMx_EGR register."]
    #[inline(always)]
    pub fn PSC(&mut self) -> PSC_W<'_, PSC_SPEC> {
        PSC_W::new(self, 0)
    }
}
#[doc = "TIM7 prescaler\n\nYou can [`read`](crate::Reg::read) this register and get [`psc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSC_SPEC;
impl crate::RegisterSpec for PSC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`psc::R`](R) reader structure"]
impl crate::Readable for PSC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psc::W`](W) writer structure"]
impl crate::Writable for PSC_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PSC to value 0"]
impl crate::Resettable for PSC_SPEC {}
