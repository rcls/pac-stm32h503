#[doc = "Register `AF1` reader"]
pub type R = crate::R<AF1_SPEC>;
#[doc = "Register `AF1` writer"]
pub type W = crate::W<AF1_SPEC>;
#[doc = "TIMx_BKIN input enable This bit enables the TIMx_BKIN alternate function input for the timer's tim_brk input. TIMx_BKIN input is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKINE_A {
    #[doc = "0: TIMx_BKIN input disabled"]
    B_0x0 = 0,
    #[doc = "1: TIMx_BKIN input enabled"]
    B_0x1 = 1,
}
impl From<BKINE_A> for bool {
    #[inline(always)]
    fn from(variant: BKINE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKINE` reader - TIMx_BKIN input enable This bit enables the TIMx_BKIN alternate function input for the timer's tim_brk input. TIMx_BKIN input is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKINE_R = crate::BitReader<BKINE_A>;
impl BKINE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKINE_A {
        match self.bits {
            false => BKINE_A::B_0x0,
            true => BKINE_A::B_0x1,
        }
    }
    #[doc = "TIMx_BKIN input disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKINE_A::B_0x0
    }
    #[doc = "TIMx_BKIN input enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKINE_A::B_0x1
    }
}
#[doc = "Field `BKINE` writer - TIMx_BKIN input enable This bit enables the TIMx_BKIN alternate function input for the timer's tim_brk input. TIMx_BKIN input is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKINE_W<'a, REG> = crate::BitWriter<'a, REG, BKINE_A>;
impl<'a, REG> BKINE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIMx_BKIN input disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKINE_A::B_0x0)
    }
    #[doc = "TIMx_BKIN input enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKINE_A::B_0x1)
    }
}
#[doc = "tim_brk_cmp1 enable This bit enables the tim_brk_cmp1 for the timer's tim_brk input. tim_brk_cmp1 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKCMP1E_A {
    #[doc = "0: tim_brk_cmp1 input disabled"]
    B_0x0 = 0,
    #[doc = "1: tim_brk_cmp1 input enabled"]
    B_0x1 = 1,
}
impl From<BKCMP1E_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP1E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKCMP1E` reader - tim_brk_cmp1 enable This bit enables the tim_brk_cmp1 for the timer's tim_brk input. tim_brk_cmp1 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP1E_R = crate::BitReader<BKCMP1E_A>;
impl BKCMP1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKCMP1E_A {
        match self.bits {
            false => BKCMP1E_A::B_0x0,
            true => BKCMP1E_A::B_0x1,
        }
    }
    #[doc = "tim_brk_cmp1 input disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKCMP1E_A::B_0x0
    }
    #[doc = "tim_brk_cmp1 input enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKCMP1E_A::B_0x1
    }
}
#[doc = "Field `BKCMP1E` writer - tim_brk_cmp1 enable This bit enables the tim_brk_cmp1 for the timer's tim_brk input. tim_brk_cmp1 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP1E_W<'a, REG> = crate::BitWriter<'a, REG, BKCMP1E_A>;
impl<'a, REG> BKCMP1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk_cmp1 input disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP1E_A::B_0x0)
    }
    #[doc = "tim_brk_cmp1 input enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP1E_A::B_0x1)
    }
}
#[doc = "tim_brk_cmp2 enable This bit enables the tim_brk_cmp2 for the timer's tim_brk input. tim_brk_cmp2 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKCMP2E_A {
    #[doc = "0: tim_brk_cmp2 input disabled"]
    B_0x0 = 0,
    #[doc = "1: tim_brk_cmp2 input enabled"]
    B_0x1 = 1,
}
impl From<BKCMP2E_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP2E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKCMP2E` reader - tim_brk_cmp2 enable This bit enables the tim_brk_cmp2 for the timer's tim_brk input. tim_brk_cmp2 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP2E_R = crate::BitReader<BKCMP2E_A>;
impl BKCMP2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKCMP2E_A {
        match self.bits {
            false => BKCMP2E_A::B_0x0,
            true => BKCMP2E_A::B_0x1,
        }
    }
    #[doc = "tim_brk_cmp2 input disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKCMP2E_A::B_0x0
    }
    #[doc = "tim_brk_cmp2 input enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKCMP2E_A::B_0x1
    }
}
#[doc = "Field `BKCMP2E` writer - tim_brk_cmp2 enable This bit enables the tim_brk_cmp2 for the timer's tim_brk input. tim_brk_cmp2 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP2E_W<'a, REG> = crate::BitWriter<'a, REG, BKCMP2E_A>;
impl<'a, REG> BKCMP2E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk_cmp2 input disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP2E_A::B_0x0)
    }
    #[doc = "tim_brk_cmp2 input enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP2E_A::B_0x1)
    }
}
#[doc = "tim_brk_cmp3 enable This bit enables the tim_brk_cmp3 for the timer's tim_brk input. tim_brk_cmp3 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKCMP3E_A {
    #[doc = "0: tim_brk_cmp3 input disabled"]
    B_0x0 = 0,
    #[doc = "1: tim_brk_cmp3 input enabled"]
    B_0x1 = 1,
}
impl From<BKCMP3E_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP3E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKCMP3E` reader - tim_brk_cmp3 enable This bit enables the tim_brk_cmp3 for the timer's tim_brk input. tim_brk_cmp3 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP3E_R = crate::BitReader<BKCMP3E_A>;
impl BKCMP3E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKCMP3E_A {
        match self.bits {
            false => BKCMP3E_A::B_0x0,
            true => BKCMP3E_A::B_0x1,
        }
    }
    #[doc = "tim_brk_cmp3 input disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKCMP3E_A::B_0x0
    }
    #[doc = "tim_brk_cmp3 input enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKCMP3E_A::B_0x1
    }
}
#[doc = "Field `BKCMP3E` writer - tim_brk_cmp3 enable This bit enables the tim_brk_cmp3 for the timer's tim_brk input. tim_brk_cmp3 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP3E_W<'a, REG> = crate::BitWriter<'a, REG, BKCMP3E_A>;
impl<'a, REG> BKCMP3E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk_cmp3 input disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP3E_A::B_0x0)
    }
    #[doc = "tim_brk_cmp3 input enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP3E_A::B_0x1)
    }
}
#[doc = "tim_brk_cmp4 enable This bit enables the tim_brk_cmp4 for the timer's tim_brk input. tim_brk_cmp4 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKCMP4E_A {
    #[doc = "0: tim_brk_cmp4 input disabled"]
    B_0x0 = 0,
    #[doc = "1: tim_brk_cmp4 input enabled"]
    B_0x1 = 1,
}
impl From<BKCMP4E_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP4E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKCMP4E` reader - tim_brk_cmp4 enable This bit enables the tim_brk_cmp4 for the timer's tim_brk input. tim_brk_cmp4 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP4E_R = crate::BitReader<BKCMP4E_A>;
impl BKCMP4E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKCMP4E_A {
        match self.bits {
            false => BKCMP4E_A::B_0x0,
            true => BKCMP4E_A::B_0x1,
        }
    }
    #[doc = "tim_brk_cmp4 input disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKCMP4E_A::B_0x0
    }
    #[doc = "tim_brk_cmp4 input enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKCMP4E_A::B_0x1
    }
}
#[doc = "Field `BKCMP4E` writer - tim_brk_cmp4 enable This bit enables the tim_brk_cmp4 for the timer's tim_brk input. tim_brk_cmp4 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP4E_W<'a, REG> = crate::BitWriter<'a, REG, BKCMP4E_A>;
impl<'a, REG> BKCMP4E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk_cmp4 input disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP4E_A::B_0x0)
    }
    #[doc = "tim_brk_cmp4 input enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP4E_A::B_0x1)
    }
}
#[doc = "tim_brk_cmp5 enable This bit enables the tim_brk_cmp5 for the timer's tim_brk input. tim_brk_cmp5 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKCMP5E_A {
    #[doc = "0: tim_brk_cmp5 input disabled"]
    B_0x0 = 0,
    #[doc = "1: tim_brk_cmp5 input enabled"]
    B_0x1 = 1,
}
impl From<BKCMP5E_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP5E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKCMP5E` reader - tim_brk_cmp5 enable This bit enables the tim_brk_cmp5 for the timer's tim_brk input. tim_brk_cmp5 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP5E_R = crate::BitReader<BKCMP5E_A>;
impl BKCMP5E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKCMP5E_A {
        match self.bits {
            false => BKCMP5E_A::B_0x0,
            true => BKCMP5E_A::B_0x1,
        }
    }
    #[doc = "tim_brk_cmp5 input disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKCMP5E_A::B_0x0
    }
    #[doc = "tim_brk_cmp5 input enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKCMP5E_A::B_0x1
    }
}
#[doc = "Field `BKCMP5E` writer - tim_brk_cmp5 enable This bit enables the tim_brk_cmp5 for the timer's tim_brk input. tim_brk_cmp5 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP5E_W<'a, REG> = crate::BitWriter<'a, REG, BKCMP5E_A>;
impl<'a, REG> BKCMP5E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk_cmp5 input disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP5E_A::B_0x0)
    }
    #[doc = "tim_brk_cmp5 input enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP5E_A::B_0x1)
    }
}
#[doc = "tim_brk_cmp6 enable This bit enables the tim_brk_cmp6 for the timer's tim_brk input. tim_brk_cmp6 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKCMP6E_A {
    #[doc = "0: tim_brk_cmp6 input disabled"]
    B_0x0 = 0,
    #[doc = "1: tim_brk_cmp6 input enabled"]
    B_0x1 = 1,
}
impl From<BKCMP6E_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP6E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKCMP6E` reader - tim_brk_cmp6 enable This bit enables the tim_brk_cmp6 for the timer's tim_brk input. tim_brk_cmp6 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP6E_R = crate::BitReader<BKCMP6E_A>;
impl BKCMP6E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKCMP6E_A {
        match self.bits {
            false => BKCMP6E_A::B_0x0,
            true => BKCMP6E_A::B_0x1,
        }
    }
    #[doc = "tim_brk_cmp6 input disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKCMP6E_A::B_0x0
    }
    #[doc = "tim_brk_cmp6 input enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKCMP6E_A::B_0x1
    }
}
#[doc = "Field `BKCMP6E` writer - tim_brk_cmp6 enable This bit enables the tim_brk_cmp6 for the timer's tim_brk input. tim_brk_cmp6 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP6E_W<'a, REG> = crate::BitWriter<'a, REG, BKCMP6E_A>;
impl<'a, REG> BKCMP6E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk_cmp6 input disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP6E_A::B_0x0)
    }
    #[doc = "tim_brk_cmp6 input enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP6E_A::B_0x1)
    }
}
#[doc = "tim_brk_cmp7 enable This bit enables the tim_brk_cmp7 for the timer's tim_brk input. tim_brk_cmp7 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKCMP7E_A {
    #[doc = "0: tim_brk_cmp7 input disabled"]
    B_0x0 = 0,
    #[doc = "1: tim_brk_cmp7 input enabled"]
    B_0x1 = 1,
}
impl From<BKCMP7E_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP7E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKCMP7E` reader - tim_brk_cmp7 enable This bit enables the tim_brk_cmp7 for the timer's tim_brk input. tim_brk_cmp7 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP7E_R = crate::BitReader<BKCMP7E_A>;
impl BKCMP7E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKCMP7E_A {
        match self.bits {
            false => BKCMP7E_A::B_0x0,
            true => BKCMP7E_A::B_0x1,
        }
    }
    #[doc = "tim_brk_cmp7 input disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKCMP7E_A::B_0x0
    }
    #[doc = "tim_brk_cmp7 input enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKCMP7E_A::B_0x1
    }
}
#[doc = "Field `BKCMP7E` writer - tim_brk_cmp7 enable This bit enables the tim_brk_cmp7 for the timer's tim_brk input. tim_brk_cmp7 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP7E_W<'a, REG> = crate::BitWriter<'a, REG, BKCMP7E_A>;
impl<'a, REG> BKCMP7E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk_cmp7 input disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP7E_A::B_0x0)
    }
    #[doc = "tim_brk_cmp7 input enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP7E_A::B_0x1)
    }
}
#[doc = "tim_brk_cmp8 enable This bit enables the tim_brk_cmp8 for the timer's tim_brk input. tim_brk_cmp8 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKCMP8E_A {
    #[doc = "0: tim_brk_cmp8 input disabled"]
    B_0x0 = 0,
    #[doc = "1: tim_brk_cmp8 input enabled"]
    B_0x1 = 1,
}
impl From<BKCMP8E_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP8E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKCMP8E` reader - tim_brk_cmp8 enable This bit enables the tim_brk_cmp8 for the timer's tim_brk input. tim_brk_cmp8 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP8E_R = crate::BitReader<BKCMP8E_A>;
impl BKCMP8E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKCMP8E_A {
        match self.bits {
            false => BKCMP8E_A::B_0x0,
            true => BKCMP8E_A::B_0x1,
        }
    }
    #[doc = "tim_brk_cmp8 input disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKCMP8E_A::B_0x0
    }
    #[doc = "tim_brk_cmp8 input enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKCMP8E_A::B_0x1
    }
}
#[doc = "Field `BKCMP8E` writer - tim_brk_cmp8 enable This bit enables the tim_brk_cmp8 for the timer's tim_brk input. tim_brk_cmp8 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP8E_W<'a, REG> = crate::BitWriter<'a, REG, BKCMP8E_A>;
impl<'a, REG> BKCMP8E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk_cmp8 input disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP8E_A::B_0x0)
    }
    #[doc = "tim_brk_cmp8 input enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP8E_A::B_0x1)
    }
}
#[doc = "TIMx_BKIN input polarity This bit selects the TIMx_BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKINP_A {
    #[doc = "0: TIMx_BKIN input polarity is not inverted (active low if BKP = 0, active high if BKP = 1)"]
    B_0x0 = 0,
    #[doc = "1: TIMx_BKIN input polarity is inverted (active high if BKP = 0, active low if BKP = 1)"]
    B_0x1 = 1,
}
impl From<BKINP_A> for bool {
    #[inline(always)]
    fn from(variant: BKINP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKINP` reader - TIMx_BKIN input polarity This bit selects the TIMx_BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKINP_R = crate::BitReader<BKINP_A>;
impl BKINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKINP_A {
        match self.bits {
            false => BKINP_A::B_0x0,
            true => BKINP_A::B_0x1,
        }
    }
    #[doc = "TIMx_BKIN input polarity is not inverted (active low if BKP = 0, active high if BKP = 1)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKINP_A::B_0x0
    }
    #[doc = "TIMx_BKIN input polarity is inverted (active high if BKP = 0, active low if BKP = 1)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKINP_A::B_0x1
    }
}
#[doc = "Field `BKINP` writer - TIMx_BKIN input polarity This bit selects the TIMx_BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKINP_W<'a, REG> = crate::BitWriter<'a, REG, BKINP_A>;
impl<'a, REG> BKINP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIMx_BKIN input polarity is not inverted (active low if BKP = 0, active high if BKP = 1)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKINP_A::B_0x0)
    }
    #[doc = "TIMx_BKIN input polarity is inverted (active high if BKP = 0, active low if BKP = 1)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKINP_A::B_0x1)
    }
}
#[doc = "tim_brk_cmp1 input polarity This bit selects the tim_brk_cmp1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKCMP1P_A {
    #[doc = "0: tim_brk_cmp1 input polarity is not inverted (active low if BKP = 0, active high if BKP = 1)"]
    B_0x0 = 0,
    #[doc = "1: tim_brk_cmp1 input polarity is inverted (active high if BKP = 0, active low if BKP = 1)"]
    B_0x1 = 1,
}
impl From<BKCMP1P_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP1P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKCMP1P` reader - tim_brk_cmp1 input polarity This bit selects the tim_brk_cmp1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP1P_R = crate::BitReader<BKCMP1P_A>;
impl BKCMP1P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKCMP1P_A {
        match self.bits {
            false => BKCMP1P_A::B_0x0,
            true => BKCMP1P_A::B_0x1,
        }
    }
    #[doc = "tim_brk_cmp1 input polarity is not inverted (active low if BKP = 0, active high if BKP = 1)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKCMP1P_A::B_0x0
    }
    #[doc = "tim_brk_cmp1 input polarity is inverted (active high if BKP = 0, active low if BKP = 1)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKCMP1P_A::B_0x1
    }
}
#[doc = "Field `BKCMP1P` writer - tim_brk_cmp1 input polarity This bit selects the tim_brk_cmp1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP1P_W<'a, REG> = crate::BitWriter<'a, REG, BKCMP1P_A>;
impl<'a, REG> BKCMP1P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk_cmp1 input polarity is not inverted (active low if BKP = 0, active high if BKP = 1)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP1P_A::B_0x0)
    }
    #[doc = "tim_brk_cmp1 input polarity is inverted (active high if BKP = 0, active low if BKP = 1)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP1P_A::B_0x1)
    }
}
#[doc = "tim_brk_cmp2 input polarity This bit selects the tim_brk_cmp2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKCMP2P_A {
    #[doc = "0: tim_brk_cmp2 input polarity is not inverted (active low if BKP = 0, active high if BKP = 1)"]
    B_0x0 = 0,
    #[doc = "1: tim_brk_cmp2 input polarity is inverted (active high if BKP = 0, active low if BKP = 1)"]
    B_0x1 = 1,
}
impl From<BKCMP2P_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP2P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKCMP2P` reader - tim_brk_cmp2 input polarity This bit selects the tim_brk_cmp2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP2P_R = crate::BitReader<BKCMP2P_A>;
impl BKCMP2P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKCMP2P_A {
        match self.bits {
            false => BKCMP2P_A::B_0x0,
            true => BKCMP2P_A::B_0x1,
        }
    }
    #[doc = "tim_brk_cmp2 input polarity is not inverted (active low if BKP = 0, active high if BKP = 1)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKCMP2P_A::B_0x0
    }
    #[doc = "tim_brk_cmp2 input polarity is inverted (active high if BKP = 0, active low if BKP = 1)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKCMP2P_A::B_0x1
    }
}
#[doc = "Field `BKCMP2P` writer - tim_brk_cmp2 input polarity This bit selects the tim_brk_cmp2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP2P_W<'a, REG> = crate::BitWriter<'a, REG, BKCMP2P_A>;
impl<'a, REG> BKCMP2P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk_cmp2 input polarity is not inverted (active low if BKP = 0, active high if BKP = 1)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP2P_A::B_0x0)
    }
    #[doc = "tim_brk_cmp2 input polarity is inverted (active high if BKP = 0, active low if BKP = 1)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP2P_A::B_0x1)
    }
}
#[doc = "tim_brk_cmp3 input polarity This bit selects the tim_brk_cmp3 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKCMP3P_A {
    #[doc = "0: tim_brk_cmp3 input polarity is not inverted (active low if BKP = 0, active high if BKP = 1)"]
    B_0x0 = 0,
    #[doc = "1: tim_brk_cmp3 input polarity is inverted (active high if BKP = 0, active low if BKP = 1)"]
    B_0x1 = 1,
}
impl From<BKCMP3P_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP3P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKCMP3P` reader - tim_brk_cmp3 input polarity This bit selects the tim_brk_cmp3 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP3P_R = crate::BitReader<BKCMP3P_A>;
impl BKCMP3P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKCMP3P_A {
        match self.bits {
            false => BKCMP3P_A::B_0x0,
            true => BKCMP3P_A::B_0x1,
        }
    }
    #[doc = "tim_brk_cmp3 input polarity is not inverted (active low if BKP = 0, active high if BKP = 1)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKCMP3P_A::B_0x0
    }
    #[doc = "tim_brk_cmp3 input polarity is inverted (active high if BKP = 0, active low if BKP = 1)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKCMP3P_A::B_0x1
    }
}
#[doc = "Field `BKCMP3P` writer - tim_brk_cmp3 input polarity This bit selects the tim_brk_cmp3 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP3P_W<'a, REG> = crate::BitWriter<'a, REG, BKCMP3P_A>;
impl<'a, REG> BKCMP3P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk_cmp3 input polarity is not inverted (active low if BKP = 0, active high if BKP = 1)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP3P_A::B_0x0)
    }
    #[doc = "tim_brk_cmp3 input polarity is inverted (active high if BKP = 0, active low if BKP = 1)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP3P_A::B_0x1)
    }
}
#[doc = "tim_brk_cmp4 input polarity This bit selects the tim_brk_cmp4 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKCMP4P_A {
    #[doc = "0: tim_brk_cmp4 input polarity is not inverted (active low if BKP = 0, active high if BKP = 1)"]
    B_0x0 = 0,
    #[doc = "1: tim_brk_cmp4 input polarity is inverted (active high if BKP = 0, active low if BKP = 1)"]
    B_0x1 = 1,
}
impl From<BKCMP4P_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP4P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKCMP4P` reader - tim_brk_cmp4 input polarity This bit selects the tim_brk_cmp4 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP4P_R = crate::BitReader<BKCMP4P_A>;
impl BKCMP4P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKCMP4P_A {
        match self.bits {
            false => BKCMP4P_A::B_0x0,
            true => BKCMP4P_A::B_0x1,
        }
    }
    #[doc = "tim_brk_cmp4 input polarity is not inverted (active low if BKP = 0, active high if BKP = 1)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKCMP4P_A::B_0x0
    }
    #[doc = "tim_brk_cmp4 input polarity is inverted (active high if BKP = 0, active low if BKP = 1)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKCMP4P_A::B_0x1
    }
}
#[doc = "Field `BKCMP4P` writer - tim_brk_cmp4 input polarity This bit selects the tim_brk_cmp4 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKCMP4P_W<'a, REG> = crate::BitWriter<'a, REG, BKCMP4P_A>;
impl<'a, REG> BKCMP4P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk_cmp4 input polarity is not inverted (active low if BKP = 0, active high if BKP = 1)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP4P_A::B_0x0)
    }
    #[doc = "tim_brk_cmp4 input polarity is inverted (active high if BKP = 0, active low if BKP = 1)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP4P_A::B_0x1)
    }
}
#[doc = "etr_in source selection These bits select the etr_in input source. ... Refer to Section 65.3.2: TIM1 pins and internal signals for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETRSEL_A {
    #[doc = "0: tim_etr0: TIMx_ETR input"]
    B_0x0 = 0,
    #[doc = "1: tim_etr1"]
    B_0x1 = 1,
    #[doc = "15: tim_etr15"]
    B_0xF = 15,
}
impl From<ETRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ETRSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ETRSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for ETRSEL_A {}
#[doc = "Field `ETRSEL` reader - etr_in source selection These bits select the etr_in input source. ... Refer to Section 65.3.2: TIM1 pins and internal signals for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type ETRSEL_R = crate::FieldReader<ETRSEL_A>;
impl ETRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ETRSEL_A> {
        match self.bits {
            0 => Some(ETRSEL_A::B_0x0),
            1 => Some(ETRSEL_A::B_0x1),
            15 => Some(ETRSEL_A::B_0xF),
            _ => None,
        }
    }
    #[doc = "tim_etr0: TIMx_ETR input"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ETRSEL_A::B_0x0
    }
    #[doc = "tim_etr1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ETRSEL_A::B_0x1
    }
    #[doc = "tim_etr15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == ETRSEL_A::B_0xF
    }
}
#[doc = "Field `ETRSEL` writer - etr_in source selection These bits select the etr_in input source. ... Refer to Section 65.3.2: TIM1 pins and internal signals for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type ETRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ETRSEL_A>;
impl<'a, REG> ETRSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "tim_etr0: TIMx_ETR input"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL_A::B_0x0)
    }
    #[doc = "tim_etr1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL_A::B_0x1)
    }
    #[doc = "tim_etr15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL_A::B_0xF)
    }
}
impl R {
    #[doc = "Bit 0 - TIMx_BKIN input enable This bit enables the TIMx_BKIN alternate function input for the timer's tim_brk input. TIMx_BKIN input is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKINE(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tim_brk_cmp1 enable This bit enables the tim_brk_cmp1 for the timer's tim_brk input. tim_brk_cmp1 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP1E(&self) -> BKCMP1E_R {
        BKCMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - tim_brk_cmp2 enable This bit enables the tim_brk_cmp2 for the timer's tim_brk input. tim_brk_cmp2 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP2E(&self) -> BKCMP2E_R {
        BKCMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - tim_brk_cmp3 enable This bit enables the tim_brk_cmp3 for the timer's tim_brk input. tim_brk_cmp3 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP3E(&self) -> BKCMP3E_R {
        BKCMP3E_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - tim_brk_cmp4 enable This bit enables the tim_brk_cmp4 for the timer's tim_brk input. tim_brk_cmp4 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP4E(&self) -> BKCMP4E_R {
        BKCMP4E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - tim_brk_cmp5 enable This bit enables the tim_brk_cmp5 for the timer's tim_brk input. tim_brk_cmp5 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP5E(&self) -> BKCMP5E_R {
        BKCMP5E_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - tim_brk_cmp6 enable This bit enables the tim_brk_cmp6 for the timer's tim_brk input. tim_brk_cmp6 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP6E(&self) -> BKCMP6E_R {
        BKCMP6E_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - tim_brk_cmp7 enable This bit enables the tim_brk_cmp7 for the timer's tim_brk input. tim_brk_cmp7 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP7E(&self) -> BKCMP7E_R {
        BKCMP7E_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - tim_brk_cmp8 enable This bit enables the tim_brk_cmp8 for the timer's tim_brk input. tim_brk_cmp8 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP8E(&self) -> BKCMP8E_R {
        BKCMP8E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TIMx_BKIN input polarity This bit selects the TIMx_BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKINP(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - tim_brk_cmp1 input polarity This bit selects the tim_brk_cmp1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP1P(&self) -> BKCMP1P_R {
        BKCMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - tim_brk_cmp2 input polarity This bit selects the tim_brk_cmp2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP2P(&self) -> BKCMP2P_R {
        BKCMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - tim_brk_cmp3 input polarity This bit selects the tim_brk_cmp3 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP3P(&self) -> BKCMP3P_R {
        BKCMP3P_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - tim_brk_cmp4 input polarity This bit selects the tim_brk_cmp4 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP4P(&self) -> BKCMP4P_R {
        BKCMP4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:17 - etr_in source selection These bits select the etr_in input source. ... Refer to Section 65.3.2: TIM1 pins and internal signals for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ETRSEL(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TIMx_BKIN input enable This bit enables the TIMx_BKIN alternate function input for the timer's tim_brk input. TIMx_BKIN input is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKINE(&mut self) -> BKINE_W<'_, AF1_SPEC> {
        BKINE_W::new(self, 0)
    }
    #[doc = "Bit 1 - tim_brk_cmp1 enable This bit enables the tim_brk_cmp1 for the timer's tim_brk input. tim_brk_cmp1 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP1E(&mut self) -> BKCMP1E_W<'_, AF1_SPEC> {
        BKCMP1E_W::new(self, 1)
    }
    #[doc = "Bit 2 - tim_brk_cmp2 enable This bit enables the tim_brk_cmp2 for the timer's tim_brk input. tim_brk_cmp2 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP2E(&mut self) -> BKCMP2E_W<'_, AF1_SPEC> {
        BKCMP2E_W::new(self, 2)
    }
    #[doc = "Bit 3 - tim_brk_cmp3 enable This bit enables the tim_brk_cmp3 for the timer's tim_brk input. tim_brk_cmp3 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP3E(&mut self) -> BKCMP3E_W<'_, AF1_SPEC> {
        BKCMP3E_W::new(self, 3)
    }
    #[doc = "Bit 4 - tim_brk_cmp4 enable This bit enables the tim_brk_cmp4 for the timer's tim_brk input. tim_brk_cmp4 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP4E(&mut self) -> BKCMP4E_W<'_, AF1_SPEC> {
        BKCMP4E_W::new(self, 4)
    }
    #[doc = "Bit 5 - tim_brk_cmp5 enable This bit enables the tim_brk_cmp5 for the timer's tim_brk input. tim_brk_cmp5 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP5E(&mut self) -> BKCMP5E_W<'_, AF1_SPEC> {
        BKCMP5E_W::new(self, 5)
    }
    #[doc = "Bit 6 - tim_brk_cmp6 enable This bit enables the tim_brk_cmp6 for the timer's tim_brk input. tim_brk_cmp6 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP6E(&mut self) -> BKCMP6E_W<'_, AF1_SPEC> {
        BKCMP6E_W::new(self, 6)
    }
    #[doc = "Bit 7 - tim_brk_cmp7 enable This bit enables the tim_brk_cmp7 for the timer's tim_brk input. tim_brk_cmp7 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP7E(&mut self) -> BKCMP7E_W<'_, AF1_SPEC> {
        BKCMP7E_W::new(self, 7)
    }
    #[doc = "Bit 8 - tim_brk_cmp8 enable This bit enables the tim_brk_cmp8 for the timer's tim_brk input. tim_brk_cmp8 output is 'ORed' with the other tim_brk sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP8E(&mut self) -> BKCMP8E_W<'_, AF1_SPEC> {
        BKCMP8E_W::new(self, 8)
    }
    #[doc = "Bit 9 - TIMx_BKIN input polarity This bit selects the TIMx_BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKINP(&mut self) -> BKINP_W<'_, AF1_SPEC> {
        BKINP_W::new(self, 9)
    }
    #[doc = "Bit 10 - tim_brk_cmp1 input polarity This bit selects the tim_brk_cmp1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP1P(&mut self) -> BKCMP1P_W<'_, AF1_SPEC> {
        BKCMP1P_W::new(self, 10)
    }
    #[doc = "Bit 11 - tim_brk_cmp2 input polarity This bit selects the tim_brk_cmp2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP2P(&mut self) -> BKCMP2P_W<'_, AF1_SPEC> {
        BKCMP2P_W::new(self, 11)
    }
    #[doc = "Bit 12 - tim_brk_cmp3 input polarity This bit selects the tim_brk_cmp3 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP3P(&mut self) -> BKCMP3P_W<'_, AF1_SPEC> {
        BKCMP3P_W::new(self, 12)
    }
    #[doc = "Bit 13 - tim_brk_cmp4 input polarity This bit selects the tim_brk_cmp4 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKCMP4P(&mut self) -> BKCMP4P_W<'_, AF1_SPEC> {
        BKCMP4P_W::new(self, 13)
    }
    #[doc = "Bits 14:17 - etr_in source selection These bits select the etr_in input source. ... Refer to Section 65.3.2: TIM1 pins and internal signals for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ETRSEL(&mut self) -> ETRSEL_W<'_, AF1_SPEC> {
        ETRSEL_W::new(self, 14)
    }
}
#[doc = "TIM1 alternate function option register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`af1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF1_SPEC;
impl crate::RegisterSpec for AF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af1::R`](R) reader structure"]
impl crate::Readable for AF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`af1::W`](W) writer structure"]
impl crate::Writable for AF1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets AF1 to value 0x01"]
impl crate::Resettable for AF1_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
