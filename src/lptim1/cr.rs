#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "LPTIM enable The ENABLE bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: LPTIM is disabled. Writing '0' to the ENABLE bit resets all the DMA request signals (input capture and update event DMA requests)."]
    B_0x0 = 0,
    #[doc = "1: LPTIM is enabled"]
    B_0x1 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - LPTIM enable The ENABLE bit is set and cleared by software."]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::B_0x0,
            true => ENABLE_A::B_0x1,
        }
    }
    #[doc = "LPTIM is disabled. Writing '0' to the ENABLE bit resets all the DMA request signals (input capture and update event DMA requests)."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ENABLE_A::B_0x0
    }
    #[doc = "LPTIM is enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ENABLE_A::B_0x1
    }
}
#[doc = "Field `ENABLE` writer - LPTIM enable The ENABLE bit is set and cleared by software."]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, ENABLE_A>;
impl<'a, REG> ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPTIM is disabled. Writing '0' to the ENABLE bit resets all the DMA request signals (input capture and update event DMA requests)."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::B_0x0)
    }
    #[doc = "LPTIM is enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE_A::B_0x1)
    }
}
#[doc = "Field `SNGSTRT` reader - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\] = '00'), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\\[1:0\\] different than '00'), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM stops at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It is automatically reset by hardware."]
pub type SNGSTRT_R = crate::BitReader;
#[doc = "Field `SNGSTRT` writer - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\] = '00'), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\\[1:0\\] different than '00'), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM stops at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It is automatically reset by hardware."]
pub type SNGSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTSTRT` reader - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\] = '00'), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\\[1:0\\] different than '00'), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer does not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware."]
pub type CNTSTRT_R = crate::BitReader;
#[doc = "Field `CNTSTRT` writer - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\] = '00'), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\\[1:0\\] different than '00'), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer does not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware."]
pub type CNTSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNTRST` reader - Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit triggers a synchronous reset of the LPTIM_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware. COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software should consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'."]
pub type COUNTRST_R = crate::BitReader;
#[doc = "Field `COUNTRST` writer - Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit triggers a synchronous reset of the LPTIM_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware. COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software should consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'."]
pub type COUNTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTARE` reader - Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM_CNT register asynchronously resets LPTIM_CNT register content. This bit can be set only when the LPTIM is enabled."]
pub type RSTARE_R = crate::BitReader;
#[doc = "Field `RSTARE` writer - Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM_CNT register asynchronously resets LPTIM_CNT register content. This bit can be set only when the LPTIM is enabled."]
pub type RSTARE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LPTIM enable The ENABLE bit is set and cleared by software."]
    #[inline(always)]
    pub fn ENABLE(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\] = '00'), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\\[1:0\\] different than '00'), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM stops at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It is automatically reset by hardware."]
    #[inline(always)]
    pub fn SNGSTRT(&self) -> SNGSTRT_R {
        SNGSTRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\] = '00'), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\\[1:0\\] different than '00'), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer does not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware."]
    #[inline(always)]
    pub fn CNTSTRT(&self) -> CNTSTRT_R {
        CNTSTRT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit triggers a synchronous reset of the LPTIM_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware. COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software should consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'."]
    #[inline(always)]
    pub fn COUNTRST(&self) -> COUNTRST_R {
        COUNTRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM_CNT register asynchronously resets LPTIM_CNT register content. This bit can be set only when the LPTIM is enabled."]
    #[inline(always)]
    pub fn RSTARE(&self) -> RSTARE_R {
        RSTARE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPTIM enable The ENABLE bit is set and cleared by software."]
    #[inline(always)]
    pub fn ENABLE(&mut self) -> ENABLE_W<'_, CR_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\] = '00'), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\\[1:0\\] different than '00'), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM stops at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It is automatically reset by hardware."]
    #[inline(always)]
    pub fn SNGSTRT(&mut self) -> SNGSTRT_W<'_, CR_SPEC> {
        SNGSTRT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\] = '00'), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\\[1:0\\] different than '00'), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer does not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware."]
    #[inline(always)]
    pub fn CNTSTRT(&mut self) -> CNTSTRT_W<'_, CR_SPEC> {
        CNTSTRT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit triggers a synchronous reset of the LPTIM_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware. COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software should consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'."]
    #[inline(always)]
    pub fn COUNTRST(&mut self) -> COUNTRST_W<'_, CR_SPEC> {
        COUNTRST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM_CNT register asynchronously resets LPTIM_CNT register content. This bit can be set only when the LPTIM is enabled."]
    #[inline(always)]
    pub fn RSTARE(&mut self) -> RSTARE_W<'_, CR_SPEC> {
        RSTARE_W::new(self, 4)
    }
}
#[doc = "LPTIM control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {}
