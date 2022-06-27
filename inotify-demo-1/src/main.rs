use inotify::{Inotify, WatchMask};


fn main() {
    
    let mut inotify = Inotify::init()
        .expect("ERROR -> initial inotify instance");

    inotify
        .add_watch(
            "./test.log",
            WatchMask::MODIFY | WatchMask::CLOSE
        )
        .expect("Failed to add file watch !");
    
    let mut buffer = [0; 1024];
    let events = inotify.read_events_blocking(&mut buffer)
        .expect("ERROR -> while reading events");

    for event in events {
        println!("{:?}", event.name);
    }
}

