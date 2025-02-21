use chrono::{Datelike, Duration, Months, NaiveDate, TimeZone};

enum PersonType {
    Male,
    Female50,
    Female55,
}

struct RetirementCalculator {
    birth_year: i32,
    birth_month: u32,
    person_type: PersonType,
}

impl RetirementCalculator {
    pub fn calculate(&self) -> (String, f64, u32) {
        let (original_age, interval, target_age) = match self.person_type {
            PersonType::Male => (60, 4, 63),
            PersonType::Female50 => (50, 2, 55),
            PersonType::Female55 => (55, 4, 58),
        };

        let original_date = NaiveDate::from_ymd_opt(
            self.birth_year+original_age, 
            self.birth_month, 1).unwrap();

        let start_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();

        let months_diff = if original_date >= start_date {
            (original_date.year() - start_date.year()) * 12
                + (original_date.month() as i32 - start_date.month() as i32)
        } else {
            0
        };

        let possible_delay;
        if months_diff % interval == 0 {
            possible_delay = (months_diff as f32 / interval as f32).floor() as u32;
        } else {
            possible_delay = (months_diff as f32 / interval as f32).floor() as u32 + 1;
        }
        let max_delay = (target_age - original_age) * 12;
        let actual_delay = possible_delay.min(max_delay as u32);

        let retirement_date = original_date
            .checked_add_months(Months::new(actual_delay))
            .unwrap();

        let retirement_age = Self::calculate_age(
            NaiveDate::from_ymd_opt(self.birth_year, self.birth_month, 1).unwrap(),
            retirement_date,
        );

        (
            format!("{}-{:02}", retirement_date.year(), retirement_date.month()),
            retirement_age,
            actual_delay,
        )
    }

    fn calculate_age(birth_date: NaiveDate, retirement_date: NaiveDate) -> f64 {
        let total_months = (retirement_date.year() - birth_date.year()) * 12
            + (retirement_date.month()  as i32 - birth_date.month() as i32);
        (total_months as f64) / 12.0
    }
}

pub fn retire_time(time: &str, tp: &str) -> String {
    let parts: Vec<&str> = time.split('-').collect();
    let year = parts[0].parse::<i32>().unwrap();
    let month = parts[1].parse::<u32>().unwrap();

    let sex = match tp {
        "男职工" => PersonType::Male,
        "原法定退休年龄55周岁女职工" => PersonType::Female55,
        _ => PersonType::Female50,
    };

    let aaa = RetirementCalculator {
        birth_year: year,
        birth_month: month,
        person_type: sex,
    };

    let tmp = aaa.calculate();

    format!("{},{},{}", tmp.0, (tmp.1 * 100.0).round() / 100.0, tmp.2)
}
