#[doc = "Register `CR3` reader"]
pub type R = crate::R<CR3_SPEC>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<CR3_SPEC>;
#[doc = "Internal Tamper 1 no erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP1NOER_A {
    #[doc = "0: Internal Tamper 1 event erases the backup registers and all device secrets."]
    B_0x0 = 0,
    #[doc = "1: Internal Tamper 1 event does not erase the backup registers and device secrets."]
    B_0x1 = 1,
}
impl From<ITAMP1NOER_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP1NOER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP1NOER` reader - Internal Tamper 1 no erase"]
pub type ITAMP1NOER_R = crate::BitReader<ITAMP1NOER_A>;
impl ITAMP1NOER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP1NOER_A {
        match self.bits {
            false => ITAMP1NOER_A::B_0x0,
            true => ITAMP1NOER_A::B_0x1,
        }
    }
    #[doc = "Internal Tamper 1 event erases the backup registers and all device secrets."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP1NOER_A::B_0x0
    }
    #[doc = "Internal Tamper 1 event does not erase the backup registers and device secrets."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP1NOER_A::B_0x1
    }
}
#[doc = "Field `ITAMP1NOER` writer - Internal Tamper 1 no erase"]
pub type ITAMP1NOER_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP1NOER_A>;
impl<'a, REG> ITAMP1NOER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal Tamper 1 event erases the backup registers and all device secrets."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP1NOER_A::B_0x0)
    }
    #[doc = "Internal Tamper 1 event does not erase the backup registers and device secrets."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP1NOER_A::B_0x1)
    }
}
#[doc = "Internal Tamper 2 no erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP2NOER_A {
    #[doc = "0: Internal Tamper 2 event erases the backup registers and all device secretssup(1)/sup."]
    B_0x0 = 0,
    #[doc = "1: Internal Tamper 2 event does not erase the backup registers and device secretssup(2)/sup."]
    B_0x1 = 1,
}
impl From<ITAMP2NOER_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP2NOER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP2NOER` reader - Internal Tamper 2 no erase"]
pub type ITAMP2NOER_R = crate::BitReader<ITAMP2NOER_A>;
impl ITAMP2NOER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP2NOER_A {
        match self.bits {
            false => ITAMP2NOER_A::B_0x0,
            true => ITAMP2NOER_A::B_0x1,
        }
    }
    #[doc = "Internal Tamper 2 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP2NOER_A::B_0x0
    }
    #[doc = "Internal Tamper 2 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP2NOER_A::B_0x1
    }
}
#[doc = "Field `ITAMP2NOER` writer - Internal Tamper 2 no erase"]
pub type ITAMP2NOER_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP2NOER_A>;
impl<'a, REG> ITAMP2NOER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal Tamper 2 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP2NOER_A::B_0x0)
    }
    #[doc = "Internal Tamper 2 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP2NOER_A::B_0x1)
    }
}
#[doc = "Internal Tamper 3 no erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP3NOER_A {
    #[doc = "0: Internal Tamper 3 event erases the backup registers and all device secretssup(1)/sup."]
    B_0x0 = 0,
    #[doc = "1: Internal Tamper 3 event does not erase the backup registers and device secretssup(2)/sup."]
    B_0x1 = 1,
}
impl From<ITAMP3NOER_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3NOER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP3NOER` reader - Internal Tamper 3 no erase"]
pub type ITAMP3NOER_R = crate::BitReader<ITAMP3NOER_A>;
impl ITAMP3NOER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP3NOER_A {
        match self.bits {
            false => ITAMP3NOER_A::B_0x0,
            true => ITAMP3NOER_A::B_0x1,
        }
    }
    #[doc = "Internal Tamper 3 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP3NOER_A::B_0x0
    }
    #[doc = "Internal Tamper 3 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP3NOER_A::B_0x1
    }
}
#[doc = "Field `ITAMP3NOER` writer - Internal Tamper 3 no erase"]
pub type ITAMP3NOER_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP3NOER_A>;
impl<'a, REG> ITAMP3NOER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal Tamper 3 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP3NOER_A::B_0x0)
    }
    #[doc = "Internal Tamper 3 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP3NOER_A::B_0x1)
    }
}
#[doc = "Internal Tamper 4 no erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP4NOER_A {
    #[doc = "0: Internal Tamper 4 event erases the backup registers and all device secretssup(1)/sup."]
    B_0x0 = 0,
    #[doc = "1: Internal Tamper 4 event does not erase the backup registers and device secretssup(2)/sup."]
    B_0x1 = 1,
}
impl From<ITAMP4NOER_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP4NOER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP4NOER` reader - Internal Tamper 4 no erase"]
pub type ITAMP4NOER_R = crate::BitReader<ITAMP4NOER_A>;
impl ITAMP4NOER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP4NOER_A {
        match self.bits {
            false => ITAMP4NOER_A::B_0x0,
            true => ITAMP4NOER_A::B_0x1,
        }
    }
    #[doc = "Internal Tamper 4 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP4NOER_A::B_0x0
    }
    #[doc = "Internal Tamper 4 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP4NOER_A::B_0x1
    }
}
#[doc = "Field `ITAMP4NOER` writer - Internal Tamper 4 no erase"]
pub type ITAMP4NOER_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP4NOER_A>;
impl<'a, REG> ITAMP4NOER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal Tamper 4 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP4NOER_A::B_0x0)
    }
    #[doc = "Internal Tamper 4 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP4NOER_A::B_0x1)
    }
}
#[doc = "Internal Tamper 5 no erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP5NOER_A {
    #[doc = "0: Internal Tamper 5 event erases the backup registers and all device secretssup(1)/sup."]
    B_0x0 = 0,
    #[doc = "1: Internal Tamper 5 event does not erase the backup registers and device secretssup(2)/sup."]
    B_0x1 = 1,
}
impl From<ITAMP5NOER_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP5NOER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP5NOER` reader - Internal Tamper 5 no erase"]
pub type ITAMP5NOER_R = crate::BitReader<ITAMP5NOER_A>;
impl ITAMP5NOER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP5NOER_A {
        match self.bits {
            false => ITAMP5NOER_A::B_0x0,
            true => ITAMP5NOER_A::B_0x1,
        }
    }
    #[doc = "Internal Tamper 5 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP5NOER_A::B_0x0
    }
    #[doc = "Internal Tamper 5 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP5NOER_A::B_0x1
    }
}
#[doc = "Field `ITAMP5NOER` writer - Internal Tamper 5 no erase"]
pub type ITAMP5NOER_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP5NOER_A>;
impl<'a, REG> ITAMP5NOER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal Tamper 5 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP5NOER_A::B_0x0)
    }
    #[doc = "Internal Tamper 5 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP5NOER_A::B_0x1)
    }
}
#[doc = "Internal Tamper 6 no erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP6NOER_A {
    #[doc = "0: Internal Tamper 6 event erases the backup registers and all device secretssup(1)/sup."]
    B_0x0 = 0,
    #[doc = "1: Internal Tamper 6 event does not erase the backup registers and device secretssup(2)/sup."]
    B_0x1 = 1,
}
impl From<ITAMP6NOER_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP6NOER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP6NOER` reader - Internal Tamper 6 no erase"]
pub type ITAMP6NOER_R = crate::BitReader<ITAMP6NOER_A>;
impl ITAMP6NOER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP6NOER_A {
        match self.bits {
            false => ITAMP6NOER_A::B_0x0,
            true => ITAMP6NOER_A::B_0x1,
        }
    }
    #[doc = "Internal Tamper 6 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP6NOER_A::B_0x0
    }
    #[doc = "Internal Tamper 6 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP6NOER_A::B_0x1
    }
}
#[doc = "Field `ITAMP6NOER` writer - Internal Tamper 6 no erase"]
pub type ITAMP6NOER_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP6NOER_A>;
impl<'a, REG> ITAMP6NOER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal Tamper 6 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP6NOER_A::B_0x0)
    }
    #[doc = "Internal Tamper 6 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP6NOER_A::B_0x1)
    }
}
#[doc = "Internal Tamper 7 no erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP7NOER_A {
    #[doc = "0: Internal Tamper 7 event erases the backup registers and all device secretssup(1)/sup."]
    B_0x0 = 0,
    #[doc = "1: Internal Tamper 7 event does not erase the backup registers and device secretssup(2)/sup."]
    B_0x1 = 1,
}
impl From<ITAMP7NOER_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP7NOER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP7NOER` reader - Internal Tamper 7 no erase"]
pub type ITAMP7NOER_R = crate::BitReader<ITAMP7NOER_A>;
impl ITAMP7NOER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP7NOER_A {
        match self.bits {
            false => ITAMP7NOER_A::B_0x0,
            true => ITAMP7NOER_A::B_0x1,
        }
    }
    #[doc = "Internal Tamper 7 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP7NOER_A::B_0x0
    }
    #[doc = "Internal Tamper 7 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP7NOER_A::B_0x1
    }
}
#[doc = "Field `ITAMP7NOER` writer - Internal Tamper 7 no erase"]
pub type ITAMP7NOER_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP7NOER_A>;
impl<'a, REG> ITAMP7NOER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal Tamper 7 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP7NOER_A::B_0x0)
    }
    #[doc = "Internal Tamper 7 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP7NOER_A::B_0x1)
    }
}
#[doc = "Internal Tamper 8 no erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP8NOER_A {
    #[doc = "0: Internal Tamper 8 event erases the backup registers and all device secretssup(1)/sup."]
    B_0x0 = 0,
    #[doc = "1: Internal Tamper 8 event does not erase the backup registers and device secretssup(2)/sup."]
    B_0x1 = 1,
}
impl From<ITAMP8NOER_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP8NOER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP8NOER` reader - Internal Tamper 8 no erase"]
pub type ITAMP8NOER_R = crate::BitReader<ITAMP8NOER_A>;
impl ITAMP8NOER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP8NOER_A {
        match self.bits {
            false => ITAMP8NOER_A::B_0x0,
            true => ITAMP8NOER_A::B_0x1,
        }
    }
    #[doc = "Internal Tamper 8 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP8NOER_A::B_0x0
    }
    #[doc = "Internal Tamper 8 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP8NOER_A::B_0x1
    }
}
#[doc = "Field `ITAMP8NOER` writer - Internal Tamper 8 no erase"]
pub type ITAMP8NOER_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP8NOER_A>;
impl<'a, REG> ITAMP8NOER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal Tamper 8 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP8NOER_A::B_0x0)
    }
    #[doc = "Internal Tamper 8 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP8NOER_A::B_0x1)
    }
}
#[doc = "Internal Tamper 9 no erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP9NOER_A {
    #[doc = "0: Internal Tamper 9 event erases the backup registers and all device secretssup(1)/sup."]
    B_0x0 = 0,
    #[doc = "1: Internal Tamper 9 event does not erase the backup registers and device secretssup(2)/sup."]
    B_0x1 = 1,
}
impl From<ITAMP9NOER_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP9NOER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP9NOER` reader - Internal Tamper 9 no erase"]
pub type ITAMP9NOER_R = crate::BitReader<ITAMP9NOER_A>;
impl ITAMP9NOER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP9NOER_A {
        match self.bits {
            false => ITAMP9NOER_A::B_0x0,
            true => ITAMP9NOER_A::B_0x1,
        }
    }
    #[doc = "Internal Tamper 9 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP9NOER_A::B_0x0
    }
    #[doc = "Internal Tamper 9 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP9NOER_A::B_0x1
    }
}
#[doc = "Field `ITAMP9NOER` writer - Internal Tamper 9 no erase"]
pub type ITAMP9NOER_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP9NOER_A>;
impl<'a, REG> ITAMP9NOER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal Tamper 9 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP9NOER_A::B_0x0)
    }
    #[doc = "Internal Tamper 9 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP9NOER_A::B_0x1)
    }
}
#[doc = "Internal Tamper 11 no erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP11NOER_A {
    #[doc = "0: Internal Tamper 11 event erases the backup registers and all device secretssup(1)/sup."]
    B_0x0 = 0,
    #[doc = "1: Internal Tamper 11 event does not erase the backup registers and device secretssup(2)/sup."]
    B_0x1 = 1,
}
impl From<ITAMP11NOER_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP11NOER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP11NOER` reader - Internal Tamper 11 no erase"]
pub type ITAMP11NOER_R = crate::BitReader<ITAMP11NOER_A>;
impl ITAMP11NOER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP11NOER_A {
        match self.bits {
            false => ITAMP11NOER_A::B_0x0,
            true => ITAMP11NOER_A::B_0x1,
        }
    }
    #[doc = "Internal Tamper 11 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP11NOER_A::B_0x0
    }
    #[doc = "Internal Tamper 11 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP11NOER_A::B_0x1
    }
}
#[doc = "Field `ITAMP11NOER` writer - Internal Tamper 11 no erase"]
pub type ITAMP11NOER_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP11NOER_A>;
impl<'a, REG> ITAMP11NOER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal Tamper 11 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP11NOER_A::B_0x0)
    }
    #[doc = "Internal Tamper 11 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP11NOER_A::B_0x1)
    }
}
#[doc = "Internal Tamper 12 no erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP12NOER_A {
    #[doc = "0: Internal Tamper 12 event erases the backup registers and all device secretssup(1)/sup."]
    B_0x0 = 0,
    #[doc = "1: Internal Tamper 12 event does not erase the backup registers and device secretssup(2)/sup."]
    B_0x1 = 1,
}
impl From<ITAMP12NOER_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP12NOER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP12NOER` reader - Internal Tamper 12 no erase"]
pub type ITAMP12NOER_R = crate::BitReader<ITAMP12NOER_A>;
impl ITAMP12NOER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP12NOER_A {
        match self.bits {
            false => ITAMP12NOER_A::B_0x0,
            true => ITAMP12NOER_A::B_0x1,
        }
    }
    #[doc = "Internal Tamper 12 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP12NOER_A::B_0x0
    }
    #[doc = "Internal Tamper 12 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP12NOER_A::B_0x1
    }
}
#[doc = "Field `ITAMP12NOER` writer - Internal Tamper 12 no erase"]
pub type ITAMP12NOER_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP12NOER_A>;
impl<'a, REG> ITAMP12NOER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal Tamper 12 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP12NOER_A::B_0x0)
    }
    #[doc = "Internal Tamper 12 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP12NOER_A::B_0x1)
    }
}
#[doc = "Internal Tamper 13 no erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP13NOER_A {
    #[doc = "0: Internal Tamper 13 event erases the backup registers and all device secretssup(1)/sup."]
    B_0x0 = 0,
    #[doc = "1: Internal Tamper 13 event does not erase the backup registers and device secretssup(2)/sup."]
    B_0x1 = 1,
}
impl From<ITAMP13NOER_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP13NOER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP13NOER` reader - Internal Tamper 13 no erase"]
pub type ITAMP13NOER_R = crate::BitReader<ITAMP13NOER_A>;
impl ITAMP13NOER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP13NOER_A {
        match self.bits {
            false => ITAMP13NOER_A::B_0x0,
            true => ITAMP13NOER_A::B_0x1,
        }
    }
    #[doc = "Internal Tamper 13 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP13NOER_A::B_0x0
    }
    #[doc = "Internal Tamper 13 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP13NOER_A::B_0x1
    }
}
#[doc = "Field `ITAMP13NOER` writer - Internal Tamper 13 no erase"]
pub type ITAMP13NOER_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP13NOER_A>;
impl<'a, REG> ITAMP13NOER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal Tamper 13 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP13NOER_A::B_0x0)
    }
    #[doc = "Internal Tamper 13 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP13NOER_A::B_0x1)
    }
}
#[doc = "Internal Tamper 15 no erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP15NOER_A {
    #[doc = "0: Internal Tamper 15 event erases the backup registers and all device secretssup(1)/sup."]
    B_0x0 = 0,
    #[doc = "1: Internal Tamper 15 event does not erase the backup registers and device secretssup(2)/sup."]
    B_0x1 = 1,
}
impl From<ITAMP15NOER_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP15NOER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP15NOER` reader - Internal Tamper 15 no erase"]
pub type ITAMP15NOER_R = crate::BitReader<ITAMP15NOER_A>;
impl ITAMP15NOER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP15NOER_A {
        match self.bits {
            false => ITAMP15NOER_A::B_0x0,
            true => ITAMP15NOER_A::B_0x1,
        }
    }
    #[doc = "Internal Tamper 15 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP15NOER_A::B_0x0
    }
    #[doc = "Internal Tamper 15 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP15NOER_A::B_0x1
    }
}
#[doc = "Field `ITAMP15NOER` writer - Internal Tamper 15 no erase"]
pub type ITAMP15NOER_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP15NOER_A>;
impl<'a, REG> ITAMP15NOER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal Tamper 15 event erases the backup registers and all device secretssup(1)/sup."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP15NOER_A::B_0x0)
    }
    #[doc = "Internal Tamper 15 event does not erase the backup registers and device secretssup(2)/sup."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP15NOER_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Internal Tamper 1 no erase"]
    #[inline(always)]
    pub fn ITAMP1NOER(&self) -> ITAMP1NOER_R {
        ITAMP1NOER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Tamper 2 no erase"]
    #[inline(always)]
    pub fn ITAMP2NOER(&self) -> ITAMP2NOER_R {
        ITAMP2NOER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Internal Tamper 3 no erase"]
    #[inline(always)]
    pub fn ITAMP3NOER(&self) -> ITAMP3NOER_R {
        ITAMP3NOER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Internal Tamper 4 no erase"]
    #[inline(always)]
    pub fn ITAMP4NOER(&self) -> ITAMP4NOER_R {
        ITAMP4NOER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Internal Tamper 5 no erase"]
    #[inline(always)]
    pub fn ITAMP5NOER(&self) -> ITAMP5NOER_R {
        ITAMP5NOER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Internal Tamper 6 no erase"]
    #[inline(always)]
    pub fn ITAMP6NOER(&self) -> ITAMP6NOER_R {
        ITAMP6NOER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Internal Tamper 7 no erase"]
    #[inline(always)]
    pub fn ITAMP7NOER(&self) -> ITAMP7NOER_R {
        ITAMP7NOER_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Internal Tamper 8 no erase"]
    #[inline(always)]
    pub fn ITAMP8NOER(&self) -> ITAMP8NOER_R {
        ITAMP8NOER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Internal Tamper 9 no erase"]
    #[inline(always)]
    pub fn ITAMP9NOER(&self) -> ITAMP9NOER_R {
        ITAMP9NOER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Internal Tamper 11 no erase"]
    #[inline(always)]
    pub fn ITAMP11NOER(&self) -> ITAMP11NOER_R {
        ITAMP11NOER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Internal Tamper 12 no erase"]
    #[inline(always)]
    pub fn ITAMP12NOER(&self) -> ITAMP12NOER_R {
        ITAMP12NOER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Internal Tamper 13 no erase"]
    #[inline(always)]
    pub fn ITAMP13NOER(&self) -> ITAMP13NOER_R {
        ITAMP13NOER_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Internal Tamper 15 no erase"]
    #[inline(always)]
    pub fn ITAMP15NOER(&self) -> ITAMP15NOER_R {
        ITAMP15NOER_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Tamper 1 no erase"]
    #[inline(always)]
    pub fn ITAMP1NOER(&mut self) -> ITAMP1NOER_W<'_, CR3_SPEC> {
        ITAMP1NOER_W::new(self, 0)
    }
    #[doc = "Bit 1 - Internal Tamper 2 no erase"]
    #[inline(always)]
    pub fn ITAMP2NOER(&mut self) -> ITAMP2NOER_W<'_, CR3_SPEC> {
        ITAMP2NOER_W::new(self, 1)
    }
    #[doc = "Bit 2 - Internal Tamper 3 no erase"]
    #[inline(always)]
    pub fn ITAMP3NOER(&mut self) -> ITAMP3NOER_W<'_, CR3_SPEC> {
        ITAMP3NOER_W::new(self, 2)
    }
    #[doc = "Bit 3 - Internal Tamper 4 no erase"]
    #[inline(always)]
    pub fn ITAMP4NOER(&mut self) -> ITAMP4NOER_W<'_, CR3_SPEC> {
        ITAMP4NOER_W::new(self, 3)
    }
    #[doc = "Bit 4 - Internal Tamper 5 no erase"]
    #[inline(always)]
    pub fn ITAMP5NOER(&mut self) -> ITAMP5NOER_W<'_, CR3_SPEC> {
        ITAMP5NOER_W::new(self, 4)
    }
    #[doc = "Bit 5 - Internal Tamper 6 no erase"]
    #[inline(always)]
    pub fn ITAMP6NOER(&mut self) -> ITAMP6NOER_W<'_, CR3_SPEC> {
        ITAMP6NOER_W::new(self, 5)
    }
    #[doc = "Bit 6 - Internal Tamper 7 no erase"]
    #[inline(always)]
    pub fn ITAMP7NOER(&mut self) -> ITAMP7NOER_W<'_, CR3_SPEC> {
        ITAMP7NOER_W::new(self, 6)
    }
    #[doc = "Bit 7 - Internal Tamper 8 no erase"]
    #[inline(always)]
    pub fn ITAMP8NOER(&mut self) -> ITAMP8NOER_W<'_, CR3_SPEC> {
        ITAMP8NOER_W::new(self, 7)
    }
    #[doc = "Bit 8 - Internal Tamper 9 no erase"]
    #[inline(always)]
    pub fn ITAMP9NOER(&mut self) -> ITAMP9NOER_W<'_, CR3_SPEC> {
        ITAMP9NOER_W::new(self, 8)
    }
    #[doc = "Bit 10 - Internal Tamper 11 no erase"]
    #[inline(always)]
    pub fn ITAMP11NOER(&mut self) -> ITAMP11NOER_W<'_, CR3_SPEC> {
        ITAMP11NOER_W::new(self, 10)
    }
    #[doc = "Bit 11 - Internal Tamper 12 no erase"]
    #[inline(always)]
    pub fn ITAMP12NOER(&mut self) -> ITAMP12NOER_W<'_, CR3_SPEC> {
        ITAMP12NOER_W::new(self, 11)
    }
    #[doc = "Bit 12 - Internal Tamper 13 no erase"]
    #[inline(always)]
    pub fn ITAMP13NOER(&mut self) -> ITAMP13NOER_W<'_, CR3_SPEC> {
        ITAMP13NOER_W::new(self, 12)
    }
    #[doc = "Bit 14 - Internal Tamper 15 no erase"]
    #[inline(always)]
    pub fn ITAMP15NOER(&mut self) -> ITAMP15NOER_W<'_, CR3_SPEC> {
        ITAMP15NOER_W::new(self, 14)
    }
}
#[doc = "TAMP control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for CR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for CR3_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for CR3_SPEC {}
