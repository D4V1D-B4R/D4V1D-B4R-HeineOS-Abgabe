/*
 * A heap allocator that uses a linked list to manage free memory blocks.
 * It allows for dynamic memory allocation and deallocation.
 *
 * Author: Philipp Oppermann, https://os.phil-opp.com/allocator-designs/ <-- !!!
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 2026-01-13
 */

use core::mem;

use alloc::alloc::{GlobalAlloc, Layout};
use crate::allocator::global::{align_up, Locked};

/// Header of a free block in the list allocator.
struct ListNode {
    /// Size of the memory block
    size: usize,

    /// &'static mut type semantically describes an owned object behind a pointer.
    /// Basically, it’s a Box without a destructor that frees the object at the end of the scope.
    /// Its lifetime is static, meaning it will live for the entire duration of the program.
    /// Of course, this is not true in reality, as we might delete the list node at some point.
    /// But the compiler does not know this.
    next: Option<&'static mut ListNode>,
}

impl ListNode {
    /// Create a new ListNode with the given size and no next node.
    const fn new(size: usize) -> Self {
        ListNode { size, next: None }
    }

    /// Get the start address of the memory block.
    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    /// Get the end address of the memory block.
    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}

/// A linked list allocator that uses a free list to manage memory.
pub struct LinkedListAllocator {
    head: ListNode,
    heap_start: usize,
    heap_end: usize,
}

impl LinkedListAllocator {
    /// Create a new empty linked list allocator.
    pub const fn new() -> LinkedListAllocator {
        LinkedListAllocator {
            head: ListNode::new(0),
            heap_start: 0,
            heap_end: 0,
        }
    }

    /// Initialize the allocator with the heap bounds given in the constructor.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        unsafe { self.add_free_block(heap_start, heap_size) };
    }

    /// Adds the given free memory block 'addr' TO THE FRONT OF THE FREE LIST.
    unsafe fn add_free_block(&mut self, addr: usize, size: usize) {
        let mut new_node = ListNode::new(size);     // create new node of appropiate size
        new_node.next = self.head.next.take();                // take next node pointer out of head (will be replaced with new_node)
        let node_addr = addr as *mut ListNode; // given address for the new node

        unsafe {
            node_addr.write(new_node);          // writes the new_node at given address 
            self.head.next = Some(&mut *node_addr); // adds said node to the head (head -> new_node -> head.next -> ...)
        }
    }

    /// Search a free block with the given size and alignment and remove it from the list.
    fn find_free_block(&mut self, size: usize, align: usize) -> Option<(&'static mut ListNode, usize)> {
       let mut current = &mut self.head;
        //check nodes that follow current one until "None" is reached (end of list)
        while let Some(potential_free_block) = current.next.as_mut() {
            // if it is big enough...
            if let Ok(block_addr) = LinkedListAllocator::check_block_for_alloc(potential_free_block, size, align) {
                let next = potential_free_block.next.take();    // save successor to restore list
                let free_block = Some((current.next.take().unwrap(), block_addr)); // construct return value
                current.next = next;    // set successor of current list to next, removing the desired block from the lsit

                return free_block; // return free block + address of it
            }
            // if not big enough, continue search in next node
            current = current.next.as_mut().unwrap();
        }
        // nothing found
        None
    }

    /// Check if the given block is large enough for an allocation with `size` and `align`.
    fn check_block_for_alloc(block: &ListNode, size: usize, align: usize) -> Result<usize,()> {
        let block_addr = align_up(block.start_addr(), align);
        let block_end = block_addr + size;

        if block_end > block.end_addr() {
            return Err(());
        }

        let rest_size =  block.end_addr() - block_end;
        // check if there is a rest that it is usable
        if rest_size > 0 && rest_size < mem::size_of::<ListNode>() {
            return Err(());
        }

        Ok(block_addr)
    }

    /// Adjust the given layout so that the resulting allocated memory
    /// block is also capable of storing a `ListNode`.
    fn size_align(layout: Layout) -> (usize, usize) {
        let layout = layout
            .align_to(align_of::<ListNode>())
            .expect("adjusting alignment failed")
            .pad_to_align();
        let size = layout.size().max(size_of::<ListNode>());

        (size, layout.align())
    }

    /// Dump the free list for debugging purposes.
    pub fn dump_free_list(&mut self) {
        let mut current = &mut self.head;
        let mut i = 0;
        println!("Free Blocks: ");
        while let Some(viewed_block) = current.next.as_mut() {
            println!("______________________________");
            println!("Block No. {}", i);
            println!("  Address: {}", viewed_block.start_addr());
            println!("  End: {}", viewed_block.end_addr());
            println!("  Size: {}", viewed_block.size);
            println!("______________________________");
            println!("              |               ");
            println!("              V               ");

            i += 1;
            current = current.next.as_mut().unwrap();
        }
        println!("              X               ");
    }

    /// Allocate memory of the given size and alignment.
    pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let (size, align) = LinkedListAllocator::size_align(layout);

        // if some fitting block was found...
        if let Some((free_block, block_addr)) = self.find_free_block(size, align) {

            let alloc_end = block_addr + size; // address up to which everything was allocated...
            let rest_size = free_block.end_addr() - alloc_end; //...then the rest is empty and can be used

            if rest_size > 0 {  // add said rest (if present) as a new free block to the allocator
                unsafe { self.add_free_block(alloc_end, rest_size); }
            }
            // the previous free block has alread been removed in find_free_block()

            block_addr as *mut u8 // return allocated address
        } 
        // nothing free and fitting found, return null (error)
        else {
            core::ptr::null_mut()
        }
    }

    /// Free the memory block at the given pointer with the given layout.
    pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        let size = LinkedListAllocator::size_align(layout).0;

        unsafe {
            self.add_free_block(ptr as usize, size) // just add the allocated block to the end of the free list again
        }
    }
}

// Trait required by the Rust runtime for heap allocations
unsafe impl GlobalAlloc for Locked<LinkedListAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            self.lock().alloc(layout)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            self.lock().dealloc(ptr, layout);
        }
    }
}