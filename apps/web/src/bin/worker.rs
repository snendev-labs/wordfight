use gloo_worker::Registrable;
use wordfight_web::BevyWorker;

fn main() {
    console_error_panic_hook::set_once();

    BevyWorker::registrar().register();
}
