#[doc = "Register `CICR` reader"]
pub type R = crate::R<CICR_SPEC>;
#[doc = "Register `CICR` writer"]
pub type W = crate::W<CICR_SPEC>;
#[doc = "LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYC_A {
    #[doc = "0: LSIRDYF no effect (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: LSIRDYF cleared"]
    B_0x1 = 1,
}
impl From<LSIRDYC_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYC` reader - LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done."]
pub type LSIRDYC_R = crate::BitReader<LSIRDYC_A>;
impl LSIRDYC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYC_A {
        match self.bits {
            false => LSIRDYC_A::B_0x0,
            true => LSIRDYC_A::B_0x1,
        }
    }
    #[doc = "LSIRDYF no effect (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LSIRDYC_A::B_0x0
    }
    #[doc = "LSIRDYF cleared"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LSIRDYC_A::B_0x1
    }
}
#[doc = "Field `LSIRDYC` writer - LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done."]
pub type LSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYC_A>;
impl<'a, REG> LSIRDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSIRDYF no effect (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYC_A::B_0x0)
    }
    #[doc = "LSIRDYF cleared"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYC_A::B_0x1)
    }
}
#[doc = "LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYC_A {
    #[doc = "0: LSERDYF no effect (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: LSERDYF cleared"]
    B_0x1 = 1,
}
impl From<LSERDYC_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDYC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSERDYC` reader - LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done."]
pub type LSERDYC_R = crate::BitReader<LSERDYC_A>;
impl LSERDYC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSERDYC_A {
        match self.bits {
            false => LSERDYC_A::B_0x0,
            true => LSERDYC_A::B_0x1,
        }
    }
    #[doc = "LSERDYF no effect (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LSERDYC_A::B_0x0
    }
    #[doc = "LSERDYF cleared"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LSERDYC_A::B_0x1
    }
}
#[doc = "Field `LSERDYC` writer - LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done."]
pub type LSERDYC_W<'a, REG> = crate::BitWriter<'a, REG, LSERDYC_A>;
impl<'a, REG> LSERDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSERDYF no effect (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSERDYC_A::B_0x0)
    }
    #[doc = "LSERDYF cleared"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSERDYC_A::B_0x1)
    }
}
#[doc = "HSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSIRDYC_A {
    #[doc = "0: CSIRDYF no effect (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: CSIRDYF cleared"]
    B_0x1 = 1,
}
impl From<CSIRDYC_A> for bool {
    #[inline(always)]
    fn from(variant: CSIRDYC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSIRDYC` reader - HSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done."]
pub type CSIRDYC_R = crate::BitReader<CSIRDYC_A>;
impl CSIRDYC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSIRDYC_A {
        match self.bits {
            false => CSIRDYC_A::B_0x0,
            true => CSIRDYC_A::B_0x1,
        }
    }
    #[doc = "CSIRDYF no effect (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CSIRDYC_A::B_0x0
    }
    #[doc = "CSIRDYF cleared"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CSIRDYC_A::B_0x1
    }
}
#[doc = "Field `CSIRDYC` writer - HSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done."]
pub type CSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG, CSIRDYC_A>;
impl<'a, REG> CSIRDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CSIRDYF no effect (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CSIRDYC_A::B_0x0)
    }
    #[doc = "CSIRDYF cleared"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CSIRDYC_A::B_0x1)
    }
}
#[doc = "HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYC_A {
    #[doc = "0: HSIRDYF no effect (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: HSIRDYF cleared"]
    B_0x1 = 1,
}
impl From<HSIRDYC_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDYC` reader - HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done."]
pub type HSIRDYC_R = crate::BitReader<HSIRDYC_A>;
impl HSIRDYC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDYC_A {
        match self.bits {
            false => HSIRDYC_A::B_0x0,
            true => HSIRDYC_A::B_0x1,
        }
    }
    #[doc = "HSIRDYF no effect (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSIRDYC_A::B_0x0
    }
    #[doc = "HSIRDYF cleared"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSIRDYC_A::B_0x1
    }
}
#[doc = "Field `HSIRDYC` writer - HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done."]
pub type HSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG, HSIRDYC_A>;
impl<'a, REG> HSIRDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSIRDYF no effect (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDYC_A::B_0x0)
    }
    #[doc = "HSIRDYF cleared"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSIRDYC_A::B_0x1)
    }
}
#[doc = "HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYC_A {
    #[doc = "0: HSERDYF no effect (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: HSERDYF cleared"]
    B_0x1 = 1,
}
impl From<HSERDYC_A> for bool {
    #[inline(always)]
    fn from(variant: HSERDYC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSERDYC` reader - HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done."]
pub type HSERDYC_R = crate::BitReader<HSERDYC_A>;
impl HSERDYC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSERDYC_A {
        match self.bits {
            false => HSERDYC_A::B_0x0,
            true => HSERDYC_A::B_0x1,
        }
    }
    #[doc = "HSERDYF no effect (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSERDYC_A::B_0x0
    }
    #[doc = "HSERDYF cleared"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSERDYC_A::B_0x1
    }
}
#[doc = "Field `HSERDYC` writer - HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done."]
pub type HSERDYC_W<'a, REG> = crate::BitWriter<'a, REG, HSERDYC_A>;
impl<'a, REG> HSERDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSERDYF no effect (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSERDYC_A::B_0x0)
    }
    #[doc = "HSERDYF cleared"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSERDYC_A::B_0x1)
    }
}
#[doc = "HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDYC_A {
    #[doc = "0: HSI48RDYF no effect (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: HSI48RDYF cleared"]
    B_0x1 = 1,
}
impl From<HSI48RDYC_A> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDYC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI48RDYC` reader - HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done."]
pub type HSI48RDYC_R = crate::BitReader<HSI48RDYC_A>;
impl HSI48RDYC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSI48RDYC_A {
        match self.bits {
            false => HSI48RDYC_A::B_0x0,
            true => HSI48RDYC_A::B_0x1,
        }
    }
    #[doc = "HSI48RDYF no effect (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSI48RDYC_A::B_0x0
    }
    #[doc = "HSI48RDYF cleared"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSI48RDYC_A::B_0x1
    }
}
#[doc = "Field `HSI48RDYC` writer - HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done."]
pub type HSI48RDYC_W<'a, REG> = crate::BitWriter<'a, REG, HSI48RDYC_A>;
impl<'a, REG> HSI48RDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI48RDYF no effect (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSI48RDYC_A::B_0x0)
    }
    #[doc = "HSI48RDYF cleared"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSI48RDYC_A::B_0x1)
    }
}
#[doc = "PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1RDYC_A {
    #[doc = "0: PLL1RDYF no effect (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: PLL1RDYF cleared"]
    B_0x1 = 1,
}
impl From<PLL1RDYC_A> for bool {
    #[inline(always)]
    fn from(variant: PLL1RDYC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL1RDYC` reader - PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done."]
pub type PLL1RDYC_R = crate::BitReader<PLL1RDYC_A>;
impl PLL1RDYC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL1RDYC_A {
        match self.bits {
            false => PLL1RDYC_A::B_0x0,
            true => PLL1RDYC_A::B_0x1,
        }
    }
    #[doc = "PLL1RDYF no effect (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL1RDYC_A::B_0x0
    }
    #[doc = "PLL1RDYF cleared"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL1RDYC_A::B_0x1
    }
}
#[doc = "Field `PLL1RDYC` writer - PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done."]
pub type PLL1RDYC_W<'a, REG> = crate::BitWriter<'a, REG, PLL1RDYC_A>;
impl<'a, REG> PLL1RDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL1RDYF no effect (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1RDYC_A::B_0x0)
    }
    #[doc = "PLL1RDYF cleared"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1RDYC_A::B_0x1)
    }
}
#[doc = "PLL2 ready interrupt clear Set by software to clear PLL2RDYF. Reset by hardware when clear done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL2RDYC_A {
    #[doc = "0: PLL2RDYF no effect (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: PLL2RDYF cleared"]
    B_0x1 = 1,
}
impl From<PLL2RDYC_A> for bool {
    #[inline(always)]
    fn from(variant: PLL2RDYC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL2RDYC` reader - PLL2 ready interrupt clear Set by software to clear PLL2RDYF. Reset by hardware when clear done."]
pub type PLL2RDYC_R = crate::BitReader<PLL2RDYC_A>;
impl PLL2RDYC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL2RDYC_A {
        match self.bits {
            false => PLL2RDYC_A::B_0x0,
            true => PLL2RDYC_A::B_0x1,
        }
    }
    #[doc = "PLL2RDYF no effect (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL2RDYC_A::B_0x0
    }
    #[doc = "PLL2RDYF cleared"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL2RDYC_A::B_0x1
    }
}
#[doc = "Field `PLL2RDYC` writer - PLL2 ready interrupt clear Set by software to clear PLL2RDYF. Reset by hardware when clear done."]
pub type PLL2RDYC_W<'a, REG> = crate::BitWriter<'a, REG, PLL2RDYC_A>;
impl<'a, REG> PLL2RDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL2RDYF no effect (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2RDYC_A::B_0x0)
    }
    #[doc = "PLL2RDYF cleared"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2RDYC_A::B_0x1)
    }
}
#[doc = "HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSECSSC_A {
    #[doc = "0: HSECSSF no effect (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: HSECSSF cleared"]
    B_0x1 = 1,
}
impl From<HSECSSC_A> for bool {
    #[inline(always)]
    fn from(variant: HSECSSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSECSSC` reader - HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done."]
pub type HSECSSC_R = crate::BitReader<HSECSSC_A>;
impl HSECSSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSECSSC_A {
        match self.bits {
            false => HSECSSC_A::B_0x0,
            true => HSECSSC_A::B_0x1,
        }
    }
    #[doc = "HSECSSF no effect (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HSECSSC_A::B_0x0
    }
    #[doc = "HSECSSF cleared"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HSECSSC_A::B_0x1
    }
}
#[doc = "Field `HSECSSC` writer - HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done."]
pub type HSECSSC_W<'a, REG> = crate::BitWriter<'a, REG, HSECSSC_A>;
impl<'a, REG> HSECSSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSECSSF no effect (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSECSSC_A::B_0x0)
    }
    #[doc = "HSECSSF cleared"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSECSSC_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn LSIRDYC(&self) -> LSIRDYC_R {
        LSIRDYC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn LSERDYC(&self) -> LSERDYC_R {
        LSERDYC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn CSIRDYC(&self) -> CSIRDYC_R {
        CSIRDYC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn HSIRDYC(&self) -> HSIRDYC_R {
        HSIRDYC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn HSERDYC(&self) -> HSERDYC_R {
        HSERDYC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn HSI48RDYC(&self) -> HSI48RDYC_R {
        HSI48RDYC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn PLL1RDYC(&self) -> PLL1RDYC_R {
        PLL1RDYC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL2 ready interrupt clear Set by software to clear PLL2RDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn PLL2RDYC(&self) -> PLL2RDYC_R {
        PLL2RDYC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn HSECSSC(&self) -> HSECSSC_R {
        HSECSSC_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn LSIRDYC(&mut self) -> LSIRDYC_W<'_, CICR_SPEC> {
        LSIRDYC_W::new(self, 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn LSERDYC(&mut self) -> LSERDYC_W<'_, CICR_SPEC> {
        LSERDYC_W::new(self, 1)
    }
    #[doc = "Bit 2 - HSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn CSIRDYC(&mut self) -> CSIRDYC_W<'_, CICR_SPEC> {
        CSIRDYC_W::new(self, 2)
    }
    #[doc = "Bit 3 - HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn HSIRDYC(&mut self) -> HSIRDYC_W<'_, CICR_SPEC> {
        HSIRDYC_W::new(self, 3)
    }
    #[doc = "Bit 4 - HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn HSERDYC(&mut self) -> HSERDYC_W<'_, CICR_SPEC> {
        HSERDYC_W::new(self, 4)
    }
    #[doc = "Bit 5 - HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn HSI48RDYC(&mut self) -> HSI48RDYC_W<'_, CICR_SPEC> {
        HSI48RDYC_W::new(self, 5)
    }
    #[doc = "Bit 6 - PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn PLL1RDYC(&mut self) -> PLL1RDYC_W<'_, CICR_SPEC> {
        PLL1RDYC_W::new(self, 6)
    }
    #[doc = "Bit 7 - PLL2 ready interrupt clear Set by software to clear PLL2RDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn PLL2RDYC(&mut self) -> PLL2RDYC_W<'_, CICR_SPEC> {
        PLL2RDYC_W::new(self, 7)
    }
    #[doc = "Bit 10 - HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn HSECSSC(&mut self) -> HSECSSC_W<'_, CICR_SPEC> {
        HSECSSC_W::new(self, 10)
    }
}
#[doc = "RCC clock source interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`cicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CICR_SPEC;
impl crate::RegisterSpec for CICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cicr::R`](R) reader structure"]
impl crate::Readable for CICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cicr::W`](W) writer structure"]
impl crate::Writable for CICR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CICR to value 0"]
impl crate::Resettable for CICR_SPEC {}
