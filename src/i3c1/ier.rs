#[doc = "Register `IER` reader"]
pub type R = crate::R<IER_SPEC>;
#[doc = "C-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFNFIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<CFNFIE_A> for bool {
    #[inline(always)]
    fn from(variant: CFNFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFNFIE` reader - C-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)"]
pub type CFNFIE_R = crate::BitReader<CFNFIE_A>;
impl CFNFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CFNFIE_A {
        match self.bits {
            false => CFNFIE_A::B_0x0,
            true => CFNFIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CFNFIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CFNFIE_A::B_0x1
    }
}
#[doc = "S-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFNEIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<SFNEIE_A> for bool {
    #[inline(always)]
    fn from(variant: SFNEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFNEIE` reader - S-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)"]
pub type SFNEIE_R = crate::BitReader<SFNEIE_A>;
impl SFNEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SFNEIE_A {
        match self.bits {
            false => SFNEIE_A::B_0x0,
            true => SFNEIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SFNEIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SFNEIE_A::B_0x1
    }
}
#[doc = "TX-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFNFIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<TXFNFIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFNFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFNFIE` reader - TX-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)"]
pub type TXFNFIE_R = crate::BitReader<TXFNFIE_A>;
impl TXFNFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXFNFIE_A {
        match self.bits {
            false => TXFNFIE_A::B_0x0,
            true => TXFNFIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TXFNFIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TXFNFIE_A::B_0x1
    }
}
#[doc = "RX-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFNEIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<RXFNEIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFNEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFNEIE` reader - RX-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)"]
pub type RXFNEIE_R = crate::BitReader<RXFNEIE_A>;
impl RXFNEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXFNEIE_A {
        match self.bits {
            false => RXFNEIE_A::B_0x0,
            true => RXFNEIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RXFNEIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RXFNEIE_A::B_0x1
    }
}
#[doc = "frame complete interrupt enable (whatever the I3C is acting as controller/target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<FCIE_A> for bool {
    #[inline(always)]
    fn from(variant: FCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCIE` reader - frame complete interrupt enable (whatever the I3C is acting as controller/target)"]
pub type FCIE_R = crate::BitReader<FCIE_A>;
impl FCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FCIE_A {
        match self.bits {
            false => FCIE_A::B_0x0,
            true => FCIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FCIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FCIE_A::B_0x1
    }
}
#[doc = "target-initiated read end interrupt enable (when the I3C is acting as controller)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXTGTENDIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<RXTGTENDIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXTGTENDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTGTENDIE` reader - target-initiated read end interrupt enable (when the I3C is acting as controller)"]
pub type RXTGTENDIE_R = crate::BitReader<RXTGTENDIE_A>;
impl RXTGTENDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXTGTENDIE_A {
        match self.bits {
            false => RXTGTENDIE_A::B_0x0,
            true => RXTGTENDIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RXTGTENDIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RXTGTENDIE_A::B_0x1
    }
}
#[doc = "error interrupt enable (whatever the I3C is acting as controller/target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - error interrupt enable (whatever the I3C is acting as controller/target)"]
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::B_0x0,
            true => ERRIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ERRIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ERRIE_A::B_0x1
    }
}
#[doc = "IBI request interrupt enable (when the I3C is acting as controller)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IBIIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<IBIIE_A> for bool {
    #[inline(always)]
    fn from(variant: IBIIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBIIE` reader - IBI request interrupt enable (when the I3C is acting as controller)"]
pub type IBIIE_R = crate::BitReader<IBIIE_A>;
impl IBIIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IBIIE_A {
        match self.bits {
            false => IBIIE_A::B_0x0,
            true => IBIIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IBIIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IBIIE_A::B_0x1
    }
}
#[doc = "IBI end interrupt enable (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IBIENDIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<IBIENDIE_A> for bool {
    #[inline(always)]
    fn from(variant: IBIENDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBIENDIE` reader - IBI end interrupt enable (when the I3C is acting as target)"]
pub type IBIENDIE_R = crate::BitReader<IBIENDIE_A>;
impl IBIENDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IBIENDIE_A {
        match self.bits {
            false => IBIENDIE_A::B_0x0,
            true => IBIENDIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IBIENDIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IBIENDIE_A::B_0x1
    }
}
#[doc = "controller-role request interrupt enable (when the I3C is acting as controller)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<CRIE_A> for bool {
    #[inline(always)]
    fn from(variant: CRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRIE` reader - controller-role request interrupt enable (when the I3C is acting as controller)"]
pub type CRIE_R = crate::BitReader<CRIE_A>;
impl CRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRIE_A {
        match self.bits {
            false => CRIE_A::B_0x0,
            true => CRIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CRIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CRIE_A::B_0x1
    }
}
#[doc = "controller-role update interrupt enable (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRUPDIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<CRUPDIE_A> for bool {
    #[inline(always)]
    fn from(variant: CRUPDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRUPDIE` reader - controller-role update interrupt enable (when the I3C is acting as target)"]
pub type CRUPDIE_R = crate::BitReader<CRUPDIE_A>;
impl CRUPDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRUPDIE_A {
        match self.bits {
            false => CRUPDIE_A::B_0x0,
            true => CRUPDIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CRUPDIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CRUPDIE_A::B_0x1
    }
}
#[doc = "hot-join interrupt enable (when the I3C is acting as controller)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HJIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<HJIE_A> for bool {
    #[inline(always)]
    fn from(variant: HJIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HJIE` reader - hot-join interrupt enable (when the I3C is acting as controller)"]
pub type HJIE_R = crate::BitReader<HJIE_A>;
impl HJIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HJIE_A {
        match self.bits {
            false => HJIE_A::B_0x0,
            true => HJIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HJIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HJIE_A::B_0x1
    }
}
#[doc = "wakeup interrupt enable (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKPIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<WKPIE_A> for bool {
    #[inline(always)]
    fn from(variant: WKPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKPIE` reader - wakeup interrupt enable (when the I3C is acting as target)"]
pub type WKPIE_R = crate::BitReader<WKPIE_A>;
impl WKPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WKPIE_A {
        match self.bits {
            false => WKPIE_A::B_0x0,
            true => WKPIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WKPIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WKPIE_A::B_0x1
    }
}
#[doc = "GETxxx CCC interrupt enable (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GETIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<GETIE_A> for bool {
    #[inline(always)]
    fn from(variant: GETIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GETIE` reader - GETxxx CCC interrupt enable (when the I3C is acting as target)"]
pub type GETIE_R = crate::BitReader<GETIE_A>;
impl GETIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GETIE_A {
        match self.bits {
            false => GETIE_A::B_0x0,
            true => GETIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GETIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GETIE_A::B_0x1
    }
}
#[doc = "GETSTATUS CCC interrupt enable (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<STAIE_A> for bool {
    #[inline(always)]
    fn from(variant: STAIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAIE` reader - GETSTATUS CCC interrupt enable (when the I3C is acting as target)"]
pub type STAIE_R = crate::BitReader<STAIE_A>;
impl STAIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STAIE_A {
        match self.bits {
            false => STAIE_A::B_0x0,
            true => STAIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == STAIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == STAIE_A::B_0x1
    }
}
#[doc = "ENTDAA/RSTDAA/SETNEWDA CCC interrupt enable (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAUPDIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<DAUPDIE_A> for bool {
    #[inline(always)]
    fn from(variant: DAUPDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAUPDIE` reader - ENTDAA/RSTDAA/SETNEWDA CCC interrupt enable (when the I3C is acting as target)"]
pub type DAUPDIE_R = crate::BitReader<DAUPDIE_A>;
impl DAUPDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAUPDIE_A {
        match self.bits {
            false => DAUPDIE_A::B_0x0,
            true => DAUPDIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DAUPDIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DAUPDIE_A::B_0x1
    }
}
#[doc = "SETMWL CCC interrupt enable (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MWLUPDIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<MWLUPDIE_A> for bool {
    #[inline(always)]
    fn from(variant: MWLUPDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MWLUPDIE` reader - SETMWL CCC interrupt enable (when the I3C is acting as target)"]
pub type MWLUPDIE_R = crate::BitReader<MWLUPDIE_A>;
impl MWLUPDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MWLUPDIE_A {
        match self.bits {
            false => MWLUPDIE_A::B_0x0,
            true => MWLUPDIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MWLUPDIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MWLUPDIE_A::B_0x1
    }
}
#[doc = "SETMRL CCC interrupt enable (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRLUPDIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<MRLUPDIE_A> for bool {
    #[inline(always)]
    fn from(variant: MRLUPDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRLUPDIE` reader - SETMRL CCC interrupt enable (when the I3C is acting as target)"]
pub type MRLUPDIE_R = crate::BitReader<MRLUPDIE_A>;
impl MRLUPDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MRLUPDIE_A {
        match self.bits {
            false => MRLUPDIE_A::B_0x0,
            true => MRLUPDIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MRLUPDIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MRLUPDIE_A::B_0x1
    }
}
#[doc = "reset pattern interrupt enable (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<RSTIE_A> for bool {
    #[inline(always)]
    fn from(variant: RSTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTIE` reader - reset pattern interrupt enable (when the I3C is acting as target)"]
pub type RSTIE_R = crate::BitReader<RSTIE_A>;
impl RSTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSTIE_A {
        match self.bits {
            false => RSTIE_A::B_0x0,
            true => RSTIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RSTIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RSTIE_A::B_0x1
    }
}
#[doc = "ENTASx CCC interrupt enable (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASUPDIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<ASUPDIE_A> for bool {
    #[inline(always)]
    fn from(variant: ASUPDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASUPDIE` reader - ENTASx CCC interrupt enable (when the I3C is acting as target)"]
pub type ASUPDIE_R = crate::BitReader<ASUPDIE_A>;
impl ASUPDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASUPDIE_A {
        match self.bits {
            false => ASUPDIE_A::B_0x0,
            true => ASUPDIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ASUPDIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ASUPDIE_A::B_0x1
    }
}
#[doc = "ENEC/DISEC CCC interrupt enable (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTUPDIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<INTUPDIE_A> for bool {
    #[inline(always)]
    fn from(variant: INTUPDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTUPDIE` reader - ENEC/DISEC CCC interrupt enable (when the I3C is acting as target)"]
pub type INTUPDIE_R = crate::BitReader<INTUPDIE_A>;
impl INTUPDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTUPDIE_A {
        match self.bits {
            false => INTUPDIE_A::B_0x0,
            true => INTUPDIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == INTUPDIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == INTUPDIE_A::B_0x1
    }
}
#[doc = "DEFTGTS CCC interrupt enable (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEFIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<DEFIE_A> for bool {
    #[inline(always)]
    fn from(variant: DEFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEFIE` reader - DEFTGTS CCC interrupt enable (when the I3C is acting as target)"]
pub type DEFIE_R = crate::BitReader<DEFIE_A>;
impl DEFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DEFIE_A {
        match self.bits {
            false => DEFIE_A::B_0x0,
            true => DEFIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DEFIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DEFIE_A::B_0x1
    }
}
#[doc = "DEFGRPA CCC interrupt enable (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRPIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<GRPIE_A> for bool {
    #[inline(always)]
    fn from(variant: GRPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GRPIE` reader - DEFGRPA CCC interrupt enable (when the I3C is acting as target)"]
pub type GRPIE_R = crate::BitReader<GRPIE_A>;
impl GRPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GRPIE_A {
        match self.bits {
            false => GRPIE_A::B_0x0,
            true => GRPIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GRPIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GRPIE_A::B_0x1
    }
}
impl R {
    #[doc = "Bit 2 - C-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub fn CFNFIE(&self) -> CFNFIE_R {
        CFNFIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - S-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub fn SFNEIE(&self) -> SFNEIE_R {
        SFNEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub fn TXFNFIE(&self) -> TXFNFIE_R {
        TXFNFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub fn RXFNEIE(&self) -> RXFNEIE_R {
        RXFNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - frame complete interrupt enable (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub fn FCIE(&self) -> FCIE_R {
        FCIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - target-initiated read end interrupt enable (when the I3C is acting as controller)"]
    #[inline(always)]
    pub fn RXTGTENDIE(&self) -> RXTGTENDIE_R {
        RXTGTENDIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - error interrupt enable (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub fn ERRIE(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - IBI request interrupt enable (when the I3C is acting as controller)"]
    #[inline(always)]
    pub fn IBIIE(&self) -> IBIIE_R {
        IBIIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - IBI end interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn IBIENDIE(&self) -> IBIENDIE_R {
        IBIENDIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - controller-role request interrupt enable (when the I3C is acting as controller)"]
    #[inline(always)]
    pub fn CRIE(&self) -> CRIE_R {
        CRIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - controller-role update interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn CRUPDIE(&self) -> CRUPDIE_R {
        CRUPDIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - hot-join interrupt enable (when the I3C is acting as controller)"]
    #[inline(always)]
    pub fn HJIE(&self) -> HJIE_R {
        HJIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - wakeup interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn WKPIE(&self) -> WKPIE_R {
        WKPIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GETxxx CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn GETIE(&self) -> GETIE_R {
        GETIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GETSTATUS CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn STAIE(&self) -> STAIE_R {
        STAIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ENTDAA/RSTDAA/SETNEWDA CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn DAUPDIE(&self) -> DAUPDIE_R {
        DAUPDIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SETMWL CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn MWLUPDIE(&self) -> MWLUPDIE_R {
        MWLUPDIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SETMRL CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn MRLUPDIE(&self) -> MRLUPDIE_R {
        MRLUPDIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - reset pattern interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn RSTIE(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ENTASx CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn ASUPDIE(&self) -> ASUPDIE_R {
        ASUPDIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ENEC/DISEC CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn INTUPDIE(&self) -> INTUPDIE_R {
        INTUPDIE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DEFTGTS CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn DEFIE(&self) -> DEFIE_R {
        DEFIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DEFGRPA CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn GRPIE(&self) -> GRPIE_R {
        GRPIE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "I3C interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IER_SPEC {}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {}
