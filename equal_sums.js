// Given an array of numbers, determine whether the array can be split into two groups with equal sums
// Returns either an array representing one of the possible groups, or false if not possible
// This solution is quadratic time, O(n^2), because it utilises two nested for loops
// There is also an elegant yet inefficient recursive solution: https://github.com/mirandaio/codingbat/blob/master/java/recursion-2/splitArray.java 

function find_equal_sums(numbers) {
    const sum = numbers.reduce((a, b) => a + b)
    if (sum % 2 == 1) return false
    const target_sum = sum / 2
    const sums_map = { "0": [] }
    for (n of numbers) {
        for (k of Object.keys(sums_map)) {
            const new_key = +k + n
            if (!sums_map.hasOwnProperty(new_key)) {
                sums_map[new_key] = [...sums_map[k], n]
            }
            if (new_key == target_sum) {
                return sums_map[new_key]
            }
        }
    }
    return false
}

function timer(f) {
    const start_time = Date.now()
    f()
    console.log(`timer: ${(Date.now() - start_time) / 1000}`)
}

const test_1 = [5, 2, 3]
const test_2 = [6, 1, 2, 4]
const test_3 = [20, 70, 40, 100, 10]
const test_4 = [1, 1, 2, 100, 23, 38, 1, 22, 18, 17, 3, 2, 10, 15, 1, 1, 1]
const test_5 = [1, 2, 3, 4, 5, 6, 7]
console.log("test 1", find_equal_sums(test_1)) // [ 5 ]
console.log("test 2", find_equal_sums(test_2)) // false
console.log("test 3", find_equal_sums(test_3)) // [20, 100]
timer(() => console.log("test 4:", find_equal_sums(test_4))) // [1, 1, 2, 100, 23, 1]
timer(() => console.log("test 5:", find_equal_sums(test_5)))