#[doc = "Register `NSCR` reader"]
pub type R = crate::R<NSCR_SPEC>;
#[doc = "Register `NSCR` writer"]
pub type W = crate::W<NSCR_SPEC>;
#[doc = "configuration lock bit This bit locks the FLASH_NSCR register. The correct write sequence to FLASH_NSKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_A {
    #[doc = "0: FLASH_NSCR register unlocked"]
    B_0x0 = 0,
    #[doc = "1: FLASH_NSCR register locked"]
    B_0x1 = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - configuration lock bit This bit locks the FLASH_NSCR register. The correct write sequence to FLASH_NSKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change."]
pub type LOCK_R = crate::BitReader<LOCK_A>;
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::B_0x0,
            true => LOCK_A::B_0x1,
        }
    }
    #[doc = "FLASH_NSCR register unlocked"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LOCK_A::B_0x0
    }
    #[doc = "FLASH_NSCR register locked"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LOCK_A::B_0x1
    }
}
#[doc = "Field `LOCK` writer - configuration lock bit This bit locks the FLASH_NSCR register. The correct write sequence to FLASH_NSKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change."]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG, LOCK_A>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FLASH_NSCR register unlocked"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_A::B_0x0)
    }
    #[doc = "FLASH_NSCR register locked"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_A::B_0x1)
    }
}
#[doc = "programming control bit PG can be programmed only when LOCK is cleared to 0. PG allows programming in Bank1 and Bank2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PG_A {
    #[doc = "0: programming disabled"]
    B_0x0 = 0,
    #[doc = "1: programming enabled"]
    B_0x1 = 1,
}
impl From<PG_A> for bool {
    #[inline(always)]
    fn from(variant: PG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PG` reader - programming control bit PG can be programmed only when LOCK is cleared to 0. PG allows programming in Bank1 and Bank2."]
pub type PG_R = crate::BitReader<PG_A>;
impl PG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PG_A {
        match self.bits {
            false => PG_A::B_0x0,
            true => PG_A::B_0x1,
        }
    }
    #[doc = "programming disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PG_A::B_0x0
    }
    #[doc = "programming enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PG_A::B_0x1
    }
}
#[doc = "Field `PG` writer - programming control bit PG can be programmed only when LOCK is cleared to 0. PG allows programming in Bank1 and Bank2."]
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG, PG_A>;
impl<'a, REG> PG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "programming disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PG_A::B_0x0)
    }
    #[doc = "programming enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PG_A::B_0x1)
    }
}
#[doc = "sector erase request Setting SER bit to 1 requests a sector erase. SER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SER_A {
    #[doc = "0: sector erase not requested"]
    B_0x0 = 0,
    #[doc = "1: sector erase requested"]
    B_0x1 = 1,
}
impl From<SER_A> for bool {
    #[inline(always)]
    fn from(variant: SER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SER` reader - sector erase request Setting SER bit to 1 requests a sector erase. SER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised."]
pub type SER_R = crate::BitReader<SER_A>;
impl SER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SER_A {
        match self.bits {
            false => SER_A::B_0x0,
            true => SER_A::B_0x1,
        }
    }
    #[doc = "sector erase not requested"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SER_A::B_0x0
    }
    #[doc = "sector erase requested"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SER_A::B_0x1
    }
}
#[doc = "Field `SER` writer - sector erase request Setting SER bit to 1 requests a sector erase. SER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised."]
pub type SER_W<'a, REG> = crate::BitWriter<'a, REG, SER_A>;
impl<'a, REG> SER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "sector erase not requested"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SER_A::B_0x0)
    }
    #[doc = "sector erase requested"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SER_A::B_0x1)
    }
}
#[doc = "erase request Setting BER bit to 1 requests a bank erase operation (user Flash memory only). BER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised. Note: Write protection error is triggered when a bank erase is required and some sectors are protected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BER_A {
    #[doc = "0: bank erase not requested"]
    B_0x0 = 0,
    #[doc = "1: bank erase requested"]
    B_0x1 = 1,
}
impl From<BER_A> for bool {
    #[inline(always)]
    fn from(variant: BER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BER` reader - erase request Setting BER bit to 1 requests a bank erase operation (user Flash memory only). BER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised. Note: Write protection error is triggered when a bank erase is required and some sectors are protected."]
pub type BER_R = crate::BitReader<BER_A>;
impl BER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BER_A {
        match self.bits {
            false => BER_A::B_0x0,
            true => BER_A::B_0x1,
        }
    }
    #[doc = "bank erase not requested"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BER_A::B_0x0
    }
    #[doc = "bank erase requested"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BER_A::B_0x1
    }
}
#[doc = "Field `BER` writer - erase request Setting BER bit to 1 requests a bank erase operation (user Flash memory only). BER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised. Note: Write protection error is triggered when a bank erase is required and some sectors are protected."]
pub type BER_W<'a, REG> = crate::BitWriter<'a, REG, BER_A>;
impl<'a, REG> BER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bank erase not requested"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BER_A::B_0x0)
    }
    #[doc = "bank erase requested"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BER_A::B_0x1)
    }
}
#[doc = "Field `FW` reader - write forcing control bit FW forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW can be programmed only when LOCK is cleared to 0. The embedded Flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it leads to permanent ECC error. Write forcing is effective only if the write buffer is not empty (in particular, FW does not start several write operations when the force-write operations are performed consecutively). Since there is just one write buffer, FW can force a write in bank1 or bank2."]
pub type FW_R = crate::BitReader;
#[doc = "Field `FW` writer - write forcing control bit FW forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW can be programmed only when LOCK is cleared to 0. The embedded Flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it leads to permanent ECC error. Write forcing is effective only if the write buffer is not empty (in particular, FW does not start several write operations when the force-write operations are performed consecutively). Since there is just one write buffer, FW can force a write in bank1 or bank2."]
pub type FW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRT` reader - erase start control bit STRT bit is used to start a sector erase or a bank erase operation. STRT can be programmed only when LOCK is cleared to 0. STRT is reset at the end of the operation or when an error occurs. It cannot be reseted by software."]
pub type STRT_R = crate::BitReader;
#[doc = "Field `STRT` writer - erase start control bit STRT bit is used to start a sector erase or a bank erase operation. STRT can be programmed only when LOCK is cleared to 0. STRT is reset at the end of the operation or when an error occurs. It cannot be reseted by software."]
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SNB can be programmed only when LOCK is cleared to 0. ...\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SNB_A {
    #[doc = "0: Sector 0 selected"]
    B_0x0 = 0,
    #[doc = "1: Sector 1 selected"]
    B_0x1 = 1,
    #[doc = "7: Sector 7 selected"]
    B_0x7 = 7,
}
impl From<SNB_A> for u8 {
    #[inline(always)]
    fn from(variant: SNB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SNB_A {
    type Ux = u8;
}
impl crate::IsEnum for SNB_A {}
#[doc = "Field `SNB` reader - sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SNB can be programmed only when LOCK is cleared to 0. ..."]
pub type SNB_R = crate::FieldReader<SNB_A>;
impl SNB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SNB_A> {
        match self.bits {
            0 => Some(SNB_A::B_0x0),
            1 => Some(SNB_A::B_0x1),
            7 => Some(SNB_A::B_0x7),
            _ => None,
        }
    }
    #[doc = "Sector 0 selected"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SNB_A::B_0x0
    }
    #[doc = "Sector 1 selected"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SNB_A::B_0x1
    }
    #[doc = "Sector 7 selected"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SNB_A::B_0x7
    }
}
#[doc = "Field `SNB` writer - sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SNB can be programmed only when LOCK is cleared to 0. ..."]
pub type SNB_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SNB_A>;
impl<'a, REG> SNB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sector 0 selected"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SNB_A::B_0x0)
    }
    #[doc = "Sector 1 selected"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SNB_A::B_0x1)
    }
    #[doc = "Sector 7 selected"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SNB_A::B_0x7)
    }
}
#[doc = "Mass erase request Setting MER bit to 1 requests a mass erase operation (user Flash memory only). MER can be programmed only when LOCK is cleared to 0. If BER or SER are both set, a PGSERR is raised. Error is triggered when a mass erase is required and some sectors are protected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MER_A {
    #[doc = "0: mass erase not requested"]
    B_0x0 = 0,
    #[doc = "1: mass erase requested"]
    B_0x1 = 1,
}
impl From<MER_A> for bool {
    #[inline(always)]
    fn from(variant: MER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MER` reader - Mass erase request Setting MER bit to 1 requests a mass erase operation (user Flash memory only). MER can be programmed only when LOCK is cleared to 0. If BER or SER are both set, a PGSERR is raised. Error is triggered when a mass erase is required and some sectors are protected."]
pub type MER_R = crate::BitReader<MER_A>;
impl MER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MER_A {
        match self.bits {
            false => MER_A::B_0x0,
            true => MER_A::B_0x1,
        }
    }
    #[doc = "mass erase not requested"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MER_A::B_0x0
    }
    #[doc = "mass erase requested"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MER_A::B_0x1
    }
}
#[doc = "Field `MER` writer - Mass erase request Setting MER bit to 1 requests a mass erase operation (user Flash memory only). MER can be programmed only when LOCK is cleared to 0. If BER or SER are both set, a PGSERR is raised. Error is triggered when a mass erase is required and some sectors are protected."]
pub type MER_W<'a, REG> = crate::BitWriter<'a, REG, MER_A>;
impl<'a, REG> MER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "mass erase not requested"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MER_A::B_0x0)
    }
    #[doc = "mass erase requested"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MER_A::B_0x1)
    }
}
#[doc = "end of operation interrupt control bit Setting EOPIE bit to 1 enables the generation of an interrupt at the end of a program or erase operation. EOPIE can be programmed only when LOCK is cleared to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPIE_A {
    #[doc = "0: no interrupt generated at the end of operation."]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled when at the end of operation"]
    B_0x1 = 1,
}
impl From<EOPIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOPIE` reader - end of operation interrupt control bit Setting EOPIE bit to 1 enables the generation of an interrupt at the end of a program or erase operation. EOPIE can be programmed only when LOCK is cleared to 0."]
pub type EOPIE_R = crate::BitReader<EOPIE_A>;
impl EOPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOPIE_A {
        match self.bits {
            false => EOPIE_A::B_0x0,
            true => EOPIE_A::B_0x1,
        }
    }
    #[doc = "no interrupt generated at the end of operation."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EOPIE_A::B_0x0
    }
    #[doc = "interrupt enabled when at the end of operation"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EOPIE_A::B_0x1
    }
}
#[doc = "Field `EOPIE` writer - end of operation interrupt control bit Setting EOPIE bit to 1 enables the generation of an interrupt at the end of a program or erase operation. EOPIE can be programmed only when LOCK is cleared to 0."]
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG, EOPIE_A>;
impl<'a, REG> EOPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no interrupt generated at the end of operation."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOPIE_A::B_0x0)
    }
    #[doc = "interrupt enabled when at the end of operation"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOPIE_A::B_0x1)
    }
}
#[doc = "write protection error interrupt enable bit When WRPERRIE bit is set to 1, an interrupt is generated when a protection error occurs during a program operation. WRPERRIE can be programmed only when LOCK is cleared to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPERRIE_A {
    #[doc = "0: no interrupt generated when a protection error occurs"]
    B_0x0 = 0,
    #[doc = "1: interrupt generated when a protection error occurs"]
    B_0x1 = 1,
}
impl From<WRPERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: WRPERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRPERRIE` reader - write protection error interrupt enable bit When WRPERRIE bit is set to 1, an interrupt is generated when a protection error occurs during a program operation. WRPERRIE can be programmed only when LOCK is cleared to 0."]
pub type WRPERRIE_R = crate::BitReader<WRPERRIE_A>;
impl WRPERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WRPERRIE_A {
        match self.bits {
            false => WRPERRIE_A::B_0x0,
            true => WRPERRIE_A::B_0x1,
        }
    }
    #[doc = "no interrupt generated when a protection error occurs"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WRPERRIE_A::B_0x0
    }
    #[doc = "interrupt generated when a protection error occurs"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WRPERRIE_A::B_0x1
    }
}
#[doc = "Field `WRPERRIE` writer - write protection error interrupt enable bit When WRPERRIE bit is set to 1, an interrupt is generated when a protection error occurs during a program operation. WRPERRIE can be programmed only when LOCK is cleared to 0."]
pub type WRPERRIE_W<'a, REG> = crate::BitWriter<'a, REG, WRPERRIE_A>;
impl<'a, REG> WRPERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no interrupt generated when a protection error occurs"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WRPERRIE_A::B_0x0)
    }
    #[doc = "interrupt generated when a protection error occurs"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WRPERRIE_A::B_0x1)
    }
}
#[doc = "programming sequence error interrupt enable bit When PGSERRIE bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation. PGSERRIE can be programmed only when LOCK is cleared to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGSERRIE_A {
    #[doc = "0: no interrupt generated when a sequence error occurs"]
    B_0x0 = 0,
    #[doc = "1: interrupt generated when sequence error occurs"]
    B_0x1 = 1,
}
impl From<PGSERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: PGSERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGSERRIE` reader - programming sequence error interrupt enable bit When PGSERRIE bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation. PGSERRIE can be programmed only when LOCK is cleared to 0."]
pub type PGSERRIE_R = crate::BitReader<PGSERRIE_A>;
impl PGSERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PGSERRIE_A {
        match self.bits {
            false => PGSERRIE_A::B_0x0,
            true => PGSERRIE_A::B_0x1,
        }
    }
    #[doc = "no interrupt generated when a sequence error occurs"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PGSERRIE_A::B_0x0
    }
    #[doc = "interrupt generated when sequence error occurs"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PGSERRIE_A::B_0x1
    }
}
#[doc = "Field `PGSERRIE` writer - programming sequence error interrupt enable bit When PGSERRIE bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation. PGSERRIE can be programmed only when LOCK is cleared to 0."]
pub type PGSERRIE_W<'a, REG> = crate::BitWriter<'a, REG, PGSERRIE_A>;
impl<'a, REG> PGSERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no interrupt generated when a sequence error occurs"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PGSERRIE_A::B_0x0)
    }
    #[doc = "interrupt generated when sequence error occurs"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PGSERRIE_A::B_0x1)
    }
}
#[doc = "strobe error interrupt enable bit When STRBERRIE bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation. STRBERRIE can be programmed only when LOCK is cleared to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRBERRIE_A {
    #[doc = "0: no interrupt generated when a strobe error occurs"]
    B_0x0 = 0,
    #[doc = "1: interrupt generated when strobe error occurs."]
    B_0x1 = 1,
}
impl From<STRBERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: STRBERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRBERRIE` reader - strobe error interrupt enable bit When STRBERRIE bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation. STRBERRIE can be programmed only when LOCK is cleared to 0."]
pub type STRBERRIE_R = crate::BitReader<STRBERRIE_A>;
impl STRBERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STRBERRIE_A {
        match self.bits {
            false => STRBERRIE_A::B_0x0,
            true => STRBERRIE_A::B_0x1,
        }
    }
    #[doc = "no interrupt generated when a strobe error occurs"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == STRBERRIE_A::B_0x0
    }
    #[doc = "interrupt generated when strobe error occurs."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == STRBERRIE_A::B_0x1
    }
}
#[doc = "Field `STRBERRIE` writer - strobe error interrupt enable bit When STRBERRIE bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation. STRBERRIE can be programmed only when LOCK is cleared to 0."]
pub type STRBERRIE_W<'a, REG> = crate::BitWriter<'a, REG, STRBERRIE_A>;
impl<'a, REG> STRBERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no interrupt generated when a strobe error occurs"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(STRBERRIE_A::B_0x0)
    }
    #[doc = "interrupt generated when strobe error occurs."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(STRBERRIE_A::B_0x1)
    }
}
#[doc = "inconsistency error interrupt enable bit When INCERRIE bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation. INCERRIE can be programmed only when LOCK is cleared to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INCERRIE_A {
    #[doc = "0: no interrupt generated when a inconsistency error occurs"]
    B_0x0 = 0,
    #[doc = "1: interrupt generated when a inconsistency error occurs."]
    B_0x1 = 1,
}
impl From<INCERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: INCERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INCERRIE` reader - inconsistency error interrupt enable bit When INCERRIE bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation. INCERRIE can be programmed only when LOCK is cleared to 0."]
pub type INCERRIE_R = crate::BitReader<INCERRIE_A>;
impl INCERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INCERRIE_A {
        match self.bits {
            false => INCERRIE_A::B_0x0,
            true => INCERRIE_A::B_0x1,
        }
    }
    #[doc = "no interrupt generated when a inconsistency error occurs"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == INCERRIE_A::B_0x0
    }
    #[doc = "interrupt generated when a inconsistency error occurs."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == INCERRIE_A::B_0x1
    }
}
#[doc = "Field `INCERRIE` writer - inconsistency error interrupt enable bit When INCERRIE bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation. INCERRIE can be programmed only when LOCK is cleared to 0."]
pub type INCERRIE_W<'a, REG> = crate::BitWriter<'a, REG, INCERRIE_A>;
impl<'a, REG> INCERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no interrupt generated when a inconsistency error occurs"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(INCERRIE_A::B_0x0)
    }
    #[doc = "interrupt generated when a inconsistency error occurs."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(INCERRIE_A::B_0x1)
    }
}
#[doc = "Option byte change error interrupt enable bit OPTCHANGEERRIE bit controls if an interrupt has to be generated when an error occurs during an option byte change. This bit can be programmed only when LOCK bit is cleared to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTCHANGEERRIE_A {
    #[doc = "0: no interrupt is generated when an error occurs during an option byte change"]
    B_0x0 = 0,
    #[doc = "1: an interrupt is generated when and error occurs during an option byte change."]
    B_0x1 = 1,
}
impl From<OPTCHANGEERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: OPTCHANGEERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPTCHANGEERRIE` reader - Option byte change error interrupt enable bit OPTCHANGEERRIE bit controls if an interrupt has to be generated when an error occurs during an option byte change. This bit can be programmed only when LOCK bit is cleared to 0."]
pub type OPTCHANGEERRIE_R = crate::BitReader<OPTCHANGEERRIE_A>;
impl OPTCHANGEERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPTCHANGEERRIE_A {
        match self.bits {
            false => OPTCHANGEERRIE_A::B_0x0,
            true => OPTCHANGEERRIE_A::B_0x1,
        }
    }
    #[doc = "no interrupt is generated when an error occurs during an option byte change"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OPTCHANGEERRIE_A::B_0x0
    }
    #[doc = "an interrupt is generated when and error occurs during an option byte change."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OPTCHANGEERRIE_A::B_0x1
    }
}
#[doc = "Field `OPTCHANGEERRIE` writer - Option byte change error interrupt enable bit OPTCHANGEERRIE bit controls if an interrupt has to be generated when an error occurs during an option byte change. This bit can be programmed only when LOCK bit is cleared to 0."]
pub type OPTCHANGEERRIE_W<'a, REG> = crate::BitWriter<'a, REG, OPTCHANGEERRIE_A>;
impl<'a, REG> OPTCHANGEERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no interrupt is generated when an error occurs during an option byte change"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OPTCHANGEERRIE_A::B_0x0)
    }
    #[doc = "an interrupt is generated when and error occurs during an option byte change."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OPTCHANGEERRIE_A::B_0x1)
    }
}
#[doc = "Bank selector bit BKSEL can only be programmed when LOCK is cleared to 0. The bit selects physical bank, SWAP_BANK setting is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKSEL_A {
    #[doc = "0: Bank1 is selected for Bank erase / sector erase / interrupt enable"]
    B_0x0 = 0,
    #[doc = "1: Bank2 is selected for BER / SER"]
    B_0x1 = 1,
}
impl From<BKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: BKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKSEL` reader - Bank selector bit BKSEL can only be programmed when LOCK is cleared to 0. The bit selects physical bank, SWAP_BANK setting is ignored."]
pub type BKSEL_R = crate::BitReader<BKSEL_A>;
impl BKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKSEL_A {
        match self.bits {
            false => BKSEL_A::B_0x0,
            true => BKSEL_A::B_0x1,
        }
    }
    #[doc = "Bank1 is selected for Bank erase / sector erase / interrupt enable"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKSEL_A::B_0x0
    }
    #[doc = "Bank2 is selected for BER / SER"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKSEL_A::B_0x1
    }
}
#[doc = "Field `BKSEL` writer - Bank selector bit BKSEL can only be programmed when LOCK is cleared to 0. The bit selects physical bank, SWAP_BANK setting is ignored."]
pub type BKSEL_W<'a, REG> = crate::BitWriter<'a, REG, BKSEL_A>;
impl<'a, REG> BKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bank1 is selected for Bank erase / sector erase / interrupt enable"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKSEL_A::B_0x0)
    }
    #[doc = "Bank2 is selected for BER / SER"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKSEL_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - configuration lock bit This bit locks the FLASH_NSCR register. The correct write sequence to FLASH_NSKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change."]
    #[inline(always)]
    pub fn LOCK(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - programming control bit PG can be programmed only when LOCK is cleared to 0. PG allows programming in Bank1 and Bank2."]
    #[inline(always)]
    pub fn PG(&self) -> PG_R {
        PG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - sector erase request Setting SER bit to 1 requests a sector erase. SER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised."]
    #[inline(always)]
    pub fn SER(&self) -> SER_R {
        SER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - erase request Setting BER bit to 1 requests a bank erase operation (user Flash memory only). BER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised. Note: Write protection error is triggered when a bank erase is required and some sectors are protected."]
    #[inline(always)]
    pub fn BER(&self) -> BER_R {
        BER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - write forcing control bit FW forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW can be programmed only when LOCK is cleared to 0. The embedded Flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it leads to permanent ECC error. Write forcing is effective only if the write buffer is not empty (in particular, FW does not start several write operations when the force-write operations are performed consecutively). Since there is just one write buffer, FW can force a write in bank1 or bank2."]
    #[inline(always)]
    pub fn FW(&self) -> FW_R {
        FW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - erase start control bit STRT bit is used to start a sector erase or a bank erase operation. STRT can be programmed only when LOCK is cleared to 0. STRT is reset at the end of the operation or when an error occurs. It cannot be reseted by software."]
    #[inline(always)]
    pub fn STRT(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SNB can be programmed only when LOCK is cleared to 0. ..."]
    #[inline(always)]
    pub fn SNB(&self) -> SNB_R {
        SNB_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 15 - Mass erase request Setting MER bit to 1 requests a mass erase operation (user Flash memory only). MER can be programmed only when LOCK is cleared to 0. If BER or SER are both set, a PGSERR is raised. Error is triggered when a mass erase is required and some sectors are protected."]
    #[inline(always)]
    pub fn MER(&self) -> MER_R {
        MER_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - end of operation interrupt control bit Setting EOPIE bit to 1 enables the generation of an interrupt at the end of a program or erase operation. EOPIE can be programmed only when LOCK is cleared to 0."]
    #[inline(always)]
    pub fn EOPIE(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - write protection error interrupt enable bit When WRPERRIE bit is set to 1, an interrupt is generated when a protection error occurs during a program operation. WRPERRIE can be programmed only when LOCK is cleared to 0."]
    #[inline(always)]
    pub fn WRPERRIE(&self) -> WRPERRIE_R {
        WRPERRIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - programming sequence error interrupt enable bit When PGSERRIE bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation. PGSERRIE can be programmed only when LOCK is cleared to 0."]
    #[inline(always)]
    pub fn PGSERRIE(&self) -> PGSERRIE_R {
        PGSERRIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - strobe error interrupt enable bit When STRBERRIE bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation. STRBERRIE can be programmed only when LOCK is cleared to 0."]
    #[inline(always)]
    pub fn STRBERRIE(&self) -> STRBERRIE_R {
        STRBERRIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - inconsistency error interrupt enable bit When INCERRIE bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation. INCERRIE can be programmed only when LOCK is cleared to 0."]
    #[inline(always)]
    pub fn INCERRIE(&self) -> INCERRIE_R {
        INCERRIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - Option byte change error interrupt enable bit OPTCHANGEERRIE bit controls if an interrupt has to be generated when an error occurs during an option byte change. This bit can be programmed only when LOCK bit is cleared to 0."]
    #[inline(always)]
    pub fn OPTCHANGEERRIE(&self) -> OPTCHANGEERRIE_R {
        OPTCHANGEERRIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - Bank selector bit BKSEL can only be programmed when LOCK is cleared to 0. The bit selects physical bank, SWAP_BANK setting is ignored."]
    #[inline(always)]
    pub fn BKSEL(&self) -> BKSEL_R {
        BKSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - configuration lock bit This bit locks the FLASH_NSCR register. The correct write sequence to FLASH_NSKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change."]
    #[inline(always)]
    pub fn LOCK(&mut self) -> LOCK_W<'_, NSCR_SPEC> {
        LOCK_W::new(self, 0)
    }
    #[doc = "Bit 1 - programming control bit PG can be programmed only when LOCK is cleared to 0. PG allows programming in Bank1 and Bank2."]
    #[inline(always)]
    pub fn PG(&mut self) -> PG_W<'_, NSCR_SPEC> {
        PG_W::new(self, 1)
    }
    #[doc = "Bit 2 - sector erase request Setting SER bit to 1 requests a sector erase. SER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised."]
    #[inline(always)]
    pub fn SER(&mut self) -> SER_W<'_, NSCR_SPEC> {
        SER_W::new(self, 2)
    }
    #[doc = "Bit 3 - erase request Setting BER bit to 1 requests a bank erase operation (user Flash memory only). BER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised. Note: Write protection error is triggered when a bank erase is required and some sectors are protected."]
    #[inline(always)]
    pub fn BER(&mut self) -> BER_W<'_, NSCR_SPEC> {
        BER_W::new(self, 3)
    }
    #[doc = "Bit 4 - write forcing control bit FW forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW can be programmed only when LOCK is cleared to 0. The embedded Flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it leads to permanent ECC error. Write forcing is effective only if the write buffer is not empty (in particular, FW does not start several write operations when the force-write operations are performed consecutively). Since there is just one write buffer, FW can force a write in bank1 or bank2."]
    #[inline(always)]
    pub fn FW(&mut self) -> FW_W<'_, NSCR_SPEC> {
        FW_W::new(self, 4)
    }
    #[doc = "Bit 5 - erase start control bit STRT bit is used to start a sector erase or a bank erase operation. STRT can be programmed only when LOCK is cleared to 0. STRT is reset at the end of the operation or when an error occurs. It cannot be reseted by software."]
    #[inline(always)]
    pub fn STRT(&mut self) -> STRT_W<'_, NSCR_SPEC> {
        STRT_W::new(self, 5)
    }
    #[doc = "Bits 6:8 - sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SNB can be programmed only when LOCK is cleared to 0. ..."]
    #[inline(always)]
    pub fn SNB(&mut self) -> SNB_W<'_, NSCR_SPEC> {
        SNB_W::new(self, 6)
    }
    #[doc = "Bit 15 - Mass erase request Setting MER bit to 1 requests a mass erase operation (user Flash memory only). MER can be programmed only when LOCK is cleared to 0. If BER or SER are both set, a PGSERR is raised. Error is triggered when a mass erase is required and some sectors are protected."]
    #[inline(always)]
    pub fn MER(&mut self) -> MER_W<'_, NSCR_SPEC> {
        MER_W::new(self, 15)
    }
    #[doc = "Bit 16 - end of operation interrupt control bit Setting EOPIE bit to 1 enables the generation of an interrupt at the end of a program or erase operation. EOPIE can be programmed only when LOCK is cleared to 0."]
    #[inline(always)]
    pub fn EOPIE(&mut self) -> EOPIE_W<'_, NSCR_SPEC> {
        EOPIE_W::new(self, 16)
    }
    #[doc = "Bit 17 - write protection error interrupt enable bit When WRPERRIE bit is set to 1, an interrupt is generated when a protection error occurs during a program operation. WRPERRIE can be programmed only when LOCK is cleared to 0."]
    #[inline(always)]
    pub fn WRPERRIE(&mut self) -> WRPERRIE_W<'_, NSCR_SPEC> {
        WRPERRIE_W::new(self, 17)
    }
    #[doc = "Bit 18 - programming sequence error interrupt enable bit When PGSERRIE bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation. PGSERRIE can be programmed only when LOCK is cleared to 0."]
    #[inline(always)]
    pub fn PGSERRIE(&mut self) -> PGSERRIE_W<'_, NSCR_SPEC> {
        PGSERRIE_W::new(self, 18)
    }
    #[doc = "Bit 19 - strobe error interrupt enable bit When STRBERRIE bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation. STRBERRIE can be programmed only when LOCK is cleared to 0."]
    #[inline(always)]
    pub fn STRBERRIE(&mut self) -> STRBERRIE_W<'_, NSCR_SPEC> {
        STRBERRIE_W::new(self, 19)
    }
    #[doc = "Bit 20 - inconsistency error interrupt enable bit When INCERRIE bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation. INCERRIE can be programmed only when LOCK is cleared to 0."]
    #[inline(always)]
    pub fn INCERRIE(&mut self) -> INCERRIE_W<'_, NSCR_SPEC> {
        INCERRIE_W::new(self, 20)
    }
    #[doc = "Bit 23 - Option byte change error interrupt enable bit OPTCHANGEERRIE bit controls if an interrupt has to be generated when an error occurs during an option byte change. This bit can be programmed only when LOCK bit is cleared to 0."]
    #[inline(always)]
    pub fn OPTCHANGEERRIE(&mut self) -> OPTCHANGEERRIE_W<'_, NSCR_SPEC> {
        OPTCHANGEERRIE_W::new(self, 23)
    }
    #[doc = "Bit 31 - Bank selector bit BKSEL can only be programmed when LOCK is cleared to 0. The bit selects physical bank, SWAP_BANK setting is ignored."]
    #[inline(always)]
    pub fn BKSEL(&mut self) -> BKSEL_W<'_, NSCR_SPEC> {
        BKSEL_W::new(self, 31)
    }
}
#[doc = "FLASH Non Secure control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NSCR_SPEC;
impl crate::RegisterSpec for NSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nscr::R`](R) reader structure"]
impl crate::Readable for NSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nscr::W`](W) writer structure"]
impl crate::Writable for NSCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets NSCR to value 0x01"]
impl crate::Resettable for NSCR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
