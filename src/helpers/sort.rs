#[derive(PartialEq)]
pub enum SortFile {
    Name,
    Type,
    Size,
}


impl ToString for SortFile {
    fn to_string(&self) -> String {
        match self {
            SortFile::Name => { "Name" }
            SortFile::Type => { "Type" }
            SortFile::Size => { "Size" }
        }.to_string()
    }
}

#[derive(PartialEq)]
pub enum SortArrangement {
    Ascending,
    Descending,
}


impl ToString for SortArrangement {
    fn to_string(&self) -> String {
        match self {
            SortArrangement::Ascending => { "Ascending" }
            SortArrangement::Descending => { "Descending" }
        }.to_string()
    }
}