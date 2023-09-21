use std::fs;
use std::collections::HashSet;


#[derive(Debug, Default)]
struct Manager { 

    root: String,
    dirs: HashSet<String>,
    files: HashSet<String>,
    ignore: HashSet<String>,
    targets: Vec<String>,
    depth: u8,
}
const DEFAULT_DEPTH: u8 = 15;

impl Manager { 
    
    pub fn remover_cmd(&self) -> String { 
        // let cmd = "rm -rf ";
        let mut cmds = String::new();
        for v in &self.targets { 
            cmds += format!("rm -rf {}\n", v).as_str();
        };

        cmds
        
    }


    pub fn do_work(& mut self) -> Result<(), String>  { 
        self.do_work_to_depth(self.depth)
    }
    fn do_work_to_depth(&mut self,mut depth: u8) -> Result<(), String> { 
        let mut _c = 0;
        let mut paths = vec![ self.root.clone() ];
        loop {
            if depth == 0 { break; }
            let mut cur_paths = vec![];
            for p in paths {
            let entrires = fs::read_dir(p).unwrap() ;
            for e in entrires {
                _c += 1;
                let e = e.unwrap().path();
                let fname = e.file_name().unwrap().to_str().unwrap();
                if self.ignore.contains(fname) { continue; }
                else if self.dirs.contains(fname) && e.is_dir() { 
                    self.targets.push( e.to_str().unwrap().to_owned());
                } else if self.files.contains(fname) && e.is_file() {
                    self.targets.push( e.to_str().unwrap().to_owned());
                } else if e.is_dir() { 
                     cur_paths.push(e.to_str().unwrap().to_owned()) 
                }
            }
            }
            paths = cur_paths;
            depth -= 1;
        }
        Ok(())
    }


    fn get_manager_from_args(args: Vec<String>) -> Manager { 


        let mut m = Manager::default();
        m.root = args[1].clone();
        let mut i = 2;
        let flag = is_flag(&args[i]);
        if !flag { 
            panic!("Expected on of -d -f -I -D after directory path");
        }
        
        loop {
            if i >= args.len() { break; }
            { 
                println!("{}", args[i]);
                if args[i] == "-h" {
                    i += 1;
                }
                else if args[i] == "-d" {
                    // m.dirs = get_args(&args, &mut i).unwrap();
                    let data = get_args(&args, &mut i).unwrap();
                    m.dirs = HashSet::from_iter(data.into_iter())
                } else if args[i] == "-f" {
                    // get_args(&args, &mut i).unwrap();
                    let data = get_args(&args, &mut i).unwrap();
                    m.files = HashSet::from_iter(data.into_iter())

                } else if args[i] == "-I" {
                    let data = get_args(&args, &mut i).unwrap();
                    m.ignore = HashSet::from_iter(data.into_iter())
                    // get_args(&args, &mut i).unwrap();
                } else if args[i] == "-D" {
                    i += 1;
                    m.depth = args[i].parse::<u8>().unwrap();
                    i += 1;
                } else { 
                    panic!("Unknown args");
                }
            }

        }
        if m.depth == 0 { 
            m.depth = DEFAULT_DEPTH;
        }
         
        m

    }



}


// flags
//

    // -d dirs
    // -f files
    // -I files to ignore
    // -D depth

const HELP_MSG: &'static str = "FDeleter help
Starting dir, directory to be searched
  -h Print this help message
  -d Name of directorys to be deleted
  -f Name of files to be deleted
  -I Directories or files to ignore
  -D depth, how many directories to recurse, default: 15

Args and usage:
[Directory to search] [FLAGS and addtional OPTIONS]
eg.-
  Fdeleter ./Programs -d node_modules target -I public src build -f package-lock.json -D 20
";


fn print_help_and_exit() { 
    println!("{}", HELP_MSG);
    std::process::exit(0);
}



fn main(){ 
    let args: Vec<String> = std::env::args().collect();
    
    println!("{args:?}");
    if args.len() <= 1 { 
        print_help_and_exit();
    }
    
    if args.len() == 2 && args[1] == "-h" { 
         print_help_and_exit();
    } 
    if args.len() == 3 && args[2] == "-h" { 
        print_help_and_exit();
    }

    let path = std::path::Path::new(&args[1] );
    let dir_exists = path.is_dir();
    if !dir_exists { 
        panic!("Directory does not exists");
    }

    let mut m = Manager::get_manager_from_args(args);

    


    let _ = m.do_work();
    
    dbg!(&m);
    println!("{}", m.remover_cmd());
}



fn is_flag(s: &str) -> bool { 
    if s.len() != 2 { return false }
    let mut chars = s.chars();
    if chars.next().unwrap() != '-'{ return false }
    if !chars.next().unwrap().is_alphabetic() { return false }
    true
}

fn get_args( arr: &Vec<String>, end:&mut usize )  -> Result<Vec<String>, String>  { 
    
    let  start = *end;
    *end += 1;
    if is_flag(&arr[*end]) { 
        return Err("No args found after flag, found another flag".to_owned()); 
    }

    while *end < arr.len() && !is_flag(&arr[*end]) { *end += 1; }
    
    Ok(Vec::from_iter(arr[start+1..*end].iter().cloned()))
}
