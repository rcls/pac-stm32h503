#[doc = "Register `PRIVCFGR` reader"]
pub type R = crate::R<PRIVCFGR_SPEC>;
#[doc = "Register `PRIVCFGR` writer"]
pub type W = crate::W<PRIVCFGR_SPEC>;
#[doc = "privileged state of channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV0_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<PRIV0_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV0` reader - privileged state of channel x"]
pub type PRIV0_R = crate::BitReader<PRIV0_A>;
impl PRIV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV0_A {
        match self.bits {
            false => PRIV0_A::B_0x0,
            true => PRIV0_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV0_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV0_A::B_0x1
    }
}
#[doc = "Field `PRIV0` writer - privileged state of channel x"]
pub type PRIV0_W<'a, REG> = crate::BitWriter<'a, REG, PRIV0_A>;
impl<'a, REG> PRIV0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV0_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV0_A::B_0x1)
    }
}
#[doc = "privileged state of channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV1_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<PRIV1_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV1` reader - privileged state of channel x"]
pub type PRIV1_R = crate::BitReader<PRIV1_A>;
impl PRIV1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV1_A {
        match self.bits {
            false => PRIV1_A::B_0x0,
            true => PRIV1_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV1_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV1_A::B_0x1
    }
}
#[doc = "Field `PRIV1` writer - privileged state of channel x"]
pub type PRIV1_W<'a, REG> = crate::BitWriter<'a, REG, PRIV1_A>;
impl<'a, REG> PRIV1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV1_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV1_A::B_0x1)
    }
}
#[doc = "privileged state of channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV2_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<PRIV2_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV2` reader - privileged state of channel x"]
pub type PRIV2_R = crate::BitReader<PRIV2_A>;
impl PRIV2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV2_A {
        match self.bits {
            false => PRIV2_A::B_0x0,
            true => PRIV2_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV2_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV2_A::B_0x1
    }
}
#[doc = "Field `PRIV2` writer - privileged state of channel x"]
pub type PRIV2_W<'a, REG> = crate::BitWriter<'a, REG, PRIV2_A>;
impl<'a, REG> PRIV2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV2_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV2_A::B_0x1)
    }
}
#[doc = "privileged state of channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV3_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<PRIV3_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV3` reader - privileged state of channel x"]
pub type PRIV3_R = crate::BitReader<PRIV3_A>;
impl PRIV3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV3_A {
        match self.bits {
            false => PRIV3_A::B_0x0,
            true => PRIV3_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV3_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV3_A::B_0x1
    }
}
#[doc = "Field `PRIV3` writer - privileged state of channel x"]
pub type PRIV3_W<'a, REG> = crate::BitWriter<'a, REG, PRIV3_A>;
impl<'a, REG> PRIV3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV3_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV3_A::B_0x1)
    }
}
#[doc = "privileged state of channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV4_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<PRIV4_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV4` reader - privileged state of channel x"]
pub type PRIV4_R = crate::BitReader<PRIV4_A>;
impl PRIV4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV4_A {
        match self.bits {
            false => PRIV4_A::B_0x0,
            true => PRIV4_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV4_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV4_A::B_0x1
    }
}
#[doc = "Field `PRIV4` writer - privileged state of channel x"]
pub type PRIV4_W<'a, REG> = crate::BitWriter<'a, REG, PRIV4_A>;
impl<'a, REG> PRIV4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV4_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV4_A::B_0x1)
    }
}
#[doc = "privileged state of channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV5_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<PRIV5_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV5` reader - privileged state of channel x"]
pub type PRIV5_R = crate::BitReader<PRIV5_A>;
impl PRIV5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV5_A {
        match self.bits {
            false => PRIV5_A::B_0x0,
            true => PRIV5_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV5_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV5_A::B_0x1
    }
}
#[doc = "Field `PRIV5` writer - privileged state of channel x"]
pub type PRIV5_W<'a, REG> = crate::BitWriter<'a, REG, PRIV5_A>;
impl<'a, REG> PRIV5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV5_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV5_A::B_0x1)
    }
}
#[doc = "privileged state of channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV6_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<PRIV6_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV6` reader - privileged state of channel x"]
pub type PRIV6_R = crate::BitReader<PRIV6_A>;
impl PRIV6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV6_A {
        match self.bits {
            false => PRIV6_A::B_0x0,
            true => PRIV6_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV6_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV6_A::B_0x1
    }
}
#[doc = "Field `PRIV6` writer - privileged state of channel x"]
pub type PRIV6_W<'a, REG> = crate::BitWriter<'a, REG, PRIV6_A>;
impl<'a, REG> PRIV6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV6_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV6_A::B_0x1)
    }
}
#[doc = "privileged state of channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV7_A {
    #[doc = "0: unprivileged"]
    B_0x0 = 0,
    #[doc = "1: privileged"]
    B_0x1 = 1,
}
impl From<PRIV7_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV7` reader - privileged state of channel x"]
pub type PRIV7_R = crate::BitReader<PRIV7_A>;
impl PRIV7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV7_A {
        match self.bits {
            false => PRIV7_A::B_0x0,
            true => PRIV7_A::B_0x1,
        }
    }
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV7_A::B_0x0
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV7_A::B_0x1
    }
}
#[doc = "Field `PRIV7` writer - privileged state of channel x"]
pub type PRIV7_W<'a, REG> = crate::BitWriter<'a, REG, PRIV7_A>;
impl<'a, REG> PRIV7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unprivileged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV7_A::B_0x0)
    }
    #[doc = "privileged"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV7_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - privileged state of channel x"]
    #[inline(always)]
    pub fn PRIV0(&self) -> PRIV0_R {
        PRIV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - privileged state of channel x"]
    #[inline(always)]
    pub fn PRIV1(&self) -> PRIV1_R {
        PRIV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - privileged state of channel x"]
    #[inline(always)]
    pub fn PRIV2(&self) -> PRIV2_R {
        PRIV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - privileged state of channel x"]
    #[inline(always)]
    pub fn PRIV3(&self) -> PRIV3_R {
        PRIV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - privileged state of channel x"]
    #[inline(always)]
    pub fn PRIV4(&self) -> PRIV4_R {
        PRIV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - privileged state of channel x"]
    #[inline(always)]
    pub fn PRIV5(&self) -> PRIV5_R {
        PRIV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - privileged state of channel x"]
    #[inline(always)]
    pub fn PRIV6(&self) -> PRIV6_R {
        PRIV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - privileged state of channel x"]
    #[inline(always)]
    pub fn PRIV7(&self) -> PRIV7_R {
        PRIV7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - privileged state of channel x"]
    #[inline(always)]
    pub fn PRIV0(&mut self) -> PRIV0_W<'_, PRIVCFGR_SPEC> {
        PRIV0_W::new(self, 0)
    }
    #[doc = "Bit 1 - privileged state of channel x"]
    #[inline(always)]
    pub fn PRIV1(&mut self) -> PRIV1_W<'_, PRIVCFGR_SPEC> {
        PRIV1_W::new(self, 1)
    }
    #[doc = "Bit 2 - privileged state of channel x"]
    #[inline(always)]
    pub fn PRIV2(&mut self) -> PRIV2_W<'_, PRIVCFGR_SPEC> {
        PRIV2_W::new(self, 2)
    }
    #[doc = "Bit 3 - privileged state of channel x"]
    #[inline(always)]
    pub fn PRIV3(&mut self) -> PRIV3_W<'_, PRIVCFGR_SPEC> {
        PRIV3_W::new(self, 3)
    }
    #[doc = "Bit 4 - privileged state of channel x"]
    #[inline(always)]
    pub fn PRIV4(&mut self) -> PRIV4_W<'_, PRIVCFGR_SPEC> {
        PRIV4_W::new(self, 4)
    }
    #[doc = "Bit 5 - privileged state of channel x"]
    #[inline(always)]
    pub fn PRIV5(&mut self) -> PRIV5_W<'_, PRIVCFGR_SPEC> {
        PRIV5_W::new(self, 5)
    }
    #[doc = "Bit 6 - privileged state of channel x"]
    #[inline(always)]
    pub fn PRIV6(&mut self) -> PRIV6_W<'_, PRIVCFGR_SPEC> {
        PRIV6_W::new(self, 6)
    }
    #[doc = "Bit 7 - privileged state of channel x"]
    #[inline(always)]
    pub fn PRIV7(&mut self) -> PRIV7_W<'_, PRIVCFGR_SPEC> {
        PRIV7_W::new(self, 7)
    }
}
#[doc = "GPDMA privileged configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`privcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIVCFGR_SPEC;
impl crate::RegisterSpec for PRIVCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`privcfgr::R`](R) reader structure"]
impl crate::Readable for PRIVCFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`privcfgr::W`](W) writer structure"]
impl crate::Writable for PRIVCFGR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PRIVCFGR to value 0"]
impl crate::Resettable for PRIVCFGR_SPEC {}
