pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut start, mut end) = (0_usize, (height.len() - 1));
        let mut max: i32 = (end - start) as i32 * Solution::min(height[start], height[end]);
        let mut curr_area: i32 = 0;
        while end - start > 1 {
            // move the lower one
            if height[start] < height[end] {
                start += 1;
                if height[start] < height[start - 1] {
                    continue;
                }
            } else {
                end -= 1;
                if height[end] < height[end + 1] {
                    continue;
                }
            }
            curr_area = (end - start) as i32 * Solution::min(height[start], height[end]);
            if curr_area > max {
                max = curr_area
            }
        }
        max
    }

    #[inline(always)]
    fn min(i: i32, j: i32) -> i32 {
        if i > j {
            j
        } else {
            i
        }
    }
    pub fn max_area_brute_force(height: Vec<i32>) -> i32 {
        let mut arr: [i32; 1000] = [0; 1000];
        let mut tmp_max_area = 0;
        let mut index: i32 = 0;
        for item in height.iter() {
            let mut index_: i32 = 0;
            for item_ in height.iter().skip(index as usize + 1) {
                if item > item_ {
                    if item_ * (index_ + 1) > tmp_max_area {
                        tmp_max_area = item_ * (index_ + 1);
                    }
                } else {
                    if item * (index_ + 1) > tmp_max_area {
                        tmp_max_area = item * (index_ + 1);
                    }
                }
                index_ += 1;
            }
            arr[index as usize] = tmp_max_area as i32;
            index += 1;
        }
        let mut max: i32 = 0;
        for i in arr.iter() {
            if *i > max {
                max = *i;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_area() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        //        assert_eq!(Solution::max_area(vec![6, 9]), 6);
        assert_eq!(Solution::max_area(vec![1, 1, 2, 1, 1, 1]), 5);
    }
}
