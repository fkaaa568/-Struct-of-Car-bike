fn main() {
#[derive(Debug)]
struct challanDetails{
    Name:String,
    NIC:String,
    CarBikeNo:String,
    vehicleType:String,
}
impl challanDetails{
    fn Createchallan(Name:String,NIC:String,CarBikeNo:String,vehicleType:String)->challanDetails{
        challanDetails{
            Name,
            NIC,
            CarBikeNo,
            vehicleType,
        }
    }


fn challanRupee(&self) -> i32{
    if &self.vehicleType == "Car"{
        500
    }
    else{
        200
    }
}
}
use std::io;
fn main(){
    let mut Name = String::new();
    let mut NIC = String::new();
    let mut CarBikeNo = String::new();
    let mut vehicleType = String::new();

    println!("Enter The Name");
    io::stdin().read_line(&mut Name).expect("");

    println!("Enter The NIC");
    io::stdin().read_line(&mut NIC).expect("");

    println!("Enter The Vehicle Number");
    io::stdin().read_line(&mut CarBikeNo).expect("");

    println!("Enter The Vehicle Type");
    io::stdin().read_line(&mut vehicleType).expect("");

    let ch_01 = challanDetails::Createchallan(Name.trim().to_string(),NIC.trim().to_string(),CarBikeNo.trim().to_string(),vehicleType.trim().to_string());
    println!("{:?}",ch_01);
    println!("The Challan Rupees is {}",ch_01.challanRupee());
}

}

