extern crate dirs;
extern crate tempfile;
extern crate notmuch;
extern crate gethostname;
extern crate maildir;
extern crate lettre;
extern crate lettre_email;

use std::ffi::OsStr;
use std::io::{self, Result, Write};
use std::fs::{self, File};
use std::rc::Rc;
use std::path::{Path, PathBuf};
use tempfile::{tempdir, tempdir_in, Builder, TempDir};
use std::net::ToSocketAddrs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use maildir::Maildir;
use lettre_email::{EmailBuilder, Header};
use lettre::SendableEmail;


pub fn timestamp_ms() -> u128 {
    let start = SystemTime::now();
    let time_since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    time_since_epoch.as_millis()
}

// A basic test interface to a valid maildir directory.
//
// This creates a valid maildir and provides a simple mechanism to
// deliver test emails to it.  It also writes a notmuch-config file
// in the top of the maildir.
pub struct MailBox {
    root_dir: TempDir,
    idcount: u32,
    maildir: Maildir
}

impl MailBox {

    // Creates a new maildir fixture. Since this is only used for tests,
    // may just panic of something is wrong
    pub fn new() -> Self {

        let root_dir = tempdir().unwrap();
        let root_path = root_dir.path().to_path_buf();

        let tmp_path = root_path.join("tmp");
        fs::create_dir(&tmp_path).unwrap();

        let cfg_fname = root_path.join("notmuch-config");
        let mut cfg_file = File::create(cfg_fname).unwrap();
        write!(cfg_file, r#"
            [database]
            path={tmppath}
            [user]
            name=Some Hacker
            primary_email=dst@example.com
            [new]
            tags=unread;inbox;
            ignore=
            [search]
            exclude_tags=deleted;spam;
            [maildir]
            synchronize_flags=true
            [crypto]
            gpg_path=gpg
        "#, tmppath=root_path.to_string_lossy()).unwrap();

        let maildir = Maildir::from(root_path.to_path_buf());
        maildir.create_dirs().unwrap();

        Self {
            root_dir,
            idcount: 0,
            maildir
        }
    }

    /// Return a new unique message ID
    // fn next_msgid(&mut self) -> String{
    //     let hostname = gethostname::gethostname();
    //     let msgid = format!("{}@{}", self.idcount, hostname.to_string_lossy());
    //     self.idcount += 1;
    //     msgid
    // }

    pub fn path(&self) -> PathBuf
    {
        self.root_dir.path().into()
    }

    pub fn hostname(&self) -> String {
        let hname = gethostname::gethostname();
        hname.to_string_lossy().into()
    }

    /// Deliver a new mail message in the mbox.
    /// This does only adds the message to maildir, does not insert it
    /// into the notmuch database.
    /// returns a tuple of (msgid, pathname).
    pub fn deliver(&self,
                   subject: Option<String>,
                   body: Option<String>,
                   to: Option<String>,
                   from: Option<String>,
                   headers: Vec<(String, String)>,
                   is_new: bool,      // Move to new dir or cur dir?
                   keywords: Option<Vec<String>>,  // List of keywords or labels
                   seen: bool,     // Seen flag (cur dir only)
                   replied: bool,  // Replied flag (cur dir only)
                   flagged: bool)  // Flagged flag (cur dir only)
        -> Result<(String, PathBuf)>
    {

        let mut builder = EmailBuilder::new()
                            .subject(subject.unwrap_or_else(|| "Test mail".to_string()));


        if let Some(val) = body {
            builder = builder.text(val);
        }

        builder = builder.to(to.unwrap_or_else(|| "to@example.com".to_string()))
                         .from(from.unwrap_or_else(|| "src@example.com".to_string()));

        for h in headers.into_iter(){
            let hdr: Header = h.into();
            builder = builder.header(hdr);
        }

        let msg:SendableEmail = builder.build().unwrap().into();

        // not sure why lettre doesn't add the host suffix itself
        let msg_id = msg.message_id().to_string() + ".lettre@localhost";
        let id = if is_new {
            self.maildir.store_new(&msg.message_to_string().unwrap().as_bytes()).unwrap()
        }else{
            let mut flags = String::from("");
            if flagged {
                flags += "F";
            }
            if replied {
                flags += "R";
            }
            if seen {
                flags += "S";
            }
            println!("flags: {:?}", flags);
            let mid = self.maildir.store_cur_with_flags(&msg.message_to_string().unwrap().as_bytes(), flags.as_str()).unwrap();

            // I have no idea what the reasoning for the :2 here is, but ok.
            format!("{}:2,{}", mid, flags)
        };
        
        // let mut flags = String::from("");
        // if flagged {
        //     flags += "F";
        // }
        // if replied {
        //     flags += "R";
        // }
        // if seen {
        //     flags += "S";
        // }
        // println!("flags: {:?}", flags);
        // let id = self.maildir.store_cur_with_flags(&msg.message_to_string().unwrap().as_bytes(), flags.as_str()).unwrap();

        // if is_new {
        //     let msgpath = format!("{}{}", id, flags);
        //     std::fs::rename(msgpath, newpath)?;

        //     self.maildir.path()
        // }

        
        let mut msgpath = self.path();
        msgpath = if is_new {
            msgpath.join("new")
        } else {
            msgpath.join("cur")
        };
        
        msgpath = msgpath.join(&id);

        Ok((msg_id, msgpath))
    }
}

impl Drop for MailBox {
    fn drop(&mut self) {
    }
}


#[derive(Clone, Debug)]
pub struct NotmuchCommand {
    maildir_path: PathBuf
}

impl NotmuchCommand {

    /// Return a function which runs notmuch commands on our test maildir.
    ///
    /// This uses the notmuch-config file created by the ``maildir``
    /// fixture.
    pub fn new(maildir_path: &PathBuf) -> Self {
        Self {
            maildir_path: maildir_path.clone()
        }
    }

    /// Run a notmuch comand.
    /// 
    /// This function runs with a timeout error as many notmuch
    /// commands may block if multiple processes are trying to open
    /// the database in write-mode.  It is all too easy to
    /// accidentally do this in the unittests.
    pub fn run<I, S>(&self, args: I) -> Result<()>
    where
        I: IntoIterator<Item=S>,
        S: AsRef<OsStr>
    {
        let cfg_fname = self.maildir_path.join("notmuch-config");

        Command::new("notmuch").env("NOTMUCH_CONFIG", &cfg_fname)
                               .args(args)
                               .status()?;
        Ok(())
    }

}


