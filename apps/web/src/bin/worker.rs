use gloo_worker::Registrable;
use wordfight_web::BevyWorker;

fn main() {
    console_error_panic_hook::set_once();

    let registrar = BevyWorker::registrar();
    registrar.register();
}
