#[doc = "Register `HR6` reader"]
pub type R = crate::R<HR6_SPEC>;
#[doc = "Field `H6` reader - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
pub type H6_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
    #[inline(always)]
    pub fn H6(&self) -> H6_R {
        H6_R::new(self.bits)
    }
}
#[doc = "HASH supplementary digest register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`hr6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HR6_SPEC;
impl crate::RegisterSpec for HR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr6::R`](R) reader structure"]
impl crate::Readable for HR6_SPEC {}
#[doc = "`reset()` method sets HR6 to value 0"]
impl crate::Resettable for HR6_SPEC {}
