use std::process::Command;

// For MacOS, Linux, I chose 'ps -eo pid,user,args' command, which lists all the processes running on the system. This makes sure there is at least 10 lines of output to 
// If Windows, 'systeminfo' is run

fn main() {

    // Macos Linux
    let output_macos_linux = Command::new("ps")
        .arg("-eo")
        .arg("pid,user,args")
        .output();
    
    // Windows
    let output_windows = Command::new("systeminfo")
        .output();

    match output_macos_linux {
        Ok(output) => {
            let output_string = String::from_utf8(output.stdout).unwrap();
            println!("output MacOs Linux 'ps aux':");
            for (i,line) in output_string.lines().enumerate() {
                println!("{}", line);
                if i > 10 {
                    break;
                }
            }
        },
        Err(e) => {
            match output_windows {
                Ok(output) => {
                    let output_string = String::from_utf8(output.stdout).unwrap();
                    println!("output Windows 'systeminfo'");
                    for (i,line) in output_string.lines().enumerate() {
                        println!("{}", line);
                        if i > 10 {
                            break;
                        }
                    }
                },
                Err(e) => {
                    println!("Failed to execute any external command: {:?}", e);
                }
            }
        }
    }
}