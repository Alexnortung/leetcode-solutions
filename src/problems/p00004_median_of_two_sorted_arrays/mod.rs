use super::Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (longest_nums, shortest_nums) = if nums1.len() > nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };
        let mut left = 0;
        let mut right = shortest_nums.len() - 1;
        let same_length = longest_nums.len() == shortest_nums.len();
        let full_length = longest_nums.len() + shortest_nums.len();

        let is_even = full_length & 1 == 0;

        if shortest_nums.len() == 0 {
            let mid = longest_nums.len() / 2;

            if is_even {
                return (longest_nums[mid] + longest_nums[mid - 1]) as f64 / 2.0;
            }

            return longest_nums[mid] as f64;
        }

        if full_length == 2 {
            return (shortest_nums[0] + longest_nums[0]) as f64 / 2.0;
        }

        if is_even {
            loop {
                let mid = (left + right) / 2;
                let mid_next = mid + 1;
                let mid_prev = usize::wrapping_sub(mid, 1);
                let mid_next_out_of_bounds = mid_next >= shortest_nums.len();
                let mid_prev_out_of_bounds = mid_prev == usize::MAX;
                let mid2 = (full_length / 2) - mid - 1;
                let mid2_next = mid2 + 1;
                let mid2_prev = usize::wrapping_sub(mid2, 1);

                // TODO
                if same_length && (mid == 0 || mid2 == 0) {
                    let (slice1, slice2) = if mid == 0 {
                        (
                            &shortest_nums[mid..=(mid + 1)],
                            &longest_nums[(mid2 - 1)..=mid2],
                        )
                    } else {
                        (
                            &shortest_nums[(mid - 1)..=mid],
                            &longest_nums[mid2..=(mid2 + 1)],
                        )
                    };

                    let max = slice1[0].max(slice2[0]);
                    let min = slice1[1].min(slice2[1]);

                    return (min + max) as f64 / 2.0;
                }

                if left == shortest_nums.len() {
                    let median_index = full_length / 2 - shortest_nums.len() - 1;
                    return (longest_nums[median_index] + longest_nums[median_index + 1]) as f64
                        / 2.0;
                }

                if right == usize::MAX {
                    let median_index = (full_length / 2) - 2;
                    return (longest_nums[median_index] + longest_nums[median_index + 1]) as f64
                        / 2.0;
                }

                if shortest_nums[mid] < longest_nums[mid2_prev] {
                    if !mid_next_out_of_bounds && longest_nums[mid2] < shortest_nums[mid_next] {
                        return (longest_nums[mid2] + longest_nums[mid2_prev]) as f64 / 2.0;
                    }
                    let new_left = mid + 1;
                    if new_left > right || left == right {
                        return (longest_nums[mid2] + longest_nums[mid2_prev]) as f64 / 2.0;
                    }
                    // move left pointer
                    left = new_left;
                    continue;
                }

                if shortest_nums[mid] > longest_nums[mid2_next] {
                    if !mid_prev_out_of_bounds && longest_nums[mid2] > shortest_nums[mid_prev] {
                        return (longest_nums[mid2] + longest_nums[mid2_next]) as f64 / 2.0;
                    }
                    let new_right = usize::wrapping_sub(mid, 1);
                    if left == right || left > new_right || new_right == usize::MAX {
                        return (longest_nums[mid2] + longest_nums[mid2_next]) as f64 / 2.0;
                    }
                    // move right pointer
                    right = new_right;
                    continue;
                }
                // At this point we know that mid is part of the median

                // Check if mid_prev is part of the median
                if !mid_prev_out_of_bounds
                    && longest_nums[mid2] < shortest_nums[mid_prev]
                    && shortest_nums[mid_prev] < longest_nums[mid2_next]
                {
                    return (shortest_nums[mid] + shortest_nums[mid_prev]) as f64 / 2.0;
                }
                // Check if mid_next is part of the median
                if !mid_next_out_of_bounds
                    && longest_nums[mid2_prev] < shortest_nums[mid_next]
                    && shortest_nums[mid_next] < longest_nums[mid2]
                {
                    return (shortest_nums[mid] + shortest_nums[mid_next]) as f64 / 2.0;
                }

                // There are no options left, so mid2 must also be the median
                return (shortest_nums[mid] + longest_nums[mid2]) as f64 / 2.0;
            }
        } else {
            loop {
                let mid = (left + right) / 2;
                let mid2 = (full_length / 2) - mid - 1;

                if left == shortest_nums.len() {
                    let median_index = full_length / 2 - shortest_nums.len();
                    return longest_nums[median_index] as f64;
                }

                if right == usize::MAX {
                    let median_index = (full_length / 2) - 2;
                    return longest_nums[median_index] as f64;
                }

                if shortest_nums[mid] < longest_nums[mid2] {
                    // if mid <= shortest_nums.len() - 1 {
                    //     if shortest_nums[mid + 1] > longest_nums[mid2] {
                    //         return longest_nums[mid2] as f64;
                    //     }
                    // }
                    // Check if mid2 is the median
                    if mid < shortest_nums.len() - 1 && longest_nums[mid2] <= shortest_nums[mid + 1]
                    {
                        return longest_nums[mid2] as f64;
                    }
                    // Does this ever happen?
                    let new_left = mid + 1;
                    if new_left > right || left == right {
                        return longest_nums[mid2] as f64;
                    }
                    // move left pointer
                    left = new_left;
                    continue;
                }

                // If mid is greater than the next mid2
                if shortest_nums[mid] > longest_nums[mid2 + 1] {
                    // Check if mid2 is the median
                    if mid > 0 && longest_nums[mid2 + 1] >= shortest_nums[mid - 1] {
                        return longest_nums[mid2 + 1] as f64;
                    }
                    let new_right = usize::wrapping_sub(mid, 1);
                    if left == right || left > new_right || new_right == usize::MAX {
                        return longest_nums[mid2 + 1] as f64;
                    }
                    // move right pointer
                    right = new_right;
                    continue;
                }

                return shortest_nums[mid] as f64;
            }
        }

        0.0
    }
}
