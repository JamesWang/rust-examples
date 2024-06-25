pub const EPOLL_CTL_ADD: i32 = 1;

//a bitflat indicates a read operation
pub const EPOLLIN: i32 = 0x1;

//a bitflat indicates we want to get events notified with epoll set to an edge-triggered mode
//that is,receive evdent once
pub const EPOLLET: i32 = 1 << 31;

#[link(name = "c")]
extern "C" {
    //create epoll queue
    pub fn epoll_create(size: i32) -> i32;

    //close the file descriptor
    pub fn close(fd: i32) -> i32;
    
    //perform operations on our empoll instance
    pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut Event) -> i32;

    //block the current thread and wait until one of the two things happens:
    //1. receive notification that an event has ocurled or 
    //2. times out
    pub fn epoll_wait(epfd: i32, events: *mut Event, maxevents: i32, timeout: i32) -> i32;
}

#[derive(Debug)]
#[repr(C, packed)] //tells struct will have padding or not, packed means no padding
pub struct Event{
    pub (crate) events: u32,
    pub (crate) epoll_data: usize,
}

impl Event {
    pub fn token(&self) ->usize {
        self.epoll_data
    }
}