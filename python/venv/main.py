from collections import deque
import math

# MISCELLANEOUS HELPER FUNCTIONS

# Function checks for ordered lists
def check_list_ordered(lst):
    for i in range(len(lst) - 1):
        if lst[i] > lst[i + 1]:
            print(f"List not ordered, {lst[i]} is before and larger than {lst[i + 1]}")
            return False  # Return False if the list is not ordered
    return True  # Return True if the list is ordered


# Function to find the smallest element in a list
def find_smallest_in_list(lst):
    smallest = lst[0]
    index_smallest = 0

    for index, element in enumerate(lst):
        if element < smallest:
            smallest = element
            index_smallest = index
    return [smallest, index_smallest]

# Function to convert a list from ascending to descending order
def asc_to_desc_list(lst):
    lst.reverse()  # Reverse the list in place
    return lst


# ALGORITHMS THAT ORDER AN ARRAY
def selection_sort(unsorted_list):
    sorted_list = []

    while len(unsorted_list) > 0:
        smallest_el, smallest_ind = find_smallest_in_list(unsorted_list)
        sorted_list.append(smallest_el)  # Append the smallest element to sorted_list
        unsorted_list.pop(smallest_ind)  # Remove the smallest element from unsorted_list
    return sorted_list

def quicksort(unsorted_list):
    if len(unsorted_list) < 2:
        return unsorted_list

    left = []
    right = []
    pivot_index = len(unsorted_list) // 2
    pivot = unsorted_list[pivot_index]

    for index, element in enumerate(unsorted_list):
        if index != pivot_index:
            if element < pivot:
                left.append(element)
            else:
                right.append(element)

    return quicksort(left) + [pivot] + quicksort(right)  # Concatenate sorted left, pivot, and sorted right


# ALGORITHMS THAT REQUIRE AN ORDERED LIST
def binary_search(ordered_list, target):
    if len(ordered_list) == 0:
        print("List is empty")
        return

    if not check_list_ordered(ordered_list):  # Ensure the list is ordered before searching
        print("List is not ordered")
        return

    bottom_index = 0
    top_index = len(ordered_list) - 1

    while bottom_index <= top_index:
        index_to_check = (bottom_index + top_index) // 2 # // is integer division in python

        if target == ordered_list[index_to_check]:
            print(f"{target} is at index {index_to_check} of the list")
            return
        elif ordered_list[index_to_check] < target:
            bottom_index = index_to_check + 1
        else:
            top_index = index_to_check - 1

    print(f"{target} is not in the list")


# GRAPH ALGORITHMS
# dict is a hashmap in Python
def breadth_first_search(graph, start):
    queue = deque()
    checked = set()  # Use a set to keep track of checked nodes

    queue.append(start)  # Add start node to the queue

    while queue:
        node_name = queue.popleft()  # Remove the front element from the queue

        if node_name in checked:  # Check if the node has already been checked
            continue
            
        checked.add(node_name)  # Mark the node as checked

        neighbours = graph.get(node_name, [])
        for neighbour in neighbours: 
            if neighbour not in checked:
                queue.append(neighbour)  # Add neighbour to the queue

def dijkstras_algorithm(weighted_graph, start_node, target_node):
    lowest_cost_to_node = {node: float('inf') for node in weighted_graph.keys()}
    preceding_node = {node: None for node in weighted_graph.keys()}

    # Populate the HashMaps with initial values:
    lowest_cost_to_node[start_node] = 0

    queue = deque()
    queue.append(start_node)

    while queue: 
        current_node = queue.popleft()
        current_cost = lowest_cost_to_node[current_node]
        neighbours = weighted_graph.get(current_node, [])

        for neighbour, weight in neighbours:
            new_cost = current_cost + weight

            # Check if new cost is less than the current known lowest cost to the neighbour
            if new_cost < lowest_cost_to_node[neighbour]:
                lowest_cost_to_node[neighbour] = new_cost
                preceding_node[neighbour] = current_node
                queue.append(neighbour) 

    # Build shortest path from 'target_node' to 'start_node'
    path = []
    current_node = target_node 

    while current_node is not None: 
        path.append(current_node)
        current_node = preceding_node[current_node] 

    path.reverse()  # Reverse the path to get from start to target
    return path
