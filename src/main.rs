use std::env;
use std::fs;
use std::str;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = "
This is to take the subdomains from a gobuster brute force and print out the ip addreses with the subsdomains 
USAGE:
subdomain_ip_lister saved/file/from/gobuser
    ";
    if args.len() != 2{
        print!("{}", usage);
    }
    let mut subdomain_ips = std::collections::HashMap::new();
    let filename = &args[1];
    let subs_file_contents = fs::read_to_string(filename).expect("Error creating output file");
    let subs_split = subs_file_contents.split("\n");
    let subs_vec: Vec<&str> = subs_split.collect();
    for sub in subs_vec{
        if sub.len() > 0{
        let sub_vec: Vec<&str> = sub.split(" ").collect();
        let sub_isolation = sub_vec[1];
        let command_output = Command::new("nslookup").arg(sub_isolation).output().expect("Failed to run nslookup");
        let command_output_string = match str::from_utf8(&command_output.stdout){
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        let command_output_string_split = command_output_string.split("\n");
        let command_output_string_vec: Vec<&str> = command_output_string_split.collect();
        for line in command_output_string_vec{
            if line.contains("Address: "){
                if line.contains("#53") == false{
                    let line_split: Vec<&str> = line.split(" ").collect();
                    let ip = line_split[1];
                    println!("{}", ip);
                    subdomain_ips.insert(sub_isolation.to_string(), ip.to_string());
                    }   
                }
            } 
        }
    }
    for (subdomain, ip) in subdomain_ips{
        let outline = format!("{}\t{}", subdomain, ip);
        println!("{}", outline);
    }
}
