use crate::components::vircon_component::VirconComponent;
use crate::local_ports::TimerLocalPorts;
use crate::constants::TIMER_PREFIX;
use chrono::{Datelike, DateTime, NaiveDate, prelude, Timelike};
use chrono::Local;
use log::info;
use crate::vircon_word::VirconWord;

pub struct Timer {
    current_date: i32,
    current_time: i32,
    frame_counter: i32,
    cycle_counter: i32,
}

impl Timer {
    pub fn new() -> Timer
    {
        info!("{} Creating a new Timer...", TIMER_PREFIX);

        let mut timer =  Timer {
            current_date: 0,
            current_time: 0,
            frame_counter: 0,
            cycle_counter: 0
        };

        //obtain current time
        let now = Local::now();
        let y_day = now.date_naive().signed_duration_since(NaiveDate::from_ymd(now.year() , 1, 1)).num_days();

        //store date as years | days
        timer.current_date = (now.year() << 16) | y_day;

        //store current time as seconds within this day.
        timer.current_time = (
                (now.hour() * 3600) +
                (now.minute() * 60) +
                now.second()
            ) as i32;

        return timer;
    }

    pub fn run_next_cycle(&mut self)
    {
        self.cycle_counter += 1;
    }


    pub fn reset(&mut self)
    {
        info!("{} Resetting Timer...", TIMER_PREFIX);

        self.cycle_counter = 0;
        self.frame_counter = 0;
    }

    pub fn change_frame(&mut self)
    {
        self.cycle_counter = 0;
        self.frame_counter += 1;

        // Current time advances each second
        if (self.frame_counter % 60) == 0
        {
            self.current_time += 1;
        }

        //current date advances each day
        if self.current_time >= 86400
        {
            self.current_time = 0;
            self.current_date += 1;

            // check if we should change year
            let year: i32 = self.current_date >> 16;
            let is_leap_year: bool = ( ((year % 4) == 0) && ((year % 100) != 0));

            let mut days_this_year = 0;
            if is_leap_year
            {
                days_this_year = 366;
            }
            else
            {
                days_this_year = 365;
            }

            let days = self.current_date & 0xFFFF;

            // if needed, advance year and reset days to 0
            if days >= days_this_year
            {
                self.current_date = (year + 1) << 16;
            }
        }
    }
}

impl VirconComponent for Timer {
    fn read_port(&mut self, local_port: i32, result: &mut VirconWord) -> bool {
        info!("{} Reading local port \"{}\"", TIMER_PREFIX, local_port);
        //Check range
        if local_port > TimerLocalPorts::CycleCounter as i32 { /*cycle counter is the latest element*/
            return false;
        }

        // provide value (for efficiency, do the checks
        // starting by the most frequently accessed ports)
        if local_port == TimerLocalPorts::FrameCounter as i32 {
            *result.as_integer = self.frame_counter;
        }
        else  if local_port == TimerLocalPorts::CycleCounter as i32 {
            *result.as_integer = self.cycle_counter;
        }
        else if local_port == TimerLocalPorts::CurrentTime as i32 {
            *result.as_integer = self.current_time;
        }
        else {
            *result.as_integer = self.current_date;
        }

        return true;
    }

    fn write_port(&mut self, local_port: i32, value: VirconWord) -> bool {
        info!("{} writing port request will be ignored as all registers are read-only.", TIMER_PREFIX);

        // ignore write request (all these registers are read-only)
        return false;
    }
}