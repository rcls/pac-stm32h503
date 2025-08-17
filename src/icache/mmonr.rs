#[doc = "Register `MMONR` reader"]
pub type R = crate::R<MMONR_SPEC>;
#[doc = "Field `MISSMON` reader - cache miss monitor counter"]
pub type MISSMON_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - cache miss monitor counter"]
    #[inline(always)]
    pub fn MISSMON(&self) -> MISSMON_R {
        MISSMON_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "ICACHE miss monitor register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmonr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMONR_SPEC;
impl crate::RegisterSpec for MMONR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmonr::R`](R) reader structure"]
impl crate::Readable for MMONR_SPEC {}
#[doc = "`reset()` method sets MMONR to value 0"]
impl crate::Resettable for MMONR_SPEC {}
