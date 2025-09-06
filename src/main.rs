#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    // 하드웨어 초기화
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // 클록 설정
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // GPIO 초기화
    let mut gpioc = dp.GPIOC.split();

    // PC13을 출력으로 설정 (많은 STM32F103 보드에서 내장 LED가 PC13에 연결됨)
    // StorM32 보드의 실제 LED 핀에 맞게 수정이 필요할 수 있습니다
    let mut led = gpioc.pc12.into_push_pull_output(&mut gpioc.crh);

    // 또는 다른 핀들을 시도해볼 수 있습니다:
    // let mut led = gpioc.pc14.into_push_pull_output(&mut gpioc.crh);
    // let mut led = gpioc.pc15.into_push_pull_output(&mut gpioc.crh);

    // 타이머 초기화 (1Hz = 1초마다 토글)
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(1.Hz()).unwrap();

    // 메인 루프
    loop {
        // LED 켜기
        led.set_low(); // 대부분의 보드에서 LOW가 LED ON

        // 0.5초 대기
        timer.start(2.Hz()).unwrap();
        nb::block!(timer.wait()).unwrap();

        // LED 끄기
        led.set_high(); // HIGH가 LED OFF

        // 0.5초 대기
        nb::block!(timer.wait()).unwrap();
    }
}

// 대안 1: 다른 핀을 사용하는 경우
/*
fn init_led_alternative_pins(gpiob: &mut stm32f1xx_hal::gpio::gpiob::Parts)
    -> stm32f1xx_hal::gpio::gpiob::PB0<Output<PushPull>> {
    // PB0을 LED로 사용하는 경우
    gpiob.pb0.into_push_pull_output(&mut gpiob.crl)
}
*/

// 대안 2: 여러 LED를 사용하는 경우
/*
struct LedArray {
    led1: PC13<Output<PushPull>>,
    led2: PC14<Output<PushPull>>,
}

impl LedArray {
    fn new(gpioc: &mut stm32f1xx_hal::gpio::gpioc::Parts) -> Self {
        Self {
            led1: gpioc.pc13.into_push_pull_output(&mut gpioc.crh),
            led2: gpioc.pc14.into_push_pull_output(&mut gpioc.crh),
        }
    }

    fn toggle_all(&mut self) {
        self.led1.toggle();
        self.led2.toggle();
    }
}
*/
