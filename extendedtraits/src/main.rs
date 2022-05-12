
trait OnOff {
    fn set_onoff(&self, b: bool) {
        println!("OnOff Default");
    }
}

trait Brightness {
    fn set_brightness(&self, brightness: i32) {
        println!("Brightness Default");
    }
}


// any type which implements 'Light' should implement 'OnOff' and 'Brightness' as well
trait Light: OnOff + Brightness {}


struct MyLight {
    state: bool
}



impl Light for MyLight {}

impl OnOff for MyLight {}


impl Brightness for MyLight {
    fn set_brightness(&self, brightness: i32) {
        println!("Brightness = {}", brightness);
    }
}



fn main() {

    let my_light = Box::new(MyLight {state: false});

    my_light.set_onoff(true);
    my_light.set_brightness(100);

}


