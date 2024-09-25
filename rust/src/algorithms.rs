use std::collections::{HashMap, HashSet, VecDeque};


//MISCELLANEOUS HELPER FUNCTIONS

//function checks for ordered collections; 
//useful to abort use of functions which require an ordered collection in the event an unordered array is passed as a parameter thereto 
//using Option<T> monad will allow this function to return 'None' or 'Some()'
//'None' and 'Some()' can be used for pattern matching in other functions
pub fn check_collection_ordered (collection: &[f64]) -> Option<()> {
    //windows() takes a slice of the collection, in this case, a window of size 2 to compare adjacent elements 
    for window  in collection.windows(2) {
        let l: f64 = window[0];
        let r: f64 = window[1];
        
        //if a left-hand element is greater than a right hand element, then the collection isn't ordered  
        if l > r {
            println!("Collection not ordered, {} is before and larger than {}", l, r);
            return None;
        }
    }
    //implicit return
    Some(())
}

pub fn find_smallest_in_collection (collection: &[f64]) -> (f64, usize) {
    //These values will need to be updated so declare as mut
    let mut smallest: f64 = collection[0];
    let mut index_smallest: usize = 0; //Rust array indexes must be usize
    
    //calling arr.iter().enumerate() returns an iterator that produces tuples (index, element)
    for (index, &element) in collection.iter().enumerate() {
        if element < smallest {
            smallest = element;
            index_smallest = index;
        }
    }
    //use a tuple to store 'smallest' & 'index_smallest' because they are different types 
    let smallest_and_index: (f64, usize) = (smallest, index_smallest);
    //leave with no semicolon for implicit return of smallest_and_index
    smallest_and_index
}

pub fn asc_to_desc_collection (asc_collection: &[f64]) -> Vec<f64> {
    let mut desc_collection:Vec<f64> = vec![];
    for &element in asc_collection.iter() {
        desc_collection.insert(0, element);
    }
    desc_collection
}


//ALGORITHMS THAT ORDER A COLLECTION
pub fn selection_sort (unsorted_collection: &[f64]) -> Vec<f64> {
    //work with vectors in the function because we need to modify lengths (i.e. .push() and .remove() are required)
    //vectors are stored on the heap (so can change size), whereas arrays are stored on the stack (so cannot change size)

    //.to_vec() copies all the elements of a collection to a vector
    //therefore, it doesn't matter is unsorted_collection is an array or a vector
    let mut unsorted_vec:Vec<f64> = unsorted_collection.to_vec();
    //creating an empty vector using the vec![] macro
    let mut sorted_vec:Vec<f64> = vec![];

    while unsorted_vec.len() > 0 {
        let (smallest_el, smallest_ind) = find_smallest_in_array(&unsorted_vec);
        sorted_vec.push(smallest_el);
        unsorted_vec.remove(smallest_ind);
    }
    //leave with no semicolon for implicit return of sorted_vec 
    sorted_vec
}

pub fn quicksort (unsorted_collection: &[f64]) -> Vec<f64> {
    let mut unsorted_vec:Vec<f64> = unsorted_collection.to_vec();

    if unsorted_vec.len() < 2 {
        return unsorted_vec;
    }

    let mut left:Vec<f64> = vec![];
    let mut right:Vec<f64> = vec![];
    let mut pivot_index: usize = unsorted_vec.len()/2;
    let mut pivot: f64 = unsorted_vec[pivot_index];
    
    //need '.enumerate()' to get the index to ensure we don't count the pivot
    for (index, &element) in unsorted_vec.iter().enumerate() {
        if index != pivot_index {
            if &element < &pivot {
                left.push(element);
            } else {
                right.push(element);
            }
        }
    }
    //'.push()' & '.extend()' adds elements to the vector in place
    //so create a sequence of code wherein the result is sorted_collection

    //RHS equivalent to sorting the 'left' collection
    let mut sorted_collection:Vec<f64> = quicksort(&left);
    
    //equivalent to pushing 'pivot' to the sorted 'left' collection
    sorted_collection.push(pivot);
    
    //equivalent to adding the sorted 'right' collection to the sorted 'left' collection and the 'pivot';
    sorted_collection.extend(quicksort(&right));
    //implicit return
    sorted_collection
}



//ALGORITHMS THAT REQUIRE AN ORDERED COLLECTION
pub fn binary_search(ordered_collection: &[f64], target: f64) {
    //set the type as a borrowed slice (i.e. &[f64]) avoids needing to copy the array
    //this function will take an array or vector as an input

    if ordered_collection.len() == 0 {
        println!("Collection is empty");
        return;
    }

    //you could do this with an 'if' but I wanted to use a 'match' pattern somewhere in this
    let ordered_or_not: Option<()> = check_collection_ordered(ordered_collection);
    match ordered_or_not {
        None => {
            println!("binary_search requires an ordered collection"); 
            return;
        }
        Some(()) => {
            println!("collection is ordered")
        }
    }

    //set as usize as these will be used to calculate a collection index, always usize in Rust
    let mut bottom_index: usize = 0; 
    let mut top_index: usize = ordered_collection.len() - 1; 

    while bottom_index <= top_index {
        //Rust always truncates the decimal part resulting from integer division
        //e.g. 5/6 would return 0 
        let index_to_check: usize = (bottom_index + top_index) / 2;

        if target == ordered_collection[index_to_check] {
            println!("{} is at index {} of the collection", target, index_to_check);
        } else if ordered_collection[index_to_check] < target {
            bottom_index = index_to_check + 1;
        } else {
            top_index = index_to_check - 1;
        }
    }
    println!("{} is not in the collection", target);
}


//GRAPH ALGORITHMS
//HashMap Strings are 'node_name' and 'neighbour_names'; 'start' is 'node_name' of first node
pub fn breadth_first_search (graph: &HashMap<String, Vec<String>>, start: String) {
    
    //where the String is the label of the node 
    let mut queue: VecDeque<String> = VecDeque::new();

    //use a HashSet which can only store unique keys (it stores keys only)
    //HashSet will have time complexity O(1) for checking if 'node_names' are in checked   
    let mut checked: HashSet<String> = HashSet::new();

    queue.push_back(start);
    //iterate through each level of graph if queue is not empty 
    //could use 'if queue.is_empty() != true' but instead use the rust 'Some(T)'
    //Some(label) is either the value of 'label' (i.e. the first String which in a given entry of 'queue') if the queue is populated, or
    //Some(label) is 'None' and the 'while' the while loop terminates
    //if 'None'
    while let Some(node_name) = queue.pop_front() {
        
        //OPTIONALLY ADD A FUNCTION THAT EVALUATES SOME PROPERTY ASSOCIATED WITH 'node_name'
        //e.g. if node_name == search_term
        //whereby the function returns a result e.g. "found "search_term" in graph"
        //the node_name could be part of a more general tuple or struct defining properties of the node which can be subject to evaluation here
        
        //insert node_name of checked node to 'checked' HashMap
        //use '.clone()' to avoid ownership issues
        //'checked.insert(arg) will:
        //add the arg to 'checked' if arg is not already in 'checked' and will return 'true', or
        //return false if arg is already present in 'checked' (and won't add arg a second time)
        //'prepending a '!' negates the result i.e. flips which bool is returned
        if !checked.insert(node_name.clone()) {
            //by making using negation we get a 'true' if 'node_name' cannot be added i.e. it has been checked before
            //in that event, we get us into this block and hit the 'continue' which will skip the remaining code in the while loop block and start the next iteration thereof
            continue;
        }
        
        //'graph.get(&node_name)' gets either:
        //i the Vec<String> associated with the key 'node_name' in graph (being implemented by a HashMap) or
        //ii 'None'

        if let Some(neighbours) = graph.get(&node_name) {
            for neighbour in neighbours.iter() {
                //'checked.contains(neighbour)' returns a bool i.e. true if 'checked' contains 'neighbour' 
                
                if !checked.contains(neighbour) {
                    queue.push_back(neighbour.clone());
                }
            }

        }

    }
}


//input: HashMap has String 'node', and a Vec; 
//each element of Vec for a given 'node' is a tuple with info on each neighbour;
//each tuple has a String 'neighbour' and a float 'weight' 
//'start_node' and 'target_nodes' self explanatory

//output: Vec<String> setting out the final shortest path and float representing the total cost of this path 
pub fn dijkstras_algorithm(
    weighted_graph: HashMap<String, Vec<(String, f64)>>, 
    start_node: String, 
    target_node: String
) -> (Vec<String>, f64) {
    
    //stores lowest cost to node; needs updating as algorithm proceeds
    let mut lowest_cost_to_node: HashMap<String, f64> = HashMap::new();

    //first 'String' stores a given node, second 'String' stores the node you came from to get to the first String
    //using Option<String> allows us to set the value to 'None' when setting up the HashMap
    let mut preceding_node: HashMap<String, Option<String>> = HashMap::new();
    
    //populate the HashMaps with initial values:
    //each node should be represented as a key in each HashMap
    //in 'preceding_node' pair with 'None' i.e. we haven't got to any of the nodes yet
    for node in weighted_graph.keys() {
        if node == &start_node {
            lowest_cost_to_node.insert(node.clone(), 0.0);
            //start node has a cost of 0
        } else {
            lowest_cost_to_node.insert(node.clone(), f64::INFINITY);
            //all non-start nodes get value of 'f64::INFINITY' i.e. the worst possible cost
        } 
        preceding_node.insert(node.clone(), None);
        //initially, there is no previous node for any of the nodes
    }

    //use a queue to track nodes for exploration
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back(start_node.clone());

    //while loop continues until queue is empty
    while let Some(current_node) = queue.pop_front() {
        //save current cost to get to node ready for comparison below
        let current_cost = lowest_cost_to_node[&current_node];

        //get each neighbour of current_node and find the associated costs
        if let Some(neighbours) = weighted_graph.get(&current_node) {
            for (neighbour, weight) in neighbours {
                let new_cost = current_cost + weight;

                //check if new cost is less than the current known lowest cost to the neighbour
                if new_cost < lowest_cost_to_node[neighbour] {
                    //if the new cost is lower, update our tracking HashMaps
                    lowest_cost_to_node.insert(neighbour.clone(), new_cost);
                    preceding_node.insert(neighbour.clone(), Some(current_node.clone()));
                    
                    //add neighbour to queue
                    queue.push_back(neighbour.clone());
                }
            } 
        }
    }

    //build shortest path from 'target_node' to 'start_node'
    let mut path = Vec::new();
    let mut current_node = Some(target_node.clone());

    while let Some(node) = current_node {
        path.push(node.clone());
        current_node = preceding_node[&node].clone();
    }

    // the path is built from end to start, so reverse it to correct order
    path.reverse();

    // return the path and the cost to the target node
    //*dereferences the resulting value of the operations here (i.e. so we get a f64 not a &f64)
    //'lowest_cost_to_node.get(&target_node)' gets the cost of getting to 'target_node'
    //'.unwrap_or(&f64::INFINITY) gives an alternative, so that if the '.get()' fails (i.e. there is no path) then the cost is given as infinity i.e. it is impossible 
    let cost_to_target = *lowest_cost_to_node.get(&target_node).unwrap_or(&f64::INFINITY);
    
    (path, cost_to_target)
}