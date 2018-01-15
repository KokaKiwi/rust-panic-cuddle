use std::panic;

pub fn cuddle() -> Box<Fn(&panic::PanicInfo) + Sync + Send + 'static> {
    let current_hook = panic::take_hook();

    Box::new(move |info| {
        println!("aww *hug*");

        if let Some(msg) = info.payload().downcast_ref::<&str>() {
            println!("You left a message, that's so nice of you: {}", msg);
        }

        (*current_hook)(info)
    })
}
