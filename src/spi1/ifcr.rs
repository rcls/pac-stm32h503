#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IFCR_SPEC>;
#[doc = "Field `EOTC` writer - end of transfer flag clear Writing a 1 into this bit clears EOT flag in the SPI_SR register"]
pub type EOTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTFC` writer - transmission transfer filled flag clear Writing a 1 into this bit clears TXTF flag in the SPI_SR register"]
pub type TXTFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDRC` writer - underrun flag clear Writing a 1 into this bit clears UDR flag in the SPI_SR register"]
pub type UDRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRC` writer - overrun flag clear Writing a 1 into this bit clears OVR flag in the SPI_SR register"]
pub type OVRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEC` writer - CRC error flag clear Writing a 1 into this bit clears CRCE flag in the SPI_SR register"]
pub type CRCEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIFREC` writer - TI frame format error flag clear Writing a 1 into this bit clears TIFRE flag in the SPI_SR register"]
pub type TIFREC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODFC` writer - mode fault flag clear Writing a 1 into this bit clears MODF flag in the SPI_SR register"]
pub type MODFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPC` writer - SUSPend flag clear Writing a 1 into this bit clears SUSP flag in the SPI_SR register"]
pub type SUSPC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 3 - end of transfer flag clear Writing a 1 into this bit clears EOT flag in the SPI_SR register"]
    #[inline(always)]
    pub fn EOTC(&mut self) -> EOTC_W<'_, IFCR_SPEC> {
        EOTC_W::new(self, 3)
    }
    #[doc = "Bit 4 - transmission transfer filled flag clear Writing a 1 into this bit clears TXTF flag in the SPI_SR register"]
    #[inline(always)]
    pub fn TXTFC(&mut self) -> TXTFC_W<'_, IFCR_SPEC> {
        TXTFC_W::new(self, 4)
    }
    #[doc = "Bit 5 - underrun flag clear Writing a 1 into this bit clears UDR flag in the SPI_SR register"]
    #[inline(always)]
    pub fn UDRC(&mut self) -> UDRC_W<'_, IFCR_SPEC> {
        UDRC_W::new(self, 5)
    }
    #[doc = "Bit 6 - overrun flag clear Writing a 1 into this bit clears OVR flag in the SPI_SR register"]
    #[inline(always)]
    pub fn OVRC(&mut self) -> OVRC_W<'_, IFCR_SPEC> {
        OVRC_W::new(self, 6)
    }
    #[doc = "Bit 7 - CRC error flag clear Writing a 1 into this bit clears CRCE flag in the SPI_SR register"]
    #[inline(always)]
    pub fn CRCEC(&mut self) -> CRCEC_W<'_, IFCR_SPEC> {
        CRCEC_W::new(self, 7)
    }
    #[doc = "Bit 8 - TI frame format error flag clear Writing a 1 into this bit clears TIFRE flag in the SPI_SR register"]
    #[inline(always)]
    pub fn TIFREC(&mut self) -> TIFREC_W<'_, IFCR_SPEC> {
        TIFREC_W::new(self, 8)
    }
    #[doc = "Bit 9 - mode fault flag clear Writing a 1 into this bit clears MODF flag in the SPI_SR register"]
    #[inline(always)]
    pub fn MODFC(&mut self) -> MODFC_W<'_, IFCR_SPEC> {
        MODFC_W::new(self, 9)
    }
    #[doc = "Bit 11 - SUSPend flag clear Writing a 1 into this bit clears SUSP flag in the SPI_SR register"]
    #[inline(always)]
    pub fn SUSPC(&mut self) -> SUSPC_W<'_, IFCR_SPEC> {
        SUSPC_W::new(self, 11)
    }
}
#[doc = "SPI/I2S interrupt/status flags clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFCR_SPEC;
impl crate::RegisterSpec for IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifcr::W`](W) writer structure"]
impl crate::Writable for IFCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCR_SPEC {}
