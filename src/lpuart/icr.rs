#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICR_SPEC>;
#[doc = "Field `PECF` writer - Parity error clear flag Writing 1 to this bit clears the PE flag in the LPUART_ISR register."]
pub type PECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FECF` writer - Framing error clear flag Writing 1 to this bit clears the FE flag in the LPUART_ISR register."]
pub type FECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NECF` writer - Noise detected clear flag Writing 1 to this bit clears the NE flag in the LPUART_ISR register."]
pub type NECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ORECF` writer - Overrun error clear flag Writing 1 to this bit clears the ORE flag in the LPUART_ISR register."]
pub type ORECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLECF` writer - Idle line detected clear flag Writing 1 to this bit clears the IDLE flag in the LPUART_ISR register."]
pub type IDLECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCF` writer - Transmission complete clear flag Writing 1 to this bit clears the TC flag in the LPUART_ISR register."]
pub type TCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSCF` writer - CTS clear flag Writing 1 to this bit clears the CTSIF flag in the LPUART_ISR register."]
pub type CTSCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMCF` writer - Character match clear flag Writing 1 to this bit clears the CMF flag in the LPUART_ISR register."]
pub type CMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUCF` writer - Wakeup from low-power mode clear flag Writing 1 to this bit clears the WUF flag in the USART_ISR register. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page 2386."]
pub type WUCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Parity error clear flag Writing 1 to this bit clears the PE flag in the LPUART_ISR register."]
    #[inline(always)]
    pub fn PECF(&mut self) -> PECF_W<'_, ICR_SPEC> {
        PECF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Framing error clear flag Writing 1 to this bit clears the FE flag in the LPUART_ISR register."]
    #[inline(always)]
    pub fn FECF(&mut self) -> FECF_W<'_, ICR_SPEC> {
        FECF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Noise detected clear flag Writing 1 to this bit clears the NE flag in the LPUART_ISR register."]
    #[inline(always)]
    pub fn NECF(&mut self) -> NECF_W<'_, ICR_SPEC> {
        NECF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun error clear flag Writing 1 to this bit clears the ORE flag in the LPUART_ISR register."]
    #[inline(always)]
    pub fn ORECF(&mut self) -> ORECF_W<'_, ICR_SPEC> {
        ORECF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Idle line detected clear flag Writing 1 to this bit clears the IDLE flag in the LPUART_ISR register."]
    #[inline(always)]
    pub fn IDLECF(&mut self) -> IDLECF_W<'_, ICR_SPEC> {
        IDLECF_W::new(self, 4)
    }
    #[doc = "Bit 6 - Transmission complete clear flag Writing 1 to this bit clears the TC flag in the LPUART_ISR register."]
    #[inline(always)]
    pub fn TCCF(&mut self) -> TCCF_W<'_, ICR_SPEC> {
        TCCF_W::new(self, 6)
    }
    #[doc = "Bit 9 - CTS clear flag Writing 1 to this bit clears the CTSIF flag in the LPUART_ISR register."]
    #[inline(always)]
    pub fn CTSCF(&mut self) -> CTSCF_W<'_, ICR_SPEC> {
        CTSCF_W::new(self, 9)
    }
    #[doc = "Bit 17 - Character match clear flag Writing 1 to this bit clears the CMF flag in the LPUART_ISR register."]
    #[inline(always)]
    pub fn CMCF(&mut self) -> CMCF_W<'_, ICR_SPEC> {
        CMCF_W::new(self, 17)
    }
    #[doc = "Bit 20 - Wakeup from low-power mode clear flag Writing 1 to this bit clears the WUF flag in the USART_ISR register. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page 2386."]
    #[inline(always)]
    pub fn WUCF(&mut self) -> WUCF_W<'_, ICR_SPEC> {
        WUCF_W::new(self, 20)
    }
}
#[doc = "LPUART interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {}
