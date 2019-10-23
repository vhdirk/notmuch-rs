extern crate dirs;
extern crate notmuch;
extern crate supercow;

use std::path::Path;
use std::sync::Arc;
use std::result::Result;
use supercow::Supercow;
use notmuch::ScopedSupercow;

use notmuch::{
    Database,
    DatabaseExt,
    Query,
    QueryExt,
    Message,
    FrozenMessage,
    Error
};

#[derive(Debug)]
pub struct AtomicOperation<'d> {
    database: ScopedSupercow<'d, Database>,
}

impl<'d> AtomicOperation<'d> {
    pub fn new<D>(db: D) -> Result<Self, Error>
    where
        D: Into<ScopedSupercow<'d, Database>>,
    {
        let database = db.into();
        database.begin_atomic()?;
        Ok(AtomicOperation{
            database
        })
    }
}

impl<'d> Drop for AtomicOperation<'d> {
    fn drop(&mut self) {
        let _ = self.database.end_atomic();
    }
}

/// Add a single file to the database
pub fn add_file<'d, D, P>(db: D, filename: &P) -> Result<Message<'d, Database>, Error>
where
    D: Into<ScopedSupercow<'d, Database>>,
    P: AsRef<Path>
{
    let mut database = db.into();

    let _atomic = AtomicOperation::new(Supercow::share(&mut database)).unwrap();

    match <Database as DatabaseExt>::index_file(Supercow::share(&mut database), filename, None) {
        Ok(msg) => {

            // scoped version of freezing a message
            {
                let _fmsg = FrozenMessage::new(&msg);
                

            }
            Ok(msg)
        },
        Err(err) => {
            Err(err)
        }
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_file() {



    }
}



//       status = notmuch_database_index_file (notmuch, filename, indexing_cli_choices.opts, &message);
//       switch (status) {
//       /* Success. */
//       case NOTMUCH_STATUS_SUCCESS:
//           state->added_messages++;
//           notmuch_message_freeze (message);
//           if (state->synchronize_flags)
//               notmuch_message_maildir_flags_to_tags (message);
  
//           for (tag = state->new_tags; *tag != NULL; tag++) {
//               if (strcmp ("unread", *tag) != 0 ||
//                   ! notmuch_message_has_maildir_flag (message, 'S')) {
//                   notmuch_message_add_tag (message, *tag);
//               }
//           }
  
//           notmuch_message_thaw (message);
//           break;
//       /* Non-fatal issues (go on to next file). */
//       case NOTMUCH_STATUS_DUPLICATE_MESSAGE_ID:
//           if (state->synchronize_flags)
//               notmuch_message_maildir_flags_to_tags (message);
//           break;
//       case NOTMUCH_STATUS_FILE_NOT_EMAIL:
//           fprintf (stderr, "Note: Ignoring non-mail file: %s\n", filename);
//           break;
//       case NOTMUCH_STATUS_FILE_ERROR:
//           /* Someone renamed/removed the file between scandir and now. */
//           state->vanished_files++;
//           fprintf (stderr, "Unexpected error with file %s\n", filename);
//           (void) print_status_database ("add_file", notmuch, status);
//           break;
//       /* Fatal issues. Don't process anymore. */
//       case NOTMUCH_STATUS_READ_ONLY_DATABASE:
//       case NOTMUCH_STATUS_XAPIAN_EXCEPTION:
//       case NOTMUCH_STATUS_OUT_OF_MEMORY:
//           (void) print_status_database ("add_file", notmuch, status);
//           goto DONE;
//       default:
//           INTERNAL_ERROR ("add_message returned unexpected value: %d", status);
//           goto DONE;
//       }
  
//       status = notmuch_database_end_atomic (notmuch);
  
//     DONE:
//       if (message)
//           notmuch_message_destroy (message);
  
//       return status;
//   }
  