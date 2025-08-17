#[doc = "Register `RDR` reader"]
pub type R = crate::R<RDR_SPEC>;
#[doc = "Field `RDR` reader - Receive data value Contains the received data character. The RDR register provides the parallel interface between the input shift register and the internal bus (see ). When receiving with the parity enabled, the value read in the MSB bit is the received parity bit."]
pub type RDR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Receive data value Contains the received data character. The RDR register provides the parallel interface between the input shift register and the internal bus (see ). When receiving with the parity enabled, the value read in the MSB bit is the received parity bit."]
    #[inline(always)]
    pub fn RDR(&self) -> RDR_R {
        RDR_R::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "USART receive data register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDR_SPEC;
impl crate::RegisterSpec for RDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdr::R`](R) reader structure"]
impl crate::Readable for RDR_SPEC {}
#[doc = "`reset()` method sets RDR to value 0"]
impl crate::Resettable for RDR_SPEC {}
