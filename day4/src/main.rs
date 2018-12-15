use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn convert_str_to_uint(s: &str) -> u32 {
    match s.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Conversion error of {}", s);
            0
        }
    }
}

fn open_input_file() -> BufReader<File> {
    let file = File::open("input").unwrap();
    return BufReader::new(file);
}

enum Status {
    Awake,
    Asleep,
    Unknown,
}

#[derive(Debug)]
enum LogAction {
    Wakes,
    Falls,
    Shift(u32), // new guard
}

#[derive(Debug)]
struct Log {
    date: Date,
    action: LogAction,
}

#[derive(Debug)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

struct Guard {
    id: u32,
    most_often_minute: u32,
    total_minutes_slept: u32,
    minute_counts: Vec<u32>,
}

impl Date {
    fn equals(&self, other: &Date) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day
    }
    // Returns true if &self has a later timestamp
    fn later(&self, other: &Date) -> bool {
        self.hour >= other.hour && self.minute >= other.minute
    }
}

impl Log {
    fn cmp(&self, other: &Log) -> Ordering {
        if self.date.year > other.date.year {
            return Ordering::Greater;
        }
        if self.date.year == other.date.year {
            if self.date.month > other.date.month {
                return Ordering::Greater;
            }
            if self.date.month == other.date.month {
                if self.date.day > other.date.day {
                    return Ordering::Greater;
                }
                if self.date.day == other.date.day {
                    if self.date.hour > other.date.hour {
                        return Ordering::Greater;
                    }
                    if self.date.hour == other.date.hour {
                        if self.date.minute > other.date.minute {
                            return Ordering::Greater;
                        }
                        if self.date.minute == other.date.minute {
                            return Ordering::Equal;
                        }
                        return Ordering::Less;
                    }
                    return Ordering::Less;
                }
                return Ordering::Less;
            }
            return Ordering::Less;
        }
        Ordering::Less
    }
}

fn update_guard(guards: &mut HashMap<u32, Guard>, guard_id: u32, start_minute: u32, minute: u32) {
    if let Some(g) = guards.get_mut(&guard_id) {
        for m in start_minute..=minute {
            g.minute_counts[m as usize] += 1;
            g.total_minutes_slept += 1;
            if g.minute_counts[m as usize] > g.minute_counts[g.most_often_minute as usize] {
                g.most_often_minute = m;
            }
        }
    }
}

fn main() {
    let buf_reader = open_input_file();

    let mut guards: HashMap<u32, Guard> = HashMap::new();
    let mut logs: Vec<Log> = Vec::new();

    let mut guard_status = Status::Unknown;
    let mut guard: u32 = 0;
    let mut last_date: Date = Date {
        year: 0,
        month: 0,
        day: 0,
        hour: 0,
        minute: 0,
    };

    for (_nr, line) in buf_reader.lines().enumerate() {
        let l = line.unwrap();
        let v: Vec<&str> = (&l).split(' ').collect();

        let mut v_iter = v.iter();
        let s_date = match v_iter.next() {
            Some(s) => &s,
            None => "",
        };
        let s_time = match v_iter.next() {
            Some(s) => s,
            None => "",
        };
        let action = match v_iter.next() {
            Some(s) => s,
            None => "",
        };
        let date_vec: Vec<&str> = s_date.split('-').collect();
        let mut date_iter = date_vec.iter();
        let time_vec: Vec<&str> = s_time.split(':').collect();
        let mut time_iter = time_vec.iter();

        let year = match date_iter.next() {
            Some(y) => convert_str_to_uint(&y.trim().trim_start_matches('[')),
            None => 0,
        };
        let month = match date_iter.next() {
            Some(m) => convert_str_to_uint(&m.trim()),
            None => 0,
        };
        let day = match date_iter.next() {
            Some(d) => convert_str_to_uint(&d.trim()),
            None => 0,
        };
        let hour = match time_iter.next() {
            Some(h) => convert_str_to_uint(&h.trim()),
            None => 23,
        };
        let minute = match time_iter.next() {
            Some(m) => convert_str_to_uint(&m.trim().trim_end_matches(']')),
            None => 0,
        };

        // New guard
        if v.len() == 6 {
            if let Some(s) = v.get(3) {
                guard = convert_str_to_uint(&s.trim()[1..]);
            }
            logs.push(Log {
                date: Date {
                    year,
                    month,
                    day,
                    hour,
                    minute,
                },
                action: LogAction::Shift(guard),
            });
        } else {
            match action {
                "wakes" => {
                    logs.push(Log {
                        date: Date {
                            year,
                            month,
                            day,
                            hour,
                            minute,
                        },
                        action: LogAction::Wakes,
                    });
                }
                "falls" => {
                    logs.push(Log {
                        date: Date {
                            year,
                            month,
                            day,
                            hour,
                            minute,
                        },
                        action: LogAction::Falls,
                    });
                }
                _ => println!("What happened? Line: {} | action: {}", &l, action),
            }
        }
    }

    logs.sort_by(|a, b| a.cmp(&b));

    for l in logs {
        match l.action {
            LogAction::Shift(g) => {
                guard = g;
                if !guards.contains_key(&g) {
                    guards.insert(
                        g,
                        Guard {
                            id: g,
                            most_often_minute: 0,
                            total_minutes_slept: 0,
                            minute_counts: vec![0; 60],
                        },
                    );
                }
                guard_status = Status::Awake;
            }
            LogAction::Wakes => {
                // 2 cases: wakes after wakes, wakes after falls
                match guard_status {
                    Status::Awake => {
                        // If hour is not 0, then the guard woke up before midnight => no one cares
                        if l.date.hour == 0 {
                            update_guard(&mut guards, guard, 0, l.date.minute - 1); //-1 since minute XX means guard is awake right at that minute
                        }
                    }
                    Status::Asleep => {
                        // Since guard was asleep, look for difference on same day
                        if last_date.equals(&l.date) {
                            if last_date.later(&l.date) {
                                // Wake before sleep => 2 separate updates
                                if last_date.hour == 0 {
                                    update_guard(&mut guards, guard, last_date.minute, 59);
                                } else {
                                    //This is impossible?
                                    update_guard(&mut guards, guard, 0, 59);
                                    println!("The \"impossible\" happened, a wake before sleep and sleep was before midnight!");
                                }
                                // The wake update
                                if l.date.hour == 0 {
                                    update_guard(&mut guards, guard, 0, l.date.minute - 1);
                                }
                                println!("Lands here? {:#?}", &l);
                            } else {
                                if last_date.hour == 0 {
                                    update_guard(
                                        &mut guards,
                                        guard,
                                        last_date.minute,
                                        l.date.minute - 1,
                                    );
                                } else {
                                    //guard fell asleep before midnight
                                    update_guard(&mut guards, guard, 0, l.date.minute - 1);
                                }
                            }
                        } else {
                            // or compute 2 updates, one for asleep (since guard was asleep for the whole duration
                            // and for the wake up now
                            if last_date.hour == 0 {
                                update_guard(&mut guards, guard, last_date.minute, 59);
                            } else {
                                update_guard(&mut guards, guard, 0, 59);
                            }
                            if l.date.hour == 0 {
                                update_guard(&mut guards, guard, 0, l.date.minute - 1);
                            }
                        }
                    }
                    Status::Unknown => {
                        println!("Not sure, should not happen.");
                    }
                };
                guard_status = Status::Awake;
            }
            LogAction::Falls => {
                // Since guard falls asleep, just check for a asleep before
                // if no asleep, remember time, update happens in awake
                match guard_status {
                    Status::Asleep => {
                        if last_date.hour == 0 {
                            update_guard(&mut guards, guard, last_date.minute, 59)
                        } else {
                            update_guard(&mut guards, guard, 0, 59);
                        }
                    }
                    _ => {} //Do nothing for any other case
                }
                guard_status = Status::Asleep;
            }
        };
        last_date = l.date;
    }

    let mut g_strategy_1: &Guard = &Guard {
        id: 0,
        most_often_minute: 0,
        total_minutes_slept: 0,
        minute_counts: vec![0; 1],
    };
    let mut g_strategy_2: &Guard = &Guard {
        id: 0,
        most_often_minute: 0,
        total_minutes_slept: 0,
        minute_counts: vec![0; 1],
    };
    for (_guard_id, guard) in guards.iter() {
        // Strategy 1
        if guard.total_minutes_slept > g_strategy_1.total_minutes_slept {
            g_strategy_1 = guard;
        }
        if guard.minute_counts[guard.most_often_minute as usize] > g_strategy_2.minute_counts[g_strategy_2.most_often_minute as usize] {
            g_strategy_2 = guard;
        }
        
    }
    println!(
        "The guard {} sleeps most often in minute {} (total {} minutes), val: {}",
        g_strategy_1.id,
        g_strategy_1.most_often_minute,
        g_strategy_1.total_minutes_slept,
        g_strategy_1.id * g_strategy_1.most_often_minute
    );
    println!(
        "The guard {} sleeps most often in minute {} for {} many times, val: {}",
        g_strategy_2.id,
        g_strategy_2.most_often_minute,
        g_strategy_2.minute_counts[g_strategy_2.most_often_minute as usize],
        g_strategy_2.id * g_strategy_2.most_often_minute
    );
}
