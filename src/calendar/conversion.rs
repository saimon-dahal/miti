use chrono::NaiveDate;
use anyhow::{Result, anyhow};
use super::bs_data::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NepaliDate {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl NepaliDate {
    pub fn new(year: u16, month: u8, day: u8) -> Result<Self> {
        if month < 1 || month > 12 {
            return Err(anyhow!("Invalid month: {}", month));
        }
        
        let days_in_month = get_days_in_month(year, month)
            .ok_or_else(|| anyhow!("Year {} not in supported range", year))?;
        
        if day < 1 || day > days_in_month {
            return Err(anyhow!("Invalid day: {} for month {}/{}", day, year, month));
        }
        
        Ok(Self { year, month, day })
    }
    
    pub fn to_string(&self) -> String {
        format!("{:04}/{:02}/{:02}", self.year, self.month, self.day)
    }
}

// Convert AD to BS
pub fn ad_to_bs(ad_date: NaiveDate) -> Result<NepaliDate> {
    let reference_ad = NaiveDate::from_ymd_opt(
        AD_REFERENCE_YEAR,
        AD_REFERENCE_MONTH,
        AD_REFERENCE_DAY
    ).ok_or_else(|| anyhow!("Invalid reference AD date"))?;
    
    let days_diff = ad_date.signed_duration_since(reference_ad).num_days();
    
    let mut bs_year = BS_REFERENCE_YEAR;
    let mut bs_month = BS_REFERENCE_MONTH;
    let mut bs_day = BS_REFERENCE_DAY;
    let mut remaining_days = days_diff;
    
    if remaining_days >= 0 {
        // Forward calculation
        while remaining_days > 0 {
            let days_in_current_month = get_days_in_month(bs_year, bs_month)
                .ok_or_else(|| anyhow!("Year {} not in supported range", bs_year))?;
            
            let days_left_in_month = days_in_current_month - bs_day + 1;
            
            if remaining_days < days_left_in_month as i64 {
                bs_day += remaining_days as u8;
                remaining_days = 0;
            } else {
                remaining_days -= days_left_in_month as i64;
                bs_day = 1;
                bs_month += 1;
                
                if bs_month > 12 {
                    bs_month = 1;
                    bs_year += 1;
                }
            }
        }
    } else {
        // Backward calculation
        remaining_days = -remaining_days;
        
        while remaining_days > 0 {
            if remaining_days < bs_day as i64 {
                bs_day -= remaining_days as u8;
                remaining_days = 0;
            } else {
                remaining_days -= bs_day as i64;
                
                if bs_month == 1 {
                    bs_month = 12;
                    bs_year -= 1;
                } else {
                    bs_month -= 1;
                }
                
                bs_day = get_days_in_month(bs_year, bs_month)
                    .ok_or_else(|| anyhow!("Year {} not in supported range", bs_year))?;
            }
        }
    }
    
    NepaliDate::new(bs_year, bs_month, bs_day)
}

// Convert BS to AD
pub fn bs_to_ad(bs_date: NepaliDate) -> Result<NaiveDate> {
    let reference_ad = NaiveDate::from_ymd_opt(
        AD_REFERENCE_YEAR,
        AD_REFERENCE_MONTH,
        AD_REFERENCE_DAY
    ).ok_or_else(|| anyhow!("Invalid reference AD date"))?;
    
    let mut days_diff: i64 = 0;
    
    // Calculate days from reference BS date to target BS date
    if bs_date.year > BS_REFERENCE_YEAR || 
       (bs_date.year == BS_REFERENCE_YEAR && bs_date.month > BS_REFERENCE_MONTH) ||
       (bs_date.year == BS_REFERENCE_YEAR && bs_date.month == BS_REFERENCE_MONTH && bs_date.day >= BS_REFERENCE_DAY) {
        // Forward calculation
        let mut current_year = BS_REFERENCE_YEAR;
        let mut current_month = BS_REFERENCE_MONTH;
        let mut current_day = BS_REFERENCE_DAY;
        
        while current_year < bs_date.year || current_month < bs_date.month || current_day < bs_date.day {
            let days_in_month = get_days_in_month(current_year, current_month)
                .ok_or_else(|| anyhow!("Year {} not in supported range", current_year))?;
            
            if current_year == bs_date.year && current_month == bs_date.month {
                days_diff += (bs_date.day - current_day) as i64;
                break;
            }
            
            days_diff += (days_in_month - current_day + 1) as i64;
            current_day = 1;
            current_month += 1;
            
            if current_month > 12 {
                current_month = 1;
                current_year += 1;
            }
        }
    } else {
        // Backward calculation
        let mut current_year = BS_REFERENCE_YEAR;
        let mut current_month = BS_REFERENCE_MONTH;
        let mut current_day = BS_REFERENCE_DAY;
        
        while current_year > bs_date.year || current_month > bs_date.month || current_day > bs_date.day {
            if current_year == bs_date.year && current_month == bs_date.month {
                days_diff -= (current_day - bs_date.day) as i64;
                break;
            }
            
            days_diff -= current_day as i64;
            
            if current_month == 1 {
                current_month = 12;
                current_year -= 1;
            } else {
                current_month -= 1;
            }
            
            current_day = get_days_in_month(current_year, current_month)
                .ok_or_else(|| anyhow!("Year {} not in supported range", current_year))?;
        }
    }
    
    reference_ad.checked_add_signed(chrono::Duration::days(days_diff))
        .ok_or_else(|| anyhow!("Date calculation overflow"))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_reference_date() {
        let ad = NaiveDate::from_ymd_opt(1943, 4, 14).unwrap();
        let bs = ad_to_bs(ad).unwrap();
        assert_eq!(bs.year, 2000);
        assert_eq!(bs.month, 1);
        assert_eq!(bs.day, 1);
    }
    
    #[test]
    fn test_roundtrip() {
        let original_ad = NaiveDate::from_ymd_opt(2024, 5, 21).unwrap();
        let bs = ad_to_bs(original_ad).unwrap();
        let converted_ad = bs_to_ad(bs).unwrap();
        assert_eq!(original_ad, converted_ad);
    }
}
