use std::io;
use glob;
use clang::SourceError;

error_chain! {
    foreign_links {
        Io(io::Error) #[doc="An I/O error"];
        Glob(glob::GlobError) #[doc="A glob error"];
        Pattern(glob::PatternError) #[doc="A glob pattern error"];
        Source(SourceError) #[doc="A clang source error"];
    }
}
