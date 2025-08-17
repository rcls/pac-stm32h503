#[doc = "Register `WRPSGN1R_CUR` reader"]
pub type R = crate::R<WRPSGN1R_CUR_SPEC>;
#[doc = "Field `WRPSG1` reader - Bank1 sector protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding sectors in bank 1 (0: write protected; 1: not write protected)"]
pub type WRPSG1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Bank1 sector protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding sectors in bank 1 (0: write protected; 1: not write protected)"]
    #[inline(always)]
    pub fn WRPSG1(&self) -> WRPSG1_R {
        WRPSG1_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "FLASH write sector protection for Bank1\n\nYou can [`read`](crate::Reg::read) this register and get [`wrpsgn1r_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRPSGN1R_CUR_SPEC;
impl crate::RegisterSpec for WRPSGN1R_CUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrpsgn1r_cur::R`](R) reader structure"]
impl crate::Readable for WRPSGN1R_CUR_SPEC {}
#[doc = "`reset()` method sets WRPSGN1R_CUR to value 0"]
impl crate::Resettable for WRPSGN1R_CUR_SPEC {}
