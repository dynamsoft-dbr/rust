# Dynamsoft Barcode Reader


![crates.io version](https://img.shields.io/badge/crates.io-v0.1.3-orange.svg?longCache=true)

FFI bindings to [Dynamsoft Barcode Reader SDK](https://www.dynamsoft.com/Products/Dynamic-Barcode-Reader.aspx). 

## License
Get the [trial license](https://www.dynamsoft.com/CustomerPortal/Portal/Triallicense.aspx).

## Contact Us
https://www.dynamsoft.com/Company/Contact.aspx

## Usage

Decode barcodes from an image file:

```rust
extern crate dbr;

use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::env;

use dbr::reader::*;

extern "C" fn callback(index: i32, barcode_type: *const c_char, barcode_value: *const c_char) {
    unsafe {
        println!(
            "Index {}, {}, {}",
            index,
            CStr::from_ptr(barcode_type).to_str().unwrap(),
            CStr::from_ptr(barcode_value).to_str().unwrap()
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please input an image file.");
        return
    }

    println!("Hello Dynamsoft Barcode Reader!");
    unsafe {
        register_callback(Some(callback));
        let image_file = CString::new(env::args().nth(1).expect("Missing argument")).unwrap();
        let license = CString::new("t0068NQAAAFKYHV9xSZDEThUtClXNzxXH9TLSj/vYcY8mSKa0RxaGw3qNynyAMJ9Ib8UPxzFsbAMIugqPO313BvfiOdmZFTY=").unwrap();
        c_decodeFile(image_file.as_ptr(), license.as_ptr());
    }

    println!("Bye!");
}
```

![Rust barcode reader](https://www.codepool.biz/wp-content/uploads/2018/10/rust-barcode.PNG)

## How to Customize the Package

1. Install [Dynamsoft Barcode Reader](https://www.dynamsoft.com/Downloads/Dynamic-Barcode-Reader-Download.aspx).

2. Get the [source code](https://github.com/dynamsoft-dbr/rust);

3. Copy `Dynamsoft\Barcode Reader <version number>\Components\C_C++\Redist\x64\DynamsoftBarcodeReaderx64.dll` to `platforms\win\DynamsoftBarcodeReaderx64.dll`. 
    
    Copy `Dynamsoft\Barcode Reader <version number>\Components\C_C++\Lib\DBRx64.lib` to `platforms\win\DBRx64.lib`

4. Add function definitions to `reader.h` and add function implementations to `reader.c`.

5. Generate `reader.rs` with [bindgen](https://github.com/rust-lang-nursery/rust-bindgen).

6. Add the local package to your Rust project:

    ```rust
    [dependencies]
    dbr = { path = "../dbr" }
    ```



