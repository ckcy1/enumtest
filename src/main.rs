//
//
// enum Cion {
//     yifen,
//     wufen,
//     yijiao,
//     yiyuan,
// }
// impl Cion{
//     fn shucion(cion:Cion){
// // match cion {
// //     Cion::yifen =>{
// //         println!("一分前")
// //     }
// //     Cion::wufen =>{
// //         println!("五分前")
// //     }
// //     _ =>{
// //         println!("无钱")
// //     }
// // }
//         if let cion = Cion::yifen {
//             println!("一分钱")
//         }
//     }
// }


fn main() {
    // let yi = Cion::yifen;
    // Cion::shucion(yi);
    //
    let x:u32=8;
    let y:Option<u32>=None;
    println!("{}",x+y.unwrap_or(0));

    println!("Hello, world!");
}
