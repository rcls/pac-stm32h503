#[doc = "Register `AFRL` reader"]
pub type R = crate::R<AFRL_SPEC>;
#[doc = "Register `AFRL` writer"]
pub type W = crate::W<AFRL_SPEC>;
#[doc = "Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL0_A {
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
impl From<AFSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL0_A {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL0_A {}
#[doc = "Field `AFSEL0` reader - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL0_R = crate::FieldReader<AFSEL0_A>;
impl AFSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL0_A {
        match self.bits {
            0 => AFSEL0_A::B_0x0,
            1 => AFSEL0_A::B_0x1,
            2 => AFSEL0_A::B_0x2,
            3 => AFSEL0_A::B_0x3,
            4 => AFSEL0_A::B_0x4,
            5 => AFSEL0_A::B_0x5,
            6 => AFSEL0_A::B_0x6,
            7 => AFSEL0_A::B_0x7,
            8 => AFSEL0_A::B_0x8,
            9 => AFSEL0_A::B_0x9,
            10 => AFSEL0_A::B_0xA,
            11 => AFSEL0_A::B_0xB,
            12 => AFSEL0_A::B_0xC,
            13 => AFSEL0_A::B_0xD,
            14 => AFSEL0_A::B_0xE,
            15 => AFSEL0_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AFSEL0_A::B_0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AFSEL0_A::B_0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == AFSEL0_A::B_0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == AFSEL0_A::B_0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == AFSEL0_A::B_0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == AFSEL0_A::B_0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == AFSEL0_A::B_0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == AFSEL0_A::B_0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == AFSEL0_A::B_0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == AFSEL0_A::B_0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == AFSEL0_A::B_0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == AFSEL0_A::B_0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == AFSEL0_A::B_0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == AFSEL0_A::B_0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == AFSEL0_A::B_0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == AFSEL0_A::B_0xF
    }
}
#[doc = "Field `AFSEL0` writer - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL0_A, crate::Safe>;
impl<'a, REG> AFSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::B_0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::B_0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::B_0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::B_0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::B_0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::B_0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::B_0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::B_0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::B_0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::B_0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::B_0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::B_0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::B_0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::B_0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::B_0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::B_0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL1_A {
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
impl From<AFSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL1_A {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL1_A {}
#[doc = "Field `AFSEL1` reader - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL1_R = crate::FieldReader<AFSEL1_A>;
impl AFSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL1_A {
        match self.bits {
            0 => AFSEL1_A::B_0x0,
            1 => AFSEL1_A::B_0x1,
            2 => AFSEL1_A::B_0x2,
            3 => AFSEL1_A::B_0x3,
            4 => AFSEL1_A::B_0x4,
            5 => AFSEL1_A::B_0x5,
            6 => AFSEL1_A::B_0x6,
            7 => AFSEL1_A::B_0x7,
            8 => AFSEL1_A::B_0x8,
            9 => AFSEL1_A::B_0x9,
            10 => AFSEL1_A::B_0xA,
            11 => AFSEL1_A::B_0xB,
            12 => AFSEL1_A::B_0xC,
            13 => AFSEL1_A::B_0xD,
            14 => AFSEL1_A::B_0xE,
            15 => AFSEL1_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AFSEL1_A::B_0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AFSEL1_A::B_0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == AFSEL1_A::B_0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == AFSEL1_A::B_0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == AFSEL1_A::B_0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == AFSEL1_A::B_0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == AFSEL1_A::B_0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == AFSEL1_A::B_0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == AFSEL1_A::B_0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == AFSEL1_A::B_0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == AFSEL1_A::B_0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == AFSEL1_A::B_0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == AFSEL1_A::B_0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == AFSEL1_A::B_0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == AFSEL1_A::B_0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == AFSEL1_A::B_0xF
    }
}
#[doc = "Field `AFSEL1` writer - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL1_A, crate::Safe>;
impl<'a, REG> AFSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1_A::B_0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1_A::B_0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1_A::B_0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1_A::B_0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1_A::B_0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1_A::B_0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1_A::B_0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1_A::B_0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1_A::B_0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1_A::B_0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1_A::B_0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1_A::B_0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1_A::B_0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1_A::B_0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1_A::B_0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1_A::B_0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL2_A {
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
impl From<AFSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL2_A {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL2_A {}
#[doc = "Field `AFSEL2` reader - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL2_R = crate::FieldReader<AFSEL2_A>;
impl AFSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL2_A {
        match self.bits {
            0 => AFSEL2_A::B_0x0,
            1 => AFSEL2_A::B_0x1,
            2 => AFSEL2_A::B_0x2,
            3 => AFSEL2_A::B_0x3,
            4 => AFSEL2_A::B_0x4,
            5 => AFSEL2_A::B_0x5,
            6 => AFSEL2_A::B_0x6,
            7 => AFSEL2_A::B_0x7,
            8 => AFSEL2_A::B_0x8,
            9 => AFSEL2_A::B_0x9,
            10 => AFSEL2_A::B_0xA,
            11 => AFSEL2_A::B_0xB,
            12 => AFSEL2_A::B_0xC,
            13 => AFSEL2_A::B_0xD,
            14 => AFSEL2_A::B_0xE,
            15 => AFSEL2_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AFSEL2_A::B_0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AFSEL2_A::B_0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == AFSEL2_A::B_0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == AFSEL2_A::B_0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == AFSEL2_A::B_0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == AFSEL2_A::B_0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == AFSEL2_A::B_0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == AFSEL2_A::B_0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == AFSEL2_A::B_0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == AFSEL2_A::B_0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == AFSEL2_A::B_0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == AFSEL2_A::B_0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == AFSEL2_A::B_0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == AFSEL2_A::B_0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == AFSEL2_A::B_0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == AFSEL2_A::B_0xF
    }
}
#[doc = "Field `AFSEL2` writer - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL2_A, crate::Safe>;
impl<'a, REG> AFSEL2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2_A::B_0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2_A::B_0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2_A::B_0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2_A::B_0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2_A::B_0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2_A::B_0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2_A::B_0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2_A::B_0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2_A::B_0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2_A::B_0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2_A::B_0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2_A::B_0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2_A::B_0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2_A::B_0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2_A::B_0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2_A::B_0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL3_A {
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
impl From<AFSEL3_A> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL3_A {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL3_A {}
#[doc = "Field `AFSEL3` reader - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL3_R = crate::FieldReader<AFSEL3_A>;
impl AFSEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL3_A {
        match self.bits {
            0 => AFSEL3_A::B_0x0,
            1 => AFSEL3_A::B_0x1,
            2 => AFSEL3_A::B_0x2,
            3 => AFSEL3_A::B_0x3,
            4 => AFSEL3_A::B_0x4,
            5 => AFSEL3_A::B_0x5,
            6 => AFSEL3_A::B_0x6,
            7 => AFSEL3_A::B_0x7,
            8 => AFSEL3_A::B_0x8,
            9 => AFSEL3_A::B_0x9,
            10 => AFSEL3_A::B_0xA,
            11 => AFSEL3_A::B_0xB,
            12 => AFSEL3_A::B_0xC,
            13 => AFSEL3_A::B_0xD,
            14 => AFSEL3_A::B_0xE,
            15 => AFSEL3_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AFSEL3_A::B_0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AFSEL3_A::B_0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == AFSEL3_A::B_0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == AFSEL3_A::B_0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == AFSEL3_A::B_0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == AFSEL3_A::B_0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == AFSEL3_A::B_0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == AFSEL3_A::B_0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == AFSEL3_A::B_0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == AFSEL3_A::B_0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == AFSEL3_A::B_0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == AFSEL3_A::B_0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == AFSEL3_A::B_0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == AFSEL3_A::B_0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == AFSEL3_A::B_0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == AFSEL3_A::B_0xF
    }
}
#[doc = "Field `AFSEL3` writer - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL3_A, crate::Safe>;
impl<'a, REG> AFSEL3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3_A::B_0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3_A::B_0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3_A::B_0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3_A::B_0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3_A::B_0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3_A::B_0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3_A::B_0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3_A::B_0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3_A::B_0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3_A::B_0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3_A::B_0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3_A::B_0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3_A::B_0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3_A::B_0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3_A::B_0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3_A::B_0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL4_A {
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
impl From<AFSEL4_A> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL4_A {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL4_A {}
#[doc = "Field `AFSEL4` reader - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL4_R = crate::FieldReader<AFSEL4_A>;
impl AFSEL4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL4_A {
        match self.bits {
            0 => AFSEL4_A::B_0x0,
            1 => AFSEL4_A::B_0x1,
            2 => AFSEL4_A::B_0x2,
            3 => AFSEL4_A::B_0x3,
            4 => AFSEL4_A::B_0x4,
            5 => AFSEL4_A::B_0x5,
            6 => AFSEL4_A::B_0x6,
            7 => AFSEL4_A::B_0x7,
            8 => AFSEL4_A::B_0x8,
            9 => AFSEL4_A::B_0x9,
            10 => AFSEL4_A::B_0xA,
            11 => AFSEL4_A::B_0xB,
            12 => AFSEL4_A::B_0xC,
            13 => AFSEL4_A::B_0xD,
            14 => AFSEL4_A::B_0xE,
            15 => AFSEL4_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AFSEL4_A::B_0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AFSEL4_A::B_0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == AFSEL4_A::B_0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == AFSEL4_A::B_0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == AFSEL4_A::B_0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == AFSEL4_A::B_0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == AFSEL4_A::B_0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == AFSEL4_A::B_0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == AFSEL4_A::B_0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == AFSEL4_A::B_0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == AFSEL4_A::B_0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == AFSEL4_A::B_0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == AFSEL4_A::B_0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == AFSEL4_A::B_0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == AFSEL4_A::B_0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == AFSEL4_A::B_0xF
    }
}
#[doc = "Field `AFSEL4` writer - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL4_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL4_A, crate::Safe>;
impl<'a, REG> AFSEL4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4_A::B_0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4_A::B_0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4_A::B_0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4_A::B_0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4_A::B_0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4_A::B_0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4_A::B_0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4_A::B_0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4_A::B_0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4_A::B_0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4_A::B_0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4_A::B_0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4_A::B_0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4_A::B_0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4_A::B_0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4_A::B_0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL5_A {
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
impl From<AFSEL5_A> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL5_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL5_A {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL5_A {}
#[doc = "Field `AFSEL5` reader - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL5_R = crate::FieldReader<AFSEL5_A>;
impl AFSEL5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL5_A {
        match self.bits {
            0 => AFSEL5_A::B_0x0,
            1 => AFSEL5_A::B_0x1,
            2 => AFSEL5_A::B_0x2,
            3 => AFSEL5_A::B_0x3,
            4 => AFSEL5_A::B_0x4,
            5 => AFSEL5_A::B_0x5,
            6 => AFSEL5_A::B_0x6,
            7 => AFSEL5_A::B_0x7,
            8 => AFSEL5_A::B_0x8,
            9 => AFSEL5_A::B_0x9,
            10 => AFSEL5_A::B_0xA,
            11 => AFSEL5_A::B_0xB,
            12 => AFSEL5_A::B_0xC,
            13 => AFSEL5_A::B_0xD,
            14 => AFSEL5_A::B_0xE,
            15 => AFSEL5_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AFSEL5_A::B_0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AFSEL5_A::B_0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == AFSEL5_A::B_0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == AFSEL5_A::B_0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == AFSEL5_A::B_0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == AFSEL5_A::B_0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == AFSEL5_A::B_0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == AFSEL5_A::B_0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == AFSEL5_A::B_0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == AFSEL5_A::B_0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == AFSEL5_A::B_0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == AFSEL5_A::B_0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == AFSEL5_A::B_0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == AFSEL5_A::B_0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == AFSEL5_A::B_0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == AFSEL5_A::B_0xF
    }
}
#[doc = "Field `AFSEL5` writer - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL5_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL5_A, crate::Safe>;
impl<'a, REG> AFSEL5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5_A::B_0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5_A::B_0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5_A::B_0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5_A::B_0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5_A::B_0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5_A::B_0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5_A::B_0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5_A::B_0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5_A::B_0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5_A::B_0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5_A::B_0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5_A::B_0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5_A::B_0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5_A::B_0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5_A::B_0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5_A::B_0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL6_A {
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
impl From<AFSEL6_A> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL6_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL6_A {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL6_A {}
#[doc = "Field `AFSEL6` reader - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL6_R = crate::FieldReader<AFSEL6_A>;
impl AFSEL6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL6_A {
        match self.bits {
            0 => AFSEL6_A::B_0x0,
            1 => AFSEL6_A::B_0x1,
            2 => AFSEL6_A::B_0x2,
            3 => AFSEL6_A::B_0x3,
            4 => AFSEL6_A::B_0x4,
            5 => AFSEL6_A::B_0x5,
            6 => AFSEL6_A::B_0x6,
            7 => AFSEL6_A::B_0x7,
            8 => AFSEL6_A::B_0x8,
            9 => AFSEL6_A::B_0x9,
            10 => AFSEL6_A::B_0xA,
            11 => AFSEL6_A::B_0xB,
            12 => AFSEL6_A::B_0xC,
            13 => AFSEL6_A::B_0xD,
            14 => AFSEL6_A::B_0xE,
            15 => AFSEL6_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AFSEL6_A::B_0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AFSEL6_A::B_0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == AFSEL6_A::B_0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == AFSEL6_A::B_0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == AFSEL6_A::B_0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == AFSEL6_A::B_0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == AFSEL6_A::B_0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == AFSEL6_A::B_0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == AFSEL6_A::B_0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == AFSEL6_A::B_0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == AFSEL6_A::B_0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == AFSEL6_A::B_0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == AFSEL6_A::B_0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == AFSEL6_A::B_0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == AFSEL6_A::B_0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == AFSEL6_A::B_0xF
    }
}
#[doc = "Field `AFSEL6` writer - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL6_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL6_A, crate::Safe>;
impl<'a, REG> AFSEL6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6_A::B_0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6_A::B_0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6_A::B_0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6_A::B_0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6_A::B_0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6_A::B_0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6_A::B_0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6_A::B_0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6_A::B_0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6_A::B_0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6_A::B_0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6_A::B_0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6_A::B_0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6_A::B_0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6_A::B_0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6_A::B_0xF)
    }
}
#[doc = "Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL7_A {
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
impl From<AFSEL7_A> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL7_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL7_A {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL7_A {}
#[doc = "Field `AFSEL7` reader - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL7_R = crate::FieldReader<AFSEL7_A>;
impl AFSEL7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL7_A {
        match self.bits {
            0 => AFSEL7_A::B_0x0,
            1 => AFSEL7_A::B_0x1,
            2 => AFSEL7_A::B_0x2,
            3 => AFSEL7_A::B_0x3,
            4 => AFSEL7_A::B_0x4,
            5 => AFSEL7_A::B_0x5,
            6 => AFSEL7_A::B_0x6,
            7 => AFSEL7_A::B_0x7,
            8 => AFSEL7_A::B_0x8,
            9 => AFSEL7_A::B_0x9,
            10 => AFSEL7_A::B_0xA,
            11 => AFSEL7_A::B_0xB,
            12 => AFSEL7_A::B_0xC,
            13 => AFSEL7_A::B_0xD,
            14 => AFSEL7_A::B_0xE,
            15 => AFSEL7_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AFSEL7_A::B_0x0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AFSEL7_A::B_0x1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == AFSEL7_A::B_0x2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == AFSEL7_A::B_0x3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == AFSEL7_A::B_0x4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == AFSEL7_A::B_0x5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == AFSEL7_A::B_0x6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == AFSEL7_A::B_0x7
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == AFSEL7_A::B_0x8
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == AFSEL7_A::B_0x9
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == AFSEL7_A::B_0xA
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == AFSEL7_A::B_0xB
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == AFSEL7_A::B_0xC
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == AFSEL7_A::B_0xD
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == AFSEL7_A::B_0xE
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == AFSEL7_A::B_0xF
    }
}
#[doc = "Field `AFSEL7` writer - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type AFSEL7_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL7_A, crate::Safe>;
impl<'a, REG> AFSEL7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7_A::B_0x0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7_A::B_0x1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7_A::B_0x2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7_A::B_0x3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7_A::B_0x4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7_A::B_0x5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7_A::B_0x6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7_A::B_0x7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7_A::B_0x8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7_A::B_0x9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7_A::B_0xA)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7_A::B_0xB)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7_A::B_0xC)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7_A::B_0xD)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7_A::B_0xE)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7_A::B_0xF)
    }
}
impl R {
    #[doc = "Bits 0:3 - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL0(&self) -> AFSEL0_R {
        AFSEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL1(&self) -> AFSEL1_R {
        AFSEL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL2(&self) -> AFSEL2_R {
        AFSEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL3(&self) -> AFSEL3_R {
        AFSEL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL4(&self) -> AFSEL4_R {
        AFSEL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL5(&self) -> AFSEL5_R {
        AFSEL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL6(&self) -> AFSEL6_R {
        AFSEL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL7(&self) -> AFSEL7_R {
        AFSEL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL0(&mut self) -> AFSEL0_W<'_, AFRL_SPEC> {
        AFSEL0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL1(&mut self) -> AFSEL1_W<'_, AFRL_SPEC> {
        AFSEL1_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL2(&mut self) -> AFSEL2_W<'_, AFRL_SPEC> {
        AFSEL2_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL3(&mut self) -> AFSEL3_W<'_, AFRL_SPEC> {
        AFSEL3_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL4(&mut self) -> AFSEL4_W<'_, AFRL_SPEC> {
        AFSEL4_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL5(&mut self) -> AFSEL5_W<'_, AFRL_SPEC> {
        AFSEL5_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL6(&mut self) -> AFSEL6_W<'_, AFRL_SPEC> {
        AFSEL6_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x I/O pin y (y = 7 to 0) These bits are written by software to configure alternate function I/Os. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn AFSEL7(&mut self) -> AFSEL7_W<'_, AFRL_SPEC> {
        AFSEL7_W::new(self, 28)
    }
}
#[doc = "GPIO alternate function low register\n\nYou can [`read`](crate::Reg::read) this register and get [`afrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFRL_SPEC;
impl crate::RegisterSpec for AFRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrl::R`](R) reader structure"]
impl crate::Readable for AFRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`afrl::W`](W) writer structure"]
impl crate::Writable for AFRL_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets AFRL to value 0"]
impl crate::Resettable for AFRL_SPEC {}
