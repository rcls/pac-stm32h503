#[doc = "Register `UDRDR` reader"]
pub type R = crate::R<UDRDR_SPEC>;
#[doc = "Register `UDRDR` writer"]
pub type W = crate::W<UDRDR_SPEC>;
#[doc = "Field `UDRDR` reader - data at slave underrun condition The register is taken into account in Slave mode and at underrun condition only. The number of bits considered depends on DSIZE bit settings of the SPI_CFG1 register. Underrun condition handling depends on setting UDRCFG bit at SPI_CFG1 register. Note: UDRDR\\[31-16\\] bits are reserved at the peripheral instances with data size limited to 16-bit. There is no constraint when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored."]
pub type UDRDR_R = crate::FieldReader<u32>;
#[doc = "Field `UDRDR` writer - data at slave underrun condition The register is taken into account in Slave mode and at underrun condition only. The number of bits considered depends on DSIZE bit settings of the SPI_CFG1 register. Underrun condition handling depends on setting UDRCFG bit at SPI_CFG1 register. Note: UDRDR\\[31-16\\] bits are reserved at the peripheral instances with data size limited to 16-bit. There is no constraint when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored."]
pub type UDRDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data at slave underrun condition The register is taken into account in Slave mode and at underrun condition only. The number of bits considered depends on DSIZE bit settings of the SPI_CFG1 register. Underrun condition handling depends on setting UDRCFG bit at SPI_CFG1 register. Note: UDRDR\\[31-16\\] bits are reserved at the peripheral instances with data size limited to 16-bit. There is no constraint when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored."]
    #[inline(always)]
    pub fn UDRDR(&self) -> UDRDR_R {
        UDRDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data at slave underrun condition The register is taken into account in Slave mode and at underrun condition only. The number of bits considered depends on DSIZE bit settings of the SPI_CFG1 register. Underrun condition handling depends on setting UDRCFG bit at SPI_CFG1 register. Note: UDRDR\\[31-16\\] bits are reserved at the peripheral instances with data size limited to 16-bit. There is no constraint when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored."]
    #[inline(always)]
    pub fn UDRDR(&mut self) -> UDRDR_W<'_, UDRDR_SPEC> {
        UDRDR_W::new(self, 0)
    }
}
#[doc = "SPI/I2S underrun data register\n\nYou can [`read`](crate::Reg::read) this register and get [`udrdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udrdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UDRDR_SPEC;
impl crate::RegisterSpec for UDRDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udrdr::R`](R) reader structure"]
impl crate::Readable for UDRDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`udrdr::W`](W) writer structure"]
impl crate::Writable for UDRDR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets UDRDR to value 0"]
impl crate::Resettable for UDRDR_SPEC {}
