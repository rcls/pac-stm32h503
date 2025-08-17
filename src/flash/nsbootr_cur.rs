#[doc = "Register `NSBOOTR_CUR` reader"]
pub type R = crate::R<NSBOOTR_CUR_SPEC>;
#[doc = "A field locking the values of SWAP_BANK, and NSBOOTADD settings.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NSBOOT_LOCK_A {
    #[doc = "195: The SWAP_BANK and NSBOOTADD can still be modified following their individual rules."]
    B_0xC3 = 195,
    #[doc = "180: The NSBOOTADD and SWAP_BANK are frozen."]
    B_0xB4 = 180,
}
impl From<NSBOOT_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: NSBOOT_LOCK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NSBOOT_LOCK_A {
    type Ux = u8;
}
impl crate::IsEnum for NSBOOT_LOCK_A {}
#[doc = "Field `NSBOOT_LOCK` reader - A field locking the values of SWAP_BANK, and NSBOOTADD settings."]
pub type NSBOOT_LOCK_R = crate::FieldReader<NSBOOT_LOCK_A>;
impl NSBOOT_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NSBOOT_LOCK_A> {
        match self.bits {
            195 => Some(NSBOOT_LOCK_A::B_0xC3),
            180 => Some(NSBOOT_LOCK_A::B_0xB4),
            _ => None,
        }
    }
    #[doc = "The SWAP_BANK and NSBOOTADD can still be modified following their individual rules."]
    #[inline(always)]
    pub fn is_B_0xC3(&self) -> bool {
        *self == NSBOOT_LOCK_A::B_0xC3
    }
    #[doc = "The NSBOOTADD and SWAP_BANK are frozen."]
    #[inline(always)]
    pub fn is_B_0xB4(&self) -> bool {
        *self == NSBOOT_LOCK_A::B_0xB4
    }
}
#[doc = "Field `NSBOOTADD` reader - unique boot entry address These bits reflect the UBE address"]
pub type NSBOOTADD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - A field locking the values of SWAP_BANK, and NSBOOTADD settings."]
    #[inline(always)]
    pub fn NSBOOT_LOCK(&self) -> NSBOOT_LOCK_R {
        NSBOOT_LOCK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - unique boot entry address These bits reflect the UBE address"]
    #[inline(always)]
    pub fn NSBOOTADD(&self) -> NSBOOTADD_R {
        NSBOOTADD_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "FLASH non-secure unique boot entry register\n\nYou can [`read`](crate::Reg::read) this register and get [`nsbootr_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NSBOOTR_CUR_SPEC;
impl crate::RegisterSpec for NSBOOTR_CUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nsbootr_cur::R`](R) reader structure"]
impl crate::Readable for NSBOOTR_CUR_SPEC {}
#[doc = "`reset()` method sets NSBOOTR_CUR to value 0"]
impl crate::Resettable for NSBOOTR_CUR_SPEC {}
