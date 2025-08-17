#[doc = "Register `MPCBB2_PRIVCFGR9` reader"]
pub type R = crate::R<MPCBB2_PRIVCFGR9_SPEC>;
#[doc = "Register `MPCBB2_PRIVCFGR9` writer"]
pub type W = crate::W<MPCBB2_PRIVCFGR9_SPEC>;
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV0_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV0_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV0` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
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
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV0_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV0_A::B_0x1
    }
}
#[doc = "Field `PRIV0` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV0_W<'a, REG> = crate::BitWriter<'a, REG, PRIV0_A>;
impl<'a, REG> PRIV0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV0_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV0_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV1_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV1_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV1` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
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
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV1_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV1_A::B_0x1
    }
}
#[doc = "Field `PRIV1` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV1_W<'a, REG> = crate::BitWriter<'a, REG, PRIV1_A>;
impl<'a, REG> PRIV1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV1_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV1_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV2_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV2_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV2` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
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
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV2_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV2_A::B_0x1
    }
}
#[doc = "Field `PRIV2` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV2_W<'a, REG> = crate::BitWriter<'a, REG, PRIV2_A>;
impl<'a, REG> PRIV2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV2_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV2_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV3_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV3_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV3` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
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
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV3_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV3_A::B_0x1
    }
}
#[doc = "Field `PRIV3` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV3_W<'a, REG> = crate::BitWriter<'a, REG, PRIV3_A>;
impl<'a, REG> PRIV3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV3_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV3_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV4_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV4_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV4` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
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
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV4_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV4_A::B_0x1
    }
}
#[doc = "Field `PRIV4` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV4_W<'a, REG> = crate::BitWriter<'a, REG, PRIV4_A>;
impl<'a, REG> PRIV4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV4_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV4_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV5_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV5_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV5` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
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
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV5_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV5_A::B_0x1
    }
}
#[doc = "Field `PRIV5` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV5_W<'a, REG> = crate::BitWriter<'a, REG, PRIV5_A>;
impl<'a, REG> PRIV5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV5_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV5_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV6_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV6_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV6` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
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
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV6_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV6_A::B_0x1
    }
}
#[doc = "Field `PRIV6` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV6_W<'a, REG> = crate::BitWriter<'a, REG, PRIV6_A>;
impl<'a, REG> PRIV6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV6_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV6_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV7_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV7_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV7` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
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
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV7_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV7_A::B_0x1
    }
}
#[doc = "Field `PRIV7` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV7_W<'a, REG> = crate::BitWriter<'a, REG, PRIV7_A>;
impl<'a, REG> PRIV7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV7_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV7_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV8_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV8_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV8` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV8_R = crate::BitReader<PRIV8_A>;
impl PRIV8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV8_A {
        match self.bits {
            false => PRIV8_A::B_0x0,
            true => PRIV8_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV8_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV8_A::B_0x1
    }
}
#[doc = "Field `PRIV8` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV8_W<'a, REG> = crate::BitWriter<'a, REG, PRIV8_A>;
impl<'a, REG> PRIV8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV8_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV8_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV9_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV9_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV9` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV9_R = crate::BitReader<PRIV9_A>;
impl PRIV9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV9_A {
        match self.bits {
            false => PRIV9_A::B_0x0,
            true => PRIV9_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV9_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV9_A::B_0x1
    }
}
#[doc = "Field `PRIV9` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV9_W<'a, REG> = crate::BitWriter<'a, REG, PRIV9_A>;
impl<'a, REG> PRIV9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV9_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV9_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV10_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV10_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV10` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV10_R = crate::BitReader<PRIV10_A>;
impl PRIV10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV10_A {
        match self.bits {
            false => PRIV10_A::B_0x0,
            true => PRIV10_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV10_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV10_A::B_0x1
    }
}
#[doc = "Field `PRIV10` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV10_W<'a, REG> = crate::BitWriter<'a, REG, PRIV10_A>;
impl<'a, REG> PRIV10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV10_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV10_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV11_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV11_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV11` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV11_R = crate::BitReader<PRIV11_A>;
impl PRIV11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV11_A {
        match self.bits {
            false => PRIV11_A::B_0x0,
            true => PRIV11_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV11_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV11_A::B_0x1
    }
}
#[doc = "Field `PRIV11` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV11_W<'a, REG> = crate::BitWriter<'a, REG, PRIV11_A>;
impl<'a, REG> PRIV11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV11_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV11_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV12_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV12_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV12` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV12_R = crate::BitReader<PRIV12_A>;
impl PRIV12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV12_A {
        match self.bits {
            false => PRIV12_A::B_0x0,
            true => PRIV12_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV12_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV12_A::B_0x1
    }
}
#[doc = "Field `PRIV12` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV12_W<'a, REG> = crate::BitWriter<'a, REG, PRIV12_A>;
impl<'a, REG> PRIV12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV12_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV12_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV13_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV13_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV13` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV13_R = crate::BitReader<PRIV13_A>;
impl PRIV13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV13_A {
        match self.bits {
            false => PRIV13_A::B_0x0,
            true => PRIV13_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV13_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV13_A::B_0x1
    }
}
#[doc = "Field `PRIV13` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV13_W<'a, REG> = crate::BitWriter<'a, REG, PRIV13_A>;
impl<'a, REG> PRIV13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV13_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV13_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV14_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV14_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV14` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV14_R = crate::BitReader<PRIV14_A>;
impl PRIV14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV14_A {
        match self.bits {
            false => PRIV14_A::B_0x0,
            true => PRIV14_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV14_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV14_A::B_0x1
    }
}
#[doc = "Field `PRIV14` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV14_W<'a, REG> = crate::BitWriter<'a, REG, PRIV14_A>;
impl<'a, REG> PRIV14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV14_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV14_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV15_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV15_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV15` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV15_R = crate::BitReader<PRIV15_A>;
impl PRIV15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV15_A {
        match self.bits {
            false => PRIV15_A::B_0x0,
            true => PRIV15_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV15_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV15_A::B_0x1
    }
}
#[doc = "Field `PRIV15` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV15_W<'a, REG> = crate::BitWriter<'a, REG, PRIV15_A>;
impl<'a, REG> PRIV15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV15_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV15_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV16_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV16_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV16` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV16_R = crate::BitReader<PRIV16_A>;
impl PRIV16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV16_A {
        match self.bits {
            false => PRIV16_A::B_0x0,
            true => PRIV16_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV16_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV16_A::B_0x1
    }
}
#[doc = "Field `PRIV16` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV16_W<'a, REG> = crate::BitWriter<'a, REG, PRIV16_A>;
impl<'a, REG> PRIV16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV16_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV16_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV17_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV17_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV17` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV17_R = crate::BitReader<PRIV17_A>;
impl PRIV17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV17_A {
        match self.bits {
            false => PRIV17_A::B_0x0,
            true => PRIV17_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV17_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV17_A::B_0x1
    }
}
#[doc = "Field `PRIV17` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV17_W<'a, REG> = crate::BitWriter<'a, REG, PRIV17_A>;
impl<'a, REG> PRIV17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV17_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV17_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV18_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV18_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV18` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV18_R = crate::BitReader<PRIV18_A>;
impl PRIV18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV18_A {
        match self.bits {
            false => PRIV18_A::B_0x0,
            true => PRIV18_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV18_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV18_A::B_0x1
    }
}
#[doc = "Field `PRIV18` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV18_W<'a, REG> = crate::BitWriter<'a, REG, PRIV18_A>;
impl<'a, REG> PRIV18_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV18_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV18_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV19_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV19_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV19` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV19_R = crate::BitReader<PRIV19_A>;
impl PRIV19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV19_A {
        match self.bits {
            false => PRIV19_A::B_0x0,
            true => PRIV19_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV19_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV19_A::B_0x1
    }
}
#[doc = "Field `PRIV19` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV19_W<'a, REG> = crate::BitWriter<'a, REG, PRIV19_A>;
impl<'a, REG> PRIV19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV19_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV19_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV20_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV20_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV20` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV20_R = crate::BitReader<PRIV20_A>;
impl PRIV20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV20_A {
        match self.bits {
            false => PRIV20_A::B_0x0,
            true => PRIV20_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV20_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV20_A::B_0x1
    }
}
#[doc = "Field `PRIV20` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV20_W<'a, REG> = crate::BitWriter<'a, REG, PRIV20_A>;
impl<'a, REG> PRIV20_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV20_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV20_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV21_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV21_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV21` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV21_R = crate::BitReader<PRIV21_A>;
impl PRIV21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV21_A {
        match self.bits {
            false => PRIV21_A::B_0x0,
            true => PRIV21_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV21_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV21_A::B_0x1
    }
}
#[doc = "Field `PRIV21` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV21_W<'a, REG> = crate::BitWriter<'a, REG, PRIV21_A>;
impl<'a, REG> PRIV21_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV21_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV21_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV22_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV22_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV22` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV22_R = crate::BitReader<PRIV22_A>;
impl PRIV22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV22_A {
        match self.bits {
            false => PRIV22_A::B_0x0,
            true => PRIV22_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV22_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV22_A::B_0x1
    }
}
#[doc = "Field `PRIV22` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV22_W<'a, REG> = crate::BitWriter<'a, REG, PRIV22_A>;
impl<'a, REG> PRIV22_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV22_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV22_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV23_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV23_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV23` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV23_R = crate::BitReader<PRIV23_A>;
impl PRIV23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV23_A {
        match self.bits {
            false => PRIV23_A::B_0x0,
            true => PRIV23_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV23_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV23_A::B_0x1
    }
}
#[doc = "Field `PRIV23` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV23_W<'a, REG> = crate::BitWriter<'a, REG, PRIV23_A>;
impl<'a, REG> PRIV23_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV23_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV23_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV24_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV24_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV24` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV24_R = crate::BitReader<PRIV24_A>;
impl PRIV24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV24_A {
        match self.bits {
            false => PRIV24_A::B_0x0,
            true => PRIV24_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV24_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV24_A::B_0x1
    }
}
#[doc = "Field `PRIV24` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV24_W<'a, REG> = crate::BitWriter<'a, REG, PRIV24_A>;
impl<'a, REG> PRIV24_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV24_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV24_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV25_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV25_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV25` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV25_R = crate::BitReader<PRIV25_A>;
impl PRIV25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV25_A {
        match self.bits {
            false => PRIV25_A::B_0x0,
            true => PRIV25_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV25_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV25_A::B_0x1
    }
}
#[doc = "Field `PRIV25` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV25_W<'a, REG> = crate::BitWriter<'a, REG, PRIV25_A>;
impl<'a, REG> PRIV25_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV25_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV25_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV26_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV26_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV26` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV26_R = crate::BitReader<PRIV26_A>;
impl PRIV26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV26_A {
        match self.bits {
            false => PRIV26_A::B_0x0,
            true => PRIV26_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV26_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV26_A::B_0x1
    }
}
#[doc = "Field `PRIV26` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV26_W<'a, REG> = crate::BitWriter<'a, REG, PRIV26_A>;
impl<'a, REG> PRIV26_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV26_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV26_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV27_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV27_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV27` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV27_R = crate::BitReader<PRIV27_A>;
impl PRIV27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV27_A {
        match self.bits {
            false => PRIV27_A::B_0x0,
            true => PRIV27_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV27_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV27_A::B_0x1
    }
}
#[doc = "Field `PRIV27` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV27_W<'a, REG> = crate::BitWriter<'a, REG, PRIV27_A>;
impl<'a, REG> PRIV27_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV27_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV27_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV28_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV28_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV28` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV28_R = crate::BitReader<PRIV28_A>;
impl PRIV28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV28_A {
        match self.bits {
            false => PRIV28_A::B_0x0,
            true => PRIV28_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV28_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV28_A::B_0x1
    }
}
#[doc = "Field `PRIV28` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV28_W<'a, REG> = crate::BitWriter<'a, REG, PRIV28_A>;
impl<'a, REG> PRIV28_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV28_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV28_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV29_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV29_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV29` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV29_R = crate::BitReader<PRIV29_A>;
impl PRIV29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV29_A {
        match self.bits {
            false => PRIV29_A::B_0x0,
            true => PRIV29_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV29_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV29_A::B_0x1
    }
}
#[doc = "Field `PRIV29` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV29_W<'a, REG> = crate::BitWriter<'a, REG, PRIV29_A>;
impl<'a, REG> PRIV29_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV29_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV29_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV30_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV30_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV30` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV30_R = crate::BitReader<PRIV30_A>;
impl PRIV30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV30_A {
        match self.bits {
            false => PRIV30_A::B_0x0,
            true => PRIV30_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV30_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV30_A::B_0x1
    }
}
#[doc = "Field `PRIV30` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV30_W<'a, REG> = crate::BitWriter<'a, REG, PRIV30_A>;
impl<'a, REG> PRIV30_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV30_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV30_A::B_0x1)
    }
}
#[doc = "Privileged configuration for block y, belonging to super-block x (y = 31 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV31_A {
    #[doc = "0: Privileged and unprivileged access to block y, belonging to super-block x"]
    B_0x0 = 0,
    #[doc = "1: Only privileged access to block y, belonging to super-block x"]
    B_0x1 = 1,
}
impl From<PRIV31_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV31` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV31_R = crate::BitReader<PRIV31_A>;
impl PRIV31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV31_A {
        match self.bits {
            false => PRIV31_A::B_0x0,
            true => PRIV31_A::B_0x1,
        }
    }
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV31_A::B_0x0
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV31_A::B_0x1
    }
}
#[doc = "Field `PRIV31` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
pub type PRIV31_W<'a, REG> = crate::BitWriter<'a, REG, PRIV31_A>;
impl<'a, REG> PRIV31_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Privileged and unprivileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV31_A::B_0x0)
    }
    #[doc = "Only privileged access to block y, belonging to super-block x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV31_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV0(&self) -> PRIV0_R {
        PRIV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV1(&self) -> PRIV1_R {
        PRIV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV2(&self) -> PRIV2_R {
        PRIV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV3(&self) -> PRIV3_R {
        PRIV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV4(&self) -> PRIV4_R {
        PRIV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV5(&self) -> PRIV5_R {
        PRIV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV6(&self) -> PRIV6_R {
        PRIV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV7(&self) -> PRIV7_R {
        PRIV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV8(&self) -> PRIV8_R {
        PRIV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV9(&self) -> PRIV9_R {
        PRIV9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV10(&self) -> PRIV10_R {
        PRIV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV11(&self) -> PRIV11_R {
        PRIV11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV12(&self) -> PRIV12_R {
        PRIV12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV13(&self) -> PRIV13_R {
        PRIV13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV14(&self) -> PRIV14_R {
        PRIV14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV15(&self) -> PRIV15_R {
        PRIV15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV16(&self) -> PRIV16_R {
        PRIV16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV17(&self) -> PRIV17_R {
        PRIV17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV18(&self) -> PRIV18_R {
        PRIV18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV19(&self) -> PRIV19_R {
        PRIV19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV20(&self) -> PRIV20_R {
        PRIV20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV21(&self) -> PRIV21_R {
        PRIV21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV22(&self) -> PRIV22_R {
        PRIV22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV23(&self) -> PRIV23_R {
        PRIV23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV24(&self) -> PRIV24_R {
        PRIV24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV25(&self) -> PRIV25_R {
        PRIV25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV26(&self) -> PRIV26_R {
        PRIV26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV27(&self) -> PRIV27_R {
        PRIV27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV28(&self) -> PRIV28_R {
        PRIV28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV29(&self) -> PRIV29_R {
        PRIV29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV30(&self) -> PRIV30_R {
        PRIV30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV31(&self) -> PRIV31_R {
        PRIV31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV0(&mut self) -> PRIV0_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV1(&mut self) -> PRIV1_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV2(&mut self) -> PRIV2_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV3(&mut self) -> PRIV3_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV4(&mut self) -> PRIV4_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV5(&mut self) -> PRIV5_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV6(&mut self) -> PRIV6_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV7(&mut self) -> PRIV7_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV8(&mut self) -> PRIV8_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV9(&mut self) -> PRIV9_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV10(&mut self) -> PRIV10_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV11(&mut self) -> PRIV11_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV12(&mut self) -> PRIV12_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV13(&mut self) -> PRIV13_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV14(&mut self) -> PRIV14_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV15(&mut self) -> PRIV15_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV16(&mut self) -> PRIV16_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV17(&mut self) -> PRIV17_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV18(&mut self) -> PRIV18_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV18_W::new(self, 18)
    }
    #[doc = "Bit 19 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV19(&mut self) -> PRIV19_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV19_W::new(self, 19)
    }
    #[doc = "Bit 20 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV20(&mut self) -> PRIV20_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV20_W::new(self, 20)
    }
    #[doc = "Bit 21 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV21(&mut self) -> PRIV21_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV21_W::new(self, 21)
    }
    #[doc = "Bit 22 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV22(&mut self) -> PRIV22_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV22_W::new(self, 22)
    }
    #[doc = "Bit 23 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV23(&mut self) -> PRIV23_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV23_W::new(self, 23)
    }
    #[doc = "Bit 24 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV24(&mut self) -> PRIV24_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV24_W::new(self, 24)
    }
    #[doc = "Bit 25 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV25(&mut self) -> PRIV25_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV25_W::new(self, 25)
    }
    #[doc = "Bit 26 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV26(&mut self) -> PRIV26_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV26_W::new(self, 26)
    }
    #[doc = "Bit 27 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV27(&mut self) -> PRIV27_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV27_W::new(self, 27)
    }
    #[doc = "Bit 28 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV28(&mut self) -> PRIV28_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV28_W::new(self, 28)
    }
    #[doc = "Bit 29 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV29(&mut self) -> PRIV29_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV29_W::new(self, 29)
    }
    #[doc = "Bit 30 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV30(&mut self) -> PRIV30_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV30_W::new(self, 30)
    }
    #[doc = "Bit 31 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0)."]
    #[inline(always)]
    pub fn PRIV31(&mut self) -> PRIV31_W<'_, MPCBB2_PRIVCFGR9_SPEC> {
        PRIV31_W::new(self, 31)
    }
}
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 9 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPCBB2_PRIVCFGR9_SPEC;
impl crate::RegisterSpec for MPCBB2_PRIVCFGR9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpcbb2_privcfgr9::R`](R) reader structure"]
impl crate::Readable for MPCBB2_PRIVCFGR9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mpcbb2_privcfgr9::W`](W) writer structure"]
impl crate::Writable for MPCBB2_PRIVCFGR9_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets MPCBB2_PRIVCFGR9 to value 0"]
impl crate::Resettable for MPCBB2_PRIVCFGR9_SPEC {}
