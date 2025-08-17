#[doc = "Register `WRPSGN2R_CUR` reader"]
pub type R = crate::R<WRPSGN2R_CUR_SPEC>;
#[doc = "Field `WRPSG2` reader - Bank2 sector protection option status byte Setting WRPSG2 bits to 0 write protects the corresponding sectors in bank 2 (0: write protected; 1: not write protected)"]
pub type WRPSG2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Bank2 sector protection option status byte Setting WRPSG2 bits to 0 write protects the corresponding sectors in bank 2 (0: write protected; 1: not write protected)"]
    #[inline(always)]
    pub fn WRPSG2(&self) -> WRPSG2_R {
        WRPSG2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "FLASH write sector protection for Bank2\n\nYou can [`read`](crate::Reg::read) this register and get [`wrpsgn2r_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRPSGN2R_CUR_SPEC;
impl crate::RegisterSpec for WRPSGN2R_CUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrpsgn2r_cur::R`](R) reader structure"]
impl crate::Readable for WRPSGN2R_CUR_SPEC {}
#[doc = "`reset()` method sets WRPSGN2R_CUR to value 0"]
impl crate::Resettable for WRPSGN2R_CUR_SPEC {}
