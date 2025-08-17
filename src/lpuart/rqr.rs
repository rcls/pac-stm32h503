#[doc = "Register `RQR` writer"]
pub type W = crate::W<RQR_SPEC>;
#[doc = "Field `SBKRQ` writer - Send break request Writing 1 to this bit sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available. Note: If the application needs to send the break character following all previously inserted data, including the ones not yet transmitted, the software should wait for the TXE flag assertion before setting the SBKRQ bit."]
pub type SBKRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMRQ` writer - Mute mode request Writing 1 to this bit puts the LPUART in Mute mode and resets the RWU flag."]
pub type MMRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFRQ` writer - Receive data flush request Writing 1 to this bit clears the RXNE flag. This enables discarding the received data without reading it, and avoid an overrun condition."]
pub type RXFRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFRQ` writer - Transmit data flush request This bit is used when FIFO mode is enabled. TXFRQ bit is set to flush the whole FIFO. This sets the flag TXFE (TXFIFO empty, bit 23 in the LPUART_ISR register). Note: In FIFO mode, the TXFNF flag is reset during the flush request until TxFIFO is empty in order to ensure that no data are written in the data register."]
pub type TXFRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 1 - Send break request Writing 1 to this bit sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available. Note: If the application needs to send the break character following all previously inserted data, including the ones not yet transmitted, the software should wait for the TXE flag assertion before setting the SBKRQ bit."]
    #[inline(always)]
    pub fn SBKRQ(&mut self) -> SBKRQ_W<'_, RQR_SPEC> {
        SBKRQ_W::new(self, 1)
    }
    #[doc = "Bit 2 - Mute mode request Writing 1 to this bit puts the LPUART in Mute mode and resets the RWU flag."]
    #[inline(always)]
    pub fn MMRQ(&mut self) -> MMRQ_W<'_, RQR_SPEC> {
        MMRQ_W::new(self, 2)
    }
    #[doc = "Bit 3 - Receive data flush request Writing 1 to this bit clears the RXNE flag. This enables discarding the received data without reading it, and avoid an overrun condition."]
    #[inline(always)]
    pub fn RXFRQ(&mut self) -> RXFRQ_W<'_, RQR_SPEC> {
        RXFRQ_W::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit data flush request This bit is used when FIFO mode is enabled. TXFRQ bit is set to flush the whole FIFO. This sets the flag TXFE (TXFIFO empty, bit 23 in the LPUART_ISR register). Note: In FIFO mode, the TXFNF flag is reset during the flush request until TxFIFO is empty in order to ensure that no data are written in the data register."]
    #[inline(always)]
    pub fn TXFRQ(&mut self) -> TXFRQ_W<'_, RQR_SPEC> {
        TXFRQ_W::new(self, 4)
    }
}
#[doc = "LPUART request register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rqr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RQR_SPEC;
impl crate::RegisterSpec for RQR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rqr::W`](W) writer structure"]
impl crate::Writable for RQR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RQR to value 0"]
impl crate::Resettable for RQR_SPEC {}
