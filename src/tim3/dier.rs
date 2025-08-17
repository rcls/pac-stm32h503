#[doc = "Register `DIER` reader"]
pub type R = crate::R<DIER_SPEC>;
#[doc = "Register `DIER` writer"]
pub type W = crate::W<DIER_SPEC>;
#[doc = "Update interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIE_A {
    #[doc = "0: Update interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Update interrupt enabled."]
    B_0x1 = 1,
}
impl From<UIE_A> for bool {
    #[inline(always)]
    fn from(variant: UIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIE` reader - Update interrupt enable"]
pub type UIE_R = crate::BitReader<UIE_A>;
impl UIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UIE_A {
        match self.bits {
            false => UIE_A::B_0x0,
            true => UIE_A::B_0x1,
        }
    }
    #[doc = "Update interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == UIE_A::B_0x0
    }
    #[doc = "Update interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == UIE_A::B_0x1
    }
}
#[doc = "Field `UIE` writer - Update interrupt enable"]
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG, UIE_A>;
impl<'a, REG> UIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UIE_A::B_0x0)
    }
    #[doc = "Update interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UIE_A::B_0x1)
    }
}
#[doc = "Capture/Compare 1 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1IE_A {
    #[doc = "0: CC1 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: CC1 interrupt enabled."]
    B_0x1 = 1,
}
impl From<CC1IE_A> for bool {
    #[inline(always)]
    fn from(variant: CC1IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1IE` reader - Capture/Compare 1 interrupt enable"]
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
    #[doc = "CC1 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC1IE_A::B_0x0
    }
    #[doc = "CC1 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC1IE_A::B_0x1
    }
}
#[doc = "Field `CC1IE` writer - Capture/Compare 1 interrupt enable"]
pub type CC1IE_W<'a, REG> = crate::BitWriter<'a, REG, CC1IE_A>;
impl<'a, REG> CC1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC1 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IE_A::B_0x0)
    }
    #[doc = "CC1 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IE_A::B_0x1)
    }
}
#[doc = "Capture/Compare 2 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC2IE_A {
    #[doc = "0: CC2 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: CC2 interrupt enabled."]
    B_0x1 = 1,
}
impl From<CC2IE_A> for bool {
    #[inline(always)]
    fn from(variant: CC2IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC2IE` reader - Capture/Compare 2 interrupt enable"]
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
    #[doc = "CC2 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC2IE_A::B_0x0
    }
    #[doc = "CC2 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC2IE_A::B_0x1
    }
}
#[doc = "Field `CC2IE` writer - Capture/Compare 2 interrupt enable"]
pub type CC2IE_W<'a, REG> = crate::BitWriter<'a, REG, CC2IE_A>;
impl<'a, REG> CC2IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC2 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC2IE_A::B_0x0)
    }
    #[doc = "CC2 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC2IE_A::B_0x1)
    }
}
#[doc = "Capture/Compare 3 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC3IE_A {
    #[doc = "0: CC3 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: CC3 interrupt enabled."]
    B_0x1 = 1,
}
impl From<CC3IE_A> for bool {
    #[inline(always)]
    fn from(variant: CC3IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC3IE` reader - Capture/Compare 3 interrupt enable"]
pub type CC3IE_R = crate::BitReader<CC3IE_A>;
impl CC3IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC3IE_A {
        match self.bits {
            false => CC3IE_A::B_0x0,
            true => CC3IE_A::B_0x1,
        }
    }
    #[doc = "CC3 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC3IE_A::B_0x0
    }
    #[doc = "CC3 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC3IE_A::B_0x1
    }
}
#[doc = "Field `CC3IE` writer - Capture/Compare 3 interrupt enable"]
pub type CC3IE_W<'a, REG> = crate::BitWriter<'a, REG, CC3IE_A>;
impl<'a, REG> CC3IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC3 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC3IE_A::B_0x0)
    }
    #[doc = "CC3 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC3IE_A::B_0x1)
    }
}
#[doc = "Capture/Compare 4 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC4IE_A {
    #[doc = "0: CC4 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: CC4 interrupt enabled."]
    B_0x1 = 1,
}
impl From<CC4IE_A> for bool {
    #[inline(always)]
    fn from(variant: CC4IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC4IE` reader - Capture/Compare 4 interrupt enable"]
pub type CC4IE_R = crate::BitReader<CC4IE_A>;
impl CC4IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC4IE_A {
        match self.bits {
            false => CC4IE_A::B_0x0,
            true => CC4IE_A::B_0x1,
        }
    }
    #[doc = "CC4 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC4IE_A::B_0x0
    }
    #[doc = "CC4 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC4IE_A::B_0x1
    }
}
#[doc = "Field `CC4IE` writer - Capture/Compare 4 interrupt enable"]
pub type CC4IE_W<'a, REG> = crate::BitWriter<'a, REG, CC4IE_A>;
impl<'a, REG> CC4IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC4 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC4IE_A::B_0x0)
    }
    #[doc = "CC4 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC4IE_A::B_0x1)
    }
}
#[doc = "Trigger interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIE_A {
    #[doc = "0: Trigger interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Trigger interrupt enabled."]
    B_0x1 = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIE` reader - Trigger interrupt enable"]
pub type TIE_R = crate::BitReader<TIE_A>;
impl TIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::B_0x0,
            true => TIE_A::B_0x1,
        }
    }
    #[doc = "Trigger interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIE_A::B_0x0
    }
    #[doc = "Trigger interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIE_A::B_0x1
    }
}
#[doc = "Field `TIE` writer - Trigger interrupt enable"]
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG, TIE_A>;
impl<'a, REG> TIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIE_A::B_0x0)
    }
    #[doc = "Trigger interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIE_A::B_0x1)
    }
}
#[doc = "Update DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDE_A {
    #[doc = "0: Update DMA request disabled."]
    B_0x0 = 0,
    #[doc = "1: Update DMA request enabled."]
    B_0x1 = 1,
}
impl From<UDE_A> for bool {
    #[inline(always)]
    fn from(variant: UDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDE` reader - Update DMA request enable"]
pub type UDE_R = crate::BitReader<UDE_A>;
impl UDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UDE_A {
        match self.bits {
            false => UDE_A::B_0x0,
            true => UDE_A::B_0x1,
        }
    }
    #[doc = "Update DMA request disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == UDE_A::B_0x0
    }
    #[doc = "Update DMA request enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == UDE_A::B_0x1
    }
}
#[doc = "Field `UDE` writer - Update DMA request enable"]
pub type UDE_W<'a, REG> = crate::BitWriter<'a, REG, UDE_A>;
impl<'a, REG> UDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update DMA request disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UDE_A::B_0x0)
    }
    #[doc = "Update DMA request enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UDE_A::B_0x1)
    }
}
#[doc = "Capture/Compare 1 DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1DE_A {
    #[doc = "0: CC1 DMA request disabled."]
    B_0x0 = 0,
    #[doc = "1: CC1 DMA request enabled."]
    B_0x1 = 1,
}
impl From<CC1DE_A> for bool {
    #[inline(always)]
    fn from(variant: CC1DE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1DE` reader - Capture/Compare 1 DMA request enable"]
pub type CC1DE_R = crate::BitReader<CC1DE_A>;
impl CC1DE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC1DE_A {
        match self.bits {
            false => CC1DE_A::B_0x0,
            true => CC1DE_A::B_0x1,
        }
    }
    #[doc = "CC1 DMA request disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC1DE_A::B_0x0
    }
    #[doc = "CC1 DMA request enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC1DE_A::B_0x1
    }
}
#[doc = "Field `CC1DE` writer - Capture/Compare 1 DMA request enable"]
pub type CC1DE_W<'a, REG> = crate::BitWriter<'a, REG, CC1DE_A>;
impl<'a, REG> CC1DE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC1 DMA request disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1DE_A::B_0x0)
    }
    #[doc = "CC1 DMA request enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1DE_A::B_0x1)
    }
}
#[doc = "Capture/Compare 2 DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC2DE_A {
    #[doc = "0: CC2 DMA request disabled."]
    B_0x0 = 0,
    #[doc = "1: CC2 DMA request enabled."]
    B_0x1 = 1,
}
impl From<CC2DE_A> for bool {
    #[inline(always)]
    fn from(variant: CC2DE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC2DE` reader - Capture/Compare 2 DMA request enable"]
pub type CC2DE_R = crate::BitReader<CC2DE_A>;
impl CC2DE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC2DE_A {
        match self.bits {
            false => CC2DE_A::B_0x0,
            true => CC2DE_A::B_0x1,
        }
    }
    #[doc = "CC2 DMA request disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC2DE_A::B_0x0
    }
    #[doc = "CC2 DMA request enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC2DE_A::B_0x1
    }
}
#[doc = "Field `CC2DE` writer - Capture/Compare 2 DMA request enable"]
pub type CC2DE_W<'a, REG> = crate::BitWriter<'a, REG, CC2DE_A>;
impl<'a, REG> CC2DE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC2 DMA request disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC2DE_A::B_0x0)
    }
    #[doc = "CC2 DMA request enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC2DE_A::B_0x1)
    }
}
#[doc = "Capture/Compare 3 DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC3DE_A {
    #[doc = "0: CC3 DMA request disabled."]
    B_0x0 = 0,
    #[doc = "1: CC3 DMA request enabled."]
    B_0x1 = 1,
}
impl From<CC3DE_A> for bool {
    #[inline(always)]
    fn from(variant: CC3DE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC3DE` reader - Capture/Compare 3 DMA request enable"]
pub type CC3DE_R = crate::BitReader<CC3DE_A>;
impl CC3DE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC3DE_A {
        match self.bits {
            false => CC3DE_A::B_0x0,
            true => CC3DE_A::B_0x1,
        }
    }
    #[doc = "CC3 DMA request disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC3DE_A::B_0x0
    }
    #[doc = "CC3 DMA request enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC3DE_A::B_0x1
    }
}
#[doc = "Field `CC3DE` writer - Capture/Compare 3 DMA request enable"]
pub type CC3DE_W<'a, REG> = crate::BitWriter<'a, REG, CC3DE_A>;
impl<'a, REG> CC3DE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC3 DMA request disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC3DE_A::B_0x0)
    }
    #[doc = "CC3 DMA request enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC3DE_A::B_0x1)
    }
}
#[doc = "Capture/Compare 4 DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC4DE_A {
    #[doc = "0: CC4 DMA request disabled."]
    B_0x0 = 0,
    #[doc = "1: CC4 DMA request enabled."]
    B_0x1 = 1,
}
impl From<CC4DE_A> for bool {
    #[inline(always)]
    fn from(variant: CC4DE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC4DE` reader - Capture/Compare 4 DMA request enable"]
pub type CC4DE_R = crate::BitReader<CC4DE_A>;
impl CC4DE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC4DE_A {
        match self.bits {
            false => CC4DE_A::B_0x0,
            true => CC4DE_A::B_0x1,
        }
    }
    #[doc = "CC4 DMA request disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC4DE_A::B_0x0
    }
    #[doc = "CC4 DMA request enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC4DE_A::B_0x1
    }
}
#[doc = "Field `CC4DE` writer - Capture/Compare 4 DMA request enable"]
pub type CC4DE_W<'a, REG> = crate::BitWriter<'a, REG, CC4DE_A>;
impl<'a, REG> CC4DE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC4 DMA request disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC4DE_A::B_0x0)
    }
    #[doc = "CC4 DMA request enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC4DE_A::B_0x1)
    }
}
#[doc = "Trigger DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDE_A {
    #[doc = "0: Trigger DMA request disabled."]
    B_0x0 = 0,
    #[doc = "1: Trigger DMA request enabled."]
    B_0x1 = 1,
}
impl From<TDE_A> for bool {
    #[inline(always)]
    fn from(variant: TDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDE` reader - Trigger DMA request enable"]
pub type TDE_R = crate::BitReader<TDE_A>;
impl TDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TDE_A {
        match self.bits {
            false => TDE_A::B_0x0,
            true => TDE_A::B_0x1,
        }
    }
    #[doc = "Trigger DMA request disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TDE_A::B_0x0
    }
    #[doc = "Trigger DMA request enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TDE_A::B_0x1
    }
}
#[doc = "Field `TDE` writer - Trigger DMA request enable"]
pub type TDE_W<'a, REG> = crate::BitWriter<'a, REG, TDE_A>;
impl<'a, REG> TDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger DMA request disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TDE_A::B_0x0)
    }
    #[doc = "Trigger DMA request enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TDE_A::B_0x1)
    }
}
#[doc = "Index interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDXIE_A {
    #[doc = "0: Index interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Index interrupt enabled"]
    B_0x1 = 1,
}
impl From<IDXIE_A> for bool {
    #[inline(always)]
    fn from(variant: IDXIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDXIE` reader - Index interrupt enable"]
pub type IDXIE_R = crate::BitReader<IDXIE_A>;
impl IDXIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDXIE_A {
        match self.bits {
            false => IDXIE_A::B_0x0,
            true => IDXIE_A::B_0x1,
        }
    }
    #[doc = "Index interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IDXIE_A::B_0x0
    }
    #[doc = "Index interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IDXIE_A::B_0x1
    }
}
#[doc = "Field `IDXIE` writer - Index interrupt enable"]
pub type IDXIE_W<'a, REG> = crate::BitWriter<'a, REG, IDXIE_A>;
impl<'a, REG> IDXIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Index interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IDXIE_A::B_0x0)
    }
    #[doc = "Index interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IDXIE_A::B_0x1)
    }
}
#[doc = "Direction change interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRIE_A {
    #[doc = "0: Direction change interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Direction change interrupt enabled"]
    B_0x1 = 1,
}
impl From<DIRIE_A> for bool {
    #[inline(always)]
    fn from(variant: DIRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRIE` reader - Direction change interrupt enable"]
pub type DIRIE_R = crate::BitReader<DIRIE_A>;
impl DIRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIRIE_A {
        match self.bits {
            false => DIRIE_A::B_0x0,
            true => DIRIE_A::B_0x1,
        }
    }
    #[doc = "Direction change interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DIRIE_A::B_0x0
    }
    #[doc = "Direction change interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DIRIE_A::B_0x1
    }
}
#[doc = "Field `DIRIE` writer - Direction change interrupt enable"]
pub type DIRIE_W<'a, REG> = crate::BitWriter<'a, REG, DIRIE_A>;
impl<'a, REG> DIRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Direction change interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRIE_A::B_0x0)
    }
    #[doc = "Direction change interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRIE_A::B_0x1)
    }
}
#[doc = "Index error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IERRIE_A {
    #[doc = "0: Index error interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Index error interrupt enabled"]
    B_0x1 = 1,
}
impl From<IERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: IERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IERRIE` reader - Index error interrupt enable"]
pub type IERRIE_R = crate::BitReader<IERRIE_A>;
impl IERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IERRIE_A {
        match self.bits {
            false => IERRIE_A::B_0x0,
            true => IERRIE_A::B_0x1,
        }
    }
    #[doc = "Index error interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IERRIE_A::B_0x0
    }
    #[doc = "Index error interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IERRIE_A::B_0x1
    }
}
#[doc = "Field `IERRIE` writer - Index error interrupt enable"]
pub type IERRIE_W<'a, REG> = crate::BitWriter<'a, REG, IERRIE_A>;
impl<'a, REG> IERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Index error interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IERRIE_A::B_0x0)
    }
    #[doc = "Index error interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IERRIE_A::B_0x1)
    }
}
#[doc = "Transition error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TERRIE_A {
    #[doc = "0: Transition error interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Transition error interrupt enabled"]
    B_0x1 = 1,
}
impl From<TERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: TERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TERRIE` reader - Transition error interrupt enable"]
pub type TERRIE_R = crate::BitReader<TERRIE_A>;
impl TERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TERRIE_A {
        match self.bits {
            false => TERRIE_A::B_0x0,
            true => TERRIE_A::B_0x1,
        }
    }
    #[doc = "Transition error interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TERRIE_A::B_0x0
    }
    #[doc = "Transition error interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TERRIE_A::B_0x1
    }
}
#[doc = "Field `TERRIE` writer - Transition error interrupt enable"]
pub type TERRIE_W<'a, REG> = crate::BitWriter<'a, REG, TERRIE_A>;
impl<'a, REG> TERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transition error interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TERRIE_A::B_0x0)
    }
    #[doc = "Transition error interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TERRIE_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn UIE(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn CC1IE(&self) -> CC1IE_R {
        CC1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt enable"]
    #[inline(always)]
    pub fn CC2IE(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt enable"]
    #[inline(always)]
    pub fn CC3IE(&self) -> CC3IE_R {
        CC3IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt enable"]
    #[inline(always)]
    pub fn CC4IE(&self) -> CC4IE_R {
        CC4IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn TIE(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn UDE(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    pub fn CC1DE(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/Compare 2 DMA request enable"]
    #[inline(always)]
    pub fn CC2DE(&self) -> CC2DE_R {
        CC2DE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 DMA request enable"]
    #[inline(always)]
    pub fn CC3DE(&self) -> CC3DE_R {
        CC3DE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 DMA request enable"]
    #[inline(always)]
    pub fn CC4DE(&self) -> CC4DE_R {
        CC4DE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn TDE(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 20 - Index interrupt enable"]
    #[inline(always)]
    pub fn IDXIE(&self) -> IDXIE_R {
        IDXIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Direction change interrupt enable"]
    #[inline(always)]
    pub fn DIRIE(&self) -> DIRIE_R {
        DIRIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Index error interrupt enable"]
    #[inline(always)]
    pub fn IERRIE(&self) -> IERRIE_R {
        IERRIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transition error interrupt enable"]
    #[inline(always)]
    pub fn TERRIE(&self) -> TERRIE_R {
        TERRIE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn UIE(&mut self) -> UIE_W<'_, DIER_SPEC> {
        UIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn CC1IE(&mut self) -> CC1IE_W<'_, DIER_SPEC> {
        CC1IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt enable"]
    #[inline(always)]
    pub fn CC2IE(&mut self) -> CC2IE_W<'_, DIER_SPEC> {
        CC2IE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt enable"]
    #[inline(always)]
    pub fn CC3IE(&mut self) -> CC3IE_W<'_, DIER_SPEC> {
        CC3IE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt enable"]
    #[inline(always)]
    pub fn CC4IE(&mut self) -> CC4IE_W<'_, DIER_SPEC> {
        CC4IE_W::new(self, 4)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn TIE(&mut self) -> TIE_W<'_, DIER_SPEC> {
        TIE_W::new(self, 6)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn UDE(&mut self) -> UDE_W<'_, DIER_SPEC> {
        UDE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    pub fn CC1DE(&mut self) -> CC1DE_W<'_, DIER_SPEC> {
        CC1DE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/Compare 2 DMA request enable"]
    #[inline(always)]
    pub fn CC2DE(&mut self) -> CC2DE_W<'_, DIER_SPEC> {
        CC2DE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Capture/Compare 3 DMA request enable"]
    #[inline(always)]
    pub fn CC3DE(&mut self) -> CC3DE_W<'_, DIER_SPEC> {
        CC3DE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Capture/Compare 4 DMA request enable"]
    #[inline(always)]
    pub fn CC4DE(&mut self) -> CC4DE_W<'_, DIER_SPEC> {
        CC4DE_W::new(self, 12)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn TDE(&mut self) -> TDE_W<'_, DIER_SPEC> {
        TDE_W::new(self, 14)
    }
    #[doc = "Bit 20 - Index interrupt enable"]
    #[inline(always)]
    pub fn IDXIE(&mut self) -> IDXIE_W<'_, DIER_SPEC> {
        IDXIE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Direction change interrupt enable"]
    #[inline(always)]
    pub fn DIRIE(&mut self) -> DIRIE_W<'_, DIER_SPEC> {
        DIRIE_W::new(self, 21)
    }
    #[doc = "Bit 22 - Index error interrupt enable"]
    #[inline(always)]
    pub fn IERRIE(&mut self) -> IERRIE_W<'_, DIER_SPEC> {
        IERRIE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Transition error interrupt enable"]
    #[inline(always)]
    pub fn TERRIE(&mut self) -> TERRIE_W<'_, DIER_SPEC> {
        TERRIE_W::new(self, 23)
    }
}
#[doc = "TIM3 DMA/Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIER_SPEC;
impl crate::RegisterSpec for DIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dier::R`](R) reader structure"]
impl crate::Readable for DIER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dier::W`](W) writer structure"]
impl crate::Writable for DIER_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DIER to value 0"]
impl crate::Resettable for DIER_SPEC {}
