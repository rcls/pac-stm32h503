#[doc = "Register `HR5` reader"]
pub type R = crate::R<HR5_SPEC>;
#[doc = "Field `H5` reader - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
pub type H5_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
    #[inline(always)]
    pub fn H5(&self) -> H5_R {
        H5_R::new(self.bits)
    }
}
#[doc = "HASH supplementary digest register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`hr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HR5_SPEC;
impl crate::RegisterSpec for HR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr5::R`](R) reader structure"]
impl crate::Readable for HR5_SPEC {}
#[doc = "`reset()` method sets HR5 to value 0"]
impl crate::Resettable for HR5_SPEC {}
