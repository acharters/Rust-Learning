
mod first_layer {
    pub mod pub_mod {
        pub fn pub_fun() {
            println!("pub_fun from first_layer");
        }
        
        fn priv_fun() {
            println!("this shouldn't print");
        }
    }

    mod priv_mod {
        pub fn func() {
            println!("this also shouldn't print");
        }
    }
}

use first_layer::pub_mod;

fn main() {
    first_layer::pub_mod::pub_fun();
    pub_mod::pub_fun();
    //these functions shouldn't work
    pub_mod::priv_fun();
    first_layer::priv_mod::func();
}
