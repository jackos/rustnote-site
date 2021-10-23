#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn main() {
    let mut list_of_statuses: Vec<Status> = (0u32..20).map(|i| Status::Value(i)).collect();
    list_of_statuses.push(Status::Stop);
    println!("{:?}", list_of_statuses);
}
