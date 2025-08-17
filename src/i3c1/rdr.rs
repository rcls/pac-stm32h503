#[doc = "Register `RDR` reader"]
pub type R = crate::R<RDR_SPEC>;
#[doc = "Field `RDB0` reader - 8-bit received data on I3C bus."]
pub type RDB0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 8-bit received data on I3C bus."]
    #[inline(always)]
    pub fn RDB0(&self) -> RDB0_R {
        RDB0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "I3C receive data byte register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDR_SPEC;
impl crate::RegisterSpec for RDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdr::R`](R) reader structure"]
impl crate::Readable for RDR_SPEC {}
#[doc = "`reset()` method sets RDR to value 0"]
impl crate::Resettable for RDR_SPEC {}
