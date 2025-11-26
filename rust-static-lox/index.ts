// Given an array of integers, write a function that determines whether the array contains any duplicates. Your function should return true if any element appears at least twice in the array, and it should return false if every element is distinct.
// Example
// For a = [1, 2, 3, 1], the output should be
// containsDuplicates(a) = true.
// There are two 1s in the given array.
// 	•		•	For a = [3, 1], the output should be
// containsDuplicates(a) = false.
// The given array contains no duplicates.
//
//

// function containsDuplicates() {
// 	const arr = [1, 2, 3, 1];
// 	let set = new Set();
//
// 	for (let i = 0; i < arr.length; i++) {
// 		const pointer = arr[i];
//
// 		if (set.has(pointer)) {
// 			return true;
// 		} else {
// 			set.add(pointer);
// 		}
// 	}
//
// 	return false;
// }

function containsDuplicates() {
	const arr = [1, 2, 3, 1];

	for (let i = 0; i < arr.length; i++) {
		const pointeri = arr[i];

		for (let j = 0; j < arr.length; j++) {
			const pointerj = arr[j];

			if (pointeri === pointerj && i !== j) {
				return true;
			}
		}
	}

	return false;
}

console.log(containsDuplicates());
