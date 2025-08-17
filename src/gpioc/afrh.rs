#[doc = "Register `AFRH` reader"]
pub type R = crate::R<AFRH_SPEC>;
#[doc = "Register `AFRH` writer"]
pub type W = crate::W<AFRH_SPEC>;
#[doc = "Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL8_A {
    #[doc = "0: AF0"]
    B_0x0 = 0,
    #[doc = "1: AF1"]
    B_0x1 = 1,
    #[doc = "2: AF2"]
    B_0x2 = 2,
    #[doc = "3: AF3"]
    B_0x3 = 3,
    #[doc = "4: AF4"]
    B_0x4 = 4,
    #[doc = "5: AF5"]
    B_0x5 = 5,
    #[doc = "6: AF6"]
    B_0x6 = 6,
    #[doc = "7: AF7"]
    B_0x7 = 7,
    #[doc = "8: AF8"]
    B_0x8 = 8,
    #[doc = "9: AF9"]
    B_0x9 = 9,
    #[doc = "10: AF10"]
    B_0xA = 10,
    #[doc = "11: AF11"]
    B_0xB = 11,
    #[doc = "12: AF12"]
    B_0xC = 12,
    #[doc = "13: AF13"]
    B_0xD = 13,
    #[doc = "14: AF14"]
    B_0xE = 14,
    #[doc = "15: AF15"]
    B_0xF = 15,
}
impl From<AFSEL8_A> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL8_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL8_A {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL8_A {}
#[doc = "Field `AFSEL8` reader - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL8_R = crate::FieldReader<AFSEL8_A>;
impl AFSEL8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL8_A {
        match self.bits {
            0 => AFSEL8_A::B_0x0,
            1 => AFSEL8_A::B_0x1,
            2 => AFSEL8_A::B_0x2,
            3 => AFSEL8_A::B_0x3,
            4 => AFSEL8_A::B_0x4,
            5 => AFSEL8_A::B_0x5,
            6 => AFSEL8_A::B_0x6,
            7 => AFSEL8_A::B_0x7,
            8 => AFSEL8_A::B_0x8,
            9 => AFSEL8_A::B_0x9,
            10 => AFSEL8_A::B_0xA,
            11 => AFSEL8_A::B_0xB,
            12 => AFSEL8_A::B_0xC,
            13 => AFSEL8_A::B_0xD,
            14 => AFSEL8_A::B_0xE,
            15 => AFSEL8_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AFSEL8_A::B_0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AFSEL8_A::B_0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == AFSEL8_A::B_0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == AFSEL8_A::B_0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == AFSEL8_A::B_0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == AFSEL8_A::B_0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == AFSEL8_A::B_0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == AFSEL8_A::B_0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == AFSEL8_A::B_0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == AFSEL8_A::B_0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == AFSEL8_A::B_0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == AFSEL8_A::B_0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == AFSEL8_A::B_0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == AFSEL8_A::B_0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == AFSEL8_A::B_0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == AFSEL8_A::B_0xF
    }
}
#[doc = "Field `AFSEL8` writer - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL8_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL8_A, crate::Safe>;
impl<'a, REG> AFSEL8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::B_0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::B_0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::B_0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::B_0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::B_0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::B_0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::B_0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::B_0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::B_0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::B_0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::B_0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::B_0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::B_0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::B_0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::B_0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::B_0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL9_A {
    #[doc = "0: AF0"]
    B_0x0 = 0,
    #[doc = "1: AF1"]
    B_0x1 = 1,
    #[doc = "2: AF2"]
    B_0x2 = 2,
    #[doc = "3: AF3"]
    B_0x3 = 3,
    #[doc = "4: AF4"]
    B_0x4 = 4,
    #[doc = "5: AF5"]
    B_0x5 = 5,
    #[doc = "6: AF6"]
    B_0x6 = 6,
    #[doc = "7: AF7"]
    B_0x7 = 7,
    #[doc = "8: AF8"]
    B_0x8 = 8,
    #[doc = "9: AF9"]
    B_0x9 = 9,
    #[doc = "10: AF10"]
    B_0xA = 10,
    #[doc = "11: AF11"]
    B_0xB = 11,
    #[doc = "12: AF12"]
    B_0xC = 12,
    #[doc = "13: AF13"]
    B_0xD = 13,
    #[doc = "14: AF14"]
    B_0xE = 14,
    #[doc = "15: AF15"]
    B_0xF = 15,
}
impl From<AFSEL9_A> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL9_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL9_A {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL9_A {}
#[doc = "Field `AFSEL9` reader - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL9_R = crate::FieldReader<AFSEL9_A>;
impl AFSEL9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL9_A {
        match self.bits {
            0 => AFSEL9_A::B_0x0,
            1 => AFSEL9_A::B_0x1,
            2 => AFSEL9_A::B_0x2,
            3 => AFSEL9_A::B_0x3,
            4 => AFSEL9_A::B_0x4,
            5 => AFSEL9_A::B_0x5,
            6 => AFSEL9_A::B_0x6,
            7 => AFSEL9_A::B_0x7,
            8 => AFSEL9_A::B_0x8,
            9 => AFSEL9_A::B_0x9,
            10 => AFSEL9_A::B_0xA,
            11 => AFSEL9_A::B_0xB,
            12 => AFSEL9_A::B_0xC,
            13 => AFSEL9_A::B_0xD,
            14 => AFSEL9_A::B_0xE,
            15 => AFSEL9_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AFSEL9_A::B_0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AFSEL9_A::B_0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == AFSEL9_A::B_0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == AFSEL9_A::B_0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == AFSEL9_A::B_0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == AFSEL9_A::B_0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == AFSEL9_A::B_0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == AFSEL9_A::B_0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == AFSEL9_A::B_0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == AFSEL9_A::B_0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == AFSEL9_A::B_0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == AFSEL9_A::B_0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == AFSEL9_A::B_0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == AFSEL9_A::B_0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == AFSEL9_A::B_0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == AFSEL9_A::B_0xF
    }
}
#[doc = "Field `AFSEL9` writer - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL9_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL9_A, crate::Safe>;
impl<'a, REG> AFSEL9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9_A::B_0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9_A::B_0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9_A::B_0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9_A::B_0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9_A::B_0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9_A::B_0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9_A::B_0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9_A::B_0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9_A::B_0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9_A::B_0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9_A::B_0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9_A::B_0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9_A::B_0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9_A::B_0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9_A::B_0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9_A::B_0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL10_A {
    #[doc = "0: AF0"]
    B_0x0 = 0,
    #[doc = "1: AF1"]
    B_0x1 = 1,
    #[doc = "2: AF2"]
    B_0x2 = 2,
    #[doc = "3: AF3"]
    B_0x3 = 3,
    #[doc = "4: AF4"]
    B_0x4 = 4,
    #[doc = "5: AF5"]
    B_0x5 = 5,
    #[doc = "6: AF6"]
    B_0x6 = 6,
    #[doc = "7: AF7"]
    B_0x7 = 7,
    #[doc = "8: AF8"]
    B_0x8 = 8,
    #[doc = "9: AF9"]
    B_0x9 = 9,
    #[doc = "10: AF10"]
    B_0xA = 10,
    #[doc = "11: AF11"]
    B_0xB = 11,
    #[doc = "12: AF12"]
    B_0xC = 12,
    #[doc = "13: AF13"]
    B_0xD = 13,
    #[doc = "14: AF14"]
    B_0xE = 14,
    #[doc = "15: AF15"]
    B_0xF = 15,
}
impl From<AFSEL10_A> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL10_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL10_A {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL10_A {}
#[doc = "Field `AFSEL10` reader - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL10_R = crate::FieldReader<AFSEL10_A>;
impl AFSEL10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL10_A {
        match self.bits {
            0 => AFSEL10_A::B_0x0,
            1 => AFSEL10_A::B_0x1,
            2 => AFSEL10_A::B_0x2,
            3 => AFSEL10_A::B_0x3,
            4 => AFSEL10_A::B_0x4,
            5 => AFSEL10_A::B_0x5,
            6 => AFSEL10_A::B_0x6,
            7 => AFSEL10_A::B_0x7,
            8 => AFSEL10_A::B_0x8,
            9 => AFSEL10_A::B_0x9,
            10 => AFSEL10_A::B_0xA,
            11 => AFSEL10_A::B_0xB,
            12 => AFSEL10_A::B_0xC,
            13 => AFSEL10_A::B_0xD,
            14 => AFSEL10_A::B_0xE,
            15 => AFSEL10_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AFSEL10_A::B_0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AFSEL10_A::B_0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == AFSEL10_A::B_0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == AFSEL10_A::B_0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == AFSEL10_A::B_0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == AFSEL10_A::B_0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == AFSEL10_A::B_0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == AFSEL10_A::B_0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == AFSEL10_A::B_0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == AFSEL10_A::B_0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == AFSEL10_A::B_0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == AFSEL10_A::B_0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == AFSEL10_A::B_0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == AFSEL10_A::B_0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == AFSEL10_A::B_0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == AFSEL10_A::B_0xF
    }
}
#[doc = "Field `AFSEL10` writer - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL10_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL10_A, crate::Safe>;
impl<'a, REG> AFSEL10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10_A::B_0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10_A::B_0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10_A::B_0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10_A::B_0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10_A::B_0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10_A::B_0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10_A::B_0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10_A::B_0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10_A::B_0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10_A::B_0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10_A::B_0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10_A::B_0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10_A::B_0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10_A::B_0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10_A::B_0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10_A::B_0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL11_A {
    #[doc = "0: AF0"]
    B_0x0 = 0,
    #[doc = "1: AF1"]
    B_0x1 = 1,
    #[doc = "2: AF2"]
    B_0x2 = 2,
    #[doc = "3: AF3"]
    B_0x3 = 3,
    #[doc = "4: AF4"]
    B_0x4 = 4,
    #[doc = "5: AF5"]
    B_0x5 = 5,
    #[doc = "6: AF6"]
    B_0x6 = 6,
    #[doc = "7: AF7"]
    B_0x7 = 7,
    #[doc = "8: AF8"]
    B_0x8 = 8,
    #[doc = "9: AF9"]
    B_0x9 = 9,
    #[doc = "10: AF10"]
    B_0xA = 10,
    #[doc = "11: AF11"]
    B_0xB = 11,
    #[doc = "12: AF12"]
    B_0xC = 12,
    #[doc = "13: AF13"]
    B_0xD = 13,
    #[doc = "14: AF14"]
    B_0xE = 14,
    #[doc = "15: AF15"]
    B_0xF = 15,
}
impl From<AFSEL11_A> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL11_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL11_A {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL11_A {}
#[doc = "Field `AFSEL11` reader - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL11_R = crate::FieldReader<AFSEL11_A>;
impl AFSEL11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL11_A {
        match self.bits {
            0 => AFSEL11_A::B_0x0,
            1 => AFSEL11_A::B_0x1,
            2 => AFSEL11_A::B_0x2,
            3 => AFSEL11_A::B_0x3,
            4 => AFSEL11_A::B_0x4,
            5 => AFSEL11_A::B_0x5,
            6 => AFSEL11_A::B_0x6,
            7 => AFSEL11_A::B_0x7,
            8 => AFSEL11_A::B_0x8,
            9 => AFSEL11_A::B_0x9,
            10 => AFSEL11_A::B_0xA,
            11 => AFSEL11_A::B_0xB,
            12 => AFSEL11_A::B_0xC,
            13 => AFSEL11_A::B_0xD,
            14 => AFSEL11_A::B_0xE,
            15 => AFSEL11_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AFSEL11_A::B_0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AFSEL11_A::B_0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == AFSEL11_A::B_0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == AFSEL11_A::B_0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == AFSEL11_A::B_0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == AFSEL11_A::B_0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == AFSEL11_A::B_0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == AFSEL11_A::B_0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == AFSEL11_A::B_0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == AFSEL11_A::B_0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == AFSEL11_A::B_0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == AFSEL11_A::B_0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == AFSEL11_A::B_0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == AFSEL11_A::B_0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == AFSEL11_A::B_0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == AFSEL11_A::B_0xF
    }
}
#[doc = "Field `AFSEL11` writer - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL11_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL11_A, crate::Safe>;
impl<'a, REG> AFSEL11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11_A::B_0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11_A::B_0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11_A::B_0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11_A::B_0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11_A::B_0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11_A::B_0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11_A::B_0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11_A::B_0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11_A::B_0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11_A::B_0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11_A::B_0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11_A::B_0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11_A::B_0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11_A::B_0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11_A::B_0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11_A::B_0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL12_A {
    #[doc = "0: AF0"]
    B_0x0 = 0,
    #[doc = "1: AF1"]
    B_0x1 = 1,
    #[doc = "2: AF2"]
    B_0x2 = 2,
    #[doc = "3: AF3"]
    B_0x3 = 3,
    #[doc = "4: AF4"]
    B_0x4 = 4,
    #[doc = "5: AF5"]
    B_0x5 = 5,
    #[doc = "6: AF6"]
    B_0x6 = 6,
    #[doc = "7: AF7"]
    B_0x7 = 7,
    #[doc = "8: AF8"]
    B_0x8 = 8,
    #[doc = "9: AF9"]
    B_0x9 = 9,
    #[doc = "10: AF10"]
    B_0xA = 10,
    #[doc = "11: AF11"]
    B_0xB = 11,
    #[doc = "12: AF12"]
    B_0xC = 12,
    #[doc = "13: AF13"]
    B_0xD = 13,
    #[doc = "14: AF14"]
    B_0xE = 14,
    #[doc = "15: AF15"]
    B_0xF = 15,
}
impl From<AFSEL12_A> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL12_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL12_A {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL12_A {}
#[doc = "Field `AFSEL12` reader - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL12_R = crate::FieldReader<AFSEL12_A>;
impl AFSEL12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL12_A {
        match self.bits {
            0 => AFSEL12_A::B_0x0,
            1 => AFSEL12_A::B_0x1,
            2 => AFSEL12_A::B_0x2,
            3 => AFSEL12_A::B_0x3,
            4 => AFSEL12_A::B_0x4,
            5 => AFSEL12_A::B_0x5,
            6 => AFSEL12_A::B_0x6,
            7 => AFSEL12_A::B_0x7,
            8 => AFSEL12_A::B_0x8,
            9 => AFSEL12_A::B_0x9,
            10 => AFSEL12_A::B_0xA,
            11 => AFSEL12_A::B_0xB,
            12 => AFSEL12_A::B_0xC,
            13 => AFSEL12_A::B_0xD,
            14 => AFSEL12_A::B_0xE,
            15 => AFSEL12_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AFSEL12_A::B_0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AFSEL12_A::B_0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == AFSEL12_A::B_0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == AFSEL12_A::B_0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == AFSEL12_A::B_0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == AFSEL12_A::B_0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == AFSEL12_A::B_0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == AFSEL12_A::B_0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == AFSEL12_A::B_0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == AFSEL12_A::B_0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == AFSEL12_A::B_0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == AFSEL12_A::B_0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == AFSEL12_A::B_0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == AFSEL12_A::B_0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == AFSEL12_A::B_0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == AFSEL12_A::B_0xF
    }
}
#[doc = "Field `AFSEL12` writer - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL12_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL12_A, crate::Safe>;
impl<'a, REG> AFSEL12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12_A::B_0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12_A::B_0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12_A::B_0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12_A::B_0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12_A::B_0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12_A::B_0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12_A::B_0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12_A::B_0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12_A::B_0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12_A::B_0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12_A::B_0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12_A::B_0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12_A::B_0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12_A::B_0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12_A::B_0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12_A::B_0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL13_A {
    #[doc = "0: AF0"]
    B_0x0 = 0,
    #[doc = "1: AF1"]
    B_0x1 = 1,
    #[doc = "2: AF2"]
    B_0x2 = 2,
    #[doc = "3: AF3"]
    B_0x3 = 3,
    #[doc = "4: AF4"]
    B_0x4 = 4,
    #[doc = "5: AF5"]
    B_0x5 = 5,
    #[doc = "6: AF6"]
    B_0x6 = 6,
    #[doc = "7: AF7"]
    B_0x7 = 7,
    #[doc = "8: AF8"]
    B_0x8 = 8,
    #[doc = "9: AF9"]
    B_0x9 = 9,
    #[doc = "10: AF10"]
    B_0xA = 10,
    #[doc = "11: AF11"]
    B_0xB = 11,
    #[doc = "12: AF12"]
    B_0xC = 12,
    #[doc = "13: AF13"]
    B_0xD = 13,
    #[doc = "14: AF14"]
    B_0xE = 14,
    #[doc = "15: AF15"]
    B_0xF = 15,
}
impl From<AFSEL13_A> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL13_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL13_A {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL13_A {}
#[doc = "Field `AFSEL13` reader - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL13_R = crate::FieldReader<AFSEL13_A>;
impl AFSEL13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL13_A {
        match self.bits {
            0 => AFSEL13_A::B_0x0,
            1 => AFSEL13_A::B_0x1,
            2 => AFSEL13_A::B_0x2,
            3 => AFSEL13_A::B_0x3,
            4 => AFSEL13_A::B_0x4,
            5 => AFSEL13_A::B_0x5,
            6 => AFSEL13_A::B_0x6,
            7 => AFSEL13_A::B_0x7,
            8 => AFSEL13_A::B_0x8,
            9 => AFSEL13_A::B_0x9,
            10 => AFSEL13_A::B_0xA,
            11 => AFSEL13_A::B_0xB,
            12 => AFSEL13_A::B_0xC,
            13 => AFSEL13_A::B_0xD,
            14 => AFSEL13_A::B_0xE,
            15 => AFSEL13_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AFSEL13_A::B_0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AFSEL13_A::B_0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == AFSEL13_A::B_0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == AFSEL13_A::B_0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == AFSEL13_A::B_0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == AFSEL13_A::B_0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == AFSEL13_A::B_0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == AFSEL13_A::B_0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == AFSEL13_A::B_0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == AFSEL13_A::B_0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == AFSEL13_A::B_0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == AFSEL13_A::B_0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == AFSEL13_A::B_0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == AFSEL13_A::B_0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == AFSEL13_A::B_0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == AFSEL13_A::B_0xF
    }
}
#[doc = "Field `AFSEL13` writer - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL13_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL13_A, crate::Safe>;
impl<'a, REG> AFSEL13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13_A::B_0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13_A::B_0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13_A::B_0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13_A::B_0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13_A::B_0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13_A::B_0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13_A::B_0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13_A::B_0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13_A::B_0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13_A::B_0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13_A::B_0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13_A::B_0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13_A::B_0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13_A::B_0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13_A::B_0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13_A::B_0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL14_A {
    #[doc = "0: AF0"]
    B_0x0 = 0,
    #[doc = "1: AF1"]
    B_0x1 = 1,
    #[doc = "2: AF2"]
    B_0x2 = 2,
    #[doc = "3: AF3"]
    B_0x3 = 3,
    #[doc = "4: AF4"]
    B_0x4 = 4,
    #[doc = "5: AF5"]
    B_0x5 = 5,
    #[doc = "6: AF6"]
    B_0x6 = 6,
    #[doc = "7: AF7"]
    B_0x7 = 7,
    #[doc = "8: AF8"]
    B_0x8 = 8,
    #[doc = "9: AF9"]
    B_0x9 = 9,
    #[doc = "10: AF10"]
    B_0xA = 10,
    #[doc = "11: AF11"]
    B_0xB = 11,
    #[doc = "12: AF12"]
    B_0xC = 12,
    #[doc = "13: AF13"]
    B_0xD = 13,
    #[doc = "14: AF14"]
    B_0xE = 14,
    #[doc = "15: AF15"]
    B_0xF = 15,
}
impl From<AFSEL14_A> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL14_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL14_A {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL14_A {}
#[doc = "Field `AFSEL14` reader - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL14_R = crate::FieldReader<AFSEL14_A>;
impl AFSEL14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL14_A {
        match self.bits {
            0 => AFSEL14_A::B_0x0,
            1 => AFSEL14_A::B_0x1,
            2 => AFSEL14_A::B_0x2,
            3 => AFSEL14_A::B_0x3,
            4 => AFSEL14_A::B_0x4,
            5 => AFSEL14_A::B_0x5,
            6 => AFSEL14_A::B_0x6,
            7 => AFSEL14_A::B_0x7,
            8 => AFSEL14_A::B_0x8,
            9 => AFSEL14_A::B_0x9,
            10 => AFSEL14_A::B_0xA,
            11 => AFSEL14_A::B_0xB,
            12 => AFSEL14_A::B_0xC,
            13 => AFSEL14_A::B_0xD,
            14 => AFSEL14_A::B_0xE,
            15 => AFSEL14_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AFSEL14_A::B_0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AFSEL14_A::B_0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == AFSEL14_A::B_0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == AFSEL14_A::B_0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == AFSEL14_A::B_0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == AFSEL14_A::B_0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == AFSEL14_A::B_0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == AFSEL14_A::B_0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == AFSEL14_A::B_0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == AFSEL14_A::B_0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == AFSEL14_A::B_0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == AFSEL14_A::B_0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == AFSEL14_A::B_0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == AFSEL14_A::B_0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == AFSEL14_A::B_0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == AFSEL14_A::B_0xF
    }
}
#[doc = "Field `AFSEL14` writer - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL14_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL14_A, crate::Safe>;
impl<'a, REG> AFSEL14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14_A::B_0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14_A::B_0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14_A::B_0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14_A::B_0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14_A::B_0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14_A::B_0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14_A::B_0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14_A::B_0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14_A::B_0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14_A::B_0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14_A::B_0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14_A::B_0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14_A::B_0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14_A::B_0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14_A::B_0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14_A::B_0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL15_A {
    #[doc = "0: AF0"]
    B_0x0 = 0,
    #[doc = "1: AF1"]
    B_0x1 = 1,
    #[doc = "2: AF2"]
    B_0x2 = 2,
    #[doc = "3: AF3"]
    B_0x3 = 3,
    #[doc = "4: AF4"]
    B_0x4 = 4,
    #[doc = "5: AF5"]
    B_0x5 = 5,
    #[doc = "6: AF6"]
    B_0x6 = 6,
    #[doc = "7: AF7"]
    B_0x7 = 7,
    #[doc = "8: AF8"]
    B_0x8 = 8,
    #[doc = "9: AF9"]
    B_0x9 = 9,
    #[doc = "10: AF10"]
    B_0xA = 10,
    #[doc = "11: AF11"]
    B_0xB = 11,
    #[doc = "12: AF12"]
    B_0xC = 12,
    #[doc = "13: AF13"]
    B_0xD = 13,
    #[doc = "14: AF14"]
    B_0xE = 14,
    #[doc = "15: AF15"]
    B_0xF = 15,
}
impl From<AFSEL15_A> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL15_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL15_A {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL15_A {}
#[doc = "Field `AFSEL15` reader - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL15_R = crate::FieldReader<AFSEL15_A>;
impl AFSEL15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL15_A {
        match self.bits {
            0 => AFSEL15_A::B_0x0,
            1 => AFSEL15_A::B_0x1,
            2 => AFSEL15_A::B_0x2,
            3 => AFSEL15_A::B_0x3,
            4 => AFSEL15_A::B_0x4,
            5 => AFSEL15_A::B_0x5,
            6 => AFSEL15_A::B_0x6,
            7 => AFSEL15_A::B_0x7,
            8 => AFSEL15_A::B_0x8,
            9 => AFSEL15_A::B_0x9,
            10 => AFSEL15_A::B_0xA,
            11 => AFSEL15_A::B_0xB,
            12 => AFSEL15_A::B_0xC,
            13 => AFSEL15_A::B_0xD,
            14 => AFSEL15_A::B_0xE,
            15 => AFSEL15_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AFSEL15_A::B_0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AFSEL15_A::B_0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == AFSEL15_A::B_0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == AFSEL15_A::B_0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == AFSEL15_A::B_0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == AFSEL15_A::B_0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == AFSEL15_A::B_0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == AFSEL15_A::B_0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == AFSEL15_A::B_0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == AFSEL15_A::B_0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == AFSEL15_A::B_0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == AFSEL15_A::B_0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == AFSEL15_A::B_0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == AFSEL15_A::B_0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == AFSEL15_A::B_0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == AFSEL15_A::B_0xF
    }
}
#[doc = "Field `AFSEL15` writer - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL15_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL15_A, crate::Safe>;
impl<'a, REG> AFSEL15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15_A::B_0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15_A::B_0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15_A::B_0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15_A::B_0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15_A::B_0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15_A::B_0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15_A::B_0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15_A::B_0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15_A::B_0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15_A::B_0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15_A::B_0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15_A::B_0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15_A::B_0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15_A::B_0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15_A::B_0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15_A::B_0xF)
    }
}
impl R {
    #[doc = "Bits 0:3 - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL8(&self) -> AFSEL8_R {
        AFSEL8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL9(&self) -> AFSEL9_R {
        AFSEL9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL10(&self) -> AFSEL10_R {
        AFSEL10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL11(&self) -> AFSEL11_R {
        AFSEL11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL12(&self) -> AFSEL12_R {
        AFSEL12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL13(&self) -> AFSEL13_R {
        AFSEL13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL14(&self) -> AFSEL14_R {
        AFSEL14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL15(&self) -> AFSEL15_R {
        AFSEL15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL8(&mut self) -> AFSEL8_W<'_, AFRH_SPEC> {
        AFSEL8_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL9(&mut self) -> AFSEL9_W<'_, AFRH_SPEC> {
        AFSEL9_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL10(&mut self) -> AFSEL10_W<'_, AFRH_SPEC> {
        AFSEL10_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL11(&mut self) -> AFSEL11_W<'_, AFRH_SPEC> {
        AFSEL11_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL12(&mut self) -> AFSEL12_W<'_, AFRH_SPEC> {
        AFSEL12_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL13(&mut self) -> AFSEL13_W<'_, AFRH_SPEC> {
        AFSEL13_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL14(&mut self) -> AFSEL14_W<'_, AFRH_SPEC> {
        AFSEL14_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x I/O pin y (y = 15 to 8) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL15(&mut self) -> AFSEL15_W<'_, AFRH_SPEC> {
        AFSEL15_W::new(self, 28)
    }
}
#[doc = "GPIO alternate function high register\n\nYou can [`read`](crate::Reg::read) this register and get [`afrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFRH_SPEC;
impl crate::RegisterSpec for AFRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrh::R`](R) reader structure"]
impl crate::Readable for AFRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`afrh::W`](W) writer structure"]
impl crate::Writable for AFRH_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets AFRH to value 0"]
impl crate::Resettable for AFRH_SPEC {}
