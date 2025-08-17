#[doc = "Register `ICR_intput` writer"]
pub type W = crate::W<ICR_INTPUT_SPEC>;
#[doc = "Field `CC1CF` writer - Capture/compare 1 clear flag Writing 1 to this bit clears the CC1IF flag in the LPTIM_ISR register."]
pub type CC1CF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRMCF` writer - Autoreload match clear flag Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register"]
pub type ARRMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTTRIGCF` writer - External trigger valid edge clear flag Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register"]
pub type EXTTRIGCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARROKCF` writer - Autoreload register update OK clear flag Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register"]
pub type ARROKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPCF` writer - Direction change to UP clear flag Writing 1 to this bit clear the UP flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type UPCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWNCF` writer - Direction change to down clear flag Writing 1 to this bit clear the DOWN flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type DOWNCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UECF` writer - Update event clear flag Writing 1 to this bit clear the UE flag in the LPTIM_ISR register."]
pub type UECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPOKCF` writer - Repetition register update OK clear flag Writing 1 to this bit clears the REPOK flag in the LPTIM_ISR register."]
pub type REPOKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2CF` writer - Capture/compare 2 clear flag Writing 1 to this bit clears the CC2IF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CC2CF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1OCF` writer - Capture/compare 1 over-capture clear flag Writing 1 to this bit clears the CC1OF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
pub type CC1OCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2OCF` writer - Capture/compare 2 over-capture clear flag Writing 1 to this bit clears the CC2OF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CC2OCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIEROKCF` writer - Interrupt enable register update OK clear flag Writing 1 to this bit clears the DIEROK flag in the LPTIM_ISR register."]
pub type DIEROKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Capture/compare 1 clear flag Writing 1 to this bit clears the CC1IF flag in the LPTIM_ISR register."]
    #[inline(always)]
    pub fn CC1CF(&mut self) -> CC1CF_W<'_, ICR_INTPUT_SPEC> {
        CC1CF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Autoreload match clear flag Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register"]
    #[inline(always)]
    pub fn ARRMCF(&mut self) -> ARRMCF_W<'_, ICR_INTPUT_SPEC> {
        ARRMCF_W::new(self, 1)
    }
    #[doc = "Bit 2 - External trigger valid edge clear flag Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register"]
    #[inline(always)]
    pub fn EXTTRIGCF(&mut self) -> EXTTRIGCF_W<'_, ICR_INTPUT_SPEC> {
        EXTTRIGCF_W::new(self, 2)
    }
    #[doc = "Bit 4 - Autoreload register update OK clear flag Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register"]
    #[inline(always)]
    pub fn ARROKCF(&mut self) -> ARROKCF_W<'_, ICR_INTPUT_SPEC> {
        ARROKCF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Direction change to UP clear flag Writing 1 to this bit clear the UP flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn UPCF(&mut self) -> UPCF_W<'_, ICR_INTPUT_SPEC> {
        UPCF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Direction change to down clear flag Writing 1 to this bit clear the DOWN flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn DOWNCF(&mut self) -> DOWNCF_W<'_, ICR_INTPUT_SPEC> {
        DOWNCF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Update event clear flag Writing 1 to this bit clear the UE flag in the LPTIM_ISR register."]
    #[inline(always)]
    pub fn UECF(&mut self) -> UECF_W<'_, ICR_INTPUT_SPEC> {
        UECF_W::new(self, 7)
    }
    #[doc = "Bit 8 - Repetition register update OK clear flag Writing 1 to this bit clears the REPOK flag in the LPTIM_ISR register."]
    #[inline(always)]
    pub fn REPOKCF(&mut self) -> REPOKCF_W<'_, ICR_INTPUT_SPEC> {
        REPOKCF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/compare 2 clear flag Writing 1 to this bit clears the CC2IF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn CC2CF(&mut self) -> CC2CF_W<'_, ICR_INTPUT_SPEC> {
        CC2CF_W::new(self, 9)
    }
    #[doc = "Bit 12 - Capture/compare 1 over-capture clear flag Writing 1 to this bit clears the CC1OF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn CC1OCF(&mut self) -> CC1OCF_W<'_, ICR_INTPUT_SPEC> {
        CC1OCF_W::new(self, 12)
    }
    #[doc = "Bit 13 - Capture/compare 2 over-capture clear flag Writing 1 to this bit clears the CC2OF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn CC2OCF(&mut self) -> CC2OCF_W<'_, ICR_INTPUT_SPEC> {
        CC2OCF_W::new(self, 13)
    }
    #[doc = "Bit 24 - Interrupt enable register update OK clear flag Writing 1 to this bit clears the DIEROK flag in the LPTIM_ISR register."]
    #[inline(always)]
    pub fn DIEROKCF(&mut self) -> DIEROKCF_W<'_, ICR_INTPUT_SPEC> {
        DIEROKCF_W::new(self, 24)
    }
}
#[doc = "LPTIM interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr_intput::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_INTPUT_SPEC;
impl crate::RegisterSpec for ICR_INTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr_intput::W`](W) writer structure"]
impl crate::Writable for ICR_INTPUT_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ICR_intput to value 0"]
impl crate::Resettable for ICR_INTPUT_SPEC {}
