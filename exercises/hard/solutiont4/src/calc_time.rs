use std::collections::{HashMap, HashSet};


const fn day_of_year(year: u32, month: u32, day: u32) -> u32 {
    let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut day_of_year = day;

    let mut m = 0;
    while m < (month - 1) {
        day_of_year += days_in_month[m as usize];
        m += 1;
    }

    if year >= 2026 {
        day_of_year += 365;
    }

    day_of_year
}
const new_year_2025: u32 = day_of_year(2025, 1, 29);
const new_year_2026: u32 = day_of_year(2026, 2, 17);

// 辅助函数：将日期范围添加到 HashSet 中
fn add_holiday_range(holidays: &mut HashSet<(u32, u32)>, start_month: u32, start_day: u32, end_month: u32, end_day: u32) {
    let mut current_month = start_month;
    let mut current_day = start_day;

    loop {
        // 将当前日期添加到 HashSet
        holidays.insert((current_month, current_day));

        // 如果到达结束日期，退出循环
        if current_month == end_month && current_day == end_day {
            break;
        }

        // 增加日期
        current_day += 1;

        // 处理月份和日期的边界
        if current_day > days_in_month(current_month) {
            current_month += 1;
            current_day = 1;
        }
    }
}

// 辅助函数：获取某个月的天数
fn days_in_month(month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => 28, // 2025 年不是闰年
        _ => panic!("无效的月份"),
    }
}

fn is_stock_day(month:u32, day: u32, holidays: &HashSet<(u32,u32)>) -> bool {
    let days = day_of_year(2025, month, day);
    let week_day: u32 = ((days - 1) % 7 + 3) % 7;

    if week_day == 0 || week_day == 6 {
        return false;
    }
    // is_holiday
    if holidays.contains(&(month,day)) {
        return false;
    }

    true
}

fn next_day(month: u32, day: u32) -> (u32, u32) {
    let mut new_month = month;
    let mut new_day = day + 1;

    if new_day > days_in_month(month) {
        new_month += 1;
        new_day = 1;
    }
    if new_month > 12 {
        new_month = 1;
    }

    (new_month, new_day)
}

pub fn time_info(time: &str) -> String {
    // 创建一个 HashSet 来存储节假日日期
    let mut holidays = HashSet::new();
    // 元旦
    add_holiday_range(&mut holidays, 1, 1, 1, 1);
    // 春节
    add_holiday_range(&mut holidays, 1, 28, 2, 4);
    // 清明节
    add_holiday_range(&mut holidays, 4, 4, 4, 6);
    // 劳动节
    add_holiday_range(&mut holidays, 5, 1, 5, 5);
    // 端午节
    add_holiday_range(&mut holidays, 5, 31, 6, 2);
    // 国庆节
    add_holiday_range(&mut holidays, 10, 1, 10, 8);



    let parts: Vec<&str> = time.split("-").collect();
    let year = parts[0].parse::<u32>().unwrap();
    let month = parts[1].parse::<u32>().unwrap();
    let day = parts[2].parse::<u32>().unwrap();
    let days = day_of_year(year, month, day);
    let remain_days = 365 - days;

    let chunjie_days = match (month,day) {
        _ if (month == 1 && day > 29) || (month > 1 )  =>  {
            new_year_2026 - days
        },
        _ => new_year_2025 - days ,
    };

    let weeks = ((days / 7)+1) % 52;
    let week_day: u32 = ((days - 1) % 7 + 3)%7;

    // 计算下一个交易日
    let mut diff = 0;
    let (mut new_month, mut new_day) = next_day(month, day);
    while !is_stock_day(new_month, new_day, &holidays) {
        diff += 1;
        (new_month, new_day) = next_day(new_month, new_day);
    }

    // println!("{weeks},{week_day},{days},{remain_days},{chunjie_days},{diff}");
    format!("{weeks},{},{days},{remain_days},{chunjie_days},{diff}", 
        week_day = match week_day {
            0 => 7,
            _ => week_day,
        }
    )
}
