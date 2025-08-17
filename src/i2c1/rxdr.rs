#[doc = "Register `RXDR` reader"]
pub type R = crate::R<RXDR_SPEC>;
#[doc = "Field `RXDATA` reader - 8-bit receive data Data byte received from the I2C bus"]
pub type RXDATA_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 8-bit receive data Data byte received from the I2C bus"]
    #[inline(always)]
    pub fn RXDATA(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "I2C receive data register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDR_SPEC;
impl crate::RegisterSpec for RXDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdr::R`](R) reader structure"]
impl crate::Readable for RXDR_SPEC {}
#[doc = "`reset()` method sets RXDR to value 0"]
impl crate::Resettable for RXDR_SPEC {}
