use futures::executor::block_on;

async fn func1() {
    println!("hello from func1")
}

async fn func2() {
    println!("hello from func2")
}

async fn all() {
    futures::join!(func1(), func2());
}


fn main() {
    block_on(all());
}
