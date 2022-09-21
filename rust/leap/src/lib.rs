pub fn is_leap_year(year: u64) -> bool {
    let resultado = year / 4;
    let resultado2 = year / 100;
    let resultado3 = year / 400;
    resultado * 4 == year && resultado2 * 100 != year || resultado3 * 400 == year
}
