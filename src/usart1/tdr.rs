#[doc = "Register `TDR` reader"]
pub type R = crate::R<TDR_SPEC>;
#[doc = "Register `TDR` writer"]
pub type W = crate::W<TDR_SPEC>;
#[doc = "Field `TDR` reader - Transmit data value Contains the data character to be transmitted. The USART_TDR register provides the parallel interface between the internal bus and the output shift register (see ). When transmitting with the parity enabled (PCE bit set to 1 in the USART_CR1 register), the value written in the MSB (bit 7 or bit 8 depending on the data length) has no effect because it is replaced by the parity. Note: This register must be written only when TXE/TXFNF=1."]
pub type TDR_R = crate::FieldReader<u16>;
#[doc = "Field `TDR` writer - Transmit data value Contains the data character to be transmitted. The USART_TDR register provides the parallel interface between the internal bus and the output shift register (see ). When transmitting with the parity enabled (PCE bit set to 1 in the USART_CR1 register), the value written in the MSB (bit 7 or bit 8 depending on the data length) has no effect because it is replaced by the parity. Note: This register must be written only when TXE/TXFNF=1."]
pub type TDR_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Transmit data value Contains the data character to be transmitted. The USART_TDR register provides the parallel interface between the internal bus and the output shift register (see ). When transmitting with the parity enabled (PCE bit set to 1 in the USART_CR1 register), the value written in the MSB (bit 7 or bit 8 depending on the data length) has no effect because it is replaced by the parity. Note: This register must be written only when TXE/TXFNF=1."]
    #[inline(always)]
    pub fn TDR(&self) -> TDR_R {
        TDR_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Transmit data value Contains the data character to be transmitted. The USART_TDR register provides the parallel interface between the internal bus and the output shift register (see ). When transmitting with the parity enabled (PCE bit set to 1 in the USART_CR1 register), the value written in the MSB (bit 7 or bit 8 depending on the data length) has no effect because it is replaced by the parity. Note: This register must be written only when TXE/TXFNF=1."]
    #[inline(always)]
    pub fn TDR(&mut self) -> TDR_W<'_, TDR_SPEC> {
        TDR_W::new(self, 0)
    }
}
#[doc = "USART transmit data register\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TDR_SPEC;
impl crate::RegisterSpec for TDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdr::R`](R) reader structure"]
impl crate::Readable for TDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tdr::W`](W) writer structure"]
impl crate::Writable for TDR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TDR to value 0"]
impl crate::Resettable for TDR_SPEC {}
