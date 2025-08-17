#[doc = "Register `DR` reader"]
pub type R = crate::R<DR_SPEC>;
#[doc = "Field `RNDATA` reader - Random data 32-bit random data which are valid when DRDY = 1. When DRDY = 0 RNDATA value is zero. It is recommended to always verify that RNG_DR is different from zero. Because when it is the case a seed error occurred between RNG_SR polling and RND_DR output reading (rare event)."]
pub type RNDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Random data 32-bit random data which are valid when DRDY = 1. When DRDY = 0 RNDATA value is zero. It is recommended to always verify that RNG_DR is different from zero. Because when it is the case a seed error occurred between RNG_SR polling and RND_DR output reading (rare event)."]
    #[inline(always)]
    pub fn RNDATA(&self) -> RNDATA_R {
        RNDATA_R::new(self.bits)
    }
}
#[doc = "RNG data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DR_SPEC {}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DR_SPEC {}
