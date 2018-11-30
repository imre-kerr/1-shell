// FIXME: Make me pass! Diff budget: 25 lines.

#[derive(Debug)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

use Duration::MilliSeconds;
use Duration::Seconds;
use Duration::Minutes;

impl PartialEq<Duration> for Duration {
    fn eq(&self, rhs: &Duration) -> bool {
        let millis = match *self {
            MilliSeconds(ms) => ms,
            Seconds(s) => s as u64 * 1000,
            Minutes(m) => m as u64 * 60000,
        };
        let rhs_millis = match *rhs {
            MilliSeconds(ms) => ms,
            Seconds(s) => s as u64 * 1000,
            Minutes(m) => m as u64 * 60000,
        };
        millis == rhs_millis
    }
}

fn main() {
    assert_eq!(Seconds(120), Minutes(2));
    assert_eq!(Seconds(420), Minutes(7));
    assert_eq!(MilliSeconds(420000), Minutes(7));
    assert_eq!(MilliSeconds(43000), Seconds(43));
}
