#[doc = "Register `IR` reader"]
pub type R = crate::R<IR_SPEC>;
#[doc = "Register `IR` writer"]
pub type W = crate::W<IR_SPEC>;
#[doc = "Rx FIFO 0 new message\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0N_A {
    #[doc = "0: No new message written to Rx FIFO 0"]
    B_0x0 = 0,
    #[doc = "1: New message written to Rx FIFO 0"]
    B_0x1 = 1,
}
impl From<RF0N_A> for bool {
    #[inline(always)]
    fn from(variant: RF0N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF0N` reader - Rx FIFO 0 new message"]
pub type RF0N_R = crate::BitReader<RF0N_A>;
impl RF0N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RF0N_A {
        match self.bits {
            false => RF0N_A::B_0x0,
            true => RF0N_A::B_0x1,
        }
    }
    #[doc = "No new message written to Rx FIFO 0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RF0N_A::B_0x0
    }
    #[doc = "New message written to Rx FIFO 0"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RF0N_A::B_0x1
    }
}
#[doc = "Field `RF0N` writer - Rx FIFO 0 new message"]
pub type RF0N_W<'a, REG> = crate::BitWriter<'a, REG, RF0N_A>;
impl<'a, REG> RF0N_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No new message written to Rx FIFO 0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF0N_A::B_0x0)
    }
    #[doc = "New message written to Rx FIFO 0"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF0N_A::B_0x1)
    }
}
#[doc = "Rx FIFO 0 full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0F_A {
    #[doc = "0: Rx FIFO 0 not full"]
    B_0x0 = 0,
    #[doc = "1: Rx FIFO 0 full"]
    B_0x1 = 1,
}
impl From<RF0F_A> for bool {
    #[inline(always)]
    fn from(variant: RF0F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF0F` reader - Rx FIFO 0 full"]
pub type RF0F_R = crate::BitReader<RF0F_A>;
impl RF0F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RF0F_A {
        match self.bits {
            false => RF0F_A::B_0x0,
            true => RF0F_A::B_0x1,
        }
    }
    #[doc = "Rx FIFO 0 not full"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RF0F_A::B_0x0
    }
    #[doc = "Rx FIFO 0 full"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RF0F_A::B_0x1
    }
}
#[doc = "Field `RF0F` writer - Rx FIFO 0 full"]
pub type RF0F_W<'a, REG> = crate::BitWriter<'a, REG, RF0F_A>;
impl<'a, REG> RF0F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rx FIFO 0 not full"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF0F_A::B_0x0)
    }
    #[doc = "Rx FIFO 0 full"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF0F_A::B_0x1)
    }
}
#[doc = "Rx FIFO 0 message lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0L_A {
    #[doc = "0: No Rx FIFO 0 message lost"]
    B_0x0 = 0,
    #[doc = "1: Rx FIFO 0 message lost"]
    B_0x1 = 1,
}
impl From<RF0L_A> for bool {
    #[inline(always)]
    fn from(variant: RF0L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF0L` reader - Rx FIFO 0 message lost"]
pub type RF0L_R = crate::BitReader<RF0L_A>;
impl RF0L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RF0L_A {
        match self.bits {
            false => RF0L_A::B_0x0,
            true => RF0L_A::B_0x1,
        }
    }
    #[doc = "No Rx FIFO 0 message lost"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RF0L_A::B_0x0
    }
    #[doc = "Rx FIFO 0 message lost"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RF0L_A::B_0x1
    }
}
#[doc = "Field `RF0L` writer - Rx FIFO 0 message lost"]
pub type RF0L_W<'a, REG> = crate::BitWriter<'a, REG, RF0L_A>;
impl<'a, REG> RF0L_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Rx FIFO 0 message lost"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF0L_A::B_0x0)
    }
    #[doc = "Rx FIFO 0 message lost"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF0L_A::B_0x1)
    }
}
#[doc = "Rx FIFO 1 new message\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF1N_A {
    #[doc = "0: No new message written to Rx FIFO 1"]
    B_0x0 = 0,
    #[doc = "1: New message written to Rx FIFO 1"]
    B_0x1 = 1,
}
impl From<RF1N_A> for bool {
    #[inline(always)]
    fn from(variant: RF1N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF1N` reader - Rx FIFO 1 new message"]
pub type RF1N_R = crate::BitReader<RF1N_A>;
impl RF1N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RF1N_A {
        match self.bits {
            false => RF1N_A::B_0x0,
            true => RF1N_A::B_0x1,
        }
    }
    #[doc = "No new message written to Rx FIFO 1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RF1N_A::B_0x0
    }
    #[doc = "New message written to Rx FIFO 1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RF1N_A::B_0x1
    }
}
#[doc = "Field `RF1N` writer - Rx FIFO 1 new message"]
pub type RF1N_W<'a, REG> = crate::BitWriter<'a, REG, RF1N_A>;
impl<'a, REG> RF1N_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No new message written to Rx FIFO 1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF1N_A::B_0x0)
    }
    #[doc = "New message written to Rx FIFO 1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF1N_A::B_0x1)
    }
}
#[doc = "Rx FIFO 1 full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF1F_A {
    #[doc = "0: Rx FIFO 1 not full"]
    B_0x0 = 0,
    #[doc = "1: Rx FIFO 1 full"]
    B_0x1 = 1,
}
impl From<RF1F_A> for bool {
    #[inline(always)]
    fn from(variant: RF1F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF1F` reader - Rx FIFO 1 full"]
pub type RF1F_R = crate::BitReader<RF1F_A>;
impl RF1F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RF1F_A {
        match self.bits {
            false => RF1F_A::B_0x0,
            true => RF1F_A::B_0x1,
        }
    }
    #[doc = "Rx FIFO 1 not full"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RF1F_A::B_0x0
    }
    #[doc = "Rx FIFO 1 full"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RF1F_A::B_0x1
    }
}
#[doc = "Field `RF1F` writer - Rx FIFO 1 full"]
pub type RF1F_W<'a, REG> = crate::BitWriter<'a, REG, RF1F_A>;
impl<'a, REG> RF1F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rx FIFO 1 not full"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF1F_A::B_0x0)
    }
    #[doc = "Rx FIFO 1 full"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF1F_A::B_0x1)
    }
}
#[doc = "Rx FIFO 1 message lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF1L_A {
    #[doc = "0: No Rx FIFO 1 message lost"]
    B_0x0 = 0,
    #[doc = "1: Rx FIFO 1 message lost"]
    B_0x1 = 1,
}
impl From<RF1L_A> for bool {
    #[inline(always)]
    fn from(variant: RF1L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF1L` reader - Rx FIFO 1 message lost"]
pub type RF1L_R = crate::BitReader<RF1L_A>;
impl RF1L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RF1L_A {
        match self.bits {
            false => RF1L_A::B_0x0,
            true => RF1L_A::B_0x1,
        }
    }
    #[doc = "No Rx FIFO 1 message lost"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RF1L_A::B_0x0
    }
    #[doc = "Rx FIFO 1 message lost"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RF1L_A::B_0x1
    }
}
#[doc = "Field `RF1L` writer - Rx FIFO 1 message lost"]
pub type RF1L_W<'a, REG> = crate::BitWriter<'a, REG, RF1L_A>;
impl<'a, REG> RF1L_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Rx FIFO 1 message lost"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF1L_A::B_0x0)
    }
    #[doc = "Rx FIFO 1 message lost"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF1L_A::B_0x1)
    }
}
#[doc = "High-priority message\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPM_A {
    #[doc = "0: No high-priority message received"]
    B_0x0 = 0,
    #[doc = "1: High-priority message received"]
    B_0x1 = 1,
}
impl From<HPM_A> for bool {
    #[inline(always)]
    fn from(variant: HPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HPM` reader - High-priority message"]
pub type HPM_R = crate::BitReader<HPM_A>;
impl HPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HPM_A {
        match self.bits {
            false => HPM_A::B_0x0,
            true => HPM_A::B_0x1,
        }
    }
    #[doc = "No high-priority message received"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HPM_A::B_0x0
    }
    #[doc = "High-priority message received"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HPM_A::B_0x1
    }
}
#[doc = "Field `HPM` writer - High-priority message"]
pub type HPM_W<'a, REG> = crate::BitWriter<'a, REG, HPM_A>;
impl<'a, REG> HPM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No high-priority message received"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HPM_A::B_0x0)
    }
    #[doc = "High-priority message received"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HPM_A::B_0x1)
    }
}
#[doc = "Transmission completed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC_A {
    #[doc = "0: No transmission completed"]
    B_0x0 = 0,
    #[doc = "1: Transmission completed"]
    B_0x1 = 1,
}
impl From<TC_A> for bool {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC` reader - Transmission completed"]
pub type TC_R = crate::BitReader<TC_A>;
impl TC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TC_A {
        match self.bits {
            false => TC_A::B_0x0,
            true => TC_A::B_0x1,
        }
    }
    #[doc = "No transmission completed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TC_A::B_0x0
    }
    #[doc = "Transmission completed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TC_A::B_0x1
    }
}
#[doc = "Field `TC` writer - Transmission completed"]
pub type TC_W<'a, REG> = crate::BitWriter<'a, REG, TC_A>;
impl<'a, REG> TC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No transmission completed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TC_A::B_0x0)
    }
    #[doc = "Transmission completed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TC_A::B_0x1)
    }
}
#[doc = "Transmission cancellation finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCF_A {
    #[doc = "0: No transmission cancellation finished"]
    B_0x0 = 0,
    #[doc = "1: Transmission cancellation finished"]
    B_0x1 = 1,
}
impl From<TCF_A> for bool {
    #[inline(always)]
    fn from(variant: TCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCF` reader - Transmission cancellation finished"]
pub type TCF_R = crate::BitReader<TCF_A>;
impl TCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCF_A {
        match self.bits {
            false => TCF_A::B_0x0,
            true => TCF_A::B_0x1,
        }
    }
    #[doc = "No transmission cancellation finished"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TCF_A::B_0x0
    }
    #[doc = "Transmission cancellation finished"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TCF_A::B_0x1
    }
}
#[doc = "Field `TCF` writer - Transmission cancellation finished"]
pub type TCF_W<'a, REG> = crate::BitWriter<'a, REG, TCF_A>;
impl<'a, REG> TCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No transmission cancellation finished"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TCF_A::B_0x0)
    }
    #[doc = "Transmission cancellation finished"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TCF_A::B_0x1)
    }
}
#[doc = "Tx FIFO empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFE_A {
    #[doc = "0: Tx FIFO non-empty"]
    B_0x0 = 0,
    #[doc = "1: Tx FIFO empty"]
    B_0x1 = 1,
}
impl From<TFE_A> for bool {
    #[inline(always)]
    fn from(variant: TFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFE` reader - Tx FIFO empty"]
pub type TFE_R = crate::BitReader<TFE_A>;
impl TFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TFE_A {
        match self.bits {
            false => TFE_A::B_0x0,
            true => TFE_A::B_0x1,
        }
    }
    #[doc = "Tx FIFO non-empty"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TFE_A::B_0x0
    }
    #[doc = "Tx FIFO empty"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TFE_A::B_0x1
    }
}
#[doc = "Field `TFE` writer - Tx FIFO empty"]
pub type TFE_W<'a, REG> = crate::BitWriter<'a, REG, TFE_A>;
impl<'a, REG> TFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx FIFO non-empty"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TFE_A::B_0x0)
    }
    #[doc = "Tx FIFO empty"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TFE_A::B_0x1)
    }
}
#[doc = "Tx event FIFO New Entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEFN_A {
    #[doc = "0: Tx event FIFO unchanged"]
    B_0x0 = 0,
    #[doc = "1: Tx handler wrote Tx event FIFO element."]
    B_0x1 = 1,
}
impl From<TEFN_A> for bool {
    #[inline(always)]
    fn from(variant: TEFN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEFN` reader - Tx event FIFO New Entry"]
pub type TEFN_R = crate::BitReader<TEFN_A>;
impl TEFN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEFN_A {
        match self.bits {
            false => TEFN_A::B_0x0,
            true => TEFN_A::B_0x1,
        }
    }
    #[doc = "Tx event FIFO unchanged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TEFN_A::B_0x0
    }
    #[doc = "Tx handler wrote Tx event FIFO element."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TEFN_A::B_0x1
    }
}
#[doc = "Field `TEFN` writer - Tx event FIFO New Entry"]
pub type TEFN_W<'a, REG> = crate::BitWriter<'a, REG, TEFN_A>;
impl<'a, REG> TEFN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx event FIFO unchanged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TEFN_A::B_0x0)
    }
    #[doc = "Tx handler wrote Tx event FIFO element."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TEFN_A::B_0x1)
    }
}
#[doc = "Tx event FIFO full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEFF_A {
    #[doc = "0: Tx event FIFO Not full"]
    B_0x0 = 0,
    #[doc = "1: Tx event FIFO full"]
    B_0x1 = 1,
}
impl From<TEFF_A> for bool {
    #[inline(always)]
    fn from(variant: TEFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEFF` reader - Tx event FIFO full"]
pub type TEFF_R = crate::BitReader<TEFF_A>;
impl TEFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEFF_A {
        match self.bits {
            false => TEFF_A::B_0x0,
            true => TEFF_A::B_0x1,
        }
    }
    #[doc = "Tx event FIFO Not full"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TEFF_A::B_0x0
    }
    #[doc = "Tx event FIFO full"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TEFF_A::B_0x1
    }
}
#[doc = "Field `TEFF` writer - Tx event FIFO full"]
pub type TEFF_W<'a, REG> = crate::BitWriter<'a, REG, TEFF_A>;
impl<'a, REG> TEFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx event FIFO Not full"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TEFF_A::B_0x0)
    }
    #[doc = "Tx event FIFO full"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TEFF_A::B_0x1)
    }
}
#[doc = "Tx event FIFO element lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEFL_A {
    #[doc = "0: No Tx event FIFO element lost"]
    B_0x0 = 0,
    #[doc = "1: Tx event FIFO element lost"]
    B_0x1 = 1,
}
impl From<TEFL_A> for bool {
    #[inline(always)]
    fn from(variant: TEFL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEFL` reader - Tx event FIFO element lost"]
pub type TEFL_R = crate::BitReader<TEFL_A>;
impl TEFL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEFL_A {
        match self.bits {
            false => TEFL_A::B_0x0,
            true => TEFL_A::B_0x1,
        }
    }
    #[doc = "No Tx event FIFO element lost"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TEFL_A::B_0x0
    }
    #[doc = "Tx event FIFO element lost"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TEFL_A::B_0x1
    }
}
#[doc = "Field `TEFL` writer - Tx event FIFO element lost"]
pub type TEFL_W<'a, REG> = crate::BitWriter<'a, REG, TEFL_A>;
impl<'a, REG> TEFL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Tx event FIFO element lost"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TEFL_A::B_0x0)
    }
    #[doc = "Tx event FIFO element lost"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TEFL_A::B_0x1)
    }
}
#[doc = "Timestamp wraparound\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSW_A {
    #[doc = "0: No timestamp counter wrap-around"]
    B_0x0 = 0,
    #[doc = "1: Timestamp counter wrapped around"]
    B_0x1 = 1,
}
impl From<TSW_A> for bool {
    #[inline(always)]
    fn from(variant: TSW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSW` reader - Timestamp wraparound"]
pub type TSW_R = crate::BitReader<TSW_A>;
impl TSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSW_A {
        match self.bits {
            false => TSW_A::B_0x0,
            true => TSW_A::B_0x1,
        }
    }
    #[doc = "No timestamp counter wrap-around"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TSW_A::B_0x0
    }
    #[doc = "Timestamp counter wrapped around"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TSW_A::B_0x1
    }
}
#[doc = "Field `TSW` writer - Timestamp wraparound"]
pub type TSW_W<'a, REG> = crate::BitWriter<'a, REG, TSW_A>;
impl<'a, REG> TSW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No timestamp counter wrap-around"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TSW_A::B_0x0)
    }
    #[doc = "Timestamp counter wrapped around"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TSW_A::B_0x1)
    }
}
#[doc = "Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRAF_A {
    #[doc = "0: No Message RAM access failure occurred"]
    B_0x0 = 0,
    #[doc = "1: Message RAM access failure occurred"]
    B_0x1 = 1,
}
impl From<MRAF_A> for bool {
    #[inline(always)]
    fn from(variant: MRAF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRAF` reader - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM."]
pub type MRAF_R = crate::BitReader<MRAF_A>;
impl MRAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MRAF_A {
        match self.bits {
            false => MRAF_A::B_0x0,
            true => MRAF_A::B_0x1,
        }
    }
    #[doc = "No Message RAM access failure occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MRAF_A::B_0x0
    }
    #[doc = "Message RAM access failure occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MRAF_A::B_0x1
    }
}
#[doc = "Field `MRAF` writer - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM."]
pub type MRAF_W<'a, REG> = crate::BitWriter<'a, REG, MRAF_A>;
impl<'a, REG> MRAF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Message RAM access failure occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MRAF_A::B_0x0)
    }
    #[doc = "Message RAM access failure occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MRAF_A::B_0x1)
    }
}
#[doc = "Timeout occurred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOO_A {
    #[doc = "0: No timeout"]
    B_0x0 = 0,
    #[doc = "1: Timeout reached"]
    B_0x1 = 1,
}
impl From<TOO_A> for bool {
    #[inline(always)]
    fn from(variant: TOO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOO` reader - Timeout occurred"]
pub type TOO_R = crate::BitReader<TOO_A>;
impl TOO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TOO_A {
        match self.bits {
            false => TOO_A::B_0x0,
            true => TOO_A::B_0x1,
        }
    }
    #[doc = "No timeout"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TOO_A::B_0x0
    }
    #[doc = "Timeout reached"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TOO_A::B_0x1
    }
}
#[doc = "Field `TOO` writer - Timeout occurred"]
pub type TOO_W<'a, REG> = crate::BitWriter<'a, REG, TOO_A>;
impl<'a, REG> TOO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No timeout"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TOO_A::B_0x0)
    }
    #[doc = "Timeout reached"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TOO_A::B_0x1)
    }
}
#[doc = "Error logging overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELO_A {
    #[doc = "0: CAN error logging counter did not overflow."]
    B_0x0 = 0,
    #[doc = "1: Overflow of CAN error logging counter occurred."]
    B_0x1 = 1,
}
impl From<ELO_A> for bool {
    #[inline(always)]
    fn from(variant: ELO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ELO` reader - Error logging overflow"]
pub type ELO_R = crate::BitReader<ELO_A>;
impl ELO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ELO_A {
        match self.bits {
            false => ELO_A::B_0x0,
            true => ELO_A::B_0x1,
        }
    }
    #[doc = "CAN error logging counter did not overflow."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ELO_A::B_0x0
    }
    #[doc = "Overflow of CAN error logging counter occurred."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ELO_A::B_0x1
    }
}
#[doc = "Field `ELO` writer - Error logging overflow"]
pub type ELO_W<'a, REG> = crate::BitWriter<'a, REG, ELO_A>;
impl<'a, REG> ELO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CAN error logging counter did not overflow."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ELO_A::B_0x0)
    }
    #[doc = "Overflow of CAN error logging counter occurred."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ELO_A::B_0x1)
    }
}
#[doc = "Error passive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EP_A {
    #[doc = "0: Error_Passive status unchanged"]
    B_0x0 = 0,
    #[doc = "1: Error_Passive status changed"]
    B_0x1 = 1,
}
impl From<EP_A> for bool {
    #[inline(always)]
    fn from(variant: EP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP` reader - Error passive"]
pub type EP_R = crate::BitReader<EP_A>;
impl EP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EP_A {
        match self.bits {
            false => EP_A::B_0x0,
            true => EP_A::B_0x1,
        }
    }
    #[doc = "Error_Passive status unchanged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EP_A::B_0x0
    }
    #[doc = "Error_Passive status changed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EP_A::B_0x1
    }
}
#[doc = "Field `EP` writer - Error passive"]
pub type EP_W<'a, REG> = crate::BitWriter<'a, REG, EP_A>;
impl<'a, REG> EP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error_Passive status unchanged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EP_A::B_0x0)
    }
    #[doc = "Error_Passive status changed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EP_A::B_0x1)
    }
}
#[doc = "Warning status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EW_A {
    #[doc = "0: Error_Warning status unchanged"]
    B_0x0 = 0,
    #[doc = "1: Error_Warning status changed"]
    B_0x1 = 1,
}
impl From<EW_A> for bool {
    #[inline(always)]
    fn from(variant: EW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EW` reader - Warning status"]
pub type EW_R = crate::BitReader<EW_A>;
impl EW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EW_A {
        match self.bits {
            false => EW_A::B_0x0,
            true => EW_A::B_0x1,
        }
    }
    #[doc = "Error_Warning status unchanged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EW_A::B_0x0
    }
    #[doc = "Error_Warning status changed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EW_A::B_0x1
    }
}
#[doc = "Field `EW` writer - Warning status"]
pub type EW_W<'a, REG> = crate::BitWriter<'a, REG, EW_A>;
impl<'a, REG> EW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error_Warning status unchanged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EW_A::B_0x0)
    }
    #[doc = "Error_Warning status changed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EW_A::B_0x1)
    }
}
#[doc = "Bus_Off status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BO_A {
    #[doc = "0: Bus_Off status unchanged"]
    B_0x0 = 0,
    #[doc = "1: Bus_Off status changed"]
    B_0x1 = 1,
}
impl From<BO_A> for bool {
    #[inline(always)]
    fn from(variant: BO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BO` reader - Bus_Off status"]
pub type BO_R = crate::BitReader<BO_A>;
impl BO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BO_A {
        match self.bits {
            false => BO_A::B_0x0,
            true => BO_A::B_0x1,
        }
    }
    #[doc = "Bus_Off status unchanged"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BO_A::B_0x0
    }
    #[doc = "Bus_Off status changed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BO_A::B_0x1
    }
}
#[doc = "Field `BO` writer - Bus_Off status"]
pub type BO_W<'a, REG> = crate::BitWriter<'a, REG, BO_A>;
impl<'a, REG> BO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus_Off status unchanged"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BO_A::B_0x0)
    }
    #[doc = "Bus_Off status changed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BO_A::B_0x1)
    }
}
#[doc = "Watchdog interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDI_A {
    #[doc = "0: No message RAM watchdog event occurred"]
    B_0x0 = 0,
    #[doc = "1: Message RAM watchdog event due to missing READY"]
    B_0x1 = 1,
}
impl From<WDI_A> for bool {
    #[inline(always)]
    fn from(variant: WDI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDI` reader - Watchdog interrupt"]
pub type WDI_R = crate::BitReader<WDI_A>;
impl WDI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDI_A {
        match self.bits {
            false => WDI_A::B_0x0,
            true => WDI_A::B_0x1,
        }
    }
    #[doc = "No message RAM watchdog event occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WDI_A::B_0x0
    }
    #[doc = "Message RAM watchdog event due to missing READY"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WDI_A::B_0x1
    }
}
#[doc = "Field `WDI` writer - Watchdog interrupt"]
pub type WDI_W<'a, REG> = crate::BitWriter<'a, REG, WDI_A>;
impl<'a, REG> WDI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No message RAM watchdog event occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WDI_A::B_0x0)
    }
    #[doc = "Message RAM watchdog event due to missing READY"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WDI_A::B_0x1)
    }
}
#[doc = "Protocol error in arbitration phase (nominal bit time is used)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEA_A {
    #[doc = "0: No protocol error in arbitration phase"]
    B_0x0 = 0,
    #[doc = "1: Protocol error in arbitration phase detected (PSR.LEC different from 0,7)"]
    B_0x1 = 1,
}
impl From<PEA_A> for bool {
    #[inline(always)]
    fn from(variant: PEA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEA` reader - Protocol error in arbitration phase (nominal bit time is used)"]
pub type PEA_R = crate::BitReader<PEA_A>;
impl PEA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEA_A {
        match self.bits {
            false => PEA_A::B_0x0,
            true => PEA_A::B_0x1,
        }
    }
    #[doc = "No protocol error in arbitration phase"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PEA_A::B_0x0
    }
    #[doc = "Protocol error in arbitration phase detected (PSR.LEC different from 0,7)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PEA_A::B_0x1
    }
}
#[doc = "Field `PEA` writer - Protocol error in arbitration phase (nominal bit time is used)"]
pub type PEA_W<'a, REG> = crate::BitWriter<'a, REG, PEA_A>;
impl<'a, REG> PEA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No protocol error in arbitration phase"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PEA_A::B_0x0)
    }
    #[doc = "Protocol error in arbitration phase detected (PSR.LEC different from 0,7)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PEA_A::B_0x1)
    }
}
#[doc = "Protocol error in data phase (data bit time is used)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PED_A {
    #[doc = "0: No protocol error in data phase"]
    B_0x0 = 0,
    #[doc = "1: Protocol error in data phase detected (PSR.DLEC different from 0,7)"]
    B_0x1 = 1,
}
impl From<PED_A> for bool {
    #[inline(always)]
    fn from(variant: PED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PED` reader - Protocol error in data phase (data bit time is used)"]
pub type PED_R = crate::BitReader<PED_A>;
impl PED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PED_A {
        match self.bits {
            false => PED_A::B_0x0,
            true => PED_A::B_0x1,
        }
    }
    #[doc = "No protocol error in data phase"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PED_A::B_0x0
    }
    #[doc = "Protocol error in data phase detected (PSR.DLEC different from 0,7)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PED_A::B_0x1
    }
}
#[doc = "Field `PED` writer - Protocol error in data phase (data bit time is used)"]
pub type PED_W<'a, REG> = crate::BitWriter<'a, REG, PED_A>;
impl<'a, REG> PED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No protocol error in data phase"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PED_A::B_0x0)
    }
    #[doc = "Protocol error in data phase detected (PSR.DLEC different from 0,7)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PED_A::B_0x1)
    }
}
#[doc = "Access to reserved address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARA_A {
    #[doc = "0: No access to reserved address occurred"]
    B_0x0 = 0,
    #[doc = "1: Access to reserved address occurred"]
    B_0x1 = 1,
}
impl From<ARA_A> for bool {
    #[inline(always)]
    fn from(variant: ARA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARA` reader - Access to reserved address"]
pub type ARA_R = crate::BitReader<ARA_A>;
impl ARA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ARA_A {
        match self.bits {
            false => ARA_A::B_0x0,
            true => ARA_A::B_0x1,
        }
    }
    #[doc = "No access to reserved address occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ARA_A::B_0x0
    }
    #[doc = "Access to reserved address occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ARA_A::B_0x1
    }
}
#[doc = "Field `ARA` writer - Access to reserved address"]
pub type ARA_W<'a, REG> = crate::BitWriter<'a, REG, ARA_A>;
impl<'a, REG> ARA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No access to reserved address occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ARA_A::B_0x0)
    }
    #[doc = "Access to reserved address occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ARA_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 new message"]
    #[inline(always)]
    pub fn RF0N(&self) -> RF0N_R {
        RF0N_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 full"]
    #[inline(always)]
    pub fn RF0F(&self) -> RF0F_R {
        RF0F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 message lost"]
    #[inline(always)]
    pub fn RF0L(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 1 new message"]
    #[inline(always)]
    pub fn RF1N(&self) -> RF1N_R {
        RF1N_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 full"]
    #[inline(always)]
    pub fn RF1F(&self) -> RF1F_R {
        RF1F_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 message lost"]
    #[inline(always)]
    pub fn RF1L(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - High-priority message"]
    #[inline(always)]
    pub fn HPM(&self) -> HPM_R {
        HPM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmission completed"]
    #[inline(always)]
    pub fn TC(&self) -> TC_R {
        TC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmission cancellation finished"]
    #[inline(always)]
    pub fn TCF(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tx FIFO empty"]
    #[inline(always)]
    pub fn TFE(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx event FIFO New Entry"]
    #[inline(always)]
    pub fn TEFN(&self) -> TEFN_R {
        TEFN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx event FIFO full"]
    #[inline(always)]
    pub fn TEFF(&self) -> TEFF_R {
        TEFF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx event FIFO element lost"]
    #[inline(always)]
    pub fn TEFL(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timestamp wraparound"]
    #[inline(always)]
    pub fn TSW(&self) -> TSW_R {
        TSW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM."]
    #[inline(always)]
    pub fn MRAF(&self) -> MRAF_R {
        MRAF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timeout occurred"]
    #[inline(always)]
    pub fn TOO(&self) -> TOO_R {
        TOO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Error logging overflow"]
    #[inline(always)]
    pub fn ELO(&self) -> ELO_R {
        ELO_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Error passive"]
    #[inline(always)]
    pub fn EP(&self) -> EP_R {
        EP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Warning status"]
    #[inline(always)]
    pub fn EW(&self) -> EW_R {
        EW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bus_Off status"]
    #[inline(always)]
    pub fn BO(&self) -> BO_R {
        BO_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Watchdog interrupt"]
    #[inline(always)]
    pub fn WDI(&self) -> WDI_R {
        WDI_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Protocol error in arbitration phase (nominal bit time is used)"]
    #[inline(always)]
    pub fn PEA(&self) -> PEA_R {
        PEA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Protocol error in data phase (data bit time is used)"]
    #[inline(always)]
    pub fn PED(&self) -> PED_R {
        PED_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Access to reserved address"]
    #[inline(always)]
    pub fn ARA(&self) -> ARA_R {
        ARA_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 new message"]
    #[inline(always)]
    pub fn RF0N(&mut self) -> RF0N_W<'_, IR_SPEC> {
        RF0N_W::new(self, 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 full"]
    #[inline(always)]
    pub fn RF0F(&mut self) -> RF0F_W<'_, IR_SPEC> {
        RF0F_W::new(self, 1)
    }
    #[doc = "Bit 2 - Rx FIFO 0 message lost"]
    #[inline(always)]
    pub fn RF0L(&mut self) -> RF0L_W<'_, IR_SPEC> {
        RF0L_W::new(self, 2)
    }
    #[doc = "Bit 3 - Rx FIFO 1 new message"]
    #[inline(always)]
    pub fn RF1N(&mut self) -> RF1N_W<'_, IR_SPEC> {
        RF1N_W::new(self, 3)
    }
    #[doc = "Bit 4 - Rx FIFO 1 full"]
    #[inline(always)]
    pub fn RF1F(&mut self) -> RF1F_W<'_, IR_SPEC> {
        RF1F_W::new(self, 4)
    }
    #[doc = "Bit 5 - Rx FIFO 1 message lost"]
    #[inline(always)]
    pub fn RF1L(&mut self) -> RF1L_W<'_, IR_SPEC> {
        RF1L_W::new(self, 5)
    }
    #[doc = "Bit 6 - High-priority message"]
    #[inline(always)]
    pub fn HPM(&mut self) -> HPM_W<'_, IR_SPEC> {
        HPM_W::new(self, 6)
    }
    #[doc = "Bit 7 - Transmission completed"]
    #[inline(always)]
    pub fn TC(&mut self) -> TC_W<'_, IR_SPEC> {
        TC_W::new(self, 7)
    }
    #[doc = "Bit 8 - Transmission cancellation finished"]
    #[inline(always)]
    pub fn TCF(&mut self) -> TCF_W<'_, IR_SPEC> {
        TCF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Tx FIFO empty"]
    #[inline(always)]
    pub fn TFE(&mut self) -> TFE_W<'_, IR_SPEC> {
        TFE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Tx event FIFO New Entry"]
    #[inline(always)]
    pub fn TEFN(&mut self) -> TEFN_W<'_, IR_SPEC> {
        TEFN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Tx event FIFO full"]
    #[inline(always)]
    pub fn TEFF(&mut self) -> TEFF_W<'_, IR_SPEC> {
        TEFF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Tx event FIFO element lost"]
    #[inline(always)]
    pub fn TEFL(&mut self) -> TEFL_W<'_, IR_SPEC> {
        TEFL_W::new(self, 12)
    }
    #[doc = "Bit 13 - Timestamp wraparound"]
    #[inline(always)]
    pub fn TSW(&mut self) -> TSW_W<'_, IR_SPEC> {
        TSW_W::new(self, 13)
    }
    #[doc = "Bit 14 - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM."]
    #[inline(always)]
    pub fn MRAF(&mut self) -> MRAF_W<'_, IR_SPEC> {
        MRAF_W::new(self, 14)
    }
    #[doc = "Bit 15 - Timeout occurred"]
    #[inline(always)]
    pub fn TOO(&mut self) -> TOO_W<'_, IR_SPEC> {
        TOO_W::new(self, 15)
    }
    #[doc = "Bit 16 - Error logging overflow"]
    #[inline(always)]
    pub fn ELO(&mut self) -> ELO_W<'_, IR_SPEC> {
        ELO_W::new(self, 16)
    }
    #[doc = "Bit 17 - Error passive"]
    #[inline(always)]
    pub fn EP(&mut self) -> EP_W<'_, IR_SPEC> {
        EP_W::new(self, 17)
    }
    #[doc = "Bit 18 - Warning status"]
    #[inline(always)]
    pub fn EW(&mut self) -> EW_W<'_, IR_SPEC> {
        EW_W::new(self, 18)
    }
    #[doc = "Bit 19 - Bus_Off status"]
    #[inline(always)]
    pub fn BO(&mut self) -> BO_W<'_, IR_SPEC> {
        BO_W::new(self, 19)
    }
    #[doc = "Bit 20 - Watchdog interrupt"]
    #[inline(always)]
    pub fn WDI(&mut self) -> WDI_W<'_, IR_SPEC> {
        WDI_W::new(self, 20)
    }
    #[doc = "Bit 21 - Protocol error in arbitration phase (nominal bit time is used)"]
    #[inline(always)]
    pub fn PEA(&mut self) -> PEA_W<'_, IR_SPEC> {
        PEA_W::new(self, 21)
    }
    #[doc = "Bit 22 - Protocol error in data phase (data bit time is used)"]
    #[inline(always)]
    pub fn PED(&mut self) -> PED_W<'_, IR_SPEC> {
        PED_W::new(self, 22)
    }
    #[doc = "Bit 23 - Access to reserved address"]
    #[inline(always)]
    pub fn ARA(&mut self) -> ARA_W<'_, IR_SPEC> {
        ARA_W::new(self, 23)
    }
}
#[doc = "FDCAN interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IR_SPEC;
impl crate::RegisterSpec for IR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir::R`](R) reader structure"]
impl crate::Readable for IR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ir::W`](W) writer structure"]
impl crate::Writable for IR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IR_SPEC {}
