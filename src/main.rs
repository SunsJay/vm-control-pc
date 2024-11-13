use vm_control::db::establish_connection;
use vm_control::init;

fn main() {

    init();
    let connection = &mut establish_connection();

}
