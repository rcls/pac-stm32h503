#[doc = "Register `HR4` reader"]
pub type R = crate::R<HR4_SPEC>;
#[doc = "Field `H4` reader - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
pub type H4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
    #[inline(always)]
    pub fn H4(&self) -> H4_R {
        H4_R::new(self.bits)
    }
}
#[doc = "HASH digest register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`hr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HR4_SPEC;
impl crate::RegisterSpec for HR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr4::R`](R) reader structure"]
impl crate::Readable for HR4_SPEC {}
#[doc = "`reset()` method sets HR4 to value 0"]
impl crate::Resettable for HR4_SPEC {}
