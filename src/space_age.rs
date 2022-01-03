// Given an age in seconds; calculate how old someone would be in given Earth years.

#[derive(Debug)]
pub struct Duration {
    earth_years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self{earth_years: s as f64 / 31557600.0 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / 0.2408467
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / 0.61519726
    }
}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / 1.8808158
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / 1.0
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / 11.862615
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / 29.447498
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / 84.016846
    }
}

impl Planet for Neptune{
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / 167.79132
    }
}
