#[doc = "Register `PRIVCFGR2` reader"]
pub type R = crate::R<PRIVCFGR2_SPEC>;
#[doc = "Register `PRIVCFGR2` writer"]
pub type W = crate::W<PRIVCFGR2_SPEC>;
#[doc = "Privilege enable on event input x (x = 42 to 37)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV37_A {
    #[doc = "0: Event privilege disabled (unprivileged)"]
    B_0x0 = 0,
    #[doc = "1: Event privilege enabled (privileged)"]
    B_0x1 = 1,
}
impl From<PRIV37_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV37_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV37` reader - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV37_R = crate::BitReader<PRIV37_A>;
impl PRIV37_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV37_A {
        match self.bits {
            false => PRIV37_A::B_0x0,
            true => PRIV37_A::B_0x1,
        }
    }
    #[doc = "Event privilege disabled (unprivileged)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV37_A::B_0x0
    }
    #[doc = "Event privilege enabled (privileged)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV37_A::B_0x1
    }
}
#[doc = "Field `PRIV37` writer - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV37_W<'a, REG> = crate::BitWriter<'a, REG, PRIV37_A>;
impl<'a, REG> PRIV37_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event privilege disabled (unprivileged)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV37_A::B_0x0)
    }
    #[doc = "Event privilege enabled (privileged)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV37_A::B_0x1)
    }
}
#[doc = "Privilege enable on event input x (x = 42 to 37)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV38_A {
    #[doc = "0: Event privilege disabled (unprivileged)"]
    B_0x0 = 0,
    #[doc = "1: Event privilege enabled (privileged)"]
    B_0x1 = 1,
}
impl From<PRIV38_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV38_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV38` reader - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV38_R = crate::BitReader<PRIV38_A>;
impl PRIV38_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV38_A {
        match self.bits {
            false => PRIV38_A::B_0x0,
            true => PRIV38_A::B_0x1,
        }
    }
    #[doc = "Event privilege disabled (unprivileged)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV38_A::B_0x0
    }
    #[doc = "Event privilege enabled (privileged)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV38_A::B_0x1
    }
}
#[doc = "Field `PRIV38` writer - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV38_W<'a, REG> = crate::BitWriter<'a, REG, PRIV38_A>;
impl<'a, REG> PRIV38_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event privilege disabled (unprivileged)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV38_A::B_0x0)
    }
    #[doc = "Event privilege enabled (privileged)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV38_A::B_0x1)
    }
}
#[doc = "Privilege enable on event input x (x = 42 to 37)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV39_A {
    #[doc = "0: Event privilege disabled (unprivileged)"]
    B_0x0 = 0,
    #[doc = "1: Event privilege enabled (privileged)"]
    B_0x1 = 1,
}
impl From<PRIV39_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV39_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV39` reader - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV39_R = crate::BitReader<PRIV39_A>;
impl PRIV39_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV39_A {
        match self.bits {
            false => PRIV39_A::B_0x0,
            true => PRIV39_A::B_0x1,
        }
    }
    #[doc = "Event privilege disabled (unprivileged)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV39_A::B_0x0
    }
    #[doc = "Event privilege enabled (privileged)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV39_A::B_0x1
    }
}
#[doc = "Field `PRIV39` writer - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV39_W<'a, REG> = crate::BitWriter<'a, REG, PRIV39_A>;
impl<'a, REG> PRIV39_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event privilege disabled (unprivileged)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV39_A::B_0x0)
    }
    #[doc = "Event privilege enabled (privileged)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV39_A::B_0x1)
    }
}
#[doc = "Privilege enable on event input x (x = 42 to 37)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV40_A {
    #[doc = "0: Event privilege disabled (unprivileged)"]
    B_0x0 = 0,
    #[doc = "1: Event privilege enabled (privileged)"]
    B_0x1 = 1,
}
impl From<PRIV40_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV40_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV40` reader - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV40_R = crate::BitReader<PRIV40_A>;
impl PRIV40_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV40_A {
        match self.bits {
            false => PRIV40_A::B_0x0,
            true => PRIV40_A::B_0x1,
        }
    }
    #[doc = "Event privilege disabled (unprivileged)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV40_A::B_0x0
    }
    #[doc = "Event privilege enabled (privileged)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV40_A::B_0x1
    }
}
#[doc = "Field `PRIV40` writer - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV40_W<'a, REG> = crate::BitWriter<'a, REG, PRIV40_A>;
impl<'a, REG> PRIV40_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event privilege disabled (unprivileged)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV40_A::B_0x0)
    }
    #[doc = "Event privilege enabled (privileged)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV40_A::B_0x1)
    }
}
#[doc = "Privilege enable on event input x (x = 42 to 37)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV41_A {
    #[doc = "0: Event privilege disabled (unprivileged)"]
    B_0x0 = 0,
    #[doc = "1: Event privilege enabled (privileged)"]
    B_0x1 = 1,
}
impl From<PRIV41_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV41_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV41` reader - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV41_R = crate::BitReader<PRIV41_A>;
impl PRIV41_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV41_A {
        match self.bits {
            false => PRIV41_A::B_0x0,
            true => PRIV41_A::B_0x1,
        }
    }
    #[doc = "Event privilege disabled (unprivileged)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV41_A::B_0x0
    }
    #[doc = "Event privilege enabled (privileged)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV41_A::B_0x1
    }
}
#[doc = "Field `PRIV41` writer - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV41_W<'a, REG> = crate::BitWriter<'a, REG, PRIV41_A>;
impl<'a, REG> PRIV41_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event privilege disabled (unprivileged)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV41_A::B_0x0)
    }
    #[doc = "Event privilege enabled (privileged)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV41_A::B_0x1)
    }
}
#[doc = "Privilege enable on event input x (x = 42 to 37)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV42_A {
    #[doc = "0: Event privilege disabled (unprivileged)"]
    B_0x0 = 0,
    #[doc = "1: Event privilege enabled (privileged)"]
    B_0x1 = 1,
}
impl From<PRIV42_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV42_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV42` reader - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV42_R = crate::BitReader<PRIV42_A>;
impl PRIV42_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV42_A {
        match self.bits {
            false => PRIV42_A::B_0x0,
            true => PRIV42_A::B_0x1,
        }
    }
    #[doc = "Event privilege disabled (unprivileged)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV42_A::B_0x0
    }
    #[doc = "Event privilege enabled (privileged)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV42_A::B_0x1
    }
}
#[doc = "Field `PRIV42` writer - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV42_W<'a, REG> = crate::BitWriter<'a, REG, PRIV42_A>;
impl<'a, REG> PRIV42_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event privilege disabled (unprivileged)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV42_A::B_0x0)
    }
    #[doc = "Event privilege enabled (privileged)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV42_A::B_0x1)
    }
}
#[doc = "Privilege enable on event input x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV47_A {
    #[doc = "0: Event privilege disabled (unprivileged)"]
    B_0x0 = 0,
    #[doc = "1: Event privilege enabled (privileged)"]
    B_0x1 = 1,
}
impl From<PRIV47_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV47_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV47` reader - Privilege enable on event input x"]
pub type PRIV47_R = crate::BitReader<PRIV47_A>;
impl PRIV47_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV47_A {
        match self.bits {
            false => PRIV47_A::B_0x0,
            true => PRIV47_A::B_0x1,
        }
    }
    #[doc = "Event privilege disabled (unprivileged)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV47_A::B_0x0
    }
    #[doc = "Event privilege enabled (privileged)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV47_A::B_0x1
    }
}
#[doc = "Field `PRIV47` writer - Privilege enable on event input x"]
pub type PRIV47_W<'a, REG> = crate::BitWriter<'a, REG, PRIV47_A>;
impl<'a, REG> PRIV47_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event privilege disabled (unprivileged)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV47_A::B_0x0)
    }
    #[doc = "Event privilege enabled (privileged)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV47_A::B_0x1)
    }
}
#[doc = "Privilege enable on event input x (x = 50 to 49)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV49_A {
    #[doc = "0: Event privilege disabled (unprivileged)"]
    B_0x0 = 0,
    #[doc = "1: Event privilege enabled (privileged)"]
    B_0x1 = 1,
}
impl From<PRIV49_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV49_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV49` reader - Privilege enable on event input x (x = 50 to 49)"]
pub type PRIV49_R = crate::BitReader<PRIV49_A>;
impl PRIV49_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV49_A {
        match self.bits {
            false => PRIV49_A::B_0x0,
            true => PRIV49_A::B_0x1,
        }
    }
    #[doc = "Event privilege disabled (unprivileged)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV49_A::B_0x0
    }
    #[doc = "Event privilege enabled (privileged)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV49_A::B_0x1
    }
}
#[doc = "Field `PRIV49` writer - Privilege enable on event input x (x = 50 to 49)"]
pub type PRIV49_W<'a, REG> = crate::BitWriter<'a, REG, PRIV49_A>;
impl<'a, REG> PRIV49_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event privilege disabled (unprivileged)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV49_A::B_0x0)
    }
    #[doc = "Event privilege enabled (privileged)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV49_A::B_0x1)
    }
}
#[doc = "Privilege enable on event input x (x = 50 to 49)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV50_A {
    #[doc = "0: Event privilege disabled (unprivileged)"]
    B_0x0 = 0,
    #[doc = "1: Event privilege enabled (privileged)"]
    B_0x1 = 1,
}
impl From<PRIV50_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV50_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV50` reader - Privilege enable on event input x (x = 50 to 49)"]
pub type PRIV50_R = crate::BitReader<PRIV50_A>;
impl PRIV50_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV50_A {
        match self.bits {
            false => PRIV50_A::B_0x0,
            true => PRIV50_A::B_0x1,
        }
    }
    #[doc = "Event privilege disabled (unprivileged)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV50_A::B_0x0
    }
    #[doc = "Event privilege enabled (privileged)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV50_A::B_0x1
    }
}
#[doc = "Field `PRIV50` writer - Privilege enable on event input x (x = 50 to 49)"]
pub type PRIV50_W<'a, REG> = crate::BitWriter<'a, REG, PRIV50_A>;
impl<'a, REG> PRIV50_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event privilege disabled (unprivileged)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV50_A::B_0x0)
    }
    #[doc = "Event privilege enabled (privileged)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV50_A::B_0x1)
    }
}
#[doc = "Privilege enable on event input x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV53_A {
    #[doc = "0: Event privilege disabled (unprivileged)"]
    B_0x0 = 0,
    #[doc = "1: Event privilege enabled (privileged)"]
    B_0x1 = 1,
}
impl From<PRIV53_A> for bool {
    #[inline(always)]
    fn from(variant: PRIV53_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV53` reader - Privilege enable on event input x"]
pub type PRIV53_R = crate::BitReader<PRIV53_A>;
impl PRIV53_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV53_A {
        match self.bits {
            false => PRIV53_A::B_0x0,
            true => PRIV53_A::B_0x1,
        }
    }
    #[doc = "Event privilege disabled (unprivileged)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIV53_A::B_0x0
    }
    #[doc = "Event privilege enabled (privileged)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIV53_A::B_0x1
    }
}
#[doc = "Field `PRIV53` writer - Privilege enable on event input x"]
pub type PRIV53_W<'a, REG> = crate::BitWriter<'a, REG, PRIV53_A>;
impl<'a, REG> PRIV53_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event privilege disabled (unprivileged)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV53_A::B_0x0)
    }
    #[doc = "Event privilege enabled (privileged)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV53_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 5 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn PRIV37(&self) -> PRIV37_R {
        PRIV37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn PRIV38(&self) -> PRIV38_R {
        PRIV38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn PRIV39(&self) -> PRIV39_R {
        PRIV39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn PRIV40(&self) -> PRIV40_R {
        PRIV40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn PRIV41(&self) -> PRIV41_R {
        PRIV41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn PRIV42(&self) -> PRIV42_R {
        PRIV42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Privilege enable on event input x"]
    #[inline(always)]
    pub fn PRIV47(&self) -> PRIV47_R {
        PRIV47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Privilege enable on event input x (x = 50 to 49)"]
    #[inline(always)]
    pub fn PRIV49(&self) -> PRIV49_R {
        PRIV49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Privilege enable on event input x (x = 50 to 49)"]
    #[inline(always)]
    pub fn PRIV50(&self) -> PRIV50_R {
        PRIV50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Privilege enable on event input x"]
    #[inline(always)]
    pub fn PRIV53(&self) -> PRIV53_R {
        PRIV53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn PRIV37(&mut self) -> PRIV37_W<'_, PRIVCFGR2_SPEC> {
        PRIV37_W::new(self, 5)
    }
    #[doc = "Bit 6 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn PRIV38(&mut self) -> PRIV38_W<'_, PRIVCFGR2_SPEC> {
        PRIV38_W::new(self, 6)
    }
    #[doc = "Bit 7 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn PRIV39(&mut self) -> PRIV39_W<'_, PRIVCFGR2_SPEC> {
        PRIV39_W::new(self, 7)
    }
    #[doc = "Bit 8 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn PRIV40(&mut self) -> PRIV40_W<'_, PRIVCFGR2_SPEC> {
        PRIV40_W::new(self, 8)
    }
    #[doc = "Bit 9 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn PRIV41(&mut self) -> PRIV41_W<'_, PRIVCFGR2_SPEC> {
        PRIV41_W::new(self, 9)
    }
    #[doc = "Bit 10 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn PRIV42(&mut self) -> PRIV42_W<'_, PRIVCFGR2_SPEC> {
        PRIV42_W::new(self, 10)
    }
    #[doc = "Bit 15 - Privilege enable on event input x"]
    #[inline(always)]
    pub fn PRIV47(&mut self) -> PRIV47_W<'_, PRIVCFGR2_SPEC> {
        PRIV47_W::new(self, 15)
    }
    #[doc = "Bit 17 - Privilege enable on event input x (x = 50 to 49)"]
    #[inline(always)]
    pub fn PRIV49(&mut self) -> PRIV49_W<'_, PRIVCFGR2_SPEC> {
        PRIV49_W::new(self, 17)
    }
    #[doc = "Bit 18 - Privilege enable on event input x (x = 50 to 49)"]
    #[inline(always)]
    pub fn PRIV50(&mut self) -> PRIV50_W<'_, PRIVCFGR2_SPEC> {
        PRIV50_W::new(self, 18)
    }
    #[doc = "Bit 21 - Privilege enable on event input x"]
    #[inline(always)]
    pub fn PRIV53(&mut self) -> PRIV53_W<'_, PRIVCFGR2_SPEC> {
        PRIV53_W::new(self, 21)
    }
}
#[doc = "EXTI privilege configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`privcfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIVCFGR2_SPEC;
impl crate::RegisterSpec for PRIVCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`privcfgr2::R`](R) reader structure"]
impl crate::Readable for PRIVCFGR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`privcfgr2::W`](W) writer structure"]
impl crate::Writable for PRIVCFGR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PRIVCFGR2 to value 0"]
impl crate::Resettable for PRIVCFGR2_SPEC {}
