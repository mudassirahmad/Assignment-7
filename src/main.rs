mod hospital{
    pub mod patient{
        pub fn patient_details(){
            println!("this is patient submodule");
        }
    }
}
fn main() {
    crate::hospital::patient::patient_details();
}
