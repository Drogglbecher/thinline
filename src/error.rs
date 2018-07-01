use clang::SourceError;
use glob;
use std::io;

error_chain! {
    foreign_links {
        // An I/O error
        Io(io::Error);

        // A 'glob' error
        Glob(glob::GlobError);

        // A 'glob' pattern error
        Pattern(glob::PatternError);

        // A 'clang' source error
        Source(SourceError);
    }
}
