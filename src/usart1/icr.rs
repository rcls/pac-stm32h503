#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICR_SPEC>;
#[doc = "Field `PECF` writer - Parity error clear flag Writing 1 to this bit clears the PE flag in the USART_ISR register."]
pub type PECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FECF` writer - Framing error clear flag Writing 1 to this bit clears the FE flag in the USART_ISR register."]
pub type FECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NECF` writer - Noise detected clear flag Writing 1 to this bit clears the NE flag in the USART_ISR register."]
pub type NECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ORECF` writer - Overrun error clear flag Writing 1 to this bit clears the ORE flag in the USART_ISR register."]
pub type ORECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLECF` writer - Idle line detected clear flag Writing 1 to this bit clears the IDLE flag in the USART_ISR register."]
pub type IDLECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFECF` writer - TXFIFO empty clear flag Writing 1 to this bit clears the TXFE flag in the USART_ISR register."]
pub type TXFECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCF` writer - Transmission complete clear flag Writing 1 to this bit clears the TC flag in the USART_ISR register."]
pub type TCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCBGTCF` writer - Transmission complete before Guard time clear flag Writing 1 to this bit clears the TCBGT flag in the USART_ISR register."]
pub type TCBGTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDCF` writer - LIN break detection clear flag Writing 1 to this bit clears the LBDF flag in the USART_ISR register. Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LBDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSCF` writer - CTS clear flag Writing 1 to this bit clears the CTSIF flag in the USART_ISR register. Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CTSCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTOCF` writer - Receiver timeout clear flag Writing 1 to this bit clears the RTOF flag in the USART_ISR register. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to page 2297."]
pub type RTOCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOBCF` writer - End of block clear flag Writing 1 to this bit clears the EOBF flag in the USART_ISR register. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type EOBCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDRCF` writer - SPI slave underrun clear flag Writing 1 to this bit clears the UDRF flag in the USART_ISR register. Note: If the USART does not support SPI slave mode, this bit is reserved and must be kept at reset value. Refer to"]
pub type UDRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMCF` writer - Character match clear flag Writing 1 to this bit clears the CMF flag in the USART_ISR register."]
pub type CMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUCF` writer - Wakeup from low-power mode clear flag Writing 1 to this bit clears the WUF flag in the USART_ISR register. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page 2297."]
pub type WUCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Parity error clear flag Writing 1 to this bit clears the PE flag in the USART_ISR register."]
    #[inline(always)]
    pub fn PECF(&mut self) -> PECF_W<'_, ICR_SPEC> {
        PECF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Framing error clear flag Writing 1 to this bit clears the FE flag in the USART_ISR register."]
    #[inline(always)]
    pub fn FECF(&mut self) -> FECF_W<'_, ICR_SPEC> {
        FECF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Noise detected clear flag Writing 1 to this bit clears the NE flag in the USART_ISR register."]
    #[inline(always)]
    pub fn NECF(&mut self) -> NECF_W<'_, ICR_SPEC> {
        NECF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun error clear flag Writing 1 to this bit clears the ORE flag in the USART_ISR register."]
    #[inline(always)]
    pub fn ORECF(&mut self) -> ORECF_W<'_, ICR_SPEC> {
        ORECF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Idle line detected clear flag Writing 1 to this bit clears the IDLE flag in the USART_ISR register."]
    #[inline(always)]
    pub fn IDLECF(&mut self) -> IDLECF_W<'_, ICR_SPEC> {
        IDLECF_W::new(self, 4)
    }
    #[doc = "Bit 5 - TXFIFO empty clear flag Writing 1 to this bit clears the TXFE flag in the USART_ISR register."]
    #[inline(always)]
    pub fn TXFECF(&mut self) -> TXFECF_W<'_, ICR_SPEC> {
        TXFECF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission complete clear flag Writing 1 to this bit clears the TC flag in the USART_ISR register."]
    #[inline(always)]
    pub fn TCCF(&mut self) -> TCCF_W<'_, ICR_SPEC> {
        TCCF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Transmission complete before Guard time clear flag Writing 1 to this bit clears the TCBGT flag in the USART_ISR register."]
    #[inline(always)]
    pub fn TCBGTCF(&mut self) -> TCBGTCF_W<'_, ICR_SPEC> {
        TCBGTCF_W::new(self, 7)
    }
    #[doc = "Bit 8 - LIN break detection clear flag Writing 1 to this bit clears the LBDF flag in the USART_ISR register. Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn LBDCF(&mut self) -> LBDCF_W<'_, ICR_SPEC> {
        LBDCF_W::new(self, 8)
    }
    #[doc = "Bit 9 - CTS clear flag Writing 1 to this bit clears the CTSIF flag in the USART_ISR register. Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn CTSCF(&mut self) -> CTSCF_W<'_, ICR_SPEC> {
        CTSCF_W::new(self, 9)
    }
    #[doc = "Bit 11 - Receiver timeout clear flag Writing 1 to this bit clears the RTOF flag in the USART_ISR register. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to page 2297."]
    #[inline(always)]
    pub fn RTOCF(&mut self) -> RTOCF_W<'_, ICR_SPEC> {
        RTOCF_W::new(self, 11)
    }
    #[doc = "Bit 12 - End of block clear flag Writing 1 to this bit clears the EOBF flag in the USART_ISR register. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn EOBCF(&mut self) -> EOBCF_W<'_, ICR_SPEC> {
        EOBCF_W::new(self, 12)
    }
    #[doc = "Bit 13 - SPI slave underrun clear flag Writing 1 to this bit clears the UDRF flag in the USART_ISR register. Note: If the USART does not support SPI slave mode, this bit is reserved and must be kept at reset value. Refer to"]
    #[inline(always)]
    pub fn UDRCF(&mut self) -> UDRCF_W<'_, ICR_SPEC> {
        UDRCF_W::new(self, 13)
    }
    #[doc = "Bit 17 - Character match clear flag Writing 1 to this bit clears the CMF flag in the USART_ISR register."]
    #[inline(always)]
    pub fn CMCF(&mut self) -> CMCF_W<'_, ICR_SPEC> {
        CMCF_W::new(self, 17)
    }
    #[doc = "Bit 20 - Wakeup from low-power mode clear flag Writing 1 to this bit clears the WUF flag in the USART_ISR register. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page 2297."]
    #[inline(always)]
    pub fn WUCF(&mut self) -> WUCF_W<'_, ICR_SPEC> {
        WUCF_W::new(self, 20)
    }
}
#[doc = "USART interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
