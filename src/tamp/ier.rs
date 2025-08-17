#[doc = "Register `IER` reader"]
pub type R = crate::R<IER_SPEC>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Tamper 1 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1IE_A {
    #[doc = "0: Tamper 1 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Tamper 1 interrupt enabled."]
    B_0x1 = 1,
}
impl From<TAMP1IE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1IE` reader - Tamper 1 interrupt enable"]
pub type TAMP1IE_R = crate::BitReader<TAMP1IE_A>;
impl TAMP1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1IE_A {
        match self.bits {
            false => TAMP1IE_A::B_0x0,
            true => TAMP1IE_A::B_0x1,
        }
    }
    #[doc = "Tamper 1 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TAMP1IE_A::B_0x0
    }
    #[doc = "Tamper 1 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TAMP1IE_A::B_0x1
    }
}
#[doc = "Field `TAMP1IE` writer - Tamper 1 interrupt enable"]
pub type TAMP1IE_W<'a, REG> = crate::BitWriter<'a, REG, TAMP1IE_A>;
impl<'a, REG> TAMP1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 1 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1IE_A::B_0x0)
    }
    #[doc = "Tamper 1 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1IE_A::B_0x1)
    }
}
#[doc = "Tamper 2 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP2IE_A {
    #[doc = "0: Tamper 2 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Tamper 2 interrupt enabled."]
    B_0x1 = 1,
}
impl From<TAMP2IE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP2IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP2IE` reader - Tamper 2 interrupt enable"]
pub type TAMP2IE_R = crate::BitReader<TAMP2IE_A>;
impl TAMP2IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP2IE_A {
        match self.bits {
            false => TAMP2IE_A::B_0x0,
            true => TAMP2IE_A::B_0x1,
        }
    }
    #[doc = "Tamper 2 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TAMP2IE_A::B_0x0
    }
    #[doc = "Tamper 2 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TAMP2IE_A::B_0x1
    }
}
#[doc = "Field `TAMP2IE` writer - Tamper 2 interrupt enable"]
pub type TAMP2IE_W<'a, REG> = crate::BitWriter<'a, REG, TAMP2IE_A>;
impl<'a, REG> TAMP2IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 2 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP2IE_A::B_0x0)
    }
    #[doc = "Tamper 2 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP2IE_A::B_0x1)
    }
}
#[doc = "Internal tamper 1 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP1IE_A {
    #[doc = "0: Internal tamper 1 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 1 interrupt enabled"]
    B_0x1 = 1,
}
impl From<ITAMP1IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP1IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP1IE` reader - Internal tamper 1 interrupt enable"]
pub type ITAMP1IE_R = crate::BitReader<ITAMP1IE_A>;
impl ITAMP1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP1IE_A {
        match self.bits {
            false => ITAMP1IE_A::B_0x0,
            true => ITAMP1IE_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 1 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP1IE_A::B_0x0
    }
    #[doc = "Internal tamper 1 interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP1IE_A::B_0x1
    }
}
#[doc = "Field `ITAMP1IE` writer - Internal tamper 1 interrupt enable"]
pub type ITAMP1IE_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP1IE_A>;
impl<'a, REG> ITAMP1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 1 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP1IE_A::B_0x0)
    }
    #[doc = "Internal tamper 1 interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP1IE_A::B_0x1)
    }
}
#[doc = "Internal tamper 2 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP2IE_A {
    #[doc = "0: Internal tamper 2 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 2 interrupt enabled."]
    B_0x1 = 1,
}
impl From<ITAMP2IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP2IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP2IE` reader - Internal tamper 2 interrupt enable"]
pub type ITAMP2IE_R = crate::BitReader<ITAMP2IE_A>;
impl ITAMP2IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP2IE_A {
        match self.bits {
            false => ITAMP2IE_A::B_0x0,
            true => ITAMP2IE_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 2 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP2IE_A::B_0x0
    }
    #[doc = "Internal tamper 2 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP2IE_A::B_0x1
    }
}
#[doc = "Field `ITAMP2IE` writer - Internal tamper 2 interrupt enable"]
pub type ITAMP2IE_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP2IE_A>;
impl<'a, REG> ITAMP2IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 2 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP2IE_A::B_0x0)
    }
    #[doc = "Internal tamper 2 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP2IE_A::B_0x1)
    }
}
#[doc = "Internal tamper 3 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP3IE_A {
    #[doc = "0: Internal tamper 3 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 3 interrupt enabled."]
    B_0x1 = 1,
}
impl From<ITAMP3IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP3IE` reader - Internal tamper 3 interrupt enable"]
pub type ITAMP3IE_R = crate::BitReader<ITAMP3IE_A>;
impl ITAMP3IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP3IE_A {
        match self.bits {
            false => ITAMP3IE_A::B_0x0,
            true => ITAMP3IE_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 3 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP3IE_A::B_0x0
    }
    #[doc = "Internal tamper 3 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP3IE_A::B_0x1
    }
}
#[doc = "Field `ITAMP3IE` writer - Internal tamper 3 interrupt enable"]
pub type ITAMP3IE_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP3IE_A>;
impl<'a, REG> ITAMP3IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 3 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP3IE_A::B_0x0)
    }
    #[doc = "Internal tamper 3 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP3IE_A::B_0x1)
    }
}
#[doc = "Internal tamper 4 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP4IE_A {
    #[doc = "0: Internal tamper 4 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 4 interrupt enabled."]
    B_0x1 = 1,
}
impl From<ITAMP4IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP4IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP4IE` reader - Internal tamper 4 interrupt enable"]
pub type ITAMP4IE_R = crate::BitReader<ITAMP4IE_A>;
impl ITAMP4IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP4IE_A {
        match self.bits {
            false => ITAMP4IE_A::B_0x0,
            true => ITAMP4IE_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 4 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP4IE_A::B_0x0
    }
    #[doc = "Internal tamper 4 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP4IE_A::B_0x1
    }
}
#[doc = "Field `ITAMP4IE` writer - Internal tamper 4 interrupt enable"]
pub type ITAMP4IE_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP4IE_A>;
impl<'a, REG> ITAMP4IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 4 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP4IE_A::B_0x0)
    }
    #[doc = "Internal tamper 4 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP4IE_A::B_0x1)
    }
}
#[doc = "Internal tamper 5 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP5IE_A {
    #[doc = "0: Internal tamper 5 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 5 interrupt enabled."]
    B_0x1 = 1,
}
impl From<ITAMP5IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP5IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP5IE` reader - Internal tamper 5 interrupt enable"]
pub type ITAMP5IE_R = crate::BitReader<ITAMP5IE_A>;
impl ITAMP5IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP5IE_A {
        match self.bits {
            false => ITAMP5IE_A::B_0x0,
            true => ITAMP5IE_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 5 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP5IE_A::B_0x0
    }
    #[doc = "Internal tamper 5 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP5IE_A::B_0x1
    }
}
#[doc = "Field `ITAMP5IE` writer - Internal tamper 5 interrupt enable"]
pub type ITAMP5IE_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP5IE_A>;
impl<'a, REG> ITAMP5IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 5 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP5IE_A::B_0x0)
    }
    #[doc = "Internal tamper 5 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP5IE_A::B_0x1)
    }
}
#[doc = "Internal tamper 6 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP6IE_A {
    #[doc = "0: Internal tamper 6 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 6 interrupt enabled."]
    B_0x1 = 1,
}
impl From<ITAMP6IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP6IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP6IE` reader - Internal tamper 6 interrupt enable"]
pub type ITAMP6IE_R = crate::BitReader<ITAMP6IE_A>;
impl ITAMP6IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP6IE_A {
        match self.bits {
            false => ITAMP6IE_A::B_0x0,
            true => ITAMP6IE_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 6 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP6IE_A::B_0x0
    }
    #[doc = "Internal tamper 6 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP6IE_A::B_0x1
    }
}
#[doc = "Field `ITAMP6IE` writer - Internal tamper 6 interrupt enable"]
pub type ITAMP6IE_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP6IE_A>;
impl<'a, REG> ITAMP6IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 6 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP6IE_A::B_0x0)
    }
    #[doc = "Internal tamper 6 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP6IE_A::B_0x1)
    }
}
#[doc = "Internal tamper 7 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP7IE_A {
    #[doc = "0: Internal tamper 7 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 7 interrupt enabled."]
    B_0x1 = 1,
}
impl From<ITAMP7IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP7IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP7IE` reader - Internal tamper 7 interrupt enable"]
pub type ITAMP7IE_R = crate::BitReader<ITAMP7IE_A>;
impl ITAMP7IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP7IE_A {
        match self.bits {
            false => ITAMP7IE_A::B_0x0,
            true => ITAMP7IE_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 7 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP7IE_A::B_0x0
    }
    #[doc = "Internal tamper 7 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP7IE_A::B_0x1
    }
}
#[doc = "Field `ITAMP7IE` writer - Internal tamper 7 interrupt enable"]
pub type ITAMP7IE_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP7IE_A>;
impl<'a, REG> ITAMP7IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 7 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP7IE_A::B_0x0)
    }
    #[doc = "Internal tamper 7 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP7IE_A::B_0x1)
    }
}
#[doc = "Internal tamper 8 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP8IE_A {
    #[doc = "0: Internal tamper 8 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 8 interrupt enabled."]
    B_0x1 = 1,
}
impl From<ITAMP8IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP8IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP8IE` reader - Internal tamper 8 interrupt enable"]
pub type ITAMP8IE_R = crate::BitReader<ITAMP8IE_A>;
impl ITAMP8IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP8IE_A {
        match self.bits {
            false => ITAMP8IE_A::B_0x0,
            true => ITAMP8IE_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 8 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP8IE_A::B_0x0
    }
    #[doc = "Internal tamper 8 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP8IE_A::B_0x1
    }
}
#[doc = "Field `ITAMP8IE` writer - Internal tamper 8 interrupt enable"]
pub type ITAMP8IE_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP8IE_A>;
impl<'a, REG> ITAMP8IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 8 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP8IE_A::B_0x0)
    }
    #[doc = "Internal tamper 8 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP8IE_A::B_0x1)
    }
}
#[doc = "Internal tamper 9 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP9IE_A {
    #[doc = "0: Internal tamper 9 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 9 interrupt enabled."]
    B_0x1 = 1,
}
impl From<ITAMP9IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP9IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP9IE` reader - Internal tamper 9 interrupt enable"]
pub type ITAMP9IE_R = crate::BitReader<ITAMP9IE_A>;
impl ITAMP9IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP9IE_A {
        match self.bits {
            false => ITAMP9IE_A::B_0x0,
            true => ITAMP9IE_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 9 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP9IE_A::B_0x0
    }
    #[doc = "Internal tamper 9 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP9IE_A::B_0x1
    }
}
#[doc = "Field `ITAMP9IE` writer - Internal tamper 9 interrupt enable"]
pub type ITAMP9IE_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP9IE_A>;
impl<'a, REG> ITAMP9IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 9 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP9IE_A::B_0x0)
    }
    #[doc = "Internal tamper 9 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP9IE_A::B_0x1)
    }
}
#[doc = "Internal tamper 11 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP11IE_A {
    #[doc = "0: Internal tamper 11 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 11 interrupt enabled."]
    B_0x1 = 1,
}
impl From<ITAMP11IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP11IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP11IE` reader - Internal tamper 11 interrupt enable"]
pub type ITAMP11IE_R = crate::BitReader<ITAMP11IE_A>;
impl ITAMP11IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP11IE_A {
        match self.bits {
            false => ITAMP11IE_A::B_0x0,
            true => ITAMP11IE_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 11 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP11IE_A::B_0x0
    }
    #[doc = "Internal tamper 11 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP11IE_A::B_0x1
    }
}
#[doc = "Field `ITAMP11IE` writer - Internal tamper 11 interrupt enable"]
pub type ITAMP11IE_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP11IE_A>;
impl<'a, REG> ITAMP11IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 11 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP11IE_A::B_0x0)
    }
    #[doc = "Internal tamper 11 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP11IE_A::B_0x1)
    }
}
#[doc = "Internal tamper 12 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP12IE_A {
    #[doc = "0: Internal tamper 12 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 12 interrupt enabled."]
    B_0x1 = 1,
}
impl From<ITAMP12IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP12IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP12IE` reader - Internal tamper 12 interrupt enable"]
pub type ITAMP12IE_R = crate::BitReader<ITAMP12IE_A>;
impl ITAMP12IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP12IE_A {
        match self.bits {
            false => ITAMP12IE_A::B_0x0,
            true => ITAMP12IE_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 12 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP12IE_A::B_0x0
    }
    #[doc = "Internal tamper 12 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP12IE_A::B_0x1
    }
}
#[doc = "Field `ITAMP12IE` writer - Internal tamper 12 interrupt enable"]
pub type ITAMP12IE_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP12IE_A>;
impl<'a, REG> ITAMP12IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 12 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP12IE_A::B_0x0)
    }
    #[doc = "Internal tamper 12 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP12IE_A::B_0x1)
    }
}
#[doc = "Internal tamper 13 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP13IE_A {
    #[doc = "0: Internal tamper 13 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 13 interrupt enabled."]
    B_0x1 = 1,
}
impl From<ITAMP13IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP13IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP13IE` reader - Internal tamper 13 interrupt enable"]
pub type ITAMP13IE_R = crate::BitReader<ITAMP13IE_A>;
impl ITAMP13IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP13IE_A {
        match self.bits {
            false => ITAMP13IE_A::B_0x0,
            true => ITAMP13IE_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 13 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP13IE_A::B_0x0
    }
    #[doc = "Internal tamper 13 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP13IE_A::B_0x1
    }
}
#[doc = "Field `ITAMP13IE` writer - Internal tamper 13 interrupt enable"]
pub type ITAMP13IE_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP13IE_A>;
impl<'a, REG> ITAMP13IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 13 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP13IE_A::B_0x0)
    }
    #[doc = "Internal tamper 13 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP13IE_A::B_0x1)
    }
}
#[doc = "Internal tamper 15 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP15IE_A {
    #[doc = "0: Internal tamper 15 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 15 interrupt enabled."]
    B_0x1 = 1,
}
impl From<ITAMP15IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP15IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP15IE` reader - Internal tamper 15 interrupt enable"]
pub type ITAMP15IE_R = crate::BitReader<ITAMP15IE_A>;
impl ITAMP15IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP15IE_A {
        match self.bits {
            false => ITAMP15IE_A::B_0x0,
            true => ITAMP15IE_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 15 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP15IE_A::B_0x0
    }
    #[doc = "Internal tamper 15 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP15IE_A::B_0x1
    }
}
#[doc = "Field `ITAMP15IE` writer - Internal tamper 15 interrupt enable"]
pub type ITAMP15IE_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP15IE_A>;
impl<'a, REG> ITAMP15IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 15 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP15IE_A::B_0x0)
    }
    #[doc = "Internal tamper 15 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP15IE_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Tamper 1 interrupt enable"]
    #[inline(always)]
    pub fn TAMP1IE(&self) -> TAMP1IE_R {
        TAMP1IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper 2 interrupt enable"]
    #[inline(always)]
    pub fn TAMP2IE(&self) -> TAMP2IE_R {
        TAMP2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Internal tamper 1 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP1IE(&self) -> ITAMP1IE_R {
        ITAMP1IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Internal tamper 2 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP2IE(&self) -> ITAMP2IE_R {
        ITAMP2IE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Internal tamper 3 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP3IE(&self) -> ITAMP3IE_R {
        ITAMP3IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Internal tamper 4 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP4IE(&self) -> ITAMP4IE_R {
        ITAMP4IE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Internal tamper 5 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP5IE(&self) -> ITAMP5IE_R {
        ITAMP5IE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Internal tamper 6 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP6IE(&self) -> ITAMP6IE_R {
        ITAMP6IE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Internal tamper 7 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP7IE(&self) -> ITAMP7IE_R {
        ITAMP7IE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Internal tamper 8 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP8IE(&self) -> ITAMP8IE_R {
        ITAMP8IE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Internal tamper 9 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP9IE(&self) -> ITAMP9IE_R {
        ITAMP9IE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Internal tamper 11 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP11IE(&self) -> ITAMP11IE_R {
        ITAMP11IE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Internal tamper 12 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP12IE(&self) -> ITAMP12IE_R {
        ITAMP12IE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Internal tamper 13 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP13IE(&self) -> ITAMP13IE_R {
        ITAMP13IE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Internal tamper 15 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP15IE(&self) -> ITAMP15IE_R {
        ITAMP15IE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper 1 interrupt enable"]
    #[inline(always)]
    pub fn TAMP1IE(&mut self) -> TAMP1IE_W<'_, IER_SPEC> {
        TAMP1IE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper 2 interrupt enable"]
    #[inline(always)]
    pub fn TAMP2IE(&mut self) -> TAMP2IE_W<'_, IER_SPEC> {
        TAMP2IE_W::new(self, 1)
    }
    #[doc = "Bit 16 - Internal tamper 1 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP1IE(&mut self) -> ITAMP1IE_W<'_, IER_SPEC> {
        ITAMP1IE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Internal tamper 2 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP2IE(&mut self) -> ITAMP2IE_W<'_, IER_SPEC> {
        ITAMP2IE_W::new(self, 17)
    }
    #[doc = "Bit 18 - Internal tamper 3 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP3IE(&mut self) -> ITAMP3IE_W<'_, IER_SPEC> {
        ITAMP3IE_W::new(self, 18)
    }
    #[doc = "Bit 19 - Internal tamper 4 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP4IE(&mut self) -> ITAMP4IE_W<'_, IER_SPEC> {
        ITAMP4IE_W::new(self, 19)
    }
    #[doc = "Bit 20 - Internal tamper 5 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP5IE(&mut self) -> ITAMP5IE_W<'_, IER_SPEC> {
        ITAMP5IE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Internal tamper 6 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP6IE(&mut self) -> ITAMP6IE_W<'_, IER_SPEC> {
        ITAMP6IE_W::new(self, 21)
    }
    #[doc = "Bit 22 - Internal tamper 7 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP7IE(&mut self) -> ITAMP7IE_W<'_, IER_SPEC> {
        ITAMP7IE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Internal tamper 8 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP8IE(&mut self) -> ITAMP8IE_W<'_, IER_SPEC> {
        ITAMP8IE_W::new(self, 23)
    }
    #[doc = "Bit 24 - Internal tamper 9 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP9IE(&mut self) -> ITAMP9IE_W<'_, IER_SPEC> {
        ITAMP9IE_W::new(self, 24)
    }
    #[doc = "Bit 26 - Internal tamper 11 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP11IE(&mut self) -> ITAMP11IE_W<'_, IER_SPEC> {
        ITAMP11IE_W::new(self, 26)
    }
    #[doc = "Bit 27 - Internal tamper 12 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP12IE(&mut self) -> ITAMP12IE_W<'_, IER_SPEC> {
        ITAMP12IE_W::new(self, 27)
    }
    #[doc = "Bit 28 - Internal tamper 13 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP13IE(&mut self) -> ITAMP13IE_W<'_, IER_SPEC> {
        ITAMP13IE_W::new(self, 28)
    }
    #[doc = "Bit 30 - Internal tamper 15 interrupt enable"]
    #[inline(always)]
    pub fn ITAMP15IE(&mut self) -> ITAMP15IE_W<'_, IER_SPEC> {
        ITAMP15IE_W::new(self, 30)
    }
}
#[doc = "TAMP interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {}
