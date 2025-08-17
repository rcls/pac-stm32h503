#[doc = "Register `HDPLCR` reader"]
pub type R = crate::R<HDPLCR_SPEC>;
#[doc = "Register `HDPLCR` writer"]
pub type W = crate::W<HDPLCR_SPEC>;
#[doc = "increment HDPL value Other: all other values allow a HDPL level increment.\n\nValue on reset: 180"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INCR_HDPL_A {
    #[doc = "180: no increment"]
    B_0xB4 = 180,
    #[doc = "106: recommended value to increment HDPL level by one"]
    B_0x6A = 106,
}
impl From<INCR_HDPL_A> for u8 {
    #[inline(always)]
    fn from(variant: INCR_HDPL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INCR_HDPL_A {
    type Ux = u8;
}
impl crate::IsEnum for INCR_HDPL_A {}
#[doc = "Field `INCR_HDPL` reader - increment HDPL value Other: all other values allow a HDPL level increment."]
pub type INCR_HDPL_R = crate::FieldReader<INCR_HDPL_A>;
impl INCR_HDPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INCR_HDPL_A> {
        match self.bits {
            180 => Some(INCR_HDPL_A::B_0xB4),
            106 => Some(INCR_HDPL_A::B_0x6A),
            _ => None,
        }
    }
    #[doc = "no increment"]
    #[inline(always)]
    pub fn is_B_0xB4(&self) -> bool {
        *self == INCR_HDPL_A::B_0xB4
    }
    #[doc = "recommended value to increment HDPL level by one"]
    #[inline(always)]
    pub fn is_B_0x6A(&self) -> bool {
        *self == INCR_HDPL_A::B_0x6A
    }
}
#[doc = "Field `INCR_HDPL` writer - increment HDPL value Other: all other values allow a HDPL level increment."]
pub type INCR_HDPL_W<'a, REG> = crate::FieldWriter<'a, REG, 8, INCR_HDPL_A>;
impl<'a, REG> INCR_HDPL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no increment"]
    #[inline(always)]
    pub fn B_0xB4(self) -> &'a mut crate::W<REG> {
        self.variant(INCR_HDPL_A::B_0xB4)
    }
    #[doc = "recommended value to increment HDPL level by one"]
    #[inline(always)]
    pub fn B_0x6A(self) -> &'a mut crate::W<REG> {
        self.variant(INCR_HDPL_A::B_0x6A)
    }
}
impl R {
    #[doc = "Bits 0:7 - increment HDPL value Other: all other values allow a HDPL level increment."]
    #[inline(always)]
    pub fn INCR_HDPL(&self) -> INCR_HDPL_R {
        INCR_HDPL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - increment HDPL value Other: all other values allow a HDPL level increment."]
    #[inline(always)]
    pub fn INCR_HDPL(&mut self) -> INCR_HDPL_W<'_, HDPLCR_SPEC> {
        INCR_HDPL_W::new(self, 0)
    }
}
#[doc = "SBS temporal isolation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hdplcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdplcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDPLCR_SPEC;
impl crate::RegisterSpec for HDPLCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdplcr::R`](R) reader structure"]
impl crate::Readable for HDPLCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hdplcr::W`](W) writer structure"]
impl crate::Writable for HDPLCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets HDPLCR to value 0xb4"]
impl crate::Resettable for HDPLCR_SPEC {
    const RESET_VALUE: u32 = 0xb4;
}
