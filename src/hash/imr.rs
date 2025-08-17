#[doc = "Register `IMR` reader"]
pub type R = crate::R<IMR_SPEC>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<IMR_SPEC>;
#[doc = "Data input interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DINIE_A {
    #[doc = "0: Data input interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Data input interrupt enabled"]
    B_0x1 = 1,
}
impl From<DINIE_A> for bool {
    #[inline(always)]
    fn from(variant: DINIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINIE` reader - Data input interrupt enable"]
pub type DINIE_R = crate::BitReader<DINIE_A>;
impl DINIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DINIE_A {
        match self.bits {
            false => DINIE_A::B_0x0,
            true => DINIE_A::B_0x1,
        }
    }
    #[doc = "Data input interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DINIE_A::B_0x0
    }
    #[doc = "Data input interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DINIE_A::B_0x1
    }
}
#[doc = "Field `DINIE` writer - Data input interrupt enable"]
pub type DINIE_W<'a, REG> = crate::BitWriter<'a, REG, DINIE_A>;
impl<'a, REG> DINIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data input interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DINIE_A::B_0x0)
    }
    #[doc = "Data input interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DINIE_A::B_0x1)
    }
}
#[doc = "Digest calculation completion interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCIE_A {
    #[doc = "0: Digest calculation completion interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Digest calculation completion interrupt enabled."]
    B_0x1 = 1,
}
impl From<DCIE_A> for bool {
    #[inline(always)]
    fn from(variant: DCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCIE` reader - Digest calculation completion interrupt enable"]
pub type DCIE_R = crate::BitReader<DCIE_A>;
impl DCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCIE_A {
        match self.bits {
            false => DCIE_A::B_0x0,
            true => DCIE_A::B_0x1,
        }
    }
    #[doc = "Digest calculation completion interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DCIE_A::B_0x0
    }
    #[doc = "Digest calculation completion interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DCIE_A::B_0x1
    }
}
#[doc = "Field `DCIE` writer - Digest calculation completion interrupt enable"]
pub type DCIE_W<'a, REG> = crate::BitWriter<'a, REG, DCIE_A>;
impl<'a, REG> DCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Digest calculation completion interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DCIE_A::B_0x0)
    }
    #[doc = "Digest calculation completion interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DCIE_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Data input interrupt enable"]
    #[inline(always)]
    pub fn DINIE(&self) -> DINIE_R {
        DINIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Digest calculation completion interrupt enable"]
    #[inline(always)]
    pub fn DCIE(&self) -> DCIE_R {
        DCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data input interrupt enable"]
    #[inline(always)]
    pub fn DINIE(&mut self) -> DINIE_W<'_, IMR_SPEC> {
        DINIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Digest calculation completion interrupt enable"]
    #[inline(always)]
    pub fn DCIE(&mut self) -> DCIE_W<'_, IMR_SPEC> {
        DCIE_W::new(self, 1)
    }
}
#[doc = "HASH interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for IMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {}
