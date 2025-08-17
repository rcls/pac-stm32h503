#[doc = "Register `CREL` reader"]
pub type R = crate::R<CREL_SPEC>;
#[doc = "Field `DAY` reader - 18"]
pub type DAY_R = crate::FieldReader;
#[doc = "Field `MON` reader - 12"]
pub type MON_R = crate::FieldReader;
#[doc = "Field `YEAR` reader - 4"]
pub type YEAR_R = crate::FieldReader;
#[doc = "Field `SUBSTEP` reader - 1"]
pub type SUBSTEP_R = crate::FieldReader;
#[doc = "Field `STEP` reader - 2"]
pub type STEP_R = crate::FieldReader;
#[doc = "Field `REL` reader - 3"]
pub type REL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 18"]
    #[inline(always)]
    pub fn DAY(&self) -> DAY_R {
        DAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 12"]
    #[inline(always)]
    pub fn MON(&self) -> MON_R {
        MON_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 4"]
    #[inline(always)]
    pub fn YEAR(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 1"]
    #[inline(always)]
    pub fn SUBSTEP(&self) -> SUBSTEP_R {
        SUBSTEP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 2"]
    #[inline(always)]
    pub fn STEP(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 3"]
    #[inline(always)]
    pub fn REL(&self) -> REL_R {
        REL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "FDCAN core release register\n\nYou can [`read`](crate::Reg::read) this register and get [`crel::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CREL_SPEC;
impl crate::RegisterSpec for CREL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crel::R`](R) reader structure"]
impl crate::Readable for CREL_SPEC {}
#[doc = "`reset()` method sets CREL to value 0x3214_1218"]
impl crate::Resettable for CREL_SPEC {
    const RESET_VALUE: u32 = 0x3214_1218;
}
