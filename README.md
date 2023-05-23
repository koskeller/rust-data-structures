Collection of data structures implemented in Rust programming language, intended for educational purposes.

## Learning recources
- [Open Data Structures](https://opendatastructures.org/)

### Circular Buffer / Ring Buffer
Circular buffer is a FIFO data structure that treats memory to be circular; that is, the read/write indices loop back to 0 after it reaches the buffer length. This is achieved by two pointers to the array, the “head” pointer and the “tail” pointer. As data is added (write) to the buffer, the head pointer is incremented and likewise, when the data is being removed (read) the tail pointer is incremented. The definition of head, tail, their movement direction and write and read location are all implementation dependent but the idea/goal remains the same.

- [Circular buffer in Wikipedia](https://en.wikipedia.org/wiki/Circular_buffer)
- [Implementing Circular Buffer in C](https://embedjournal.com/implementing-circular-buffer-embedded-c/)

### Deque
- [Double ended queue in Wikipedia](https://en.wikipedia.org/wiki/Double-ended_queue)

### Linked Lists
The principal benefit of a linked list over a conventional array is that the list elements can be easily inserted or removed without reallocation or reorganization of the entire structure.
- [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/index.html).
- [XOR Linked List in Wikipedia](https://en.wikipedia.org/wiki/XOR_linked_list)

### Rootish Array Stack
A Rootish Array Stack is an ordered array based structure that minimizes wasted space (based on Gauss's summation technique). A Rootish Array Stack consists of an array holding many fixed size arrays in ascending size.

- [Rootish Array Stack](https://aquarchitect.github.io/swift-algorithm-club/Rootish%20Array%20Stack/)