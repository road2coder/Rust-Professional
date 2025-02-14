/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd>(array: &mut [T]) {
    quick_sort(array);
}

// 快排: 分治
fn quick_sort<T: PartialOrd>(nums: &mut [T]) {
    let len = nums.len();
    if len <= 1 {
        return;
    }
    let mut right = len - 1;
    let mut left = 0;
    while left < right {
        while left < right && nums[right] >= nums[0] {
            right -= 1;
        }
        while left < right && nums[left] <= nums[0] {
            left += 1;
        }
        nums.swap(left, right);
    }
    nums.swap(0, left);
    quick_sort(&mut nums[..left]);
    quick_sort(&mut nums[left + 1..]);
}

// TODO:
// 基础: 冒泡 选择 插入 O(N ^ 2)
// 堆排序: 利用堆是完全二叉树的特性, 原地构建一个最小堆
// 归并排序

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
