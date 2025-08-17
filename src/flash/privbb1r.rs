#[doc = "Register `PRIVBB1R` reader"]
pub type R = crate::R<PRIVBB1R_SPEC>;
#[doc = "Register `PRIVBB1R` writer"]
pub type W = crate::W<PRIVBB1R_SPEC>;
#[doc = "Privileged / unprivileged 8 Kbytes Flash Bank1 sector attribute (y = 0 to 7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRIVBB1_A {
    #[doc = "0: sectors y in bank 1 is non privileged"]
    B_0x0 = 0,
    #[doc = "1: sector y in bank 1 is privileged"]
    B_0x1 = 1,
}
impl From<PRIVBB1_A> for u8 {
    #[inline(always)]
    fn from(variant: PRIVBB1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRIVBB1_A {
    type Ux = u8;
}
impl crate::IsEnum for PRIVBB1_A {}
#[doc = "Field `PRIVBB1` reader - Privileged / unprivileged 8 Kbytes Flash Bank1 sector attribute (y = 0 to 7)"]
pub type PRIVBB1_R = crate::FieldReader<PRIVBB1_A>;
impl PRIVBB1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRIVBB1_A> {
        match self.bits {
            0 => Some(PRIVBB1_A::B_0x0),
            1 => Some(PRIVBB1_A::B_0x1),
            _ => None,
        }
    }
    #[doc = "sectors y in bank 1 is non privileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIVBB1_A::B_0x0
    }
    #[doc = "sector y in bank 1 is privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIVBB1_A::B_0x1
    }
}
#[doc = "Field `PRIVBB1` writer - Privileged / unprivileged 8 Kbytes Flash Bank1 sector attribute (y = 0 to 7)"]
pub type PRIVBB1_W<'a, REG> = crate::FieldWriter<'a, REG, 8, PRIVBB1_A>;
impl<'a, REG> PRIVBB1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "sectors y in bank 1 is non privileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIVBB1_A::B_0x0)
    }
    #[doc = "sector y in bank 1 is privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIVBB1_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Privileged / unprivileged 8 Kbytes Flash Bank1 sector attribute (y = 0 to 7)"]
    #[inline(always)]
    pub fn PRIVBB1(&self) -> PRIVBB1_R {
        PRIVBB1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Privileged / unprivileged 8 Kbytes Flash Bank1 sector attribute (y = 0 to 7)"]
    #[inline(always)]
    pub fn PRIVBB1(&mut self) -> PRIVBB1_W<'_, PRIVBB1R_SPEC> {
        PRIVBB1_W::new(self, 0)
    }
}
#[doc = "FLASH privilege register for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`privbb1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privbb1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIVBB1R_SPEC;
impl crate::RegisterSpec for PRIVBB1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`privbb1r::R`](R) reader structure"]
impl crate::Readable for PRIVBB1R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`privbb1r::W`](W) writer structure"]
impl crate::Writable for PRIVBB1R_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PRIVBB1R to value 0"]
impl crate::Resettable for PRIVBB1R_SPEC {}
