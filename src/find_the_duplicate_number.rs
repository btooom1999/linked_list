fn find_duplicate(mut nums: Vec<i32>) -> i32 {
    let mut slow = 0_usize;
    let mut fast = 0_usize;

    loop {
        slow = nums[slow] as usize;
        fast = nums[nums[fast] as usize] as usize;

        if slow == fast {
            break;
        }
    }

    let mut slow2 = 0_usize;
    loop {
        slow = nums[slow] as usize;
        slow2 = nums[slow2] as usize;

        if slow == slow2 {
            return slow as i32;
        }
    }

    unreachable!()
}

pub fn main() {
    let nums = [1,3,4,2,2].to_vec();
    println!("{:?}", find_duplicate(nums));
}
