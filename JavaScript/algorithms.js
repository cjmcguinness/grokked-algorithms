// MISCELLANEOUS HELPER FUNCTIONS

// Function checks for ordered arrays
function check_array_ordered(arr) {
    for (let i = 0; i < arr.length - 1; i++) {
        if (arr[i] > arr[i + 1]) {
            console.log(`Array not ordered, ${arr[i]} is before and larger than ${arr[i + 1]}`);
            return false; // Return false if the array is not ordered
        }
    }
    return true; // Return true if the array is ordered
}

// Function to find the smallest element in an array
function find_smallest_in_array(arr) {
    let smallest = arr[0]
    let index_smallest = 0

    for (let i = 0; i < arr.length; i++) {
        if (arr[i] < smallest) {
            smallest = arr[i]
            index_smallest = i
        }
    }
    return [smallest, index_smallest]; 
}

// Function to convert an array from ascending to descending order
function asc_to_desc_array(arr) {
    return arr.slice().reverse(); // Use slice() to copy the array before reversing
}

// ALGORITHMS THAT ORDER AN ARRAY
function selection_sort(unsorted_arr) {
    let sorted_arr = [];

    while (unsorted_arr.length > 0) { 
        let [smallest_el, smallest_ind] = find_smallest_in_array(unsorted_arr); 
        sorted_arr.push(smallest_el); // '.push()' adds element at end of array
        unsorted_arr.splice(smallest_ind, 1); // args for '.splice()' are (index, number of elements to remove)
    }
    return sorted_arr;
}

function quicksort(unsorted_arr) {
    if (unsorted_arr.length < 2) { 
        return unsorted_arr;
    }

    let left = []; 
    let right = [];
    let pivot_index = Math.floor(unsorted_arr.length / 2); 
    let pivot = unsorted_arr[pivot_index];

    for (let i = 0; i < unsorted_arr.length; i++) {
        if (i !== pivot_index) { 
            if (unsorted_arr[i] < pivot) {
                left.push(unsorted_arr[i]);
            } else {
                right.push(unsorted_arr[i]);
            }
        }
    }

    return [...quicksort(left), pivot, ...quicksort(right)]; // Concatenate sorted left, pivot, and sorted right
}

// ALGORITHMS THAT REQUIRE AN ORDERED ARRAY
function binary_search(ordered_arr, target) {
    if (ordered_arr.length === 0) {
        console.log("Array is empty");
        return;
    }

    if (!check_array_ordered(ordered_arr)) { // Ensure the array is ordered before searching
        console.log("Array is not ordered");
        return;
    }

    let bottom_index = 0;
    let top_index = ordered_arr.length - 1;

    while (bottom_index <= top_index) {
        let index_to_check = Math.floor((bottom_index + top_index) / 2);

        if (target === ordered_arr[index_to_check]) {
            console.log(`${target} is at index ${index_to_check} of the array`);
            return;
        } else if (ordered_arr[index_to_check] < target) {
            bottom_index = index_to_check + 1;
        } else {
            top_index = index_to_check - 1;
        }
    }
    console.log(`${target} is not in the collection`);
}

// GRAPH ALGORITHMS
// Map is a hashmap in JavaScript, use a Map for graph
function breadth_first_search(graph, start) {
    let queue = new Deque();
    let checked = new Set();

    queue.addBack(start);
    
    while (!queue.isEmpty()) {
        let node_name = queue.removeFront();
        
        if (checked.has(node_name)) { // Check if the node has already been checked
            continue; // Skip to the next iteration if it has
        }
        checked.add(node_name); // Mark the node as checked

        let neighbours = graph.get(node_name) || []; // Use empty array as default if no neighbours

        neighbours.forEach(neighbour => { 
            if (!checked.has(neighbour)) {
                queue.addBack(neighbour);
            }
        });
    }
}

// input: 'weighted_graph' is a Map with a key of 'node' and an array with neighbour info;
// the array has label and value pairs, 'neighbour' and 'weight'
// 'start_node' and 'target_nodes' self explanatory

// output: which sets out the shortest path and the first element sets out the cost
function dijkstrasAlgorithm(weighted_graph, start_node, target_node) {
    let lowest_cost_to_node = new Map();
    let preceding_node = new Map();
    
    // Populate the HashMaps with initial values:
    let nodes = [...weighted_graph.keys()]; 
    nodes.forEach(node => {
        lowest_cost_to_node.set(node, node === start_node ? 0 : Infinity);
        preceding_node.set(node, '-');
    });

    let queue = new Deque();
    queue.addBack(start_node);

    while (!queue.isEmpty()) { 
        let current_node = queue.removeFront(); 
        let current_cost = lowest_cost_to_node.get(current_node); 
        let neighbours = weighted_graph.get(current_node) || [];

        neighbours.forEach(({ neighbour, weight }) => {
            let new_cost = current_cost + weight;

            // Check if new cost is less than the current known lowest cost to the neighbour
            if (new_cost < lowest_cost_to_node.get(neighbour)) {
                lowest_cost_to_node.set(neighbour, new_cost);
                preceding_node.set(neighbour, current_node);
                queue.addBack(neighbour); 
            }
        }); 
    }

    // Build shortest path from 'target_node' to 'start_node'
    let path = [];
    let current_node = target_node; 

    while (current_node !== '-') { 
        path.unshift(current_node);
        current_node = preceding_node.get(current_node); 
    }

    let cost_to_target = lowest_cost_to_node.get(target_node);
    path.unshift(cost_to_target); // Add the cost to the front of the path

    return path;
}
