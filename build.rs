// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // tonic_build::compile_protos("proto/helloworld.proto")?;
//     // tonic_build::compile_protos("/Users/hshamji/Desktop/repos/to_delete/helloworld-tonic/proto/helloworld.proto")?;
//     tonic_build::configure()
//         .compile(&["proto/helloworld.proto"], &["[proto"])?;
//     Ok(())
// }

// fn main(){
//     tonic_build::configure()
//         .out_dir("proto/out")
//         .compile(
//             &["/Users/hshamji/Desktop/repos/to_delete/helloworld-tonic/proto/helloworld.proto"],
//             &["/Users/hshamji/Desktop/repos/to_delete/helloworld-tonic/proto"]
//         ).expect("proto compilation error");
// }

fn main() {
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .out_dir("proto/out")
        .compile(
            &["/Users/hshamji/Desktop/repos/to_delete/helloworld-tonic/proto/tensorflow_serving/apis/prediction_service.proto"],
            &["/Users/hshamji/Desktop/repos/to_delete/helloworld-tonic/proto"],
            // &["/Users/hshamji/Desktop/repos/to_delete/helloworld-tonic/proto/tensorflow/tsl/protobuf/error_codes.proto"],
            // &["/Users/hshamji/Desktop/repos/to_delete/helloworld-tonic/proto/tensorflow/tsl/protobuf"],
            // &["/Users/hshamji/Desktop/repos/to_delete/helloworld-tonic/proto/helloworld.proto"],
            // &["/Users/hshamji/Desktop/repos/to_delete/helloworld-tonic/proto"],
        ).unwrap();
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     tonic_build::configure()
//         .build_server(false)
//         .compile(
//             &["proto/helloworld.proto"],
//             &["proto"],
//         )?;
//     Ok(())
// }
