use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: Kind,
    line: usize,
}

impl Token {
    pub fn new(kind: Kind, line: usize) -> Self {
        Self { kind, line }
    }

    pub fn kind(&self) -> Kind {
        self.kind.clone()
    }

    pub fn line(&self) -> usize {
        self.line
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Kind {
    Modulo,         // %
    Tilde,          // ~
    QuestionMark,   // ?
    LeftBoomerang,  // <
    RightBoomerang, // >
    LeftBracket,    // [
    RightBracket,   // ]
    LeftParen,      // (
    RightParen,     // )
    Assign,         // =
    Comma,          // ,
    Plus,           // +
    Minus,          // -
    Asterisk,       // *
    Slash,          // /
    Bang,           // !
    Semicolon,      // ;
    LTE,            // <=
    GTE,            // >=
    Equals,         // ==
    BangEqual,      // !=
    And,            // &&
    Or,             // ||
    Import,         // IMPOHT ME FUNC
    FuckinPiker,    // FUCKINPIKER (early exit)
    MateFuckThis,   // mate fuck this (break)
    Until,          // until
    From,           // from
    To,             // to
    Gimme,          // gimme
    Is,             // (is)
    Isa,            // (is a)
    BuggerAll,      // Bugger all (nil/null)
    Cheers,         // Cheers C***! (end of program)
    Whatabout,      // Whatabout (else)
    IllHaveA,       //  I'll Have a
    Walkabout,      // Walkabout (for loop)
    GdayMate,       // G'DAY MATE! (program start)
    IReckon,        // I reckon (var decl)
    YaReckon,       // Ya reckon (analogous to if)
    HardYakkaFor,   // Hard yakka for (function decl)
    Bail,           // bail (return)
    NahYeah,        // true
    YeahNah,        // false
    Ident(String),  // Identifier
    Number(f64),    // Number literal
    String(String), // String literal
    EOF,
}

impl Kind {
    pub fn literal(&self) -> String {
        match self {
            Kind::Import => "give'is a",
            Kind::FuckinPiker => "later mate",
            Kind::Modulo => "%",
            Kind::MateFuckThis => "loada balls",
            Kind::LeftBracket => "[",
            Kind::RightBracket => "]",
            Kind::Until => "until",
            Kind::From => "from",
            Kind::To => "to",
            Kind::Is => "is",
            Kind::Isa => "is a",
            Kind::Tilde => "~",
            Kind::QuestionMark => "?",
            Kind::LeftBoomerang => "<",
            Kind::RightBoomerang => ">",
            Kind::LeftParen => "(",
            Kind::RightParen => ")",
            Kind::Assign => "=",
            Kind::Comma => ",",
            Kind::Plus => "+",
            Kind::Minus => "-",
            Kind::Asterisk => "*",
            Kind::Slash => "/",
            Kind::Bang => "!",
            Kind::Semicolon => ";",
            Kind::LTE => "<=",
            Kind::GTE => ">=",
            Kind::Equals => "==",
            Kind::BangEqual => "!=",
            Kind::And => "&&",
            Kind::Or => "||",
            Kind::Gimme => "No worries, its",
            Kind::IllHaveA => "i'll have a",
            Kind::BuggerAll => "bugger all",
            Kind::Cheers => "see ya later", // Chook bickey (end of program)
            Kind::Whatabout => "dander", // Whatabout (else)
            Kind::Walkabout => "Giz a go", // Walkabout (for loop)
            Kind::GdayMate => "Whats the craic?", // G'DAY MATE! (program start)
            Kind::IReckon => "i reckon",    // I reckon (var decl)
            Kind::YaReckon => "ya reckon",  // Ya reckon (analogous to if)
            Kind::HardYakkaFor => "do me a wee favor", // Hard yakka for (function decl)
            Kind::Bail => "split",           // bail (return)
            Kind::NahYeah => "aye",   // true
            Kind::YeahNah => "nope",   // false
            Kind::Ident(ref s) => s.as_str(), // Identifier
            Kind::Number(n) => return format!("{}", n), // Number literal
            Kind::String(ref s) => s.as_str(), // String literal
            Kind::EOF => "EOF",
        }
        .into()
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.literal())
    }
}
