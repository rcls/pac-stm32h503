#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2_SPEC>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2_SPEC>;
#[doc = "7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the LPUART is disabled (UE=0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\] and ADD\\[7:0\\]) respectively.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDM7_A {
    #[doc = "0: 4-bit address detection"]
    B_0x0 = 0,
    #[doc = "1: 7-bit address detection (in 8-bit data mode)"]
    B_0x1 = 1,
}
impl From<ADDM7_A> for bool {
    #[inline(always)]
    fn from(variant: ADDM7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDM7` reader - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the LPUART is disabled (UE=0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\] and ADD\\[7:0\\]) respectively."]
pub type ADDM7_R = crate::BitReader<ADDM7_A>;
impl ADDM7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDM7_A {
        match self.bits {
            false => ADDM7_A::B_0x0,
            true => ADDM7_A::B_0x1,
        }
    }
    #[doc = "4-bit address detection"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ADDM7_A::B_0x0
    }
    #[doc = "7-bit address detection (in 8-bit data mode)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ADDM7_A::B_0x1
    }
}
#[doc = "Field `ADDM7` writer - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the LPUART is disabled (UE=0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\] and ADD\\[7:0\\]) respectively."]
pub type ADDM7_W<'a, REG> = crate::BitWriter<'a, REG, ADDM7_A>;
impl<'a, REG> ADDM7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "4-bit address detection"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADDM7_A::B_0x0)
    }
    #[doc = "7-bit address detection (in 8-bit data mode)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADDM7_A::B_0x1)
    }
}
#[doc = "STOP bits These bits are used for programming the stop bits. This bitfield can only be written when the LPUART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STOP_A {
    #[doc = "0: 1 stop bit"]
    B_0x0 = 0,
    #[doc = "2: 2 stop bits"]
    B_0x2 = 2,
}
impl From<STOP_A> for u8 {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STOP_A {
    type Ux = u8;
}
impl crate::IsEnum for STOP_A {}
#[doc = "Field `STOP` reader - STOP bits These bits are used for programming the stop bits. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type STOP_R = crate::FieldReader<STOP_A>;
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<STOP_A> {
        match self.bits {
            0 => Some(STOP_A::B_0x0),
            2 => Some(STOP_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == STOP_A::B_0x0
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == STOP_A::B_0x2
    }
}
#[doc = "Field `STOP` writer - STOP bits These bits are used for programming the stop bits. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type STOP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STOP_A>;
impl<'a, REG> STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::B_0x0)
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_A::B_0x2)
    }
}
#[doc = "Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWAP_A {
    #[doc = "0: TX/RX pins are used as defined in standard pinout"]
    B_0x0 = 0,
    #[doc = "1: The TX and RX pins functions are swapped. This enables to work in the case of a cross-wired connection to another UART."]
    B_0x1 = 1,
}
impl From<SWAP_A> for bool {
    #[inline(always)]
    fn from(variant: SWAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAP` reader - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type SWAP_R = crate::BitReader<SWAP_A>;
impl SWAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWAP_A {
        match self.bits {
            false => SWAP_A::B_0x0,
            true => SWAP_A::B_0x1,
        }
    }
    #[doc = "TX/RX pins are used as defined in standard pinout"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWAP_A::B_0x0
    }
    #[doc = "The TX and RX pins functions are swapped. This enables to work in the case of a cross-wired connection to another UART."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWAP_A::B_0x1
    }
}
#[doc = "Field `SWAP` writer - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type SWAP_W<'a, REG> = crate::BitWriter<'a, REG, SWAP_A>;
impl<'a, REG> SWAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX/RX pins are used as defined in standard pinout"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWAP_A::B_0x0)
    }
    #[doc = "The TX and RX pins functions are swapped. This enables to work in the case of a cross-wired connection to another UART."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWAP_A::B_0x1)
    }
}
#[doc = "RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the LPUART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXINV_A {
    #[doc = "0: RX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark)"]
    B_0x0 = 0,
    #[doc = "1: RX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle)."]
    B_0x1 = 1,
}
impl From<RXINV_A> for bool {
    #[inline(always)]
    fn from(variant: RXINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXINV` reader - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type RXINV_R = crate::BitReader<RXINV_A>;
impl RXINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXINV_A {
        match self.bits {
            false => RXINV_A::B_0x0,
            true => RXINV_A::B_0x1,
        }
    }
    #[doc = "RX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RXINV_A::B_0x0
    }
    #[doc = "RX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RXINV_A::B_0x1
    }
}
#[doc = "Field `RXINV` writer - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type RXINV_W<'a, REG> = crate::BitWriter<'a, REG, RXINV_A>;
impl<'a, REG> RXINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXINV_A::B_0x0)
    }
    #[doc = "RX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXINV_A::B_0x1)
    }
}
#[doc = "TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the LPUART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXINV_A {
    #[doc = "0: TX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark)"]
    B_0x0 = 0,
    #[doc = "1: TX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle)."]
    B_0x1 = 1,
}
impl From<TXINV_A> for bool {
    #[inline(always)]
    fn from(variant: TXINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXINV` reader - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type TXINV_R = crate::BitReader<TXINV_A>;
impl TXINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXINV_A {
        match self.bits {
            false => TXINV_A::B_0x0,
            true => TXINV_A::B_0x1,
        }
    }
    #[doc = "TX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TXINV_A::B_0x0
    }
    #[doc = "TX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TXINV_A::B_0x1
    }
}
#[doc = "Field `TXINV` writer - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type TXINV_W<'a, REG> = crate::BitWriter<'a, REG, TXINV_A>;
impl<'a, REG> TXINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXINV_A::B_0x0)
    }
    #[doc = "TX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXINV_A::B_0x1)
    }
}
#[doc = "Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATAINV_A {
    #[doc = "0: Logical data from the data register are send/received in positive/direct logic. (1=H, 0=L)"]
    B_0x0 = 0,
    #[doc = "1: Logical data from the data register are send/received in negative/inverse logic. (1=L, 0=H). The parity bit is also inverted."]
    B_0x1 = 1,
}
impl From<DATAINV_A> for bool {
    #[inline(always)]
    fn from(variant: DATAINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAINV` reader - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type DATAINV_R = crate::BitReader<DATAINV_A>;
impl DATAINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATAINV_A {
        match self.bits {
            false => DATAINV_A::B_0x0,
            true => DATAINV_A::B_0x1,
        }
    }
    #[doc = "Logical data from the data register are send/received in positive/direct logic. (1=H, 0=L)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DATAINV_A::B_0x0
    }
    #[doc = "Logical data from the data register are send/received in negative/inverse logic. (1=L, 0=H). The parity bit is also inverted."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DATAINV_A::B_0x1
    }
}
#[doc = "Field `DATAINV` writer - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type DATAINV_W<'a, REG> = crate::BitWriter<'a, REG, DATAINV_A>;
impl<'a, REG> DATAINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Logical data from the data register are send/received in positive/direct logic. (1=H, 0=L)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DATAINV_A::B_0x0)
    }
    #[doc = "Logical data from the data register are send/received in negative/inverse logic. (1=L, 0=H). The parity bit is also inverted."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DATAINV_A::B_0x1)
    }
}
#[doc = "Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSBFIRST_A {
    #[doc = "0: data is transmitted/received with data bit 0 first, following the start bit."]
    B_0x0 = 0,
    #[doc = "1: data is transmitted/received with the MSB (bit 7/8) first, following the start bit."]
    B_0x1 = 1,
}
impl From<MSBFIRST_A> for bool {
    #[inline(always)]
    fn from(variant: MSBFIRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSBFIRST` reader - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type MSBFIRST_R = crate::BitReader<MSBFIRST_A>;
impl MSBFIRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSBFIRST_A {
        match self.bits {
            false => MSBFIRST_A::B_0x0,
            true => MSBFIRST_A::B_0x1,
        }
    }
    #[doc = "data is transmitted/received with data bit 0 first, following the start bit."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MSBFIRST_A::B_0x0
    }
    #[doc = "data is transmitted/received with the MSB (bit 7/8) first, following the start bit."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MSBFIRST_A::B_0x1
    }
}
#[doc = "Field `MSBFIRST` writer - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
pub type MSBFIRST_W<'a, REG> = crate::BitWriter<'a, REG, MSBFIRST_A>;
impl<'a, REG> MSBFIRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "data is transmitted/received with data bit 0 first, following the start bit."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MSBFIRST_A::B_0x0)
    }
    #[doc = "data is transmitted/received with the MSB (bit 7/8) first, following the start bit."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MSBFIRST_A::B_0x1)
    }
}
#[doc = "Field `ADD` reader - Address of the LPUART node These bits give the address of the LPUART node in Mute mode or a character code to be recognized in low-power or Run mode: In Mute mode: they are used in multiprocessor communication to wakeup from Mute mode with 4-bit/7-bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. In 4-bit address mark detection, only ADD\\[3:0\\] bits are used. In low-power mode: they are used for wake up from low-power mode on character match. When WUS\\[1:0\\] is programmed to 0b00 (WUF active on address match), the wakeup from low-power mode is performed when the received character corresponds to the character programmed through ADD\\[6:0\\] or ADD\\[3:0\\] bitfield (depending on ADDM7 bit), and WUF interrupt is enabled by setting WUFIE bit. The MSB of the character sent by transmitter should be equal to 1. In Run mode with Mute mode inactive (for example, end-of-block detection in ModBus protocol): the whole received character (8 bits) is compared to ADD\\[7:0\\] value and CMF flag is set on match. An interrupt is generated if the CMIE bit is set. These bits can only be written when the reception is disabled (RE = 0) or when the USART is disabled (UE = 0)."]
pub type ADD_R = crate::FieldReader;
#[doc = "Field `ADD` writer - Address of the LPUART node These bits give the address of the LPUART node in Mute mode or a character code to be recognized in low-power or Run mode: In Mute mode: they are used in multiprocessor communication to wakeup from Mute mode with 4-bit/7-bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. In 4-bit address mark detection, only ADD\\[3:0\\] bits are used. In low-power mode: they are used for wake up from low-power mode on character match. When WUS\\[1:0\\] is programmed to 0b00 (WUF active on address match), the wakeup from low-power mode is performed when the received character corresponds to the character programmed through ADD\\[6:0\\] or ADD\\[3:0\\] bitfield (depending on ADDM7 bit), and WUF interrupt is enabled by setting WUFIE bit. The MSB of the character sent by transmitter should be equal to 1. In Run mode with Mute mode inactive (for example, end-of-block detection in ModBus protocol): the whole received character (8 bits) is compared to ADD\\[7:0\\] value and CMF flag is set on match. An interrupt is generated if the CMIE bit is set. These bits can only be written when the reception is disabled (RE = 0) or when the USART is disabled (UE = 0)."]
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the LPUART is disabled (UE=0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\] and ADD\\[7:0\\]) respectively."]
    #[inline(always)]
    pub fn ADDM7(&self) -> ADDM7_R {
        ADDM7_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits These bits are used for programming the stop bits. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn STOP(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn SWAP(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn RXINV(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn TXINV(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn DATAINV(&self) -> DATAINV_R {
        DATAINV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn MSBFIRST(&self) -> MSBFIRST_R {
        MSBFIRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Address of the LPUART node These bits give the address of the LPUART node in Mute mode or a character code to be recognized in low-power or Run mode: In Mute mode: they are used in multiprocessor communication to wakeup from Mute mode with 4-bit/7-bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. In 4-bit address mark detection, only ADD\\[3:0\\] bits are used. In low-power mode: they are used for wake up from low-power mode on character match. When WUS\\[1:0\\] is programmed to 0b00 (WUF active on address match), the wakeup from low-power mode is performed when the received character corresponds to the character programmed through ADD\\[6:0\\] or ADD\\[3:0\\] bitfield (depending on ADDM7 bit), and WUF interrupt is enabled by setting WUFIE bit. The MSB of the character sent by transmitter should be equal to 1. In Run mode with Mute mode inactive (for example, end-of-block detection in ModBus protocol): the whole received character (8 bits) is compared to ADD\\[7:0\\] value and CMF flag is set on match. An interrupt is generated if the CMIE bit is set. These bits can only be written when the reception is disabled (RE = 0) or when the USART is disabled (UE = 0)."]
    #[inline(always)]
    pub fn ADD(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the LPUART is disabled (UE=0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\] and ADD\\[7:0\\]) respectively."]
    #[inline(always)]
    pub fn ADDM7(&mut self) -> ADDM7_W<'_, CR2_SPEC> {
        ADDM7_W::new(self, 4)
    }
    #[doc = "Bits 12:13 - STOP bits These bits are used for programming the stop bits. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn STOP(&mut self) -> STOP_W<'_, CR2_SPEC> {
        STOP_W::new(self, 12)
    }
    #[doc = "Bit 15 - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn SWAP(&mut self) -> SWAP_W<'_, CR2_SPEC> {
        SWAP_W::new(self, 15)
    }
    #[doc = "Bit 16 - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn RXINV(&mut self) -> RXINV_W<'_, CR2_SPEC> {
        RXINV_W::new(self, 16)
    }
    #[doc = "Bit 17 - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn TXINV(&mut self) -> TXINV_W<'_, CR2_SPEC> {
        TXINV_W::new(self, 17)
    }
    #[doc = "Bit 18 - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn DATAINV(&mut self) -> DATAINV_W<'_, CR2_SPEC> {
        DATAINV_W::new(self, 18)
    }
    #[doc = "Bit 19 - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn MSBFIRST(&mut self) -> MSBFIRST_W<'_, CR2_SPEC> {
        MSBFIRST_W::new(self, 19)
    }
    #[doc = "Bits 24:31 - Address of the LPUART node These bits give the address of the LPUART node in Mute mode or a character code to be recognized in low-power or Run mode: In Mute mode: they are used in multiprocessor communication to wakeup from Mute mode with 4-bit/7-bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. In 4-bit address mark detection, only ADD\\[3:0\\] bits are used. In low-power mode: they are used for wake up from low-power mode on character match. When WUS\\[1:0\\] is programmed to 0b00 (WUF active on address match), the wakeup from low-power mode is performed when the received character corresponds to the character programmed through ADD\\[6:0\\] or ADD\\[3:0\\] bitfield (depending on ADDM7 bit), and WUF interrupt is enabled by setting WUFIE bit. The MSB of the character sent by transmitter should be equal to 1. In Run mode with Mute mode inactive (for example, end-of-block detection in ModBus protocol): the whole received character (8 bits) is compared to ADD\\[7:0\\] value and CMF flag is set on match. An interrupt is generated if the CMIE bit is set. These bits can only be written when the reception is disabled (RE = 0) or when the USART is disabled (UE = 0)."]
    #[inline(always)]
    pub fn ADD(&mut self) -> ADD_W<'_, CR2_SPEC> {
        ADD_W::new(self, 24)
    }
}
#[doc = "LPUART control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {}
