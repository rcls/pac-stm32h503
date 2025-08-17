#[doc = "Register `AF2` reader"]
pub type R = crate::R<AF2_SPEC>;
#[doc = "Register `AF2` writer"]
pub type W = crate::W<AF2_SPEC>;
#[doc = "TIMx_BKIN2 input enable This bit enables the TIMx_BKIN2 alternate function input for the timer's tim_brk2 input. TIMx_BKIN2 input is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2INE_A {
    #[doc = "0: TIMx_BKIN2 input disabled"]
    B_0x0 = 0,
    #[doc = "1: TIMx_BKIN2 input enabled"]
    B_0x1 = 1,
}
impl From<BK2INE_A> for bool {
    #[inline(always)]
    fn from(variant: BK2INE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2INE` reader - TIMx_BKIN2 input enable This bit enables the TIMx_BKIN2 alternate function input for the timer's tim_brk2 input. TIMx_BKIN2 input is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2INE_R = crate::BitReader<BK2INE_A>;
impl BK2INE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2INE_A {
        match self.bits {
            false => BK2INE_A::B_0x0,
            true => BK2INE_A::B_0x1,
        }
    }
    #[doc = "TIMx_BKIN2 input disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BK2INE_A::B_0x0
    }
    #[doc = "TIMx_BKIN2 input enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BK2INE_A::B_0x1
    }
}
#[doc = "Field `BK2INE` writer - TIMx_BKIN2 input enable This bit enables the TIMx_BKIN2 alternate function input for the timer's tim_brk2 input. TIMx_BKIN2 input is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2INE_W<'a, REG> = crate::BitWriter<'a, REG, BK2INE_A>;
impl<'a, REG> BK2INE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIMx_BKIN2 input disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INE_A::B_0x0)
    }
    #[doc = "TIMx_BKIN2 input enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INE_A::B_0x1)
    }
}
#[doc = "tim_brk2_cmp1 enable This bit enables the tim_brk2_cmp1 for the timer's tim_brk2 input. tim_brk2_cmp1 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP1E_A {
    #[doc = "0: tim_brk2_cmp1 input disabled"]
    B_0x0 = 0,
    #[doc = "1: tim_brk2_cmp1 input enabled"]
    B_0x1 = 1,
}
impl From<BK2CMP1E_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP1E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP1E` reader - tim_brk2_cmp1 enable This bit enables the tim_brk2_cmp1 for the timer's tim_brk2 input. tim_brk2_cmp1 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP1E_R = crate::BitReader<BK2CMP1E_A>;
impl BK2CMP1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP1E_A {
        match self.bits {
            false => BK2CMP1E_A::B_0x0,
            true => BK2CMP1E_A::B_0x1,
        }
    }
    #[doc = "tim_brk2_cmp1 input disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BK2CMP1E_A::B_0x0
    }
    #[doc = "tim_brk2_cmp1 input enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BK2CMP1E_A::B_0x1
    }
}
#[doc = "Field `BK2CMP1E` writer - tim_brk2_cmp1 enable This bit enables the tim_brk2_cmp1 for the timer's tim_brk2 input. tim_brk2_cmp1 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP1E_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP1E_A>;
impl<'a, REG> BK2CMP1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk2_cmp1 input disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP1E_A::B_0x0)
    }
    #[doc = "tim_brk2_cmp1 input enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP1E_A::B_0x1)
    }
}
#[doc = "tim_brk2_cmp2 enable This bit enables the tim_brk2_cmp2 for the timer's tim_brk2 input. tim_brk2_cmp2 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP2E_A {
    #[doc = "0: tim_brk2_cmp2 input disabled"]
    B_0x0 = 0,
    #[doc = "1: tim_brk2_cmp2 input enabled"]
    B_0x1 = 1,
}
impl From<BK2CMP2E_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP2E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP2E` reader - tim_brk2_cmp2 enable This bit enables the tim_brk2_cmp2 for the timer's tim_brk2 input. tim_brk2_cmp2 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP2E_R = crate::BitReader<BK2CMP2E_A>;
impl BK2CMP2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP2E_A {
        match self.bits {
            false => BK2CMP2E_A::B_0x0,
            true => BK2CMP2E_A::B_0x1,
        }
    }
    #[doc = "tim_brk2_cmp2 input disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BK2CMP2E_A::B_0x0
    }
    #[doc = "tim_brk2_cmp2 input enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BK2CMP2E_A::B_0x1
    }
}
#[doc = "Field `BK2CMP2E` writer - tim_brk2_cmp2 enable This bit enables the tim_brk2_cmp2 for the timer's tim_brk2 input. tim_brk2_cmp2 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP2E_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP2E_A>;
impl<'a, REG> BK2CMP2E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk2_cmp2 input disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP2E_A::B_0x0)
    }
    #[doc = "tim_brk2_cmp2 input enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP2E_A::B_0x1)
    }
}
#[doc = "tim_brk2_cmp3 enable This bit enables the tim_brk2_cmp3 for the timer's tim_brk2 input. tim_brk2_cmp3 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP3E_A {
    #[doc = "0: tim_brk2_cmp3 input disabled"]
    B_0x0 = 0,
    #[doc = "1: tim_brk2_cmp3 input enabled"]
    B_0x1 = 1,
}
impl From<BK2CMP3E_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP3E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP3E` reader - tim_brk2_cmp3 enable This bit enables the tim_brk2_cmp3 for the timer's tim_brk2 input. tim_brk2_cmp3 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP3E_R = crate::BitReader<BK2CMP3E_A>;
impl BK2CMP3E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP3E_A {
        match self.bits {
            false => BK2CMP3E_A::B_0x0,
            true => BK2CMP3E_A::B_0x1,
        }
    }
    #[doc = "tim_brk2_cmp3 input disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BK2CMP3E_A::B_0x0
    }
    #[doc = "tim_brk2_cmp3 input enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BK2CMP3E_A::B_0x1
    }
}
#[doc = "Field `BK2CMP3E` writer - tim_brk2_cmp3 enable This bit enables the tim_brk2_cmp3 for the timer's tim_brk2 input. tim_brk2_cmp3 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP3E_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP3E_A>;
impl<'a, REG> BK2CMP3E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk2_cmp3 input disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP3E_A::B_0x0)
    }
    #[doc = "tim_brk2_cmp3 input enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP3E_A::B_0x1)
    }
}
#[doc = "tim_brk2_cmp4 enable This bit enables the tim_brk2_cmp4 for the timer's tim_brk2 input. tim_brk2_cmp4 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP4E_A {
    #[doc = "0: tim_brk2_cmp4 input disabled"]
    B_0x0 = 0,
    #[doc = "1: tim_brk2_cmp4 input enabled"]
    B_0x1 = 1,
}
impl From<BK2CMP4E_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP4E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP4E` reader - tim_brk2_cmp4 enable This bit enables the tim_brk2_cmp4 for the timer's tim_brk2 input. tim_brk2_cmp4 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP4E_R = crate::BitReader<BK2CMP4E_A>;
impl BK2CMP4E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP4E_A {
        match self.bits {
            false => BK2CMP4E_A::B_0x0,
            true => BK2CMP4E_A::B_0x1,
        }
    }
    #[doc = "tim_brk2_cmp4 input disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BK2CMP4E_A::B_0x0
    }
    #[doc = "tim_brk2_cmp4 input enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BK2CMP4E_A::B_0x1
    }
}
#[doc = "Field `BK2CMP4E` writer - tim_brk2_cmp4 enable This bit enables the tim_brk2_cmp4 for the timer's tim_brk2 input. tim_brk2_cmp4 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP4E_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP4E_A>;
impl<'a, REG> BK2CMP4E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk2_cmp4 input disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP4E_A::B_0x0)
    }
    #[doc = "tim_brk2_cmp4 input enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP4E_A::B_0x1)
    }
}
#[doc = "tim_brk2_cmp5 enable This bit enables the tim_brk2_cmp5 for the timer's tim_brk2 input. tim_brk2_cmp5 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP5E_A {
    #[doc = "0: tim_brk2_cmp5 input disabled"]
    B_0x0 = 0,
    #[doc = "1: tim_brk2_cmp5 input enabled"]
    B_0x1 = 1,
}
impl From<BK2CMP5E_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP5E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP5E` reader - tim_brk2_cmp5 enable This bit enables the tim_brk2_cmp5 for the timer's tim_brk2 input. tim_brk2_cmp5 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP5E_R = crate::BitReader<BK2CMP5E_A>;
impl BK2CMP5E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP5E_A {
        match self.bits {
            false => BK2CMP5E_A::B_0x0,
            true => BK2CMP5E_A::B_0x1,
        }
    }
    #[doc = "tim_brk2_cmp5 input disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BK2CMP5E_A::B_0x0
    }
    #[doc = "tim_brk2_cmp5 input enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BK2CMP5E_A::B_0x1
    }
}
#[doc = "Field `BK2CMP5E` writer - tim_brk2_cmp5 enable This bit enables the tim_brk2_cmp5 for the timer's tim_brk2 input. tim_brk2_cmp5 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP5E_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP5E_A>;
impl<'a, REG> BK2CMP5E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk2_cmp5 input disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP5E_A::B_0x0)
    }
    #[doc = "tim_brk2_cmp5 input enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP5E_A::B_0x1)
    }
}
#[doc = "tim_brk2_cmp6 enable This bit enables the tim_brk2_cmp6 for the timer's tim_brk2 input. tim_brk2_cmp6 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP6E_A {
    #[doc = "0: tim_brk2_cmp6 input disabled"]
    B_0x0 = 0,
    #[doc = "1: tim_brk2_cmp6 input enabled"]
    B_0x1 = 1,
}
impl From<BK2CMP6E_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP6E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP6E` reader - tim_brk2_cmp6 enable This bit enables the tim_brk2_cmp6 for the timer's tim_brk2 input. tim_brk2_cmp6 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP6E_R = crate::BitReader<BK2CMP6E_A>;
impl BK2CMP6E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP6E_A {
        match self.bits {
            false => BK2CMP6E_A::B_0x0,
            true => BK2CMP6E_A::B_0x1,
        }
    }
    #[doc = "tim_brk2_cmp6 input disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BK2CMP6E_A::B_0x0
    }
    #[doc = "tim_brk2_cmp6 input enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BK2CMP6E_A::B_0x1
    }
}
#[doc = "Field `BK2CMP6E` writer - tim_brk2_cmp6 enable This bit enables the tim_brk2_cmp6 for the timer's tim_brk2 input. tim_brk2_cmp6 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP6E_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP6E_A>;
impl<'a, REG> BK2CMP6E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk2_cmp6 input disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP6E_A::B_0x0)
    }
    #[doc = "tim_brk2_cmp6 input enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP6E_A::B_0x1)
    }
}
#[doc = "tim_brk2_cmp7 enable This bit enables the tim_brk2_cmp7 for the timer's tim_brk2 input. tim_brk2_cmp7 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP7E_A {
    #[doc = "0: tim_brk2_cmp7 input disabled"]
    B_0x0 = 0,
    #[doc = "1: tim_brk2_cmp7 input enabled"]
    B_0x1 = 1,
}
impl From<BK2CMP7E_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP7E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP7E` reader - tim_brk2_cmp7 enable This bit enables the tim_brk2_cmp7 for the timer's tim_brk2 input. tim_brk2_cmp7 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP7E_R = crate::BitReader<BK2CMP7E_A>;
impl BK2CMP7E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP7E_A {
        match self.bits {
            false => BK2CMP7E_A::B_0x0,
            true => BK2CMP7E_A::B_0x1,
        }
    }
    #[doc = "tim_brk2_cmp7 input disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BK2CMP7E_A::B_0x0
    }
    #[doc = "tim_brk2_cmp7 input enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BK2CMP7E_A::B_0x1
    }
}
#[doc = "Field `BK2CMP7E` writer - tim_brk2_cmp7 enable This bit enables the tim_brk2_cmp7 for the timer's tim_brk2 input. tim_brk2_cmp7 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP7E_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP7E_A>;
impl<'a, REG> BK2CMP7E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk2_cmp7 input disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP7E_A::B_0x0)
    }
    #[doc = "tim_brk2_cmp7 input enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP7E_A::B_0x1)
    }
}
#[doc = "tim_brk2_cmp8 enable This bit enables the tim_brk2_cmp8 for the timer's tim_brk2 input. tim_brk2_cmp8 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP8E_A {
    #[doc = "0: tim_brk2_cmp8 input disabled"]
    B_0x0 = 0,
    #[doc = "1: tim_brk2_cmp8 input enabled"]
    B_0x1 = 1,
}
impl From<BK2CMP8E_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP8E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP8E` reader - tim_brk2_cmp8 enable This bit enables the tim_brk2_cmp8 for the timer's tim_brk2 input. tim_brk2_cmp8 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP8E_R = crate::BitReader<BK2CMP8E_A>;
impl BK2CMP8E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP8E_A {
        match self.bits {
            false => BK2CMP8E_A::B_0x0,
            true => BK2CMP8E_A::B_0x1,
        }
    }
    #[doc = "tim_brk2_cmp8 input disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BK2CMP8E_A::B_0x0
    }
    #[doc = "tim_brk2_cmp8 input enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BK2CMP8E_A::B_0x1
    }
}
#[doc = "Field `BK2CMP8E` writer - tim_brk2_cmp8 enable This bit enables the tim_brk2_cmp8 for the timer's tim_brk2 input. tim_brk2_cmp8 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP8E_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP8E_A>;
impl<'a, REG> BK2CMP8E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk2_cmp8 input disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP8E_A::B_0x0)
    }
    #[doc = "tim_brk2_cmp8 input enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP8E_A::B_0x1)
    }
}
#[doc = "TIMx_BKIN2 input polarity This bit selects the TIMx_BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2INP_A {
    #[doc = "0: TIMx_BKIN2 input polarity is not inverted (active low if BK2P = 0, active high if BK2P = 1)"]
    B_0x0 = 0,
    #[doc = "1: TIMx_BKIN2 input polarity is inverted (active high if BK2P = 0, active low if BK2P = 1)"]
    B_0x1 = 1,
}
impl From<BK2INP_A> for bool {
    #[inline(always)]
    fn from(variant: BK2INP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2INP` reader - TIMx_BKIN2 input polarity This bit selects the TIMx_BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2INP_R = crate::BitReader<BK2INP_A>;
impl BK2INP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2INP_A {
        match self.bits {
            false => BK2INP_A::B_0x0,
            true => BK2INP_A::B_0x1,
        }
    }
    #[doc = "TIMx_BKIN2 input polarity is not inverted (active low if BK2P = 0, active high if BK2P = 1)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BK2INP_A::B_0x0
    }
    #[doc = "TIMx_BKIN2 input polarity is inverted (active high if BK2P = 0, active low if BK2P = 1)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BK2INP_A::B_0x1
    }
}
#[doc = "Field `BK2INP` writer - TIMx_BKIN2 input polarity This bit selects the TIMx_BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2INP_W<'a, REG> = crate::BitWriter<'a, REG, BK2INP_A>;
impl<'a, REG> BK2INP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIMx_BKIN2 input polarity is not inverted (active low if BK2P = 0, active high if BK2P = 1)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INP_A::B_0x0)
    }
    #[doc = "TIMx_BKIN2 input polarity is inverted (active high if BK2P = 0, active low if BK2P = 1)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INP_A::B_0x1)
    }
}
#[doc = "tim_brk2_cmp1 input polarity This bit selects the tim_brk2_cmp1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP1P_A {
    #[doc = "0: tim_brk2_cmp1 input polarity is not inverted (active low if BK2P = 0, active high if BK2P = 1)"]
    B_0x0 = 0,
    #[doc = "1: tim_brk2_cmp1 input polarity is inverted (active high if BK2P = 0, active low if BK2P = 1)"]
    B_0x1 = 1,
}
impl From<BK2CMP1P_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP1P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP1P` reader - tim_brk2_cmp1 input polarity This bit selects the tim_brk2_cmp1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP1P_R = crate::BitReader<BK2CMP1P_A>;
impl BK2CMP1P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP1P_A {
        match self.bits {
            false => BK2CMP1P_A::B_0x0,
            true => BK2CMP1P_A::B_0x1,
        }
    }
    #[doc = "tim_brk2_cmp1 input polarity is not inverted (active low if BK2P = 0, active high if BK2P = 1)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BK2CMP1P_A::B_0x0
    }
    #[doc = "tim_brk2_cmp1 input polarity is inverted (active high if BK2P = 0, active low if BK2P = 1)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BK2CMP1P_A::B_0x1
    }
}
#[doc = "Field `BK2CMP1P` writer - tim_brk2_cmp1 input polarity This bit selects the tim_brk2_cmp1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP1P_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP1P_A>;
impl<'a, REG> BK2CMP1P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk2_cmp1 input polarity is not inverted (active low if BK2P = 0, active high if BK2P = 1)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP1P_A::B_0x0)
    }
    #[doc = "tim_brk2_cmp1 input polarity is inverted (active high if BK2P = 0, active low if BK2P = 1)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP1P_A::B_0x1)
    }
}
#[doc = "tim_brk2_cmp2 input polarity This bit selects the tim_brk2_cmp2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP2P_A {
    #[doc = "0: tim_brk2_cmp2 input polarity is not inverted (active low if BK2P = 0, active high if BK2P = 1)"]
    B_0x0 = 0,
    #[doc = "1: tim_brk2_cmp2 input polarity is inverted (active high if BK2P = 0, active low if BK2P = 1)"]
    B_0x1 = 1,
}
impl From<BK2CMP2P_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP2P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP2P` reader - tim_brk2_cmp2 input polarity This bit selects the tim_brk2_cmp2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP2P_R = crate::BitReader<BK2CMP2P_A>;
impl BK2CMP2P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP2P_A {
        match self.bits {
            false => BK2CMP2P_A::B_0x0,
            true => BK2CMP2P_A::B_0x1,
        }
    }
    #[doc = "tim_brk2_cmp2 input polarity is not inverted (active low if BK2P = 0, active high if BK2P = 1)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BK2CMP2P_A::B_0x0
    }
    #[doc = "tim_brk2_cmp2 input polarity is inverted (active high if BK2P = 0, active low if BK2P = 1)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BK2CMP2P_A::B_0x1
    }
}
#[doc = "Field `BK2CMP2P` writer - tim_brk2_cmp2 input polarity This bit selects the tim_brk2_cmp2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP2P_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP2P_A>;
impl<'a, REG> BK2CMP2P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk2_cmp2 input polarity is not inverted (active low if BK2P = 0, active high if BK2P = 1)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP2P_A::B_0x0)
    }
    #[doc = "tim_brk2_cmp2 input polarity is inverted (active high if BK2P = 0, active low if BK2P = 1)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP2P_A::B_0x1)
    }
}
#[doc = "tim_brk2_cmp3 input polarity This bit selects the tim_brk2_cmp3 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP3P_A {
    #[doc = "0: tim_brk2_cmp3 input polarity is not inverted (active low if BK2P = 0, active high if BK2P = 1)"]
    B_0x0 = 0,
    #[doc = "1: tim_brk2_cmp3 input polarity is inverted (active high if BK2P = 0, active low if BK2P = 1)"]
    B_0x1 = 1,
}
impl From<BK2CMP3P_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP3P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP3P` reader - tim_brk2_cmp3 input polarity This bit selects the tim_brk2_cmp3 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP3P_R = crate::BitReader<BK2CMP3P_A>;
impl BK2CMP3P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP3P_A {
        match self.bits {
            false => BK2CMP3P_A::B_0x0,
            true => BK2CMP3P_A::B_0x1,
        }
    }
    #[doc = "tim_brk2_cmp3 input polarity is not inverted (active low if BK2P = 0, active high if BK2P = 1)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BK2CMP3P_A::B_0x0
    }
    #[doc = "tim_brk2_cmp3 input polarity is inverted (active high if BK2P = 0, active low if BK2P = 1)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BK2CMP3P_A::B_0x1
    }
}
#[doc = "Field `BK2CMP3P` writer - tim_brk2_cmp3 input polarity This bit selects the tim_brk2_cmp3 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP3P_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP3P_A>;
impl<'a, REG> BK2CMP3P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk2_cmp3 input polarity is not inverted (active low if BK2P = 0, active high if BK2P = 1)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP3P_A::B_0x0)
    }
    #[doc = "tim_brk2_cmp3 input polarity is inverted (active high if BK2P = 0, active low if BK2P = 1)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP3P_A::B_0x1)
    }
}
#[doc = "tim_brk2_cmp4 input polarity This bit selects the tim_brk2_cmp4 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP4P_A {
    #[doc = "0: tim_brk2_cmp4 input polarity is not inverted (active low if BK2P = 0, active high if BK2P = 1)"]
    B_0x0 = 0,
    #[doc = "1: tim_brk2_cmp4 input polarity is inverted (active high if BK2P = 0, active low if BK2P = 1)"]
    B_0x1 = 1,
}
impl From<BK2CMP4P_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP4P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP4P` reader - tim_brk2_cmp4 input polarity This bit selects the tim_brk2_cmp4 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP4P_R = crate::BitReader<BK2CMP4P_A>;
impl BK2CMP4P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP4P_A {
        match self.bits {
            false => BK2CMP4P_A::B_0x0,
            true => BK2CMP4P_A::B_0x1,
        }
    }
    #[doc = "tim_brk2_cmp4 input polarity is not inverted (active low if BK2P = 0, active high if BK2P = 1)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BK2CMP4P_A::B_0x0
    }
    #[doc = "tim_brk2_cmp4 input polarity is inverted (active high if BK2P = 0, active low if BK2P = 1)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BK2CMP4P_A::B_0x1
    }
}
#[doc = "Field `BK2CMP4P` writer - tim_brk2_cmp4 input polarity This bit selects the tim_brk2_cmp4 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP4P_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP4P_A>;
impl<'a, REG> BK2CMP4P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_brk2_cmp4 input polarity is not inverted (active low if BK2P = 0, active high if BK2P = 1)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP4P_A::B_0x0)
    }
    #[doc = "tim_brk2_cmp4 input polarity is inverted (active high if BK2P = 0, active low if BK2P = 1)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP4P_A::B_0x1)
    }
}
#[doc = "ocref_clr source selection These bits select the ocref_clr input source. ... Refer to Section 65.3.2: TIM1 pins and internal signals for product specific information. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OCRSEL_A {
    #[doc = "0: tim_ocref_clr0"]
    B_0x0 = 0,
    #[doc = "1: tim_ocref_clr1"]
    B_0x1 = 1,
    #[doc = "7: tim_ocref_clr7"]
    B_0x7 = 7,
}
impl From<OCRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OCRSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OCRSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for OCRSEL_A {}
#[doc = "Field `OCRSEL` reader - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to Section 65.3.2: TIM1 pins and internal signals for product specific information. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OCRSEL_R = crate::FieldReader<OCRSEL_A>;
impl OCRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OCRSEL_A> {
        match self.bits {
            0 => Some(OCRSEL_A::B_0x0),
            1 => Some(OCRSEL_A::B_0x1),
            7 => Some(OCRSEL_A::B_0x7),
            _ => None,
        }
    }
    #[doc = "tim_ocref_clr0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OCRSEL_A::B_0x0
    }
    #[doc = "tim_ocref_clr1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OCRSEL_A::B_0x1
    }
    #[doc = "tim_ocref_clr7"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == OCRSEL_A::B_0x7
    }
}
#[doc = "Field `OCRSEL` writer - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to Section 65.3.2: TIM1 pins and internal signals for product specific information. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OCRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, OCRSEL_A>;
impl<'a, REG> OCRSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "tim_ocref_clr0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OCRSEL_A::B_0x0)
    }
    #[doc = "tim_ocref_clr1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OCRSEL_A::B_0x1)
    }
    #[doc = "tim_ocref_clr7"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(OCRSEL_A::B_0x7)
    }
}
impl R {
    #[doc = "Bit 0 - TIMx_BKIN2 input enable This bit enables the TIMx_BKIN2 alternate function input for the timer's tim_brk2 input. TIMx_BKIN2 input is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2INE(&self) -> BK2INE_R {
        BK2INE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tim_brk2_cmp1 enable This bit enables the tim_brk2_cmp1 for the timer's tim_brk2 input. tim_brk2_cmp1 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP1E(&self) -> BK2CMP1E_R {
        BK2CMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - tim_brk2_cmp2 enable This bit enables the tim_brk2_cmp2 for the timer's tim_brk2 input. tim_brk2_cmp2 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP2E(&self) -> BK2CMP2E_R {
        BK2CMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - tim_brk2_cmp3 enable This bit enables the tim_brk2_cmp3 for the timer's tim_brk2 input. tim_brk2_cmp3 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP3E(&self) -> BK2CMP3E_R {
        BK2CMP3E_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - tim_brk2_cmp4 enable This bit enables the tim_brk2_cmp4 for the timer's tim_brk2 input. tim_brk2_cmp4 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP4E(&self) -> BK2CMP4E_R {
        BK2CMP4E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - tim_brk2_cmp5 enable This bit enables the tim_brk2_cmp5 for the timer's tim_brk2 input. tim_brk2_cmp5 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP5E(&self) -> BK2CMP5E_R {
        BK2CMP5E_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - tim_brk2_cmp6 enable This bit enables the tim_brk2_cmp6 for the timer's tim_brk2 input. tim_brk2_cmp6 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP6E(&self) -> BK2CMP6E_R {
        BK2CMP6E_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - tim_brk2_cmp7 enable This bit enables the tim_brk2_cmp7 for the timer's tim_brk2 input. tim_brk2_cmp7 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP7E(&self) -> BK2CMP7E_R {
        BK2CMP7E_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - tim_brk2_cmp8 enable This bit enables the tim_brk2_cmp8 for the timer's tim_brk2 input. tim_brk2_cmp8 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP8E(&self) -> BK2CMP8E_R {
        BK2CMP8E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TIMx_BKIN2 input polarity This bit selects the TIMx_BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2INP(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - tim_brk2_cmp1 input polarity This bit selects the tim_brk2_cmp1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP1P(&self) -> BK2CMP1P_R {
        BK2CMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - tim_brk2_cmp2 input polarity This bit selects the tim_brk2_cmp2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP2P(&self) -> BK2CMP2P_R {
        BK2CMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - tim_brk2_cmp3 input polarity This bit selects the tim_brk2_cmp3 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP3P(&self) -> BK2CMP3P_R {
        BK2CMP3P_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - tim_brk2_cmp4 input polarity This bit selects the tim_brk2_cmp4 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP4P(&self) -> BK2CMP4P_R {
        BK2CMP4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:18 - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to Section 65.3.2: TIM1 pins and internal signals for product specific information. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn OCRSEL(&self) -> OCRSEL_R {
        OCRSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TIMx_BKIN2 input enable This bit enables the TIMx_BKIN2 alternate function input for the timer's tim_brk2 input. TIMx_BKIN2 input is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2INE(&mut self) -> BK2INE_W<'_, AF2_SPEC> {
        BK2INE_W::new(self, 0)
    }
    #[doc = "Bit 1 - tim_brk2_cmp1 enable This bit enables the tim_brk2_cmp1 for the timer's tim_brk2 input. tim_brk2_cmp1 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP1E(&mut self) -> BK2CMP1E_W<'_, AF2_SPEC> {
        BK2CMP1E_W::new(self, 1)
    }
    #[doc = "Bit 2 - tim_brk2_cmp2 enable This bit enables the tim_brk2_cmp2 for the timer's tim_brk2 input. tim_brk2_cmp2 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP2E(&mut self) -> BK2CMP2E_W<'_, AF2_SPEC> {
        BK2CMP2E_W::new(self, 2)
    }
    #[doc = "Bit 3 - tim_brk2_cmp3 enable This bit enables the tim_brk2_cmp3 for the timer's tim_brk2 input. tim_brk2_cmp3 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP3E(&mut self) -> BK2CMP3E_W<'_, AF2_SPEC> {
        BK2CMP3E_W::new(self, 3)
    }
    #[doc = "Bit 4 - tim_brk2_cmp4 enable This bit enables the tim_brk2_cmp4 for the timer's tim_brk2 input. tim_brk2_cmp4 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP4E(&mut self) -> BK2CMP4E_W<'_, AF2_SPEC> {
        BK2CMP4E_W::new(self, 4)
    }
    #[doc = "Bit 5 - tim_brk2_cmp5 enable This bit enables the tim_brk2_cmp5 for the timer's tim_brk2 input. tim_brk2_cmp5 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP5E(&mut self) -> BK2CMP5E_W<'_, AF2_SPEC> {
        BK2CMP5E_W::new(self, 5)
    }
    #[doc = "Bit 6 - tim_brk2_cmp6 enable This bit enables the tim_brk2_cmp6 for the timer's tim_brk2 input. tim_brk2_cmp6 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP6E(&mut self) -> BK2CMP6E_W<'_, AF2_SPEC> {
        BK2CMP6E_W::new(self, 6)
    }
    #[doc = "Bit 7 - tim_brk2_cmp7 enable This bit enables the tim_brk2_cmp7 for the timer's tim_brk2 input. tim_brk2_cmp7 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP7E(&mut self) -> BK2CMP7E_W<'_, AF2_SPEC> {
        BK2CMP7E_W::new(self, 7)
    }
    #[doc = "Bit 8 - tim_brk2_cmp8 enable This bit enables the tim_brk2_cmp8 for the timer's tim_brk2 input. tim_brk2_cmp8 output is 'ORed' with the other tim_brk2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP8E(&mut self) -> BK2CMP8E_W<'_, AF2_SPEC> {
        BK2CMP8E_W::new(self, 8)
    }
    #[doc = "Bit 9 - TIMx_BKIN2 input polarity This bit selects the TIMx_BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2INP(&mut self) -> BK2INP_W<'_, AF2_SPEC> {
        BK2INP_W::new(self, 9)
    }
    #[doc = "Bit 10 - tim_brk2_cmp1 input polarity This bit selects the tim_brk2_cmp1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP1P(&mut self) -> BK2CMP1P_W<'_, AF2_SPEC> {
        BK2CMP1P_W::new(self, 10)
    }
    #[doc = "Bit 11 - tim_brk2_cmp2 input polarity This bit selects the tim_brk2_cmp2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP2P(&mut self) -> BK2CMP2P_W<'_, AF2_SPEC> {
        BK2CMP2P_W::new(self, 11)
    }
    #[doc = "Bit 12 - tim_brk2_cmp3 input polarity This bit selects the tim_brk2_cmp3 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP3P(&mut self) -> BK2CMP3P_W<'_, AF2_SPEC> {
        BK2CMP3P_W::new(self, 12)
    }
    #[doc = "Bit 13 - tim_brk2_cmp4 input polarity This bit selects the tim_brk2_cmp4 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2CMP4P(&mut self) -> BK2CMP4P_W<'_, AF2_SPEC> {
        BK2CMP4P_W::new(self, 13)
    }
    #[doc = "Bits 16:18 - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to Section 65.3.2: TIM1 pins and internal signals for product specific information. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn OCRSEL(&mut self) -> OCRSEL_W<'_, AF2_SPEC> {
        OCRSEL_W::new(self, 16)
    }
}
#[doc = "TIM1 alternate function register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`af2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF2_SPEC;
impl crate::RegisterSpec for AF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af2::R`](R) reader structure"]
impl crate::Readable for AF2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`af2::W`](W) writer structure"]
impl crate::Writable for AF2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets AF2 to value 0x01"]
impl crate::Resettable for AF2_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
