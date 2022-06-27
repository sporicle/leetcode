use std::collections::HashMap;

// https://leetcode.com/problems/first-missing-positive/
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut hmap = HashMap::new();
        let mut low = nums[0];
        let mut high = nums[0];
        for i in nums {
            if !hmap.contains_key(&i){
                hmap.insert(i, 0);
            }
            
            if i < low && i > 0{
                low = i;
            }
            if i > high && i > 0{
                high = i;
            }
        }
        
        if low > 1{
            return 1; 
        }
        
        for i in 1..high {
            if !hmap.contains_key(&i){
                return i;
            }
        }
        
        if high > 0 {
            return high+1;
        }
        else {
            return 1;
        }
    }
}