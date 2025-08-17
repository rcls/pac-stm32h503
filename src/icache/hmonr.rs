#[doc = "Register `HMONR` reader"]
pub type R = crate::R<HMONR_SPEC>;
#[doc = "Field `HITMON` reader - cache hit monitor counter"]
pub type HITMON_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - cache hit monitor counter"]
    #[inline(always)]
    pub fn HITMON(&self) -> HITMON_R {
        HITMON_R::new(self.bits)
    }
}
#[doc = "ICACHE hit monitor register\n\nYou can [`read`](crate::Reg::read) this register and get [`hmonr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HMONR_SPEC;
impl crate::RegisterSpec for HMONR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hmonr::R`](R) reader structure"]
impl crate::Readable for HMONR_SPEC {}
#[doc = "`reset()` method sets HMONR to value 0"]
impl crate::Resettable for HMONR_SPEC {}
