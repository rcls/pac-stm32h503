#[doc = "Register `HR7` reader"]
pub type R = crate::R<HR7_SPEC>;
#[doc = "Field `H7` reader - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
pub type H7_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
    #[inline(always)]
    pub fn H7(&self) -> H7_R {
        H7_R::new(self.bits)
    }
}
#[doc = "HASH supplementary digest register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`hr7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HR7_SPEC;
impl crate::RegisterSpec for HR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr7::R`](R) reader structure"]
impl crate::Readable for HR7_SPEC {}
#[doc = "`reset()` method sets HR7 to value 0"]
impl crate::Resettable for HR7_SPEC {}
