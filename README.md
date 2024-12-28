# Algo in Rust

![alt text](image.png)

## What does this repo contain?
Various algorithms written in Rust. Useful when learning competitive programming or basics of Rust.
- ### **Red Black Tree**
    It's a **self-balancing binary search tree** where each node has an additional color property (red or black) to ensure that the tree remains balanced during insertions and deletions. Its key properties include: every path from the root to a null pointer has the same number of black nodes, and no two consecutive red nodes can exist, guaranteeing O(log⁡n) time for search, insertion, and deletion operations.

    ## Usage
    - add a node  
        ```add <num>```
    - remove a node  
        ```remove <num>```
    - display tree   
        ```show```
    - print height of the tree  
        ```height```
    - print number of red nodes  
        ```red```
    - quit program  
        ```quit```
- ### **LCA**
    The Lowest Common Ancestor (LCA) algorithm is used to find the lowest (or deepest) node that is **an ancestor of two given nodes** in a tree. It has applications in various areas like network routing, file systems, and genealogy trees. Efficient LCA solutions often involve preprocessing the tree with techniques like binary lifting or sparse tables, allowing LCA queries to be answered in O(log⁡N) or O(1) time after O(Nlog⁡N) preprocessing.
- ### **Segment tree**
    A Segment Tree is a data structure used to **efficiently perform range queries and updates on an array**. It divides the array into segments and represents them in a binary tree format. Each node stores information about a segment of the array, such as its sum, minimum, or maximum. With a segment tree, operations like range queries and updates can be performed in O(log⁡N) time, making it suitable for scenarios where frequent updates and queries are required.
- ### **MergeSort**
    MergeSort is a **divide-and-conquer sorting algorithm that recursively splits the array into smaller subarrays** until each contains one element, then merges them back together in sorted order. It is stable, efficient for large datasets, and has a time complexity of O(nlog⁡n).
- ### QuickSort**
    QuickSort is a **divide-and-conquer sorting algorithm that selects a pivot element, partitions the array into two subarrays** (elements less than the pivot and elements greater than the pivot), and recursively sorts them. It is efficient with an average time complexity of O(nlog⁡n), but its worst-case complexity is O(n^2) if the pivot is poorly chosen.

## Wanna help Ferris learn Algo?
*Ferris smiles encouragingly*
1. Fork the repository.
2. Create a new branch  
    ```git checkout -b feature-branch```
3. Make your changes and commit them  
    ```git commit -am 'Add new feature'```
4. Push to your branch  
    ```git push origin feature-branch```
5. Create a pull request.

## Or test it locally?
```git clone https://github.com/mzums/algo_in_rust```

## Requirements
Make sure you have Rust installed before testing the projects locally.