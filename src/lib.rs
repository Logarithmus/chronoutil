extern crate chrono;

mod relative_duration;
mod rrule;

mod leap_year {
    use chrono::Datelike;

    pub fn is_leap_year(year: i32) -> bool {
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }

    pub fn in_leap_year<D: Datelike>(date: D) -> bool {
        is_leap_year(date.year())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::collections::HashSet;

        #[test]
        fn test_leap_year_cases() {
            let _leap_years: Vec<i32> = vec![
                1904, 1908, 1912, 1916, 1920, 1924, 1928, 1932, 1936, 1940, 1944, 1948, 1952, 1956,
                1960, 1964, 1968, 1972, 1976, 1980, 1984, 1988, 1992, 1996, 2000, 2004, 2008, 2012,
                2016, 2020,
            ];
            let leap_years_1900_to_2020: HashSet<i32> = _leap_years.into_iter().collect();

            for year in 1900..2021 {
                assert_eq!(is_leap_year(year), leap_years_1900_to_2020.contains(&year))
            }
        }
    }
}

mod delta {
    use super::leap_year::is_leap_year;
    use chrono::Datelike;

    pub fn shift_months<D: Datelike>(date: D, m: i32) -> D {
        let mut year = date.year() + m / 12;
        let mut month = date.month() as i32 + m % 12;
        let mut day = date.day();

        if month < 1 {
            year -= 1;
            month += 12;
        }

        if day > 28 && month == 2 {
            if is_leap_year(year) {
                day = 29
            } else {
                day = 28
            }
        } else if day == 31 && (month == 4 || month == 6 || month == 9 || month == 11) {
            day = 30;
        };

        // This is slow but guaranteed to succeed (short of interger overflow)
        date.with_day(1)
            .unwrap()
            .with_year(year)
            .unwrap()
            .with_month(month as u32)
            .unwrap()
            .with_day(day)
            .unwrap()
    }

    pub fn shift_years<D: Datelike>(date: D, x: i32) -> D {
        shift_months(date, x * 12)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use chrono::naive::NaiveDate;

        #[test]
        fn test_shift_months() {
            let base = NaiveDate::from_ymd(2020, 1, 31);

            assert_eq!(shift_months(base, 0), NaiveDate::from_ymd(2020, 1, 31));
            assert_eq!(shift_months(base, 1), NaiveDate::from_ymd(2020, 2, 29));
            assert_eq!(shift_months(base, 2), NaiveDate::from_ymd(2020, 3, 31));
            assert_eq!(shift_months(base, 3), NaiveDate::from_ymd(2020, 4, 30));
            assert_eq!(shift_months(base, 4), NaiveDate::from_ymd(2020, 5, 31));
            assert_eq!(shift_months(base, 5), NaiveDate::from_ymd(2020, 6, 30));
            assert_eq!(shift_months(base, 6), NaiveDate::from_ymd(2020, 7, 31));
            assert_eq!(shift_months(base, 7), NaiveDate::from_ymd(2020, 8, 31));
            assert_eq!(shift_months(base, 8), NaiveDate::from_ymd(2020, 9, 30));
            assert_eq!(shift_months(base, 9), NaiveDate::from_ymd(2020, 10, 31));
            assert_eq!(shift_months(base, 10), NaiveDate::from_ymd(2020, 11, 30));
            assert_eq!(shift_months(base, 11), NaiveDate::from_ymd(2020, 12, 31));
            assert_eq!(shift_months(base, 12), NaiveDate::from_ymd(2021, 1, 31));
            assert_eq!(shift_months(base, 13), NaiveDate::from_ymd(2021, 2, 28));

            assert_eq!(shift_months(base, -1), NaiveDate::from_ymd(2019, 12, 31));
            assert_eq!(shift_months(base, -2), NaiveDate::from_ymd(2019, 11, 30));
            assert_eq!(shift_months(base, -3), NaiveDate::from_ymd(2019, 10, 31));
            assert_eq!(shift_months(base, -4), NaiveDate::from_ymd(2019, 9, 30));
            assert_eq!(shift_months(base, -5), NaiveDate::from_ymd(2019, 8, 31));
            assert_eq!(shift_months(base, -6), NaiveDate::from_ymd(2019, 7, 31));
            assert_eq!(shift_months(base, -7), NaiveDate::from_ymd(2019, 6, 30));
            assert_eq!(shift_months(base, -8), NaiveDate::from_ymd(2019, 5, 31));
            assert_eq!(shift_months(base, -9), NaiveDate::from_ymd(2019, 4, 30));
            assert_eq!(shift_months(base, -10), NaiveDate::from_ymd(2019, 3, 31));
            assert_eq!(shift_months(base, -11), NaiveDate::from_ymd(2019, 2, 28));
            assert_eq!(shift_months(base, -12), NaiveDate::from_ymd(2019, 1, 31));
            assert_eq!(shift_months(base, -13), NaiveDate::from_ymd(2018, 12, 31));

            assert_eq!(shift_months(base, 1265), NaiveDate::from_ymd(2125, 6, 30));
        }

        #[test]
        fn test_shift_years() {
            let base = NaiveDate::from_ymd(2020, 2, 29);

            assert_eq!(shift_years(base, 0), NaiveDate::from_ymd(2020, 2, 29));
            assert_eq!(shift_years(base, 1), NaiveDate::from_ymd(2021, 2, 28));
            assert_eq!(shift_years(base, 4), NaiveDate::from_ymd(2024, 2, 29));
            assert_eq!(shift_years(base, 80), NaiveDate::from_ymd(2100, 2, 28));
            assert_eq!(shift_years(base, -1), NaiveDate::from_ymd(2019, 2, 28));
            assert_eq!(shift_years(base, -4), NaiveDate::from_ymd(2016, 2, 29));
            assert_eq!(shift_years(base, -20), NaiveDate::from_ymd(2000, 2, 29));
            assert_eq!(shift_years(base, -120), NaiveDate::from_ymd(1900, 2, 28));
        }
    }
}
