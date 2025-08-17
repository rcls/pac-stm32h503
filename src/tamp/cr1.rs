#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1_SPEC>;
#[doc = "Tamper detection on TAMP_IN1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1E_A {
    #[doc = "0: Tamper detection on TAMP_IN1 is disabled."]
    B_0x0 = 0,
    #[doc = "1: Tamper detection on TAMP_IN1 is enabled."]
    B_0x1 = 1,
}
impl From<TAMP1E_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1E` reader - Tamper detection on TAMP_IN1 enable"]
pub type TAMP1E_R = crate::BitReader<TAMP1E_A>;
impl TAMP1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1E_A {
        match self.bits {
            false => TAMP1E_A::B_0x0,
            true => TAMP1E_A::B_0x1,
        }
    }
    #[doc = "Tamper detection on TAMP_IN1 is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TAMP1E_A::B_0x0
    }
    #[doc = "Tamper detection on TAMP_IN1 is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TAMP1E_A::B_0x1
    }
}
#[doc = "Field `TAMP1E` writer - Tamper detection on TAMP_IN1 enable"]
pub type TAMP1E_W<'a, REG> = crate::BitWriter<'a, REG, TAMP1E_A>;
impl<'a, REG> TAMP1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper detection on TAMP_IN1 is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1E_A::B_0x0)
    }
    #[doc = "Tamper detection on TAMP_IN1 is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1E_A::B_0x1)
    }
}
#[doc = "Tamper detection on TAMP_IN2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP2E_A {
    #[doc = "0: Tamper detection on TAMP_IN2 is disabled."]
    B_0x0 = 0,
    #[doc = "1: Tamper detection on TAMP_IN2 is enabled."]
    B_0x1 = 1,
}
impl From<TAMP2E_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP2E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP2E` reader - Tamper detection on TAMP_IN2 enable"]
pub type TAMP2E_R = crate::BitReader<TAMP2E_A>;
impl TAMP2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP2E_A {
        match self.bits {
            false => TAMP2E_A::B_0x0,
            true => TAMP2E_A::B_0x1,
        }
    }
    #[doc = "Tamper detection on TAMP_IN2 is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TAMP2E_A::B_0x0
    }
    #[doc = "Tamper detection on TAMP_IN2 is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TAMP2E_A::B_0x1
    }
}
#[doc = "Field `TAMP2E` writer - Tamper detection on TAMP_IN2 enable"]
pub type TAMP2E_W<'a, REG> = crate::BitWriter<'a, REG, TAMP2E_A>;
impl<'a, REG> TAMP2E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper detection on TAMP_IN2 is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP2E_A::B_0x0)
    }
    #[doc = "Tamper detection on TAMP_IN2 is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP2E_A::B_0x1)
    }
}
#[doc = "Internal tamper 1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP1E_A {
    #[doc = "0: Internal tamper 1 disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 1 enabled."]
    B_0x1 = 1,
}
impl From<ITAMP1E_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP1E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP1E` reader - Internal tamper 1 enable"]
pub type ITAMP1E_R = crate::BitReader<ITAMP1E_A>;
impl ITAMP1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP1E_A {
        match self.bits {
            false => ITAMP1E_A::B_0x0,
            true => ITAMP1E_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 1 disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP1E_A::B_0x0
    }
    #[doc = "Internal tamper 1 enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP1E_A::B_0x1
    }
}
#[doc = "Field `ITAMP1E` writer - Internal tamper 1 enable"]
pub type ITAMP1E_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP1E_A>;
impl<'a, REG> ITAMP1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 1 disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP1E_A::B_0x0)
    }
    #[doc = "Internal tamper 1 enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP1E_A::B_0x1)
    }
}
#[doc = "Internal tamper 2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP2E_A {
    #[doc = "0: Internal tamper 2 disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 2 enabled."]
    B_0x1 = 1,
}
impl From<ITAMP2E_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP2E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP2E` reader - Internal tamper 2 enable"]
pub type ITAMP2E_R = crate::BitReader<ITAMP2E_A>;
impl ITAMP2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP2E_A {
        match self.bits {
            false => ITAMP2E_A::B_0x0,
            true => ITAMP2E_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 2 disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP2E_A::B_0x0
    }
    #[doc = "Internal tamper 2 enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP2E_A::B_0x1
    }
}
#[doc = "Field `ITAMP2E` writer - Internal tamper 2 enable"]
pub type ITAMP2E_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP2E_A>;
impl<'a, REG> ITAMP2E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 2 disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP2E_A::B_0x0)
    }
    #[doc = "Internal tamper 2 enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP2E_A::B_0x1)
    }
}
#[doc = "Internal tamper 3 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP3E_A {
    #[doc = "0: Internal tamper 3 disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 3 enabled."]
    B_0x1 = 1,
}
impl From<ITAMP3E_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP3E` reader - Internal tamper 3 enable"]
pub type ITAMP3E_R = crate::BitReader<ITAMP3E_A>;
impl ITAMP3E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP3E_A {
        match self.bits {
            false => ITAMP3E_A::B_0x0,
            true => ITAMP3E_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 3 disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP3E_A::B_0x0
    }
    #[doc = "Internal tamper 3 enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP3E_A::B_0x1
    }
}
#[doc = "Field `ITAMP3E` writer - Internal tamper 3 enable"]
pub type ITAMP3E_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP3E_A>;
impl<'a, REG> ITAMP3E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 3 disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP3E_A::B_0x0)
    }
    #[doc = "Internal tamper 3 enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP3E_A::B_0x1)
    }
}
#[doc = "Internal tamper 4 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP4E_A {
    #[doc = "0: Internal tamper 4 disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 4 enabled."]
    B_0x1 = 1,
}
impl From<ITAMP4E_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP4E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP4E` reader - Internal tamper 4 enable"]
pub type ITAMP4E_R = crate::BitReader<ITAMP4E_A>;
impl ITAMP4E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP4E_A {
        match self.bits {
            false => ITAMP4E_A::B_0x0,
            true => ITAMP4E_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 4 disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP4E_A::B_0x0
    }
    #[doc = "Internal tamper 4 enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP4E_A::B_0x1
    }
}
#[doc = "Field `ITAMP4E` writer - Internal tamper 4 enable"]
pub type ITAMP4E_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP4E_A>;
impl<'a, REG> ITAMP4E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 4 disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP4E_A::B_0x0)
    }
    #[doc = "Internal tamper 4 enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP4E_A::B_0x1)
    }
}
#[doc = "Internal tamper 5 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP5E_A {
    #[doc = "0: Internal tamper 5 disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 5 enabled."]
    B_0x1 = 1,
}
impl From<ITAMP5E_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP5E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP5E` reader - Internal tamper 5 enable"]
pub type ITAMP5E_R = crate::BitReader<ITAMP5E_A>;
impl ITAMP5E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP5E_A {
        match self.bits {
            false => ITAMP5E_A::B_0x0,
            true => ITAMP5E_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 5 disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP5E_A::B_0x0
    }
    #[doc = "Internal tamper 5 enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP5E_A::B_0x1
    }
}
#[doc = "Field `ITAMP5E` writer - Internal tamper 5 enable"]
pub type ITAMP5E_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP5E_A>;
impl<'a, REG> ITAMP5E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 5 disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP5E_A::B_0x0)
    }
    #[doc = "Internal tamper 5 enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP5E_A::B_0x1)
    }
}
#[doc = "Internal tamper 6 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP6E_A {
    #[doc = "0: Internal tamper 6 disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 6 enabled."]
    B_0x1 = 1,
}
impl From<ITAMP6E_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP6E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP6E` reader - Internal tamper 6 enable"]
pub type ITAMP6E_R = crate::BitReader<ITAMP6E_A>;
impl ITAMP6E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP6E_A {
        match self.bits {
            false => ITAMP6E_A::B_0x0,
            true => ITAMP6E_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 6 disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP6E_A::B_0x0
    }
    #[doc = "Internal tamper 6 enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP6E_A::B_0x1
    }
}
#[doc = "Field `ITAMP6E` writer - Internal tamper 6 enable"]
pub type ITAMP6E_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP6E_A>;
impl<'a, REG> ITAMP6E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 6 disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP6E_A::B_0x0)
    }
    #[doc = "Internal tamper 6 enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP6E_A::B_0x1)
    }
}
#[doc = "Internal tamper 7 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP7E_A {
    #[doc = "0: Internal tamper 7 disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 7 enabled"]
    B_0x1 = 1,
}
impl From<ITAMP7E_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP7E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP7E` reader - Internal tamper 7 enable"]
pub type ITAMP7E_R = crate::BitReader<ITAMP7E_A>;
impl ITAMP7E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP7E_A {
        match self.bits {
            false => ITAMP7E_A::B_0x0,
            true => ITAMP7E_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 7 disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP7E_A::B_0x0
    }
    #[doc = "Internal tamper 7 enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP7E_A::B_0x1
    }
}
#[doc = "Field `ITAMP7E` writer - Internal tamper 7 enable"]
pub type ITAMP7E_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP7E_A>;
impl<'a, REG> ITAMP7E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 7 disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP7E_A::B_0x0)
    }
    #[doc = "Internal tamper 7 enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP7E_A::B_0x1)
    }
}
#[doc = "Internal tamper 8 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP8E_A {
    #[doc = "0: Internal tamper 8 disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 8 enabled."]
    B_0x1 = 1,
}
impl From<ITAMP8E_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP8E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP8E` reader - Internal tamper 8 enable"]
pub type ITAMP8E_R = crate::BitReader<ITAMP8E_A>;
impl ITAMP8E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP8E_A {
        match self.bits {
            false => ITAMP8E_A::B_0x0,
            true => ITAMP8E_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 8 disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP8E_A::B_0x0
    }
    #[doc = "Internal tamper 8 enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP8E_A::B_0x1
    }
}
#[doc = "Field `ITAMP8E` writer - Internal tamper 8 enable"]
pub type ITAMP8E_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP8E_A>;
impl<'a, REG> ITAMP8E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 8 disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP8E_A::B_0x0)
    }
    #[doc = "Internal tamper 8 enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP8E_A::B_0x1)
    }
}
#[doc = "Internal tamper 9 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP9E_A {
    #[doc = "0: Internal tamper 9 disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 9 enabled."]
    B_0x1 = 1,
}
impl From<ITAMP9E_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP9E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP9E` reader - Internal tamper 9 enable"]
pub type ITAMP9E_R = crate::BitReader<ITAMP9E_A>;
impl ITAMP9E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP9E_A {
        match self.bits {
            false => ITAMP9E_A::B_0x0,
            true => ITAMP9E_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 9 disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP9E_A::B_0x0
    }
    #[doc = "Internal tamper 9 enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP9E_A::B_0x1
    }
}
#[doc = "Field `ITAMP9E` writer - Internal tamper 9 enable"]
pub type ITAMP9E_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP9E_A>;
impl<'a, REG> ITAMP9E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 9 disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP9E_A::B_0x0)
    }
    #[doc = "Internal tamper 9 enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP9E_A::B_0x1)
    }
}
#[doc = "Internal tamper 11 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP11E_A {
    #[doc = "0: Internal tamper 11 disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 11 enabled."]
    B_0x1 = 1,
}
impl From<ITAMP11E_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP11E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP11E` reader - Internal tamper 11 enable"]
pub type ITAMP11E_R = crate::BitReader<ITAMP11E_A>;
impl ITAMP11E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP11E_A {
        match self.bits {
            false => ITAMP11E_A::B_0x0,
            true => ITAMP11E_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 11 disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP11E_A::B_0x0
    }
    #[doc = "Internal tamper 11 enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP11E_A::B_0x1
    }
}
#[doc = "Field `ITAMP11E` writer - Internal tamper 11 enable"]
pub type ITAMP11E_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP11E_A>;
impl<'a, REG> ITAMP11E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 11 disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP11E_A::B_0x0)
    }
    #[doc = "Internal tamper 11 enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP11E_A::B_0x1)
    }
}
#[doc = "Internal tamper 12 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP12E_A {
    #[doc = "0: Internal tamper 12 disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 12 enabled."]
    B_0x1 = 1,
}
impl From<ITAMP12E_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP12E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP12E` reader - Internal tamper 12 enable"]
pub type ITAMP12E_R = crate::BitReader<ITAMP12E_A>;
impl ITAMP12E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP12E_A {
        match self.bits {
            false => ITAMP12E_A::B_0x0,
            true => ITAMP12E_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 12 disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP12E_A::B_0x0
    }
    #[doc = "Internal tamper 12 enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP12E_A::B_0x1
    }
}
#[doc = "Field `ITAMP12E` writer - Internal tamper 12 enable"]
pub type ITAMP12E_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP12E_A>;
impl<'a, REG> ITAMP12E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 12 disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP12E_A::B_0x0)
    }
    #[doc = "Internal tamper 12 enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP12E_A::B_0x1)
    }
}
#[doc = "Internal tamper 13 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP13E_A {
    #[doc = "0: Internal tamper 13 disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 13 enabled."]
    B_0x1 = 1,
}
impl From<ITAMP13E_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP13E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP13E` reader - Internal tamper 13 enable"]
pub type ITAMP13E_R = crate::BitReader<ITAMP13E_A>;
impl ITAMP13E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP13E_A {
        match self.bits {
            false => ITAMP13E_A::B_0x0,
            true => ITAMP13E_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 13 disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP13E_A::B_0x0
    }
    #[doc = "Internal tamper 13 enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP13E_A::B_0x1
    }
}
#[doc = "Field `ITAMP13E` writer - Internal tamper 13 enable"]
pub type ITAMP13E_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP13E_A>;
impl<'a, REG> ITAMP13E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 13 disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP13E_A::B_0x0)
    }
    #[doc = "Internal tamper 13 enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP13E_A::B_0x1)
    }
}
#[doc = "Internal tamper 15 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP15E_A {
    #[doc = "0: Internal tamper 15 disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 15 enabled."]
    B_0x1 = 1,
}
impl From<ITAMP15E_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP15E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP15E` reader - Internal tamper 15 enable"]
pub type ITAMP15E_R = crate::BitReader<ITAMP15E_A>;
impl ITAMP15E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP15E_A {
        match self.bits {
            false => ITAMP15E_A::B_0x0,
            true => ITAMP15E_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 15 disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP15E_A::B_0x0
    }
    #[doc = "Internal tamper 15 enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP15E_A::B_0x1
    }
}
#[doc = "Field `ITAMP15E` writer - Internal tamper 15 enable"]
pub type ITAMP15E_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP15E_A>;
impl<'a, REG> ITAMP15E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 15 disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP15E_A::B_0x0)
    }
    #[doc = "Internal tamper 15 enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP15E_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Tamper detection on TAMP_IN1 enable"]
    #[inline(always)]
    pub fn TAMP1E(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper detection on TAMP_IN2 enable"]
    #[inline(always)]
    pub fn TAMP2E(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Internal tamper 1 enable"]
    #[inline(always)]
    pub fn ITAMP1E(&self) -> ITAMP1E_R {
        ITAMP1E_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Internal tamper 2 enable"]
    #[inline(always)]
    pub fn ITAMP2E(&self) -> ITAMP2E_R {
        ITAMP2E_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Internal tamper 3 enable"]
    #[inline(always)]
    pub fn ITAMP3E(&self) -> ITAMP3E_R {
        ITAMP3E_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Internal tamper 4 enable"]
    #[inline(always)]
    pub fn ITAMP4E(&self) -> ITAMP4E_R {
        ITAMP4E_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Internal tamper 5 enable"]
    #[inline(always)]
    pub fn ITAMP5E(&self) -> ITAMP5E_R {
        ITAMP5E_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Internal tamper 6 enable"]
    #[inline(always)]
    pub fn ITAMP6E(&self) -> ITAMP6E_R {
        ITAMP6E_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Internal tamper 7 enable"]
    #[inline(always)]
    pub fn ITAMP7E(&self) -> ITAMP7E_R {
        ITAMP7E_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Internal tamper 8 enable"]
    #[inline(always)]
    pub fn ITAMP8E(&self) -> ITAMP8E_R {
        ITAMP8E_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Internal tamper 9 enable"]
    #[inline(always)]
    pub fn ITAMP9E(&self) -> ITAMP9E_R {
        ITAMP9E_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Internal tamper 11 enable"]
    #[inline(always)]
    pub fn ITAMP11E(&self) -> ITAMP11E_R {
        ITAMP11E_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Internal tamper 12 enable"]
    #[inline(always)]
    pub fn ITAMP12E(&self) -> ITAMP12E_R {
        ITAMP12E_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Internal tamper 13 enable"]
    #[inline(always)]
    pub fn ITAMP13E(&self) -> ITAMP13E_R {
        ITAMP13E_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Internal tamper 15 enable"]
    #[inline(always)]
    pub fn ITAMP15E(&self) -> ITAMP15E_R {
        ITAMP15E_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper detection on TAMP_IN1 enable"]
    #[inline(always)]
    pub fn TAMP1E(&mut self) -> TAMP1E_W<'_, CR1_SPEC> {
        TAMP1E_W::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper detection on TAMP_IN2 enable"]
    #[inline(always)]
    pub fn TAMP2E(&mut self) -> TAMP2E_W<'_, CR1_SPEC> {
        TAMP2E_W::new(self, 1)
    }
    #[doc = "Bit 16 - Internal tamper 1 enable"]
    #[inline(always)]
    pub fn ITAMP1E(&mut self) -> ITAMP1E_W<'_, CR1_SPEC> {
        ITAMP1E_W::new(self, 16)
    }
    #[doc = "Bit 17 - Internal tamper 2 enable"]
    #[inline(always)]
    pub fn ITAMP2E(&mut self) -> ITAMP2E_W<'_, CR1_SPEC> {
        ITAMP2E_W::new(self, 17)
    }
    #[doc = "Bit 18 - Internal tamper 3 enable"]
    #[inline(always)]
    pub fn ITAMP3E(&mut self) -> ITAMP3E_W<'_, CR1_SPEC> {
        ITAMP3E_W::new(self, 18)
    }
    #[doc = "Bit 19 - Internal tamper 4 enable"]
    #[inline(always)]
    pub fn ITAMP4E(&mut self) -> ITAMP4E_W<'_, CR1_SPEC> {
        ITAMP4E_W::new(self, 19)
    }
    #[doc = "Bit 20 - Internal tamper 5 enable"]
    #[inline(always)]
    pub fn ITAMP5E(&mut self) -> ITAMP5E_W<'_, CR1_SPEC> {
        ITAMP5E_W::new(self, 20)
    }
    #[doc = "Bit 21 - Internal tamper 6 enable"]
    #[inline(always)]
    pub fn ITAMP6E(&mut self) -> ITAMP6E_W<'_, CR1_SPEC> {
        ITAMP6E_W::new(self, 21)
    }
    #[doc = "Bit 22 - Internal tamper 7 enable"]
    #[inline(always)]
    pub fn ITAMP7E(&mut self) -> ITAMP7E_W<'_, CR1_SPEC> {
        ITAMP7E_W::new(self, 22)
    }
    #[doc = "Bit 23 - Internal tamper 8 enable"]
    #[inline(always)]
    pub fn ITAMP8E(&mut self) -> ITAMP8E_W<'_, CR1_SPEC> {
        ITAMP8E_W::new(self, 23)
    }
    #[doc = "Bit 24 - Internal tamper 9 enable"]
    #[inline(always)]
    pub fn ITAMP9E(&mut self) -> ITAMP9E_W<'_, CR1_SPEC> {
        ITAMP9E_W::new(self, 24)
    }
    #[doc = "Bit 26 - Internal tamper 11 enable"]
    #[inline(always)]
    pub fn ITAMP11E(&mut self) -> ITAMP11E_W<'_, CR1_SPEC> {
        ITAMP11E_W::new(self, 26)
    }
    #[doc = "Bit 27 - Internal tamper 12 enable"]
    #[inline(always)]
    pub fn ITAMP12E(&mut self) -> ITAMP12E_W<'_, CR1_SPEC> {
        ITAMP12E_W::new(self, 27)
    }
    #[doc = "Bit 28 - Internal tamper 13 enable"]
    #[inline(always)]
    pub fn ITAMP13E(&mut self) -> ITAMP13E_W<'_, CR1_SPEC> {
        ITAMP13E_W::new(self, 28)
    }
    #[doc = "Bit 30 - Internal tamper 15 enable"]
    #[inline(always)]
    pub fn ITAMP15E(&mut self) -> ITAMP15E_W<'_, CR1_SPEC> {
        ITAMP15E_W::new(self, 30)
    }
}
#[doc = "TAMP control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {}
