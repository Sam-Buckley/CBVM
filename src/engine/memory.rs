pub struct Heap {
    memory: Vec<u8>,
    allocated: Vec<(usize, usize)>,
    allocated_size: usize,
}

impl Heap {
    pub fn new(size: usize) -> Heap {
        Heap {
            memory: vec![0; size],
            allocated: vec![],
            allocated_size: 0,
        }
    }
    //write functions to allocate, realloc, free, read and write
    //they should take into account allignment, padding, size etc
    fn find_available_space(&self, size: usize) -> Option<usize> {
        let mut start = 0;

        for &(allocated_start, allocated_end) in &self.allocated {
            if allocated_start - start >= size {
                return Some(start);
            }
            start = allocated_end + 1;
        }

        if self.memory.len() - start >= size {
            Some(start)
        } else {
            None
        }
    }
    pub fn allocate(&mut self, size: usize) -> Result<usize, ()> {
        //find a space in memory that is big enough to fit the data
        //if there is no space, return an error
        //otherwise, allocate the space and return the pointer
        if let Some(start) = self.find_available_space(size) {
            let end = start + size-1;
            self.allocated.push((start, end));
            //fill the allocated space with 0s
            for i in start..end {
                self.memory[i] = 0;
            }
            return Ok(start);
        }
        else {
            segfault(self.memory.len(), "Allocating OOB".to_string(), "allocate");
            return Err(());
        }
    }
    pub fn realloc(&mut self, pos: usize, size: usize) -> Result<usize, ()> {
        //check if the extra space is available next to the allocated space, if it is, allocate it
        //if it isn't, allocate a new space and copy the data over
        //if the size is smaller than the allocated space, free the extra space
        //if the size is 0, free the allocated space
        //if the size is the same, do nothing
        let (start, end) = self.allocated[pos];
        //check if memory from end to end+size is available
        if self.memory[end..end+size].iter().all(|&x| x == 0){
            //if it is, allocate it
            self.allocated[pos] = (start, end+size);
            //fill the allocated space with 0s
            for i in start..end+size {
                self.memory[i] = 0;
            }
            return Ok(start);
        }
        else {
            //otherwise, allocate a new space and copy the data over
            let new_pos = self.allocate(size).unwrap();
            for i in start..end {
                self.memory[new_pos+i] = self.memory[i];
            }
            //free the old space
            self.free(pos).unwrap();
            return Ok(new_pos);
        }
    }
    pub fn write(&mut self, pos: usize, data: u8) -> Result<(), ()> {
        //if pos is within allocated memory, write to it
        for (start, end) in self.allocated.iter() {
            if pos >= *start && pos <= *end {
                self.memory[pos] = data;
                return Ok(())
            }
        }
        //otherwise segfault
        segfault(pos, "out of bounds".to_string(), "write");
        return Err(());
    }
    pub fn read(&self, pos: usize, size: usize) -> Result<Vec<u8>, ()> {
        //read from pos to pos+size
        let mut data = Vec::new();
        for i in &self.memory[pos..pos+size] {
            data.push(*i);
        }
        return Ok(data);
    }
    pub fn sizeof(&self, pos: usize) -> Result<usize, ()> {
        //return the size of the allocated space at pos
        for (start, end) in self.allocated.iter() {
            if pos >= *start && pos <= *end {
                return Ok((end-start+1) * 8);
            }
        }
        segfault(pos, "out of bounds".to_string(), "sizeof");
        return Err(());
    }
    pub fn free(&mut self, pos: usize) -> Result<(), ()> {
        if let Some(start) = self.allocated.iter().position(|&(start, end)| pos >= start && pos <= end) {
            let range = self.allocated.remove(start);
            for i in range.0..=range.1 {
                self.memory[i] = 0;
            }
            Ok(())
        } else {
            segfault(pos, "out of bounds".to_string(), "free");
            Err(())
        }
    }
    
    
}

fn segfault(position: usize, reason: String, func: &str) {
    //create a detailed message as to why the segfault occured and where, with color
    let red = "\x1b[31m";
    let reset = "\x1b[0m";
    println!("{}Segmentation fault{} at {}, position: {}{}{}", red, reset, func, red, position, reset);
    println!("Reason: {}", reason);
    std::process::exit(0)
}

pub struct Stack {
    memory: Vec<u8>,
    ptr: usize,
}
impl Stack {
    pub fn new() -> Stack {
        Stack {
            memory: vec![0; 8192],
            ptr: 0,
        }
    }
}

impl Stack {
    pub fn push(&mut self, data: u8) {
        self.memory[self.ptr] = data;
        self.ptr += 1;
    }
    pub fn pop(&mut self) -> u8 {
        self.ptr -= 1;
        self.memory[self.ptr]
    }
    pub fn peek(&self) -> u8 {
        self.memory[self.ptr-1]
    }
    pub fn get(&self, offset: usize) -> u8 {
        self.memory[self.ptr-offset]
    }
    pub fn swap(&mut self) {
        let temp = self.memory[self.ptr-1];
        self.memory[self.ptr-1] = self.memory[self.ptr-2];
        self.memory[self.ptr-2] = temp;
    }
    pub fn dup(&mut self) {
        self.memory[self.ptr] = self.memory[self.ptr-1];
        self.ptr += 1;
    }
    pub fn drop(&mut self) {
        self.memory[self.ptr] = 0;
        self.ptr -= 1;
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}
impl Default for Heap {
    fn default() -> Self {
        Self::new(8192)
    }
}