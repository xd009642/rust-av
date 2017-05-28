use num::rational::Rational32;

pub struct TimeInfo {
    pub pts: Option<i64>,
    pub dts: Option<i64>,
    pub duration: Option<i64>,
    pub timebase: Rational32,
}
