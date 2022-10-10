use neon::prelude::*;
use rfd::FileDialog;
use zip::ZipArchive;
use zip::read::ZipFile;
use std::fs::File;
use std::io::{BufReader, Read};

static mut extract: bool = false;
static mut num: i32 = -1;
static mut out_path: String =  String::new();

struct ZipTest{
}

static mut zipfile: Option<zip::ZipArchive<BufReader<File>>> = None;

fn get_zip_file(zip: &mut ZipArchive<BufReader<File>>, index: i32) -> ZipFile{
    zip.by_index(index as usize).unwrap()
}

impl ZipTest{
    fn new() -> ZipTest{
        return ZipTest{}
    }
    fn select_zip(mut cx: FunctionContext) -> JsResult<JsArray> {
        let array = cx.empty_array();
     //   let channel = cx.channel();
        let file = FileDialog::new()
        .set_directory("/")
        .add_filter("Zip Files", &["zip"])
        .pick_file();
        if let Some(path) = file{
            let f = File::open(path.clone()).unwrap();
            let mut reader = BufReader::new(f);
            let mut zip = zip::ZipArchive::new(reader);
            if zip.is_ok(){
                unsafe{zipfile = Some(zip.unwrap());              
            }
    }
    else{
        return cx.throw_error(zip.unwrap_err().to_string());
    }
    unsafe{
                            for i in 0..zipfile.as_ref().unwrap().len() {
                                let zipf = zipfile.as_mut().unwrap();
                                let mut file = get_zip_file(zipf, i as i32);
                                let name = cx.string(file.name());
                                array.set(&mut cx, i as u32, name);
                            }    
                        }
                    }
                    Ok(array)
                }
    fn extract_zip(mut cx: FunctionContext) -> JsResult<JsNumber>{
        unsafe{
            let index = cx.argument::<JsNumber>(0).unwrap_or(cx.number(0));

            let mut file = zipfile.as_mut().unwrap().by_index(index.value(&mut cx) as usize).unwrap();
            if file.is_dir(){
                let dir_path = FileDialog::new()
            .set_directory("/")
            .set_file_name(file.name())
            .pick_folder();
            let mut is_dir = false;
            let mut i = index.value(&mut cx) as i32 + 1;
            std::fs::create_dir_all(dir_path.as_ref().unwrap().join(file.name().replace("/", "\\")));
            while is_dir == false{
                let mut file = zipfile.as_mut().unwrap().by_index(i as usize).unwrap();
                let path = dir_path.as_ref().unwrap().join(file.name().replace("/", "\\"));
                let mut vec = Vec::new();
            file.read_to_end(&mut vec);
            std::fs::write(path, vec);
            i+=1;
            is_dir = file.is_dir();
            }
            }
            else{
                let file_path = FileDialog::new()
            .set_directory("/")
            .set_file_name(file.name())
            .save_file();
            let mut vec = Vec::new();
            file.read_to_end(&mut vec);
            if file_path.is_some(){
                std::fs::write(file_path.unwrap(), vec);
            }
            }
            Ok(cx.number(0)) 
        }
      
    }
    fn extract_all(mut cx: FunctionContext) -> JsResult<JsNumber>{
        unsafe{
            let index = cx.argument::<JsNumber>(0).unwrap_or(cx.number(0));

            let mut file = zipfile.as_mut().unwrap().by_index(index.value(&mut cx) as usize).unwrap();
            let dir_path = FileDialog::new()
            .set_directory("/")
            .set_file_name(file.name())
            .pick_folder();
            for i in 0..zipfile.as_ref().unwrap().len() {
                if dir_path.is_none(){
                    break;
                }
                let zipf = zipfile.as_mut().unwrap();
                let mut file = get_zip_file(zipf, i as i32);
                let path = dir_path.as_ref().unwrap().join(file.name().replace("/", "\\"));

                if file.is_dir(){
                    std::fs::create_dir_all(dir_path.as_ref().unwrap().join(file.name().replace("/", "\\")));
                }
                else{
                    let mut vec = Vec::new();
            file.read_to_end(&mut vec);
            std::fs::write(path, vec);
                }
            }
            Ok(cx.number(0)) 
      
    }
}
}


fn rust_print(mut cx: FunctionContext) -> JsResult<JsString>{
    let text = cx.argument::<JsString>(0).unwrap();
    let text = text.value(&mut cx);
    println!("{text}");
    Ok(cx.string(""))
}



#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("select_zip", ZipTest::select_zip)?;
    cx.export_function("extract_zip", ZipTest::extract_zip)?;
    cx.export_function("extract_all", ZipTest::extract_all)?;
    cx.export_function("rust_print", rust_print)?;
    Ok(())
}