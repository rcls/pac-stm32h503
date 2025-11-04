#[doc = "Register `M2WPR1` reader"]
pub type R = crate::R<M2WPR1_SPEC>;
#[doc = "Register `M2WPR1` writer"]
pub type W = crate::W<M2WPR1_SPEC>;
#[doc = "SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0WP_A {
    #[doc = "0: Write protection of SRAM2 1-Kbyte page y is disabled."]
    B_0x0 = 0,
    #[doc = "1: Write protection of SRAM2 1-Kbyte page y is enabled."]
    B_0x1 = 1,
}
impl From<P0WP_A> for bool {
    #[inline(always)]
    fn from(variant: P0WP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P0WP_R = crate::BitReader<P0WP_A>;
impl P0WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0WP_A {
        match self.bits {
            false => P0WP_A::B_0x0,
            true => P0WP_A::B_0x1,
        }
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == P0WP_A::B_0x0
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == P0WP_A::B_0x1
    }
}
#[doc = "Field `P0WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P0WP_W<'a, REG> = crate::BitWriter<'a, REG, P0WP_A>;
impl<'a, REG> P0WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(P0WP_A::B_0x0)
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(P0WP_A::B_0x1)
    }
}
#[doc = "SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1WP_A {
    #[doc = "0: Write protection of SRAM2 1-Kbyte page y is disabled."]
    B_0x0 = 0,
    #[doc = "1: Write protection of SRAM2 1-Kbyte page y is enabled."]
    B_0x1 = 1,
}
impl From<P1WP_A> for bool {
    #[inline(always)]
    fn from(variant: P1WP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P1WP_R = crate::BitReader<P1WP_A>;
impl P1WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1WP_A {
        match self.bits {
            false => P1WP_A::B_0x0,
            true => P1WP_A::B_0x1,
        }
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == P1WP_A::B_0x0
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == P1WP_A::B_0x1
    }
}
#[doc = "Field `P1WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P1WP_W<'a, REG> = crate::BitWriter<'a, REG, P1WP_A>;
impl<'a, REG> P1WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(P1WP_A::B_0x0)
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(P1WP_A::B_0x1)
    }
}
#[doc = "SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2WP_A {
    #[doc = "0: Write protection of SRAM2 1-Kbyte page y is disabled."]
    B_0x0 = 0,
    #[doc = "1: Write protection of SRAM2 1-Kbyte page y is enabled."]
    B_0x1 = 1,
}
impl From<P2WP_A> for bool {
    #[inline(always)]
    fn from(variant: P2WP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P2WP_R = crate::BitReader<P2WP_A>;
impl P2WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2WP_A {
        match self.bits {
            false => P2WP_A::B_0x0,
            true => P2WP_A::B_0x1,
        }
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == P2WP_A::B_0x0
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == P2WP_A::B_0x1
    }
}
#[doc = "Field `P2WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P2WP_W<'a, REG> = crate::BitWriter<'a, REG, P2WP_A>;
impl<'a, REG> P2WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(P2WP_A::B_0x0)
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(P2WP_A::B_0x1)
    }
}
#[doc = "SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P3WP_A {
    #[doc = "0: Write protection of SRAM2 1-Kbyte page y is disabled."]
    B_0x0 = 0,
    #[doc = "1: Write protection of SRAM2 1-Kbyte page y is enabled."]
    B_0x1 = 1,
}
impl From<P3WP_A> for bool {
    #[inline(always)]
    fn from(variant: P3WP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P3WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P3WP_R = crate::BitReader<P3WP_A>;
impl P3WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P3WP_A {
        match self.bits {
            false => P3WP_A::B_0x0,
            true => P3WP_A::B_0x1,
        }
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == P3WP_A::B_0x0
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == P3WP_A::B_0x1
    }
}
#[doc = "Field `P3WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P3WP_W<'a, REG> = crate::BitWriter<'a, REG, P3WP_A>;
impl<'a, REG> P3WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(P3WP_A::B_0x0)
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(P3WP_A::B_0x1)
    }
}
#[doc = "SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P4WP_A {
    #[doc = "0: Write protection of SRAM2 1-Kbyte page y is disabled."]
    B_0x0 = 0,
    #[doc = "1: Write protection of SRAM2 1-Kbyte page y is enabled."]
    B_0x1 = 1,
}
impl From<P4WP_A> for bool {
    #[inline(always)]
    fn from(variant: P4WP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P4WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P4WP_R = crate::BitReader<P4WP_A>;
impl P4WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P4WP_A {
        match self.bits {
            false => P4WP_A::B_0x0,
            true => P4WP_A::B_0x1,
        }
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == P4WP_A::B_0x0
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == P4WP_A::B_0x1
    }
}
#[doc = "Field `P4WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P4WP_W<'a, REG> = crate::BitWriter<'a, REG, P4WP_A>;
impl<'a, REG> P4WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(P4WP_A::B_0x0)
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(P4WP_A::B_0x1)
    }
}
#[doc = "SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P5WP_A {
    #[doc = "0: Write protection of SRAM2 1-Kbyte page y is disabled."]
    B_0x0 = 0,
    #[doc = "1: Write protection of SRAM2 1-Kbyte page y is enabled."]
    B_0x1 = 1,
}
impl From<P5WP_A> for bool {
    #[inline(always)]
    fn from(variant: P5WP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P5WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P5WP_R = crate::BitReader<P5WP_A>;
impl P5WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P5WP_A {
        match self.bits {
            false => P5WP_A::B_0x0,
            true => P5WP_A::B_0x1,
        }
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == P5WP_A::B_0x0
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == P5WP_A::B_0x1
    }
}
#[doc = "Field `P5WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P5WP_W<'a, REG> = crate::BitWriter<'a, REG, P5WP_A>;
impl<'a, REG> P5WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(P5WP_A::B_0x0)
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(P5WP_A::B_0x1)
    }
}
#[doc = "SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P6WP_A {
    #[doc = "0: Write protection of SRAM2 1-Kbyte page y is disabled."]
    B_0x0 = 0,
    #[doc = "1: Write protection of SRAM2 1-Kbyte page y is enabled."]
    B_0x1 = 1,
}
impl From<P6WP_A> for bool {
    #[inline(always)]
    fn from(variant: P6WP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P6WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P6WP_R = crate::BitReader<P6WP_A>;
impl P6WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P6WP_A {
        match self.bits {
            false => P6WP_A::B_0x0,
            true => P6WP_A::B_0x1,
        }
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == P6WP_A::B_0x0
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == P6WP_A::B_0x1
    }
}
#[doc = "Field `P6WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P6WP_W<'a, REG> = crate::BitWriter<'a, REG, P6WP_A>;
impl<'a, REG> P6WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(P6WP_A::B_0x0)
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(P6WP_A::B_0x1)
    }
}
#[doc = "SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P7WP_A {
    #[doc = "0: Write protection of SRAM2 1-Kbyte page y is disabled."]
    B_0x0 = 0,
    #[doc = "1: Write protection of SRAM2 1-Kbyte page y is enabled."]
    B_0x1 = 1,
}
impl From<P7WP_A> for bool {
    #[inline(always)]
    fn from(variant: P7WP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P7WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P7WP_R = crate::BitReader<P7WP_A>;
impl P7WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P7WP_A {
        match self.bits {
            false => P7WP_A::B_0x0,
            true => P7WP_A::B_0x1,
        }
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == P7WP_A::B_0x0
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == P7WP_A::B_0x1
    }
}
#[doc = "Field `P7WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P7WP_W<'a, REG> = crate::BitWriter<'a, REG, P7WP_A>;
impl<'a, REG> P7WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(P7WP_A::B_0x0)
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(P7WP_A::B_0x1)
    }
}
#[doc = "SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P8WP_A {
    #[doc = "0: Write protection of SRAM2 1-Kbyte page y is disabled."]
    B_0x0 = 0,
    #[doc = "1: Write protection of SRAM2 1-Kbyte page y is enabled."]
    B_0x1 = 1,
}
impl From<P8WP_A> for bool {
    #[inline(always)]
    fn from(variant: P8WP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P8WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P8WP_R = crate::BitReader<P8WP_A>;
impl P8WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P8WP_A {
        match self.bits {
            false => P8WP_A::B_0x0,
            true => P8WP_A::B_0x1,
        }
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == P8WP_A::B_0x0
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == P8WP_A::B_0x1
    }
}
#[doc = "Field `P8WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P8WP_W<'a, REG> = crate::BitWriter<'a, REG, P8WP_A>;
impl<'a, REG> P8WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(P8WP_A::B_0x0)
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(P8WP_A::B_0x1)
    }
}
#[doc = "SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P9WP_A {
    #[doc = "0: Write protection of SRAM2 1-Kbyte page y is disabled."]
    B_0x0 = 0,
    #[doc = "1: Write protection of SRAM2 1-Kbyte page y is enabled."]
    B_0x1 = 1,
}
impl From<P9WP_A> for bool {
    #[inline(always)]
    fn from(variant: P9WP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P9WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P9WP_R = crate::BitReader<P9WP_A>;
impl P9WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P9WP_A {
        match self.bits {
            false => P9WP_A::B_0x0,
            true => P9WP_A::B_0x1,
        }
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == P9WP_A::B_0x0
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == P9WP_A::B_0x1
    }
}
#[doc = "Field `P9WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P9WP_W<'a, REG> = crate::BitWriter<'a, REG, P9WP_A>;
impl<'a, REG> P9WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(P9WP_A::B_0x0)
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(P9WP_A::B_0x1)
    }
}
#[doc = "SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P10WP_A {
    #[doc = "0: Write protection of SRAM2 1-Kbyte page y is disabled."]
    B_0x0 = 0,
    #[doc = "1: Write protection of SRAM2 1-Kbyte page y is enabled."]
    B_0x1 = 1,
}
impl From<P10WP_A> for bool {
    #[inline(always)]
    fn from(variant: P10WP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P10WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P10WP_R = crate::BitReader<P10WP_A>;
impl P10WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P10WP_A {
        match self.bits {
            false => P10WP_A::B_0x0,
            true => P10WP_A::B_0x1,
        }
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == P10WP_A::B_0x0
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == P10WP_A::B_0x1
    }
}
#[doc = "Field `P10WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P10WP_W<'a, REG> = crate::BitWriter<'a, REG, P10WP_A>;
impl<'a, REG> P10WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(P10WP_A::B_0x0)
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(P10WP_A::B_0x1)
    }
}
#[doc = "SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P11WP_A {
    #[doc = "0: Write protection of SRAM2 1-Kbyte page y is disabled."]
    B_0x0 = 0,
    #[doc = "1: Write protection of SRAM2 1-Kbyte page y is enabled."]
    B_0x1 = 1,
}
impl From<P11WP_A> for bool {
    #[inline(always)]
    fn from(variant: P11WP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P11WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P11WP_R = crate::BitReader<P11WP_A>;
impl P11WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P11WP_A {
        match self.bits {
            false => P11WP_A::B_0x0,
            true => P11WP_A::B_0x1,
        }
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == P11WP_A::B_0x0
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == P11WP_A::B_0x1
    }
}
#[doc = "Field `P11WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P11WP_W<'a, REG> = crate::BitWriter<'a, REG, P11WP_A>;
impl<'a, REG> P11WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(P11WP_A::B_0x0)
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(P11WP_A::B_0x1)
    }
}
#[doc = "SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P12WP_A {
    #[doc = "0: Write protection of SRAM2 1-Kbyte page y is disabled."]
    B_0x0 = 0,
    #[doc = "1: Write protection of SRAM2 1-Kbyte page y is enabled."]
    B_0x1 = 1,
}
impl From<P12WP_A> for bool {
    #[inline(always)]
    fn from(variant: P12WP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P12WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P12WP_R = crate::BitReader<P12WP_A>;
impl P12WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P12WP_A {
        match self.bits {
            false => P12WP_A::B_0x0,
            true => P12WP_A::B_0x1,
        }
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == P12WP_A::B_0x0
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == P12WP_A::B_0x1
    }
}
#[doc = "Field `P12WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P12WP_W<'a, REG> = crate::BitWriter<'a, REG, P12WP_A>;
impl<'a, REG> P12WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(P12WP_A::B_0x0)
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(P12WP_A::B_0x1)
    }
}
#[doc = "SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P13WP_A {
    #[doc = "0: Write protection of SRAM2 1-Kbyte page y is disabled."]
    B_0x0 = 0,
    #[doc = "1: Write protection of SRAM2 1-Kbyte page y is enabled."]
    B_0x1 = 1,
}
impl From<P13WP_A> for bool {
    #[inline(always)]
    fn from(variant: P13WP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P13WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P13WP_R = crate::BitReader<P13WP_A>;
impl P13WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P13WP_A {
        match self.bits {
            false => P13WP_A::B_0x0,
            true => P13WP_A::B_0x1,
        }
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == P13WP_A::B_0x0
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == P13WP_A::B_0x1
    }
}
#[doc = "Field `P13WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P13WP_W<'a, REG> = crate::BitWriter<'a, REG, P13WP_A>;
impl<'a, REG> P13WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(P13WP_A::B_0x0)
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(P13WP_A::B_0x1)
    }
}
#[doc = "SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P14WP_A {
    #[doc = "0: Write protection of SRAM2 1-Kbyte page y is disabled."]
    B_0x0 = 0,
    #[doc = "1: Write protection of SRAM2 1-Kbyte page y is enabled."]
    B_0x1 = 1,
}
impl From<P14WP_A> for bool {
    #[inline(always)]
    fn from(variant: P14WP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P14WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P14WP_R = crate::BitReader<P14WP_A>;
impl P14WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P14WP_A {
        match self.bits {
            false => P14WP_A::B_0x0,
            true => P14WP_A::B_0x1,
        }
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == P14WP_A::B_0x0
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == P14WP_A::B_0x1
    }
}
#[doc = "Field `P14WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P14WP_W<'a, REG> = crate::BitWriter<'a, REG, P14WP_A>;
impl<'a, REG> P14WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(P14WP_A::B_0x0)
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(P14WP_A::B_0x1)
    }
}
#[doc = "SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P15WP_A {
    #[doc = "0: Write protection of SRAM2 1-Kbyte page y is disabled."]
    B_0x0 = 0,
    #[doc = "1: Write protection of SRAM2 1-Kbyte page y is enabled."]
    B_0x1 = 1,
}
impl From<P15WP_A> for bool {
    #[inline(always)]
    fn from(variant: P15WP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P15WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P15WP_R = crate::BitReader<P15WP_A>;
impl P15WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P15WP_A {
        match self.bits {
            false => P15WP_A::B_0x0,
            true => P15WP_A::B_0x1,
        }
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == P15WP_A::B_0x0
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == P15WP_A::B_0x1
    }
}
#[doc = "Field `P15WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P15WP_W<'a, REG> = crate::BitWriter<'a, REG, P15WP_A>;
impl<'a, REG> P15WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write protection of SRAM2 1-Kbyte page y is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(P15WP_A::B_0x0)
    }
    #[doc = "Write protection of SRAM2 1-Kbyte page y is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(P15WP_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P0WP(&self) -> P0WP_R {
        P0WP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P1WP(&self) -> P1WP_R {
        P1WP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P2WP(&self) -> P2WP_R {
        P2WP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P3WP(&self) -> P3WP_R {
        P3WP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P4WP(&self) -> P4WP_R {
        P4WP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P5WP(&self) -> P5WP_R {
        P5WP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P6WP(&self) -> P6WP_R {
        P6WP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P7WP(&self) -> P7WP_R {
        P7WP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P8WP(&self) -> P8WP_R {
        P8WP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P9WP(&self) -> P9WP_R {
        P9WP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P10WP(&self) -> P10WP_R {
        P10WP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P11WP(&self) -> P11WP_R {
        P11WP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P12WP(&self) -> P12WP_R {
        P12WP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P13WP(&self) -> P13WP_R {
        P13WP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P14WP(&self) -> P14WP_R {
        P14WP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P15WP(&self) -> P15WP_R {
        P15WP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P0WP(&mut self) -> P0WP_W<'_, M2WPR1_SPEC> {
        P0WP_W::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P1WP(&mut self) -> P1WP_W<'_, M2WPR1_SPEC> {
        P1WP_W::new(self, 1)
    }
    #[doc = "Bit 2 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P2WP(&mut self) -> P2WP_W<'_, M2WPR1_SPEC> {
        P2WP_W::new(self, 2)
    }
    #[doc = "Bit 3 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P3WP(&mut self) -> P3WP_W<'_, M2WPR1_SPEC> {
        P3WP_W::new(self, 3)
    }
    #[doc = "Bit 4 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P4WP(&mut self) -> P4WP_W<'_, M2WPR1_SPEC> {
        P4WP_W::new(self, 4)
    }
    #[doc = "Bit 5 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P5WP(&mut self) -> P5WP_W<'_, M2WPR1_SPEC> {
        P5WP_W::new(self, 5)
    }
    #[doc = "Bit 6 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P6WP(&mut self) -> P6WP_W<'_, M2WPR1_SPEC> {
        P6WP_W::new(self, 6)
    }
    #[doc = "Bit 7 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P7WP(&mut self) -> P7WP_W<'_, M2WPR1_SPEC> {
        P7WP_W::new(self, 7)
    }
    #[doc = "Bit 8 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P8WP(&mut self) -> P8WP_W<'_, M2WPR1_SPEC> {
        P8WP_W::new(self, 8)
    }
    #[doc = "Bit 9 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P9WP(&mut self) -> P9WP_W<'_, M2WPR1_SPEC> {
        P9WP_W::new(self, 9)
    }
    #[doc = "Bit 10 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P10WP(&mut self) -> P10WP_W<'_, M2WPR1_SPEC> {
        P10WP_W::new(self, 10)
    }
    #[doc = "Bit 11 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P11WP(&mut self) -> P11WP_W<'_, M2WPR1_SPEC> {
        P11WP_W::new(self, 11)
    }
    #[doc = "Bit 12 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P12WP(&mut self) -> P12WP_W<'_, M2WPR1_SPEC> {
        P12WP_W::new(self, 12)
    }
    #[doc = "Bit 13 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P13WP(&mut self) -> P13WP_W<'_, M2WPR1_SPEC> {
        P13WP_W::new(self, 13)
    }
    #[doc = "Bit 14 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P14WP(&mut self) -> P14WP_W<'_, M2WPR1_SPEC> {
        P14WP_W::new(self, 14)
    }
    #[doc = "Bit 15 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn P15WP(&mut self) -> P15WP_W<'_, M2WPR1_SPEC> {
        P15WP_W::new(self, 15)
    }
}
#[doc = "RAMCFG memory 2 write protection register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`m2wpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2wpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M2WPR1_SPEC;
impl crate::RegisterSpec for M2WPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m2wpr1::R`](R) reader structure"]
impl crate::Readable for M2WPR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`m2wpr1::W`](W) writer structure"]
impl crate::Writable for M2WPR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets M2WPR1 to value 0"]
impl crate::Resettable for M2WPR1_SPEC {}
