#[doc = "Register `RXGFC` reader"]
pub type R = crate::R<RXGFC_SPEC>;
#[doc = "Register `RXGFC` writer"]
pub type W = crate::W<RXGFC_SPEC>;
#[doc = "Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRFE_A {
    #[doc = "0: Filter remote frames with 29-bit standard IDs"]
    B_0x0 = 0,
    #[doc = "1: Reject all remote frames with 29-bit standard IDs"]
    B_0x1 = 1,
}
impl From<RRFE_A> for bool {
    #[inline(always)]
    fn from(variant: RRFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRFE` reader - Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type RRFE_R = crate::BitReader<RRFE_A>;
impl RRFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RRFE_A {
        match self.bits {
            false => RRFE_A::B_0x0,
            true => RRFE_A::B_0x1,
        }
    }
    #[doc = "Filter remote frames with 29-bit standard IDs"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RRFE_A::B_0x0
    }
    #[doc = "Reject all remote frames with 29-bit standard IDs"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RRFE_A::B_0x1
    }
}
#[doc = "Field `RRFE` writer - Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type RRFE_W<'a, REG> = crate::BitWriter<'a, REG, RRFE_A>;
impl<'a, REG> RRFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filter remote frames with 29-bit standard IDs"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RRFE_A::B_0x0)
    }
    #[doc = "Reject all remote frames with 29-bit standard IDs"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RRFE_A::B_0x1)
    }
}
#[doc = "Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRFS_A {
    #[doc = "0: Filter remote frames with 11-bit standard IDs"]
    B_0x0 = 0,
    #[doc = "1: Reject all remote frames with 11-bit standard IDs"]
    B_0x1 = 1,
}
impl From<RRFS_A> for bool {
    #[inline(always)]
    fn from(variant: RRFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRFS` reader - Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type RRFS_R = crate::BitReader<RRFS_A>;
impl RRFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RRFS_A {
        match self.bits {
            false => RRFS_A::B_0x0,
            true => RRFS_A::B_0x1,
        }
    }
    #[doc = "Filter remote frames with 11-bit standard IDs"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RRFS_A::B_0x0
    }
    #[doc = "Reject all remote frames with 11-bit standard IDs"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RRFS_A::B_0x1
    }
}
#[doc = "Field `RRFS` writer - Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type RRFS_W<'a, REG> = crate::BitWriter<'a, REG, RRFS_A>;
impl<'a, REG> RRFS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filter remote frames with 11-bit standard IDs"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RRFS_A::B_0x0)
    }
    #[doc = "Reject all remote frames with 11-bit standard IDs"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RRFS_A::B_0x1)
    }
}
#[doc = "Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ANFE_A {
    #[doc = "0: Accept in Rx FIFO 0"]
    B_0x0 = 0,
    #[doc = "1: Accept in Rx FIFO 1"]
    B_0x1 = 1,
    #[doc = "2: Reject"]
    B_0x2 = 2,
    #[doc = "3: Reject"]
    B_0x3 = 3,
}
impl From<ANFE_A> for u8 {
    #[inline(always)]
    fn from(variant: ANFE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ANFE_A {
    type Ux = u8;
}
impl crate::IsEnum for ANFE_A {}
#[doc = "Field `ANFE` reader - Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type ANFE_R = crate::FieldReader<ANFE_A>;
impl ANFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ANFE_A {
        match self.bits {
            0 => ANFE_A::B_0x0,
            1 => ANFE_A::B_0x1,
            2 => ANFE_A::B_0x2,
            3 => ANFE_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ANFE_A::B_0x0
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ANFE_A::B_0x1
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == ANFE_A::B_0x2
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == ANFE_A::B_0x3
    }
}
#[doc = "Field `ANFE` writer - Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type ANFE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ANFE_A, crate::Safe>;
impl<'a, REG> ANFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ANFE_A::B_0x0)
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ANFE_A::B_0x1)
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ANFE_A::B_0x2)
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ANFE_A::B_0x3)
    }
}
#[doc = "Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ANFS_A {
    #[doc = "0: Accept in Rx FIFO 0"]
    B_0x0 = 0,
    #[doc = "1: Accept in Rx FIFO 1"]
    B_0x1 = 1,
    #[doc = "2: Reject"]
    B_0x2 = 2,
    #[doc = "3: Reject"]
    B_0x3 = 3,
}
impl From<ANFS_A> for u8 {
    #[inline(always)]
    fn from(variant: ANFS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ANFS_A {
    type Ux = u8;
}
impl crate::IsEnum for ANFS_A {}
#[doc = "Field `ANFS` reader - Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type ANFS_R = crate::FieldReader<ANFS_A>;
impl ANFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ANFS_A {
        match self.bits {
            0 => ANFS_A::B_0x0,
            1 => ANFS_A::B_0x1,
            2 => ANFS_A::B_0x2,
            3 => ANFS_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ANFS_A::B_0x0
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ANFS_A::B_0x1
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == ANFS_A::B_0x2
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == ANFS_A::B_0x3
    }
}
#[doc = "Field `ANFS` writer - Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type ANFS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ANFS_A, crate::Safe>;
impl<'a, REG> ANFS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ANFS_A::B_0x0)
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ANFS_A::B_0x1)
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ANFS_A::B_0x2)
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ANFS_A::B_0x3)
    }
}
#[doc = "Field `F1OM` reader - FIFO 1 operation mode (overwrite or blocking) This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type F1OM_R = crate::BitReader;
#[doc = "Field `F1OM` writer - FIFO 1 operation mode (overwrite or blocking) This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type F1OM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F0OM` reader - FIFO 0 operation mode (overwrite or blocking) This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type F0OM_R = crate::BitReader;
#[doc = "Field `F0OM` writer - FIFO 0 operation mode (overwrite or blocking) This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type F0OM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "List size standard 28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LSS_A {
    #[doc = "0: No standard message ID filter"]
    B_0x0 = 0,
    #[doc = "1: Number of standard message ID filter elements"]
    B_0x1 = 1,
    #[doc = "2: Number of standard message ID filter elements"]
    B_0x2 = 2,
    #[doc = "3: Number of standard message ID filter elements"]
    B_0x3 = 3,
    #[doc = "4: Number of standard message ID filter elements"]
    B_0x4 = 4,
    #[doc = "5: Number of standard message ID filter elements"]
    B_0x5 = 5,
    #[doc = "6: Number of standard message ID filter elements"]
    B_0x6 = 6,
    #[doc = "7: Number of standard message ID filter elements"]
    B_0x7 = 7,
    #[doc = "8: Number of standard message ID filter elements"]
    B_0x8 = 8,
    #[doc = "9: Number of standard message ID filter elements"]
    B_0x9 = 9,
    #[doc = "10: Number of standard message ID filter elements"]
    B_0xa = 10,
    #[doc = "11: Number of standard message ID filter elements"]
    B_0xb = 11,
    #[doc = "12: Number of standard message ID filter elements"]
    B_0xc = 12,
    #[doc = "13: Number of standard message ID filter elements"]
    B_0xd = 13,
    #[doc = "14: Number of standard message ID filter elements"]
    B_0xe = 14,
    #[doc = "15: Number of standard message ID filter elements"]
    B_0xf = 15,
    #[doc = "16: Number of standard message ID filter elements"]
    B_0x10 = 16,
    #[doc = "17: Number of standard message ID filter elements"]
    B_0x11 = 17,
    #[doc = "18: Number of standard message ID filter elements"]
    B_0x12 = 18,
    #[doc = "19: Number of standard message ID filter elements"]
    B_0x13 = 19,
    #[doc = "20: Number of standard message ID filter elements"]
    B_0x14 = 20,
    #[doc = "21: Number of standard message ID filter elements"]
    B_0x15 = 21,
    #[doc = "22: Number of standard message ID filter elements"]
    B_0x16 = 22,
    #[doc = "23: Number of standard message ID filter elements"]
    B_0x17 = 23,
    #[doc = "24: Number of standard message ID filter elements"]
    B_0x18 = 24,
    #[doc = "25: Number of standard message ID filter elements"]
    B_0x19 = 25,
    #[doc = "26: Number of standard message ID filter elements"]
    B_0x1a = 26,
    #[doc = "27: Number of standard message ID filter elements"]
    B_0x1b = 27,
    #[doc = "28: Number of standard message ID filter elements"]
    B_0x1c = 28,
}
impl From<LSS_A> for u8 {
    #[inline(always)]
    fn from(variant: LSS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LSS_A {
    type Ux = u8;
}
impl crate::IsEnum for LSS_A {}
#[doc = "Field `LSS` reader - List size standard 28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type LSS_R = crate::FieldReader<LSS_A>;
impl LSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LSS_A> {
        match self.bits {
            0 => Some(LSS_A::B_0x0),
            1 => Some(LSS_A::B_0x1),
            2 => Some(LSS_A::B_0x2),
            3 => Some(LSS_A::B_0x3),
            4 => Some(LSS_A::B_0x4),
            5 => Some(LSS_A::B_0x5),
            6 => Some(LSS_A::B_0x6),
            7 => Some(LSS_A::B_0x7),
            8 => Some(LSS_A::B_0x8),
            9 => Some(LSS_A::B_0x9),
            10 => Some(LSS_A::B_0xa),
            11 => Some(LSS_A::B_0xb),
            12 => Some(LSS_A::B_0xc),
            13 => Some(LSS_A::B_0xd),
            14 => Some(LSS_A::B_0xe),
            15 => Some(LSS_A::B_0xf),
            16 => Some(LSS_A::B_0x10),
            17 => Some(LSS_A::B_0x11),
            18 => Some(LSS_A::B_0x12),
            19 => Some(LSS_A::B_0x13),
            20 => Some(LSS_A::B_0x14),
            21 => Some(LSS_A::B_0x15),
            22 => Some(LSS_A::B_0x16),
            23 => Some(LSS_A::B_0x17),
            24 => Some(LSS_A::B_0x18),
            25 => Some(LSS_A::B_0x19),
            26 => Some(LSS_A::B_0x1a),
            27 => Some(LSS_A::B_0x1b),
            28 => Some(LSS_A::B_0x1c),
            _ => None,
        }
    }
    #[doc = "No standard message ID filter"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LSS_A::B_0x0
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LSS_A::B_0x1
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == LSS_A::B_0x2
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == LSS_A::B_0x3
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == LSS_A::B_0x4
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == LSS_A::B_0x5
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == LSS_A::B_0x6
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == LSS_A::B_0x7
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == LSS_A::B_0x8
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == LSS_A::B_0x9
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0xa(&self) -> bool {
        *self == LSS_A::B_0xa
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0xb(&self) -> bool {
        *self == LSS_A::B_0xb
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0xc(&self) -> bool {
        *self == LSS_A::B_0xc
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0xd(&self) -> bool {
        *self == LSS_A::B_0xd
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0xe(&self) -> bool {
        *self == LSS_A::B_0xe
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0xf(&self) -> bool {
        *self == LSS_A::B_0xf
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x10(&self) -> bool {
        *self == LSS_A::B_0x10
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x11(&self) -> bool {
        *self == LSS_A::B_0x11
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x12(&self) -> bool {
        *self == LSS_A::B_0x12
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x13(&self) -> bool {
        *self == LSS_A::B_0x13
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x14(&self) -> bool {
        *self == LSS_A::B_0x14
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x15(&self) -> bool {
        *self == LSS_A::B_0x15
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x16(&self) -> bool {
        *self == LSS_A::B_0x16
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x17(&self) -> bool {
        *self == LSS_A::B_0x17
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x18(&self) -> bool {
        *self == LSS_A::B_0x18
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x19(&self) -> bool {
        *self == LSS_A::B_0x19
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x1a(&self) -> bool {
        *self == LSS_A::B_0x1a
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x1b(&self) -> bool {
        *self == LSS_A::B_0x1b
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x1c(&self) -> bool {
        *self == LSS_A::B_0x1c
    }
}
#[doc = "Field `LSS` writer - List size standard 28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type LSS_W<'a, REG> = crate::FieldWriter<'a, REG, 5, LSS_A>;
impl<'a, REG> LSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No standard message ID filter"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x0)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x1)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x2)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x3)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x4)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x5)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x6)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x7)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x8)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x9)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0xa(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0xa)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0xb(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0xb)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0xc(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0xc)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0xd(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0xd)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0xe(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0xe)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0xf(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0xf)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0x10(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x10)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0x11(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x11)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0x12(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x12)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0x13(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x13)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0x14(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x14)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0x15(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x15)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0x16(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x16)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0x17(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x17)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0x18(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x18)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0x19(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x19)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0x1a(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x1a)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0x1b(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x1b)
    }
    #[doc = "Number of standard message ID filter elements"]
    #[inline(always)]
    pub fn B_0x1c(self) -> &'a mut crate::W<REG> {
        self.variant(LSS_A::B_0x1c)
    }
}
#[doc = "List size extended 8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LSE_A {
    #[doc = "0: No extended message ID filter"]
    B_0x0 = 0,
    #[doc = "1: Number of extended message ID filter elements"]
    B_0x1 = 1,
    #[doc = "2: Number of extended message ID filter elements"]
    B_0x2 = 2,
    #[doc = "3: Number of extended message ID filter elements"]
    B_0x3 = 3,
    #[doc = "4: Number of extended message ID filter elements"]
    B_0x4 = 4,
    #[doc = "5: Number of extended message ID filter elements"]
    B_0x5 = 5,
    #[doc = "6: Number of extended message ID filter elements"]
    B_0x6 = 6,
    #[doc = "7: Number of extended message ID filter elements"]
    B_0x7 = 7,
    #[doc = "8: Number of extended message ID filter elements"]
    B_0x8 = 8,
}
impl From<LSE_A> for u8 {
    #[inline(always)]
    fn from(variant: LSE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LSE_A {
    type Ux = u8;
}
impl crate::IsEnum for LSE_A {}
#[doc = "Field `LSE` reader - List size extended 8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type LSE_R = crate::FieldReader<LSE_A>;
impl LSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LSE_A> {
        match self.bits {
            0 => Some(LSE_A::B_0x0),
            1 => Some(LSE_A::B_0x1),
            2 => Some(LSE_A::B_0x2),
            3 => Some(LSE_A::B_0x3),
            4 => Some(LSE_A::B_0x4),
            5 => Some(LSE_A::B_0x5),
            6 => Some(LSE_A::B_0x6),
            7 => Some(LSE_A::B_0x7),
            8 => Some(LSE_A::B_0x8),
            _ => None,
        }
    }
    #[doc = "No extended message ID filter"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LSE_A::B_0x0
    }
    #[doc = "Number of extended message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LSE_A::B_0x1
    }
    #[doc = "Number of extended message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == LSE_A::B_0x2
    }
    #[doc = "Number of extended message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == LSE_A::B_0x3
    }
    #[doc = "Number of extended message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == LSE_A::B_0x4
    }
    #[doc = "Number of extended message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == LSE_A::B_0x5
    }
    #[doc = "Number of extended message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == LSE_A::B_0x6
    }
    #[doc = "Number of extended message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == LSE_A::B_0x7
    }
    #[doc = "Number of extended message ID filter elements"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == LSE_A::B_0x8
    }
}
#[doc = "Field `LSE` writer - List size extended 8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type LSE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, LSE_A>;
impl<'a, REG> LSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No extended message ID filter"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSE_A::B_0x0)
    }
    #[doc = "Number of extended message ID filter elements"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSE_A::B_0x1)
    }
    #[doc = "Number of extended message ID filter elements"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(LSE_A::B_0x2)
    }
    #[doc = "Number of extended message ID filter elements"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(LSE_A::B_0x3)
    }
    #[doc = "Number of extended message ID filter elements"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(LSE_A::B_0x4)
    }
    #[doc = "Number of extended message ID filter elements"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(LSE_A::B_0x5)
    }
    #[doc = "Number of extended message ID filter elements"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(LSE_A::B_0x6)
    }
    #[doc = "Number of extended message ID filter elements"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(LSE_A::B_0x7)
    }
    #[doc = "Number of extended message ID filter elements"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(LSE_A::B_0x8)
    }
}
impl R {
    #[doc = "Bit 0 - Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn RRFE(&self) -> RRFE_R {
        RRFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn RRFS(&self) -> RRFS_R {
        RRFS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn ANFE(&self) -> ANFE_R {
        ANFE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn ANFS(&self) -> ANFS_R {
        ANFS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - FIFO 1 operation mode (overwrite or blocking) This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn F1OM(&self) -> F1OM_R {
        F1OM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FIFO 0 operation mode (overwrite or blocking) This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn F0OM(&self) -> F0OM_R {
        F0OM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:20 - List size standard 28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn LSS(&self) -> LSS_R {
        LSS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - List size extended 8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn LSE(&self) -> LSE_R {
        LSE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn RRFE(&mut self) -> RRFE_W<'_, RXGFC_SPEC> {
        RRFE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn RRFS(&mut self) -> RRFS_W<'_, RXGFC_SPEC> {
        RRFS_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn ANFE(&mut self) -> ANFE_W<'_, RXGFC_SPEC> {
        ANFE_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn ANFS(&mut self) -> ANFS_W<'_, RXGFC_SPEC> {
        ANFS_W::new(self, 4)
    }
    #[doc = "Bit 8 - FIFO 1 operation mode (overwrite or blocking) This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn F1OM(&mut self) -> F1OM_W<'_, RXGFC_SPEC> {
        F1OM_W::new(self, 8)
    }
    #[doc = "Bit 9 - FIFO 0 operation mode (overwrite or blocking) This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn F0OM(&mut self) -> F0OM_W<'_, RXGFC_SPEC> {
        F0OM_W::new(self, 9)
    }
    #[doc = "Bits 16:20 - List size standard 28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn LSS(&mut self) -> LSS_W<'_, RXGFC_SPEC> {
        LSS_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - List size extended 8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn LSE(&mut self) -> LSE_W<'_, RXGFC_SPEC> {
        LSE_W::new(self, 24)
    }
}
#[doc = "FDCAN global filter configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxgfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxgfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXGFC_SPEC;
impl crate::RegisterSpec for RXGFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxgfc::R`](R) reader structure"]
impl crate::Readable for RXGFC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxgfc::W`](W) writer structure"]
impl crate::Writable for RXGFC_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RXGFC to value 0"]
impl crate::Resettable for RXGFC_SPEC {}
