# Data Structures
Collection of data structures implemented in Rust programming language, intended for educational purposes.

|            | get(i)         | insert(i)          | remove(i)          | append   | split_off(i)        |
|------------|----------------|--------------------|--------------------|----------|---------------------|
| Vec        | O(1)           | O(n-i)\*           | O(n-i)             | O(m)\*   | O(n-i)              |
| VecDeque   | O(1)           | O(min(i, n-i))\*   | O(min(i, n-i))     | O(m)\*   | O(min(i, n-i))      |
| LinkedList | O(min(i, n-i)) | O(min(i, n-i))     | O(min(i, n-i))     | O(1)     | O(min(i, n-i))      |
| HashMap    | O(1)~          | O(1)~*\            | O(1)~              | N/A      | N/A                 |
| HashSet    | O(1)~          | O(1)~*\            | O(1)~              | N/A      | N/A                 |
| BTreeMap   | O(log(n))      | O(log(n))          | O(log(n))          | O(log(n))| O(n+m)              |

## Stack (LIFO)
	┌───┬───┬───┬───┬───┐
	│ a │ b │ c │ d │ e │
	└───┴───┴───┴───┴───┘
	                  ↑
	                 Push & Pop

Stack is an abstract data type that serves as a collection of elements, with two main operations:
- Push, which adds an element to the collection, and
- Pop, which removes the most recently added element that was not yet removed.
- [Stack in Wiki](https://en.wikipedia.org/wiki/Stack_(abstract_data_type))

## Queue (FIFO)
	┌───┬───┬───┬───┬───┐
	│ e │ d │ c │ b │ a │
	└───┴───┴───┴───┴───┘
	  ↑               ↑
	Push             Pop

Queue is a collection of entities that are maintained in a sequence and can be modified by the addition of entities at one end of the sequence and the removal of entities from the other end of the sequence.
- [Queue in Wiki](https://en.wikipedia.org/wiki/Queue_(abstract_data_type))

## Deque
	┌───┬───┬───┬───┬───┐
	│ e │ d │ c │ b │ a │
	└───┴───┴───┴───┴───┘
	  ↑               ↑
	Push & Pop       Push & Pop

Double-ended queue is an abstract data type that generalizes a queue, for which elements can be added to or removed from either the front (head) or back (tail).
- [Double ended queue in Wiki](https://en.wikipedia.org/wiki/Double-ended_queue)

## Circular Buffer / Ring Buffer
	┌───┬───┬───┬───┬───┐
	│   │ a │ b │ c │   │
	└───┴───┴───┴───┴───┘
        ↑       ↑
      Tail     Head

Circular buffer is a FIFO data structure that treats memory to be circular. This is achieved by two pointers to the array, the “head” pointer and the “tail” pointer. As data is added (write) to the buffer, the head pointer is incremented and likewise, when the data is being removed (read) the tail pointer is incremented.
- [Circular buffer in Wiki](https://en.wikipedia.org/wiki/Circular_buffer)
- [Implementing Circular Buffer in C](https://embedjournal.com/implementing-circular-buffer-embedded-c/)

## Linked List
	┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐
	│ a │→│ b │→│ c │→│ d │→│ e │
	└───┘ └───┘ └───┘ └───┘ └───┘

The principal benefit of a linked list over a conventional array is that the list elements can be easily inserted or removed without reallocation or reorganization of the entire structure.

- [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/index.html).
- [XOR Linked List in Wiki](https://en.wikipedia.org/wiki/XOR_linked_list)

## Skip List
	┌───┐                   ┌───┐
	│   │→                  │ d │
	└───┘                   └───┘
	┌───┐       ┌───┐       ┌───┐
	│   │→      │ b │→      │ d │
	└───┘       └───┘       └───┘
	┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐
	│   │→│ a │→│ b │→│ c │→│ d │
	└───┘ └───┘ └───┘ └───┘ └───┘

Skip list is a probabilistic data structure that allows  O(log n) average complexity for search as well as O(log n) average complexity for insertion within an ordered sequence of n elements. Thus it can get the best features of a sorted array (for searching) while maintaining a linked list-like structure that allows insertion, which is not possible with a static array.
-[Skip list in Wiki](https://en.wikipedia.org/wiki/Skip_list)

## Hash Table (Hash Map)
	┌───┬───┬───┬───┬───┐
	│ 0 │ 1 │ 2 │ 3 │ 4 │
	└───┴───┴───┴───┴───┘
    ↓       ↓       ↓
	┌───┐   ┌───┐   ┌───┐
	│ b │   │ c │   │ a │
	└───┘   └───┘   └───┘
Hash table is a data structure that implements an associative array or dictionary. It is an abstract data type that maps keys to values. A hash table uses a hash function to compute an index, into an array of buckets or slots. During lookup, the key is hashed and the resulting hash indicates where the corresponding value is stored.

Ideally, the hash function will assign each key to a unique bucket, but most hash table designs employ an imperfect hash function, which might cause hash collisions where the hash function generates the same index for more than one key

### Collision resolution
- **Separate chaining**. Linked list or array with key–value pair for each search array index. The collided items are chained together, which can be traversed to access the item with a unique search key.
- **Open addressing**. Every entry record is stored in the bucket array itself, and the hash resolution is performed through probing.
	- **Linear probing**, in which the interval between probes is fixed (usually 1).
	- **Quadratic probing**, in which the interval between probes is increased by adding the successive outputs of a quadratic polynomial to the value given by the original hash computation.
	- **Double hashing**, in which the interval between probes is computed by a secondary hash function.

- [Hash table in Wiki](https://en.wikipedia.org/wiki/Hash_table)

## Binary Tree
Binary tree is tree data structure in which each node has at most two children, which are referred to as the left child and the right child.

For most computer science applications, binary trees are rooted: A special node, `r` of degree at most two is called the root of the tree. For every node, `u != r`, the second node on the path from `u` to `r` is called the parent of `u`. Each of the other nodes adjacent to `u` is called a child of `u`. Most of the binary trees we are interested in are ordered, so we distinguish between the left child and right child of `u`.

Terminology:
- Depth of a node - lenght of the path from `u` to `r`
- Ancestor & descendant nodes
- Subtree subtree of a node, `u`, is the binary tree that is rooted at `u` and contains all of `u`'s descendants
- A node, `u`, is a **leaf** if it has no children

- [Binary tree in Wiki](https://en.wikipedia.org/wiki/Binary_tree)

## Graphs

## Rootish Array Stack
	┌───┐   ┌───┐
	│ 1 │ → │ a │
	└───┘   └───┘
	┌───┐   ┌───┬───┐
	│ 2 │ → │ b │ c │
	└───┘   └───┴───┘
	┌───┐   ┌───┬───┬───┐
	│ 3 │ → │ d │ e │ f │
	└───┘   └───┴───┴───┘

A Rootish Array Stack is an ordered array based structure that minimizes wasted space (based on Gauss's summation technique). A Rootish Array Stack consists of an array holding many fixed size arrays in ascending size.
- [Rootish Array Stack](https://aquarchitect.github.io/swift-algorithm-club/Rootish%20Array%20Stack/)

## Links
- [Open Data Structures](https://opendatastructures.org/)
