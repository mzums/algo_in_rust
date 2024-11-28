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