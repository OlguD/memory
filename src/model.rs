#[derive(Debug)]
pub enum Source {
    Manual,
    Git
}

#[derive(Debug)]
pub enum Kind {
    Decision,
    Pattern,
    Bug,
    Note,
}

#[derive(Debug)]
pub struct Memory {
    id: i64,
    source: Source,
    source_ref: Option<String>,
    kind: Kind,
    content: String,
    project: i64,
    created_at: String,
    updated_at: String,
}


#[derive(Debug)]
pub struct NewMemory {
    pub source: Source,
    pub source_ref: Option<String>,
    pub kind: Kind,
    pub content: String,
    pub project_id: i64
}

#[derive(Debug)]
pub struct Project {
    id: i64,
    project_id: i64,
    name: i64,
    path: i64,
    created_at: String,
}



impl Source {
    pub fn as_str(&self) -> &'static str {
        match self {
            Source::Manual => "manual",
            Source::Git => "git",
        }
    }

    pub fn from_str(s: &str) -> Option<Source> {
        match s {
            "manual" => Some(Source::Manual),
            "git" => Some(Source::Git),
            _ => None,
        }
    }
}

impl Kind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Kind::Decision => "decision",  
            Kind::Pattern => "pattern", 
            Kind::Bug => "bug", 
            Kind::Note => "note"
        }
    }
    pub fn from_str(s: &str) -> Option<Kind> {
        match s {
            "decision" => Some(Kind::Decision),
            "pattern" => Some(Kind::Pattern),
            "bug" => Some(Kind::Bug),
            "note" => Some(Kind::Note),
            _ => None,
        }
    }
}


