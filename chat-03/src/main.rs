
fn main() {
    let alice = User {
        id: UserId(1),
        name: "Alice".into(),
        gender: Gender::Female,
    };

    let bob = User {
        id: UserId(2),
        name: "Bob".into(),
        gender: Gender::Male,
    };

    let topic = Topic {
        id: TopicId(1),
        name: "rust".into(),
        owner: UserId(1),
    };

    let event_join = Event::Join((alice.id, topic.id));
    let event_leave = Event::Leave((alice.id, topic.id));
    let event_message = Event::Message((alice.id, topic.id, "Hello world !".into()));

    println!("alice: {:?}", alice);
    println!("bob: {:?}", bob);

    println!("e_j: {:?}, e_l: {:?}, e_m {:?}", event_join, event_leave, event_message);
}

#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Male = 1,
    Female = 2,
}


#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}


