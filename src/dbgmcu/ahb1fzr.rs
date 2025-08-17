#[doc = "Register `AHB1FZR` reader"]
pub type R = crate::R<AHB1FZR_SPEC>;
#[doc = "Register `AHB1FZR` writer"]
pub type W = crate::W<AHB1FZR_SPEC>;
#[doc = "GPDMA1 channel 0 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_GPDMA1_0_STOP_A {
    #[doc = "0: normal operation. GPDMA1 channel 0 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. GPDMA1 channel 0 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_GPDMA1_0_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_GPDMA1_0_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_GPDMA1_0_STOP` reader - GPDMA1 channel 0 stop in debug"]
pub type DBG_GPDMA1_0_STOP_R = crate::BitReader<DBG_GPDMA1_0_STOP_A>;
impl DBG_GPDMA1_0_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_GPDMA1_0_STOP_A {
        match self.bits {
            false => DBG_GPDMA1_0_STOP_A::B_0x0,
            true => DBG_GPDMA1_0_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. GPDMA1 channel 0 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_GPDMA1_0_STOP_A::B_0x0
    }
    #[doc = "stop in debug. GPDMA1 channel 0 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_GPDMA1_0_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_GPDMA1_0_STOP` writer - GPDMA1 channel 0 stop in debug"]
pub type DBG_GPDMA1_0_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_GPDMA1_0_STOP_A>;
impl<'a, REG> DBG_GPDMA1_0_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. GPDMA1 channel 0 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA1_0_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. GPDMA1 channel 0 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA1_0_STOP_A::B_0x1)
    }
}
#[doc = "GPDMA1 channel 1 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_GPDMA1_1_STOP_A {
    #[doc = "0: normal operation. GPDMA1 channel 1 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. GPDMA1 channel 1 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_GPDMA1_1_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_GPDMA1_1_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_GPDMA1_1_STOP` reader - GPDMA1 channel 1 stop in debug"]
pub type DBG_GPDMA1_1_STOP_R = crate::BitReader<DBG_GPDMA1_1_STOP_A>;
impl DBG_GPDMA1_1_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_GPDMA1_1_STOP_A {
        match self.bits {
            false => DBG_GPDMA1_1_STOP_A::B_0x0,
            true => DBG_GPDMA1_1_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. GPDMA1 channel 1 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_GPDMA1_1_STOP_A::B_0x0
    }
    #[doc = "stop in debug. GPDMA1 channel 1 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_GPDMA1_1_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_GPDMA1_1_STOP` writer - GPDMA1 channel 1 stop in debug"]
pub type DBG_GPDMA1_1_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_GPDMA1_1_STOP_A>;
impl<'a, REG> DBG_GPDMA1_1_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. GPDMA1 channel 1 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA1_1_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. GPDMA1 channel 1 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA1_1_STOP_A::B_0x1)
    }
}
#[doc = "GPDMA1 channel 2 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_GPDMA1_2_STOP_A {
    #[doc = "0: normal operation. GPDMA1 channel 2 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. GPDMA1 channel 2 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_GPDMA1_2_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_GPDMA1_2_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_GPDMA1_2_STOP` reader - GPDMA1 channel 2 stop in debug"]
pub type DBG_GPDMA1_2_STOP_R = crate::BitReader<DBG_GPDMA1_2_STOP_A>;
impl DBG_GPDMA1_2_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_GPDMA1_2_STOP_A {
        match self.bits {
            false => DBG_GPDMA1_2_STOP_A::B_0x0,
            true => DBG_GPDMA1_2_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. GPDMA1 channel 2 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_GPDMA1_2_STOP_A::B_0x0
    }
    #[doc = "stop in debug. GPDMA1 channel 2 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_GPDMA1_2_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_GPDMA1_2_STOP` writer - GPDMA1 channel 2 stop in debug"]
pub type DBG_GPDMA1_2_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_GPDMA1_2_STOP_A>;
impl<'a, REG> DBG_GPDMA1_2_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. GPDMA1 channel 2 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA1_2_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. GPDMA1 channel 2 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA1_2_STOP_A::B_0x1)
    }
}
#[doc = "GPDMA1 channel 3 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_GPDMA1_3_STOP_A {
    #[doc = "0: normal operation. GPDMA1 channel 3 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. GPDMA1 channel 3 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_GPDMA1_3_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_GPDMA1_3_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_GPDMA1_3_STOP` reader - GPDMA1 channel 3 stop in debug"]
pub type DBG_GPDMA1_3_STOP_R = crate::BitReader<DBG_GPDMA1_3_STOP_A>;
impl DBG_GPDMA1_3_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_GPDMA1_3_STOP_A {
        match self.bits {
            false => DBG_GPDMA1_3_STOP_A::B_0x0,
            true => DBG_GPDMA1_3_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. GPDMA1 channel 3 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_GPDMA1_3_STOP_A::B_0x0
    }
    #[doc = "stop in debug. GPDMA1 channel 3 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_GPDMA1_3_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_GPDMA1_3_STOP` writer - GPDMA1 channel 3 stop in debug"]
pub type DBG_GPDMA1_3_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_GPDMA1_3_STOP_A>;
impl<'a, REG> DBG_GPDMA1_3_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. GPDMA1 channel 3 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA1_3_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. GPDMA1 channel 3 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA1_3_STOP_A::B_0x1)
    }
}
#[doc = "GPDMA1 channel 4 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_GPDMA1_4_STOP_A {
    #[doc = "0: normal operation. GPDMA1 channel 4 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. GPDMA1 channel 4 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_GPDMA1_4_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_GPDMA1_4_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_GPDMA1_4_STOP` reader - GPDMA1 channel 4 stop in debug"]
pub type DBG_GPDMA1_4_STOP_R = crate::BitReader<DBG_GPDMA1_4_STOP_A>;
impl DBG_GPDMA1_4_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_GPDMA1_4_STOP_A {
        match self.bits {
            false => DBG_GPDMA1_4_STOP_A::B_0x0,
            true => DBG_GPDMA1_4_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. GPDMA1 channel 4 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_GPDMA1_4_STOP_A::B_0x0
    }
    #[doc = "stop in debug. GPDMA1 channel 4 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_GPDMA1_4_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_GPDMA1_4_STOP` writer - GPDMA1 channel 4 stop in debug"]
pub type DBG_GPDMA1_4_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_GPDMA1_4_STOP_A>;
impl<'a, REG> DBG_GPDMA1_4_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. GPDMA1 channel 4 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA1_4_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. GPDMA1 channel 4 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA1_4_STOP_A::B_0x1)
    }
}
#[doc = "GPDMA1 channel 5 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_GPDMA1_5_STOP_A {
    #[doc = "0: normal operation. GPDMA1 channel 5 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. GPDMA1 channel 5 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_GPDMA1_5_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_GPDMA1_5_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_GPDMA1_5_STOP` reader - GPDMA1 channel 5 stop in debug"]
pub type DBG_GPDMA1_5_STOP_R = crate::BitReader<DBG_GPDMA1_5_STOP_A>;
impl DBG_GPDMA1_5_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_GPDMA1_5_STOP_A {
        match self.bits {
            false => DBG_GPDMA1_5_STOP_A::B_0x0,
            true => DBG_GPDMA1_5_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. GPDMA1 channel 5 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_GPDMA1_5_STOP_A::B_0x0
    }
    #[doc = "stop in debug. GPDMA1 channel 5 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_GPDMA1_5_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_GPDMA1_5_STOP` writer - GPDMA1 channel 5 stop in debug"]
pub type DBG_GPDMA1_5_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_GPDMA1_5_STOP_A>;
impl<'a, REG> DBG_GPDMA1_5_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. GPDMA1 channel 5 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA1_5_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. GPDMA1 channel 5 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA1_5_STOP_A::B_0x1)
    }
}
#[doc = "GPDMA1 channel 6 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_GPDMA1_6_STOP_A {
    #[doc = "0: normal operation. GPDMA1 channel 6 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. GPDMA1 channel 6 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_GPDMA1_6_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_GPDMA1_6_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_GPDMA1_6_STOP` reader - GPDMA1 channel 6 stop in debug"]
pub type DBG_GPDMA1_6_STOP_R = crate::BitReader<DBG_GPDMA1_6_STOP_A>;
impl DBG_GPDMA1_6_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_GPDMA1_6_STOP_A {
        match self.bits {
            false => DBG_GPDMA1_6_STOP_A::B_0x0,
            true => DBG_GPDMA1_6_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. GPDMA1 channel 6 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_GPDMA1_6_STOP_A::B_0x0
    }
    #[doc = "stop in debug. GPDMA1 channel 6 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_GPDMA1_6_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_GPDMA1_6_STOP` writer - GPDMA1 channel 6 stop in debug"]
pub type DBG_GPDMA1_6_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_GPDMA1_6_STOP_A>;
impl<'a, REG> DBG_GPDMA1_6_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. GPDMA1 channel 6 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA1_6_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. GPDMA1 channel 6 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA1_6_STOP_A::B_0x1)
    }
}
#[doc = "GPDMA1 channel 7 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_GPDMA1_7_STOP_A {
    #[doc = "0: normal operation. GPDMA1 channel 7 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. GPDMA1 channel 7 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_GPDMA1_7_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_GPDMA1_7_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_GPDMA1_7_STOP` reader - GPDMA1 channel 7 stop in debug"]
pub type DBG_GPDMA1_7_STOP_R = crate::BitReader<DBG_GPDMA1_7_STOP_A>;
impl DBG_GPDMA1_7_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_GPDMA1_7_STOP_A {
        match self.bits {
            false => DBG_GPDMA1_7_STOP_A::B_0x0,
            true => DBG_GPDMA1_7_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. GPDMA1 channel 7 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_GPDMA1_7_STOP_A::B_0x0
    }
    #[doc = "stop in debug. GPDMA1 channel 7 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_GPDMA1_7_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_GPDMA1_7_STOP` writer - GPDMA1 channel 7 stop in debug"]
pub type DBG_GPDMA1_7_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_GPDMA1_7_STOP_A>;
impl<'a, REG> DBG_GPDMA1_7_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. GPDMA1 channel 7 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA1_7_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. GPDMA1 channel 7 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA1_7_STOP_A::B_0x1)
    }
}
#[doc = "GPDMA2 channel 0 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_GPDMA2_0_STOP_A {
    #[doc = "0: normal operation. GPDMA2 channel 0 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. GPDMA2 channel 0 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_GPDMA2_0_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_GPDMA2_0_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_GPDMA2_0_STOP` reader - GPDMA2 channel 0 stop in debug"]
pub type DBG_GPDMA2_0_STOP_R = crate::BitReader<DBG_GPDMA2_0_STOP_A>;
impl DBG_GPDMA2_0_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_GPDMA2_0_STOP_A {
        match self.bits {
            false => DBG_GPDMA2_0_STOP_A::B_0x0,
            true => DBG_GPDMA2_0_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. GPDMA2 channel 0 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_GPDMA2_0_STOP_A::B_0x0
    }
    #[doc = "stop in debug. GPDMA2 channel 0 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_GPDMA2_0_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_GPDMA2_0_STOP` writer - GPDMA2 channel 0 stop in debug"]
pub type DBG_GPDMA2_0_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_GPDMA2_0_STOP_A>;
impl<'a, REG> DBG_GPDMA2_0_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. GPDMA2 channel 0 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA2_0_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. GPDMA2 channel 0 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA2_0_STOP_A::B_0x1)
    }
}
#[doc = "GPDMA2 channel 1 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_GPDMA2_1_STOP_A {
    #[doc = "0: normal operation. GPDMA2 channel 1 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. GPDMA2 channel 1 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_GPDMA2_1_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_GPDMA2_1_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_GPDMA2_1_STOP` reader - GPDMA2 channel 1 stop in debug"]
pub type DBG_GPDMA2_1_STOP_R = crate::BitReader<DBG_GPDMA2_1_STOP_A>;
impl DBG_GPDMA2_1_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_GPDMA2_1_STOP_A {
        match self.bits {
            false => DBG_GPDMA2_1_STOP_A::B_0x0,
            true => DBG_GPDMA2_1_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. GPDMA2 channel 1 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_GPDMA2_1_STOP_A::B_0x0
    }
    #[doc = "stop in debug. GPDMA2 channel 1 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_GPDMA2_1_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_GPDMA2_1_STOP` writer - GPDMA2 channel 1 stop in debug"]
pub type DBG_GPDMA2_1_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_GPDMA2_1_STOP_A>;
impl<'a, REG> DBG_GPDMA2_1_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. GPDMA2 channel 1 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA2_1_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. GPDMA2 channel 1 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA2_1_STOP_A::B_0x1)
    }
}
#[doc = "GPDMA2 channel 2 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_GPDMA2_2_STOP_A {
    #[doc = "0: normal operation. GPDMA2 channel 2 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. GPDMA2 channel 2 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_GPDMA2_2_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_GPDMA2_2_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_GPDMA2_2_STOP` reader - GPDMA2 channel 2 stop in debug"]
pub type DBG_GPDMA2_2_STOP_R = crate::BitReader<DBG_GPDMA2_2_STOP_A>;
impl DBG_GPDMA2_2_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_GPDMA2_2_STOP_A {
        match self.bits {
            false => DBG_GPDMA2_2_STOP_A::B_0x0,
            true => DBG_GPDMA2_2_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. GPDMA2 channel 2 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_GPDMA2_2_STOP_A::B_0x0
    }
    #[doc = "stop in debug. GPDMA2 channel 2 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_GPDMA2_2_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_GPDMA2_2_STOP` writer - GPDMA2 channel 2 stop in debug"]
pub type DBG_GPDMA2_2_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_GPDMA2_2_STOP_A>;
impl<'a, REG> DBG_GPDMA2_2_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. GPDMA2 channel 2 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA2_2_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. GPDMA2 channel 2 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA2_2_STOP_A::B_0x1)
    }
}
#[doc = "GPDMA2 channel 3 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_GPDMA2_3_STOP_A {
    #[doc = "0: normal operation. GPDMA2 channel 3 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. GPDMA2 channel 3 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_GPDMA2_3_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_GPDMA2_3_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_GPDMA2_3_STOP` reader - GPDMA2 channel 3 stop in debug"]
pub type DBG_GPDMA2_3_STOP_R = crate::BitReader<DBG_GPDMA2_3_STOP_A>;
impl DBG_GPDMA2_3_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_GPDMA2_3_STOP_A {
        match self.bits {
            false => DBG_GPDMA2_3_STOP_A::B_0x0,
            true => DBG_GPDMA2_3_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. GPDMA2 channel 3 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_GPDMA2_3_STOP_A::B_0x0
    }
    #[doc = "stop in debug. GPDMA2 channel 3 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_GPDMA2_3_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_GPDMA2_3_STOP` writer - GPDMA2 channel 3 stop in debug"]
pub type DBG_GPDMA2_3_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_GPDMA2_3_STOP_A>;
impl<'a, REG> DBG_GPDMA2_3_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. GPDMA2 channel 3 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA2_3_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. GPDMA2 channel 3 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA2_3_STOP_A::B_0x1)
    }
}
#[doc = "GPDMA2 channel 4 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_GPDMA2_4_STOP_A {
    #[doc = "0: normal operation. GPDMA2 channel 4 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. GPDMA2 channel 4 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_GPDMA2_4_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_GPDMA2_4_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_GPDMA2_4_STOP` reader - GPDMA2 channel 4 stop in debug"]
pub type DBG_GPDMA2_4_STOP_R = crate::BitReader<DBG_GPDMA2_4_STOP_A>;
impl DBG_GPDMA2_4_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_GPDMA2_4_STOP_A {
        match self.bits {
            false => DBG_GPDMA2_4_STOP_A::B_0x0,
            true => DBG_GPDMA2_4_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. GPDMA2 channel 4 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_GPDMA2_4_STOP_A::B_0x0
    }
    #[doc = "stop in debug. GPDMA2 channel 4 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_GPDMA2_4_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_GPDMA2_4_STOP` writer - GPDMA2 channel 4 stop in debug"]
pub type DBG_GPDMA2_4_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_GPDMA2_4_STOP_A>;
impl<'a, REG> DBG_GPDMA2_4_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. GPDMA2 channel 4 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA2_4_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. GPDMA2 channel 4 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA2_4_STOP_A::B_0x1)
    }
}
#[doc = "GPDMA2 channel 5 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_GPDMA2_5_STOP_A {
    #[doc = "0: normal operation. GPDMA2 channel 5 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. GPDMA2 channel 5 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_GPDMA2_5_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_GPDMA2_5_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_GPDMA2_5_STOP` reader - GPDMA2 channel 5 stop in debug"]
pub type DBG_GPDMA2_5_STOP_R = crate::BitReader<DBG_GPDMA2_5_STOP_A>;
impl DBG_GPDMA2_5_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_GPDMA2_5_STOP_A {
        match self.bits {
            false => DBG_GPDMA2_5_STOP_A::B_0x0,
            true => DBG_GPDMA2_5_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. GPDMA2 channel 5 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_GPDMA2_5_STOP_A::B_0x0
    }
    #[doc = "stop in debug. GPDMA2 channel 5 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_GPDMA2_5_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_GPDMA2_5_STOP` writer - GPDMA2 channel 5 stop in debug"]
pub type DBG_GPDMA2_5_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_GPDMA2_5_STOP_A>;
impl<'a, REG> DBG_GPDMA2_5_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. GPDMA2 channel 5 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA2_5_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. GPDMA2 channel 5 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA2_5_STOP_A::B_0x1)
    }
}
#[doc = "GPDMA2 channel 6 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_GPDMA2_6_STOP_A {
    #[doc = "0: normal operation. GPDMA2 channel 6 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. GPDMA2 channel 6 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_GPDMA2_6_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_GPDMA2_6_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_GPDMA2_6_STOP` reader - GPDMA2 channel 6 stop in debug"]
pub type DBG_GPDMA2_6_STOP_R = crate::BitReader<DBG_GPDMA2_6_STOP_A>;
impl DBG_GPDMA2_6_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_GPDMA2_6_STOP_A {
        match self.bits {
            false => DBG_GPDMA2_6_STOP_A::B_0x0,
            true => DBG_GPDMA2_6_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. GPDMA2 channel 6 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_GPDMA2_6_STOP_A::B_0x0
    }
    #[doc = "stop in debug. GPDMA2 channel 6 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_GPDMA2_6_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_GPDMA2_6_STOP` writer - GPDMA2 channel 6 stop in debug"]
pub type DBG_GPDMA2_6_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_GPDMA2_6_STOP_A>;
impl<'a, REG> DBG_GPDMA2_6_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. GPDMA2 channel 6 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA2_6_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. GPDMA2 channel 6 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA2_6_STOP_A::B_0x1)
    }
}
#[doc = "GPDMA2 channel 7 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_GPDMA2_7_STOP_A {
    #[doc = "0: normal operation. GPDMA2 channel 7 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. GPDMA2 channel 7 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_GPDMA2_7_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_GPDMA2_7_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_GPDMA2_7_STOP` reader - GPDMA2 channel 7 stop in debug"]
pub type DBG_GPDMA2_7_STOP_R = crate::BitReader<DBG_GPDMA2_7_STOP_A>;
impl DBG_GPDMA2_7_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_GPDMA2_7_STOP_A {
        match self.bits {
            false => DBG_GPDMA2_7_STOP_A::B_0x0,
            true => DBG_GPDMA2_7_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. GPDMA2 channel 7 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_GPDMA2_7_STOP_A::B_0x0
    }
    #[doc = "stop in debug. GPDMA2 channel 7 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_GPDMA2_7_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_GPDMA2_7_STOP` writer - GPDMA2 channel 7 stop in debug"]
pub type DBG_GPDMA2_7_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_GPDMA2_7_STOP_A>;
impl<'a, REG> DBG_GPDMA2_7_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. GPDMA2 channel 7 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA2_7_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. GPDMA2 channel 7 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_GPDMA2_7_STOP_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - GPDMA1 channel 0 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA1_0_STOP(&self) -> DBG_GPDMA1_0_STOP_R {
        DBG_GPDMA1_0_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPDMA1 channel 1 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA1_1_STOP(&self) -> DBG_GPDMA1_1_STOP_R {
        DBG_GPDMA1_1_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPDMA1 channel 2 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA1_2_STOP(&self) -> DBG_GPDMA1_2_STOP_R {
        DBG_GPDMA1_2_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPDMA1 channel 3 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA1_3_STOP(&self) -> DBG_GPDMA1_3_STOP_R {
        DBG_GPDMA1_3_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPDMA1 channel 4 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA1_4_STOP(&self) -> DBG_GPDMA1_4_STOP_R {
        DBG_GPDMA1_4_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPDMA1 channel 5 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA1_5_STOP(&self) -> DBG_GPDMA1_5_STOP_R {
        DBG_GPDMA1_5_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPDMA1 channel 6 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA1_6_STOP(&self) -> DBG_GPDMA1_6_STOP_R {
        DBG_GPDMA1_6_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPDMA1 channel 7 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA1_7_STOP(&self) -> DBG_GPDMA1_7_STOP_R {
        DBG_GPDMA1_7_STOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - GPDMA2 channel 0 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA2_0_STOP(&self) -> DBG_GPDMA2_0_STOP_R {
        DBG_GPDMA2_0_STOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPDMA2 channel 1 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA2_1_STOP(&self) -> DBG_GPDMA2_1_STOP_R {
        DBG_GPDMA2_1_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPDMA2 channel 2 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA2_2_STOP(&self) -> DBG_GPDMA2_2_STOP_R {
        DBG_GPDMA2_2_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPDMA2 channel 3 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA2_3_STOP(&self) -> DBG_GPDMA2_3_STOP_R {
        DBG_GPDMA2_3_STOP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPDMA2 channel 4 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA2_4_STOP(&self) -> DBG_GPDMA2_4_STOP_R {
        DBG_GPDMA2_4_STOP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GPDMA2 channel 5 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA2_5_STOP(&self) -> DBG_GPDMA2_5_STOP_R {
        DBG_GPDMA2_5_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPDMA2 channel 6 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA2_6_STOP(&self) -> DBG_GPDMA2_6_STOP_R {
        DBG_GPDMA2_6_STOP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPDMA2 channel 7 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA2_7_STOP(&self) -> DBG_GPDMA2_7_STOP_R {
        DBG_GPDMA2_7_STOP_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPDMA1 channel 0 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA1_0_STOP(&mut self) -> DBG_GPDMA1_0_STOP_W<'_, AHB1FZR_SPEC> {
        DBG_GPDMA1_0_STOP_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPDMA1 channel 1 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA1_1_STOP(&mut self) -> DBG_GPDMA1_1_STOP_W<'_, AHB1FZR_SPEC> {
        DBG_GPDMA1_1_STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - GPDMA1 channel 2 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA1_2_STOP(&mut self) -> DBG_GPDMA1_2_STOP_W<'_, AHB1FZR_SPEC> {
        DBG_GPDMA1_2_STOP_W::new(self, 2)
    }
    #[doc = "Bit 3 - GPDMA1 channel 3 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA1_3_STOP(&mut self) -> DBG_GPDMA1_3_STOP_W<'_, AHB1FZR_SPEC> {
        DBG_GPDMA1_3_STOP_W::new(self, 3)
    }
    #[doc = "Bit 4 - GPDMA1 channel 4 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA1_4_STOP(&mut self) -> DBG_GPDMA1_4_STOP_W<'_, AHB1FZR_SPEC> {
        DBG_GPDMA1_4_STOP_W::new(self, 4)
    }
    #[doc = "Bit 5 - GPDMA1 channel 5 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA1_5_STOP(&mut self) -> DBG_GPDMA1_5_STOP_W<'_, AHB1FZR_SPEC> {
        DBG_GPDMA1_5_STOP_W::new(self, 5)
    }
    #[doc = "Bit 6 - GPDMA1 channel 6 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA1_6_STOP(&mut self) -> DBG_GPDMA1_6_STOP_W<'_, AHB1FZR_SPEC> {
        DBG_GPDMA1_6_STOP_W::new(self, 6)
    }
    #[doc = "Bit 7 - GPDMA1 channel 7 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA1_7_STOP(&mut self) -> DBG_GPDMA1_7_STOP_W<'_, AHB1FZR_SPEC> {
        DBG_GPDMA1_7_STOP_W::new(self, 7)
    }
    #[doc = "Bit 16 - GPDMA2 channel 0 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA2_0_STOP(&mut self) -> DBG_GPDMA2_0_STOP_W<'_, AHB1FZR_SPEC> {
        DBG_GPDMA2_0_STOP_W::new(self, 16)
    }
    #[doc = "Bit 17 - GPDMA2 channel 1 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA2_1_STOP(&mut self) -> DBG_GPDMA2_1_STOP_W<'_, AHB1FZR_SPEC> {
        DBG_GPDMA2_1_STOP_W::new(self, 17)
    }
    #[doc = "Bit 18 - GPDMA2 channel 2 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA2_2_STOP(&mut self) -> DBG_GPDMA2_2_STOP_W<'_, AHB1FZR_SPEC> {
        DBG_GPDMA2_2_STOP_W::new(self, 18)
    }
    #[doc = "Bit 19 - GPDMA2 channel 3 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA2_3_STOP(&mut self) -> DBG_GPDMA2_3_STOP_W<'_, AHB1FZR_SPEC> {
        DBG_GPDMA2_3_STOP_W::new(self, 19)
    }
    #[doc = "Bit 20 - GPDMA2 channel 4 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA2_4_STOP(&mut self) -> DBG_GPDMA2_4_STOP_W<'_, AHB1FZR_SPEC> {
        DBG_GPDMA2_4_STOP_W::new(self, 20)
    }
    #[doc = "Bit 21 - GPDMA2 channel 5 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA2_5_STOP(&mut self) -> DBG_GPDMA2_5_STOP_W<'_, AHB1FZR_SPEC> {
        DBG_GPDMA2_5_STOP_W::new(self, 21)
    }
    #[doc = "Bit 22 - GPDMA2 channel 6 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA2_6_STOP(&mut self) -> DBG_GPDMA2_6_STOP_W<'_, AHB1FZR_SPEC> {
        DBG_GPDMA2_6_STOP_W::new(self, 22)
    }
    #[doc = "Bit 23 - GPDMA2 channel 7 stop in debug"]
    #[inline(always)]
    pub fn DBG_GPDMA2_7_STOP(&mut self) -> DBG_GPDMA2_7_STOP_W<'_, AHB1FZR_SPEC> {
        DBG_GPDMA2_7_STOP_W::new(self, 23)
    }
}
#[doc = "DBGMCU AHB1 peripheral freeze register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1fzr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1fzr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB1FZR_SPEC;
impl crate::RegisterSpec for AHB1FZR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1fzr::R`](R) reader structure"]
impl crate::Readable for AHB1FZR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb1fzr::W`](W) writer structure"]
impl crate::Writable for AHB1FZR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets AHB1FZR to value 0"]
impl crate::Resettable for AHB1FZR_SPEC {}
