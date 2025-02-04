use anyhow::Result;
use std::process::Command;
use which::which;
use std::path::Path;
use crate::error::ApxError;


// Finds out if we're running inside a container or on the host.
fn running_in_container() -> bool
{
    Path::new("/run/.containerenv").exists()
}

// Finds the apx binary path, solving for containerised runs.
pub fn get_apx_bin() -> String{

    if running_in_container()
    {
        let host_spawn = get_host_spawn_bin();
        let apx = format!("{host_spawn} apx");
        return apx;
    }

    match which("apx")
    {
        Ok(s) => s.display().to_string(),
        Err(e) => {
            println!("{}", e);

            "/usr/bin/apx".into()
        },
    }
}

// Finds the host-spawn path, used in cases where we're running inside a container.
fn get_host_spawn_bin() -> String{
    match which("host-spawn")
    {
        Ok(s) => s.display().to_string(),
        Err(_e) => "/usr/bin/host-spawn".into()
    }
}


// Runs an apx command, resolving required binaries.
pub fn run_apx(args: &str, ignore_errors: bool) -> Result<String>
{
    let apx_bin = get_apx_bin();

    println!("{}", apx_bin);

    return run_command(format!("{apx_bin} {args}"), ignore_errors)
}


// Runs a shell command.
fn run_command(args: String, ignore_errors: bool) -> Result<String>
{

    let output = Command::new("sh")
                        .arg("-c")
                        .arg(args)
                        .output();

    match output{
        Ok(out) => 
        {
            if out.status.success()
            {
                Ok(String::from_utf8(out.stdout).unwrap())
            }
            else if ignore_errors
            {
                Ok(String::from_utf8(out.stderr).unwrap())
            }
            else
            {
                Err(ApxError::CommandError { error: String::from_utf8(out.stderr).unwrap()}.into())
            }
        }
        Err(e) => 
        {
            println!("Error {}", e);
            if ignore_errors
            {
                Ok("".into())
            }
            else 
            {
                Err(e.into())
            }   
         }
    }
    
}
