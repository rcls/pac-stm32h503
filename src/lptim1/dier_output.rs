#[doc = "Register `DIER_output` reader"]
pub type R = crate::R<DIER_OUTPUT_SPEC>;
#[doc = "Register `DIER_output` writer"]
pub type W = crate::W<DIER_OUTPUT_SPEC>;
#[doc = "Capture/compare 1 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1IE_A {
    #[doc = "0: Capture/compare 1 interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Capture/compare 1 interrupt enabled"]
    B_0x1 = 1,
}
impl From<CC1IE_A> for bool {
    #[inline(always)]
    fn from(variant: CC1IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1IE` reader - Capture/compare 1 interrupt enable"]
pub type CC1IE_R = crate::BitReader<CC1IE_A>;
impl CC1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC1IE_A {
        match self.bits {
            false => CC1IE_A::B_0x0,
            true => CC1IE_A::B_0x1,
        }
    }
    #[doc = "Capture/compare 1 interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC1IE_A::B_0x0
    }
    #[doc = "Capture/compare 1 interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC1IE_A::B_0x1
    }
}
#[doc = "Field `CC1IE` writer - Capture/compare 1 interrupt enable"]
pub type CC1IE_W<'a, REG> = crate::BitWriter<'a, REG, CC1IE_A>;
impl<'a, REG> CC1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture/compare 1 interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IE_A::B_0x0)
    }
    #[doc = "Capture/compare 1 interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IE_A::B_0x1)
    }
}
#[doc = "Autoreload match Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARRMIE_A {
    #[doc = "0: ARRM interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: ARRM interrupt enabled"]
    B_0x1 = 1,
}
impl From<ARRMIE_A> for bool {
    #[inline(always)]
    fn from(variant: ARRMIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARRMIE` reader - Autoreload match Interrupt Enable"]
pub type ARRMIE_R = crate::BitReader<ARRMIE_A>;
impl ARRMIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ARRMIE_A {
        match self.bits {
            false => ARRMIE_A::B_0x0,
            true => ARRMIE_A::B_0x1,
        }
    }
    #[doc = "ARRM interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ARRMIE_A::B_0x0
    }
    #[doc = "ARRM interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ARRMIE_A::B_0x1
    }
}
#[doc = "Field `ARRMIE` writer - Autoreload match Interrupt Enable"]
pub type ARRMIE_W<'a, REG> = crate::BitWriter<'a, REG, ARRMIE_A>;
impl<'a, REG> ARRMIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ARRM interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ARRMIE_A::B_0x0)
    }
    #[doc = "ARRM interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ARRMIE_A::B_0x1)
    }
}
#[doc = "External trigger valid edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTTRIGIE_A {
    #[doc = "0: EXTTRIG interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: EXTTRIG interrupt enabled"]
    B_0x1 = 1,
}
impl From<EXTTRIGIE_A> for bool {
    #[inline(always)]
    fn from(variant: EXTTRIGIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTTRIGIE` reader - External trigger valid edge Interrupt Enable"]
pub type EXTTRIGIE_R = crate::BitReader<EXTTRIGIE_A>;
impl EXTTRIGIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTTRIGIE_A {
        match self.bits {
            false => EXTTRIGIE_A::B_0x0,
            true => EXTTRIGIE_A::B_0x1,
        }
    }
    #[doc = "EXTTRIG interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EXTTRIGIE_A::B_0x0
    }
    #[doc = "EXTTRIG interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EXTTRIGIE_A::B_0x1
    }
}
#[doc = "Field `EXTTRIGIE` writer - External trigger valid edge Interrupt Enable"]
pub type EXTTRIGIE_W<'a, REG> = crate::BitWriter<'a, REG, EXTTRIGIE_A>;
impl<'a, REG> EXTTRIGIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EXTTRIG interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTTRIGIE_A::B_0x0)
    }
    #[doc = "EXTTRIG interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTTRIGIE_A::B_0x1)
    }
}
#[doc = "Compare register 1 update OK interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP1OKIE_A {
    #[doc = "0: CMPOK register 1 interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: CMPOK register 1 interrupt enabled"]
    B_0x1 = 1,
}
impl From<CMP1OKIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMP1OKIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP1OKIE` reader - Compare register 1 update OK interrupt enable"]
pub type CMP1OKIE_R = crate::BitReader<CMP1OKIE_A>;
impl CMP1OKIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMP1OKIE_A {
        match self.bits {
            false => CMP1OKIE_A::B_0x0,
            true => CMP1OKIE_A::B_0x1,
        }
    }
    #[doc = "CMPOK register 1 interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CMP1OKIE_A::B_0x0
    }
    #[doc = "CMPOK register 1 interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CMP1OKIE_A::B_0x1
    }
}
#[doc = "Field `CMP1OKIE` writer - Compare register 1 update OK interrupt enable"]
pub type CMP1OKIE_W<'a, REG> = crate::BitWriter<'a, REG, CMP1OKIE_A>;
impl<'a, REG> CMP1OKIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CMPOK register 1 interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CMP1OKIE_A::B_0x0)
    }
    #[doc = "CMPOK register 1 interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CMP1OKIE_A::B_0x1)
    }
}
#[doc = "Autoreload register update OK Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARROKIE_A {
    #[doc = "0: ARROK interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: ARROK interrupt enabled"]
    B_0x1 = 1,
}
impl From<ARROKIE_A> for bool {
    #[inline(always)]
    fn from(variant: ARROKIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARROKIE` reader - Autoreload register update OK Interrupt Enable"]
pub type ARROKIE_R = crate::BitReader<ARROKIE_A>;
impl ARROKIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ARROKIE_A {
        match self.bits {
            false => ARROKIE_A::B_0x0,
            true => ARROKIE_A::B_0x1,
        }
    }
    #[doc = "ARROK interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ARROKIE_A::B_0x0
    }
    #[doc = "ARROK interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ARROKIE_A::B_0x1
    }
}
#[doc = "Field `ARROKIE` writer - Autoreload register update OK Interrupt Enable"]
pub type ARROKIE_W<'a, REG> = crate::BitWriter<'a, REG, ARROKIE_A>;
impl<'a, REG> ARROKIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ARROK interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ARROKIE_A::B_0x0)
    }
    #[doc = "ARROK interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ARROKIE_A::B_0x1)
    }
}
#[doc = "Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPIE_A {
    #[doc = "0: UP interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: UP interrupt enabled"]
    B_0x1 = 1,
}
impl From<UPIE_A> for bool {
    #[inline(always)]
    fn from(variant: UPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPIE` reader - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type UPIE_R = crate::BitReader<UPIE_A>;
impl UPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UPIE_A {
        match self.bits {
            false => UPIE_A::B_0x0,
            true => UPIE_A::B_0x1,
        }
    }
    #[doc = "UP interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == UPIE_A::B_0x0
    }
    #[doc = "UP interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == UPIE_A::B_0x1
    }
}
#[doc = "Field `UPIE` writer - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type UPIE_W<'a, REG> = crate::BitWriter<'a, REG, UPIE_A>;
impl<'a, REG> UPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UP interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UPIE_A::B_0x0)
    }
    #[doc = "UP interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UPIE_A::B_0x1)
    }
}
#[doc = "Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOWNIE_A {
    #[doc = "0: DOWN interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: DOWN interrupt enabled"]
    B_0x1 = 1,
}
impl From<DOWNIE_A> for bool {
    #[inline(always)]
    fn from(variant: DOWNIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOWNIE` reader - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type DOWNIE_R = crate::BitReader<DOWNIE_A>;
impl DOWNIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DOWNIE_A {
        match self.bits {
            false => DOWNIE_A::B_0x0,
            true => DOWNIE_A::B_0x1,
        }
    }
    #[doc = "DOWN interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DOWNIE_A::B_0x0
    }
    #[doc = "DOWN interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DOWNIE_A::B_0x1
    }
}
#[doc = "Field `DOWNIE` writer - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type DOWNIE_W<'a, REG> = crate::BitWriter<'a, REG, DOWNIE_A>;
impl<'a, REG> DOWNIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DOWN interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DOWNIE_A::B_0x0)
    }
    #[doc = "DOWN interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DOWNIE_A::B_0x1)
    }
}
#[doc = "Update event interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UEIE_A {
    #[doc = "0: Update event interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Update event interrupt enabled"]
    B_0x1 = 1,
}
impl From<UEIE_A> for bool {
    #[inline(always)]
    fn from(variant: UEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UEIE` reader - Update event interrupt enable"]
pub type UEIE_R = crate::BitReader<UEIE_A>;
impl UEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UEIE_A {
        match self.bits {
            false => UEIE_A::B_0x0,
            true => UEIE_A::B_0x1,
        }
    }
    #[doc = "Update event interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == UEIE_A::B_0x0
    }
    #[doc = "Update event interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == UEIE_A::B_0x1
    }
}
#[doc = "Field `UEIE` writer - Update event interrupt enable"]
pub type UEIE_W<'a, REG> = crate::BitWriter<'a, REG, UEIE_A>;
impl<'a, REG> UEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update event interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UEIE_A::B_0x0)
    }
    #[doc = "Update event interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UEIE_A::B_0x1)
    }
}
#[doc = "Repetition register update OK interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REPOKIE_A {
    #[doc = "0: Repetition register update OK interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Repetition register update OK interrupt enabled"]
    B_0x1 = 1,
}
impl From<REPOKIE_A> for bool {
    #[inline(always)]
    fn from(variant: REPOKIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPOKIE` reader - Repetition register update OK interrupt Enable"]
pub type REPOKIE_R = crate::BitReader<REPOKIE_A>;
impl REPOKIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REPOKIE_A {
        match self.bits {
            false => REPOKIE_A::B_0x0,
            true => REPOKIE_A::B_0x1,
        }
    }
    #[doc = "Repetition register update OK interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == REPOKIE_A::B_0x0
    }
    #[doc = "Repetition register update OK interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == REPOKIE_A::B_0x1
    }
}
#[doc = "Field `REPOKIE` writer - Repetition register update OK interrupt Enable"]
pub type REPOKIE_W<'a, REG> = crate::BitWriter<'a, REG, REPOKIE_A>;
impl<'a, REG> REPOKIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Repetition register update OK interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(REPOKIE_A::B_0x0)
    }
    #[doc = "Repetition register update OK interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(REPOKIE_A::B_0x1)
    }
}
#[doc = "Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC2IE_A {
    #[doc = "0: Capture/compare 2 interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Capture/compare 2 interrupt enabled"]
    B_0x1 = 1,
}
impl From<CC2IE_A> for bool {
    #[inline(always)]
    fn from(variant: CC2IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC2IE` reader - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CC2IE_R = crate::BitReader<CC2IE_A>;
impl CC2IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC2IE_A {
        match self.bits {
            false => CC2IE_A::B_0x0,
            true => CC2IE_A::B_0x1,
        }
    }
    #[doc = "Capture/compare 2 interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC2IE_A::B_0x0
    }
    #[doc = "Capture/compare 2 interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC2IE_A::B_0x1
    }
}
#[doc = "Field `CC2IE` writer - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CC2IE_W<'a, REG> = crate::BitWriter<'a, REG, CC2IE_A>;
impl<'a, REG> CC2IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture/compare 2 interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC2IE_A::B_0x0)
    }
    #[doc = "Capture/compare 2 interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC2IE_A::B_0x1)
    }
}
#[doc = "Compare register 2 update OK interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP2OKIE_A {
    #[doc = "0: CMPOK register 2 interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: CMPOK register 2 interrupt enabled"]
    B_0x1 = 1,
}
impl From<CMP2OKIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMP2OKIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP2OKIE` reader - Compare register 2 update OK interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CMP2OKIE_R = crate::BitReader<CMP2OKIE_A>;
impl CMP2OKIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMP2OKIE_A {
        match self.bits {
            false => CMP2OKIE_A::B_0x0,
            true => CMP2OKIE_A::B_0x1,
        }
    }
    #[doc = "CMPOK register 2 interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CMP2OKIE_A::B_0x0
    }
    #[doc = "CMPOK register 2 interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CMP2OKIE_A::B_0x1
    }
}
#[doc = "Field `CMP2OKIE` writer - Compare register 2 update OK interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CMP2OKIE_W<'a, REG> = crate::BitWriter<'a, REG, CMP2OKIE_A>;
impl<'a, REG> CMP2OKIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CMPOK register 2 interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CMP2OKIE_A::B_0x0)
    }
    #[doc = "CMPOK register 2 interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CMP2OKIE_A::B_0x1)
    }
}
#[doc = "Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UEDE_A {
    #[doc = "0: UE DMA request disabled. Writing '0' to the UEDE bit resets the associated ue_dma_req signal."]
    B_0x0 = 0,
    #[doc = "1: UE DMA request enabled"]
    B_0x1 = 1,
}
impl From<UEDE_A> for bool {
    #[inline(always)]
    fn from(variant: UEDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UEDE` reader - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
pub type UEDE_R = crate::BitReader<UEDE_A>;
impl UEDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UEDE_A {
        match self.bits {
            false => UEDE_A::B_0x0,
            true => UEDE_A::B_0x1,
        }
    }
    #[doc = "UE DMA request disabled. Writing '0' to the UEDE bit resets the associated ue_dma_req signal."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == UEDE_A::B_0x0
    }
    #[doc = "UE DMA request enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == UEDE_A::B_0x1
    }
}
#[doc = "Field `UEDE` writer - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
pub type UEDE_W<'a, REG> = crate::BitWriter<'a, REG, UEDE_A>;
impl<'a, REG> UEDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UE DMA request disabled. Writing '0' to the UEDE bit resets the associated ue_dma_req signal."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UEDE_A::B_0x0)
    }
    #[doc = "UE DMA request enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UEDE_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Capture/compare 1 interrupt enable"]
    #[inline(always)]
    pub fn CC1IE(&self) -> CC1IE_R {
        CC1IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Autoreload match Interrupt Enable"]
    #[inline(always)]
    pub fn ARRMIE(&self) -> ARRMIE_R {
        ARRMIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    pub fn EXTTRIGIE(&self) -> EXTTRIGIE_R {
        EXTTRIGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare register 1 update OK interrupt enable"]
    #[inline(always)]
    pub fn CMP1OKIE(&self) -> CMP1OKIE_R {
        CMP1OKIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    pub fn ARROKIE(&self) -> ARROKIE_R {
        ARROKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn UPIE(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn DOWNIE(&self) -> DOWNIE_R {
        DOWNIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Update event interrupt enable"]
    #[inline(always)]
    pub fn UEIE(&self) -> UEIE_R {
        UEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Repetition register update OK interrupt Enable"]
    #[inline(always)]
    pub fn REPOKIE(&self) -> REPOKIE_R {
        REPOKIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn CC2IE(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 19 - Compare register 2 update OK interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn CMP2OKIE(&self) -> CMP2OKIE_R {
        CMP2OKIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn UEDE(&self) -> UEDE_R {
        UEDE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare 1 interrupt enable"]
    #[inline(always)]
    pub fn CC1IE(&mut self) -> CC1IE_W<'_, DIER_OUTPUT_SPEC> {
        CC1IE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Autoreload match Interrupt Enable"]
    #[inline(always)]
    pub fn ARRMIE(&mut self) -> ARRMIE_W<'_, DIER_OUTPUT_SPEC> {
        ARRMIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    pub fn EXTTRIGIE(&mut self) -> EXTTRIGIE_W<'_, DIER_OUTPUT_SPEC> {
        EXTTRIGIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Compare register 1 update OK interrupt enable"]
    #[inline(always)]
    pub fn CMP1OKIE(&mut self) -> CMP1OKIE_W<'_, DIER_OUTPUT_SPEC> {
        CMP1OKIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    pub fn ARROKIE(&mut self) -> ARROKIE_W<'_, DIER_OUTPUT_SPEC> {
        ARROKIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn UPIE(&mut self) -> UPIE_W<'_, DIER_OUTPUT_SPEC> {
        UPIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn DOWNIE(&mut self) -> DOWNIE_W<'_, DIER_OUTPUT_SPEC> {
        DOWNIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Update event interrupt enable"]
    #[inline(always)]
    pub fn UEIE(&mut self) -> UEIE_W<'_, DIER_OUTPUT_SPEC> {
        UEIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Repetition register update OK interrupt Enable"]
    #[inline(always)]
    pub fn REPOKIE(&mut self) -> REPOKIE_W<'_, DIER_OUTPUT_SPEC> {
        REPOKIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn CC2IE(&mut self) -> CC2IE_W<'_, DIER_OUTPUT_SPEC> {
        CC2IE_W::new(self, 9)
    }
    #[doc = "Bit 19 - Compare register 2 update OK interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn CMP2OKIE(&mut self) -> CMP2OKIE_W<'_, DIER_OUTPUT_SPEC> {
        CMP2OKIE_W::new(self, 19)
    }
    #[doc = "Bit 23 - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn UEDE(&mut self) -> UEDE_W<'_, DIER_OUTPUT_SPEC> {
        UEDE_W::new(self, 23)
    }
}
#[doc = "LPTIM1 interrupt enable register \\[alternate\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`dier_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIER_OUTPUT_SPEC;
impl crate::RegisterSpec for DIER_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dier_output::R`](R) reader structure"]
impl crate::Readable for DIER_OUTPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dier_output::W`](W) writer structure"]
impl crate::Writable for DIER_OUTPUT_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DIER_output to value 0"]
impl crate::Resettable for DIER_OUTPUT_SPEC {}
