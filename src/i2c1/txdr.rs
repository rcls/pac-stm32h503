#[doc = "Register `TXDR` reader"]
pub type R = crate::R<TXDR_SPEC>;
#[doc = "Register `TXDR` writer"]
pub type W = crate::W<TXDR_SPEC>;
#[doc = "Field `TXDATA` reader - 8-bit transmit data Data byte to be transmitted to the I2C bus Note: These bits can be written only when TXE=1."]
pub type TXDATA_R = crate::FieldReader;
#[doc = "Field `TXDATA` writer - 8-bit transmit data Data byte to be transmitted to the I2C bus Note: These bits can be written only when TXE=1."]
pub type TXDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit transmit data Data byte to be transmitted to the I2C bus Note: These bits can be written only when TXE=1."]
    #[inline(always)]
    pub fn TXDATA(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit transmit data Data byte to be transmitted to the I2C bus Note: These bits can be written only when TXE=1."]
    #[inline(always)]
    pub fn TXDATA(&mut self) -> TXDATA_W<'_, TXDR_SPEC> {
        TXDATA_W::new(self, 0)
    }
}
#[doc = "I2C transmit data register\n\nYou can [`read`](crate::Reg::read) this register and get [`txdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDR_SPEC;
impl crate::RegisterSpec for TXDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdr::R`](R) reader structure"]
impl crate::Readable for TXDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txdr::W`](W) writer structure"]
impl crate::Writable for TXDR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TXDR to value 0"]
impl crate::Resettable for TXDR_SPEC {}
