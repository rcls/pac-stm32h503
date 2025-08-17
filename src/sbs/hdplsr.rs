#[doc = "Register `HDPLSR` reader"]
pub type R = crate::R<HDPLSR_SPEC>;
#[doc = "temporal isolation level This bitfield returns the current temporal isolation level.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HDPL_A {
    #[doc = "180: HDPL0, RSS"]
    B_0xB4 = 180,
    #[doc = "81: HDPL1, iRoT"]
    B_0x51 = 81,
    #[doc = "138: HDPL2, uRoT"]
    B_0x8A = 138,
    #[doc = "111: HDPL3, application"]
    B_0x6F = 111,
}
impl From<HDPL_A> for u8 {
    #[inline(always)]
    fn from(variant: HDPL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HDPL_A {
    type Ux = u8;
}
impl crate::IsEnum for HDPL_A {}
#[doc = "Field `HDPL` reader - temporal isolation level This bitfield returns the current temporal isolation level."]
pub type HDPL_R = crate::FieldReader<HDPL_A>;
impl HDPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HDPL_A> {
        match self.bits {
            180 => Some(HDPL_A::B_0xB4),
            81 => Some(HDPL_A::B_0x51),
            138 => Some(HDPL_A::B_0x8A),
            111 => Some(HDPL_A::B_0x6F),
            _ => None,
        }
    }
    #[doc = "HDPL0, RSS"]
    #[inline(always)]
    pub fn is_B_0xB4(&self) -> bool {
        *self == HDPL_A::B_0xB4
    }
    #[doc = "HDPL1, iRoT"]
    #[inline(always)]
    pub fn is_B_0x51(&self) -> bool {
        *self == HDPL_A::B_0x51
    }
    #[doc = "HDPL2, uRoT"]
    #[inline(always)]
    pub fn is_B_0x8A(&self) -> bool {
        *self == HDPL_A::B_0x8A
    }
    #[doc = "HDPL3, application"]
    #[inline(always)]
    pub fn is_B_0x6F(&self) -> bool {
        *self == HDPL_A::B_0x6F
    }
}
impl R {
    #[doc = "Bits 0:7 - temporal isolation level This bitfield returns the current temporal isolation level."]
    #[inline(always)]
    pub fn HDPL(&self) -> HDPL_R {
        HDPL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SBS temporal isolation status register\n\nYou can [`read`](crate::Reg::read) this register and get [`hdplsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDPLSR_SPEC;
impl crate::RegisterSpec for HDPLSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdplsr::R`](R) reader structure"]
impl crate::Readable for HDPLSR_SPEC {}
#[doc = "`reset()` method sets HDPLSR to value 0"]
impl crate::Resettable for HDPLSR_SPEC {}
