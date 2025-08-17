#[doc = "Register `PLL1DIVR` reader"]
pub type R = crate::R<PLL1DIVR_SPEC>;
#[doc = "Register `PLL1DIVR` writer"]
pub type W = crate::W<PLL1DIVR_SPEC>;
#[doc = "Multiplication factor for PLL1VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved\n\nValue on reset: 128"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PLL1N_A {
    #[doc = "3: PLL1N = 4"]
    B_0x3 = 3,
    #[doc = "4: PLL1N = 5"]
    B_0x4 = 4,
    #[doc = "5: PLL1N = 6"]
    B_0x5 = 5,
    #[doc = "128: PLL1N = 129 (default after reset)"]
    B_0x80 = 128,
    #[doc = "511: PLL1N = 512"]
    B_0x1FF = 511,
}
impl From<PLL1N_A> for u16 {
    #[inline(always)]
    fn from(variant: PLL1N_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL1N_A {
    type Ux = u16;
}
impl crate::IsEnum for PLL1N_A {}
#[doc = "Field `PLL1N` reader - Multiplication factor for PLL1VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved"]
pub type PLL1N_R = crate::FieldReader<PLL1N_A>;
impl PLL1N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLL1N_A> {
        match self.bits {
            3 => Some(PLL1N_A::B_0x3),
            4 => Some(PLL1N_A::B_0x4),
            5 => Some(PLL1N_A::B_0x5),
            128 => Some(PLL1N_A::B_0x80),
            511 => Some(PLL1N_A::B_0x1FF),
            _ => None,
        }
    }
    #[doc = "PLL1N = 4"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PLL1N_A::B_0x3
    }
    #[doc = "PLL1N = 5"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == PLL1N_A::B_0x4
    }
    #[doc = "PLL1N = 6"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == PLL1N_A::B_0x5
    }
    #[doc = "PLL1N = 129 (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x80(&self) -> bool {
        *self == PLL1N_A::B_0x80
    }
    #[doc = "PLL1N = 512"]
    #[inline(always)]
    pub fn is_B_0x1FF(&self) -> bool {
        *self == PLL1N_A::B_0x1FF
    }
}
#[doc = "Field `PLL1N` writer - Multiplication factor for PLL1VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved"]
pub type PLL1N_W<'a, REG> = crate::FieldWriter<'a, REG, 9, PLL1N_A>;
impl<'a, REG> PLL1N_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "PLL1N = 4"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1N_A::B_0x3)
    }
    #[doc = "PLL1N = 5"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1N_A::B_0x4)
    }
    #[doc = "PLL1N = 6"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1N_A::B_0x5)
    }
    #[doc = "PLL1N = 129 (default after reset)"]
    #[inline(always)]
    pub fn B_0x80(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1N_A::B_0x80)
    }
    #[doc = "PLL1N = 512"]
    #[inline(always)]
    pub fn B_0x1FF(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1N_A::B_0x1FF)
    }
}
#[doc = "PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ...\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL1P_A {
    #[doc = "0: pll1_p_ck = vco1_ck"]
    B_0x0 = 0,
    #[doc = "1: pll1_p_ck = vco1_ck / 2 (default after reset)"]
    B_0x1 = 1,
    #[doc = "2: Not allowed"]
    B_0x2 = 2,
    #[doc = "3: pll1_p_ck = vco1_ck / 4"]
    B_0x3 = 3,
    #[doc = "127: pll1_p_ck = vco1_ck / 128"]
    B_0x7F = 127,
}
impl From<PLL1P_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL1P_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL1P_A {
    type Ux = u8;
}
impl crate::IsEnum for PLL1P_A {}
#[doc = "Field `PLL1P` reader - PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ..."]
pub type PLL1P_R = crate::FieldReader<PLL1P_A>;
impl PLL1P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLL1P_A> {
        match self.bits {
            0 => Some(PLL1P_A::B_0x0),
            1 => Some(PLL1P_A::B_0x1),
            2 => Some(PLL1P_A::B_0x2),
            3 => Some(PLL1P_A::B_0x3),
            127 => Some(PLL1P_A::B_0x7F),
            _ => None,
        }
    }
    #[doc = "pll1_p_ck = vco1_ck"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL1P_A::B_0x0
    }
    #[doc = "pll1_p_ck = vco1_ck / 2 (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL1P_A::B_0x1
    }
    #[doc = "Not allowed"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PLL1P_A::B_0x2
    }
    #[doc = "pll1_p_ck = vco1_ck / 4"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PLL1P_A::B_0x3
    }
    #[doc = "pll1_p_ck = vco1_ck / 128"]
    #[inline(always)]
    pub fn is_B_0x7F(&self) -> bool {
        *self == PLL1P_A::B_0x7F
    }
}
#[doc = "Field `PLL1P` writer - PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ..."]
pub type PLL1P_W<'a, REG> = crate::FieldWriter<'a, REG, 7, PLL1P_A>;
impl<'a, REG> PLL1P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pll1_p_ck = vco1_ck"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1P_A::B_0x0)
    }
    #[doc = "pll1_p_ck = vco1_ck / 2 (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1P_A::B_0x1)
    }
    #[doc = "Not allowed"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1P_A::B_0x2)
    }
    #[doc = "pll1_p_ck = vco1_ck / 4"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1P_A::B_0x3)
    }
    #[doc = "pll1_p_ck = vco1_ck / 128"]
    #[inline(always)]
    pub fn B_0x7F(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1P_A::B_0x7F)
    }
}
#[doc = "PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL1Q_A {
    #[doc = "0: pll1_q_ck = vco1_ck"]
    B_0x0 = 0,
    #[doc = "1: pll1_q_ck = vco1_ck / 2 (default after reset)"]
    B_0x1 = 1,
    #[doc = "2: pll1_q_ck = vco1_ck / 3"]
    B_0x2 = 2,
    #[doc = "3: pll1_q_ck = vco1_ck / 4"]
    B_0x3 = 3,
    #[doc = "127: pll1_q_ck = vco1_ck / 128"]
    B_0x7F = 127,
}
impl From<PLL1Q_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL1Q_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL1Q_A {
    type Ux = u8;
}
impl crate::IsEnum for PLL1Q_A {}
#[doc = "Field `PLL1Q` reader - PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
pub type PLL1Q_R = crate::FieldReader<PLL1Q_A>;
impl PLL1Q_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLL1Q_A> {
        match self.bits {
            0 => Some(PLL1Q_A::B_0x0),
            1 => Some(PLL1Q_A::B_0x1),
            2 => Some(PLL1Q_A::B_0x2),
            3 => Some(PLL1Q_A::B_0x3),
            127 => Some(PLL1Q_A::B_0x7F),
            _ => None,
        }
    }
    #[doc = "pll1_q_ck = vco1_ck"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL1Q_A::B_0x0
    }
    #[doc = "pll1_q_ck = vco1_ck / 2 (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL1Q_A::B_0x1
    }
    #[doc = "pll1_q_ck = vco1_ck / 3"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PLL1Q_A::B_0x2
    }
    #[doc = "pll1_q_ck = vco1_ck / 4"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PLL1Q_A::B_0x3
    }
    #[doc = "pll1_q_ck = vco1_ck / 128"]
    #[inline(always)]
    pub fn is_B_0x7F(&self) -> bool {
        *self == PLL1Q_A::B_0x7F
    }
}
#[doc = "Field `PLL1Q` writer - PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
pub type PLL1Q_W<'a, REG> = crate::FieldWriter<'a, REG, 7, PLL1Q_A>;
impl<'a, REG> PLL1Q_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pll1_q_ck = vco1_ck"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1Q_A::B_0x0)
    }
    #[doc = "pll1_q_ck = vco1_ck / 2 (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1Q_A::B_0x1)
    }
    #[doc = "pll1_q_ck = vco1_ck / 3"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1Q_A::B_0x2)
    }
    #[doc = "pll1_q_ck = vco1_ck / 4"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1Q_A::B_0x3)
    }
    #[doc = "pll1_q_ck = vco1_ck / 128"]
    #[inline(always)]
    pub fn B_0x7F(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1Q_A::B_0x7F)
    }
}
#[doc = "PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL1R_A {
    #[doc = "0: pll1_r_ck = Not allowed"]
    B_0x0 = 0,
    #[doc = "1: pll1_r_ck = vco1_ck / 2 (default after reset)"]
    B_0x1 = 1,
    #[doc = "2: pll1_r_ck = vco1_ck / 3"]
    B_0x2 = 2,
    #[doc = "3: pll1_r_ck = vco1_ck / 4"]
    B_0x3 = 3,
    #[doc = "127: pll1_r_ck = vco1_ck / 128"]
    B_0x7F = 127,
}
impl From<PLL1R_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL1R_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL1R_A {
    type Ux = u8;
}
impl crate::IsEnum for PLL1R_A {}
#[doc = "Field `PLL1R` reader - PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
pub type PLL1R_R = crate::FieldReader<PLL1R_A>;
impl PLL1R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLL1R_A> {
        match self.bits {
            0 => Some(PLL1R_A::B_0x0),
            1 => Some(PLL1R_A::B_0x1),
            2 => Some(PLL1R_A::B_0x2),
            3 => Some(PLL1R_A::B_0x3),
            127 => Some(PLL1R_A::B_0x7F),
            _ => None,
        }
    }
    #[doc = "pll1_r_ck = Not allowed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL1R_A::B_0x0
    }
    #[doc = "pll1_r_ck = vco1_ck / 2 (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL1R_A::B_0x1
    }
    #[doc = "pll1_r_ck = vco1_ck / 3"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PLL1R_A::B_0x2
    }
    #[doc = "pll1_r_ck = vco1_ck / 4"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PLL1R_A::B_0x3
    }
    #[doc = "pll1_r_ck = vco1_ck / 128"]
    #[inline(always)]
    pub fn is_B_0x7F(&self) -> bool {
        *self == PLL1R_A::B_0x7F
    }
}
#[doc = "Field `PLL1R` writer - PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
pub type PLL1R_W<'a, REG> = crate::FieldWriter<'a, REG, 7, PLL1R_A>;
impl<'a, REG> PLL1R_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pll1_r_ck = Not allowed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1R_A::B_0x0)
    }
    #[doc = "pll1_r_ck = vco1_ck / 2 (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1R_A::B_0x1)
    }
    #[doc = "pll1_r_ck = vco1_ck / 3"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1R_A::B_0x2)
    }
    #[doc = "pll1_r_ck = vco1_ck / 4"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1R_A::B_0x3)
    }
    #[doc = "pll1_r_ck = vco1_ck / 128"]
    #[inline(always)]
    pub fn B_0x7F(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1R_A::B_0x7F)
    }
}
impl R {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved"]
    #[inline(always)]
    pub fn PLL1N(&self) -> PLL1N_R {
        PLL1N_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ..."]
    #[inline(always)]
    pub fn PLL1P(&self) -> PLL1P_R {
        PLL1P_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
    #[inline(always)]
    pub fn PLL1Q(&self) -> PLL1Q_R {
        PLL1Q_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
    #[inline(always)]
    pub fn PLL1R(&self) -> PLL1R_R {
        PLL1R_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved"]
    #[inline(always)]
    pub fn PLL1N(&mut self) -> PLL1N_W<'_, PLL1DIVR_SPEC> {
        PLL1N_W::new(self, 0)
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ..."]
    #[inline(always)]
    pub fn PLL1P(&mut self) -> PLL1P_W<'_, PLL1DIVR_SPEC> {
        PLL1P_W::new(self, 9)
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
    #[inline(always)]
    pub fn PLL1Q(&mut self) -> PLL1Q_W<'_, PLL1DIVR_SPEC> {
        PLL1Q_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
    #[inline(always)]
    pub fn PLL1R(&mut self) -> PLL1R_W<'_, PLL1DIVR_SPEC> {
        PLL1R_W::new(self, 24)
    }
}
#[doc = "RCC PLL1 dividers register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1divr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1divr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL1DIVR_SPEC;
impl crate::RegisterSpec for PLL1DIVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll1divr::R`](R) reader structure"]
impl crate::Readable for PLL1DIVR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll1divr::W`](W) writer structure"]
impl crate::Writable for PLL1DIVR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PLL1DIVR to value 0x0101_0280"]
impl crate::Resettable for PLL1DIVR_SPEC {
    const RESET_VALUE: u32 = 0x0101_0280;
}
