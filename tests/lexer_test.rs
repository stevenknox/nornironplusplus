use aussie_plus_plus::{
    lexer,
    token::{Kind, Token},
};

fn test_lexing(src: &str, expected_tokens: Vec<Token>, expected_error: bool) {
    let mut lexer = lexer::Lexer::new(src.chars());
    let (tokens, had_error) = lexer.lex();

    assert_eq!(had_error, expected_error);
    {
        let mut i = 0;
        for token in &tokens {
            if &expected_tokens[i] != token {
                println!("expected: {:?} but got {:?}", expected_tokens[i], token);
            }
            assert_eq!(expected_tokens[i].clone(), token.clone());
            i += 1;
        }
    }
    assert_eq!(tokens, expected_tokens);
}

#[test]
pub fn test_operators() {
    test_lexing(
        "5 < 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::LeftBoomerang, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 > 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::RightBoomerang, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 <= 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::LTE, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 >= 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::GTE, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 + 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::Plus, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 - 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::Minus, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 * 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::Asterisk, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 / 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::Slash, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 == 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::Equals, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 != 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::BangEqual, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 && 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::And, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 || 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::Or, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "!Yeah, Nah",
        vec![
            Token::new(Kind::Bang, 1),
            Token::new(Kind::YeahNah, 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );
}

#[test]
pub fn test_separation() {
    test_lexing(
        "BLIMEY MATEWALKABOUT",
        vec![
            Token::new(Kind::Ident("LIMEY".into()), 1),
            Token::new(Kind::Ident("MATEWALKABOUT".into()), 1),
            Token::new(Kind::EOF, 1),
        ],
        true,
    );
}

#[test]
pub fn test_lex_keyword_casing() {
    let kinds = vec![
        Kind::ChookBickey,
        Kind::Walkabout,
        Kind::BlimeyMate,
        Kind::IRecon,
        Kind::YaRecon,
        Kind::HardYakkaFor,
        Kind::Bail,
        Kind::YeahNah,
        Kind::NahYeah,
    ];

    fn test_random_case(kind: &Kind, iterations: usize) {
        let mut random_case: String;
        for _ in 0..iterations {
            random_case = kind
                .literal()
                .chars()
                .into_iter()
                .map(|c| {
                    if rand::random() {
                        c.to_ascii_uppercase()
                    } else {
                        c.to_ascii_lowercase()
                    }
                })
                .collect();

            let expected = {
                if kind.clone() == Kind::ChookBickey {
                    vec![Token::new(Kind::ChookBickey, 1)]
                } else {
                    vec![Token::new(kind.clone(), 1), Token::new(Kind::EOF, 1)]
                }
            };

            test_lexing(format!("{}", random_case).as_str(), expected, false);
        }
    }

    for kind in kinds {
        test_random_case(&kind, 5);
    }
}

#[test]
pub fn test_lex_hard_yakka() {
    test_lexing(
        "Hard yakka for dummyFunction ( x ) <
            bail \"dinkum\"
        >",
        vec![
            Token::new(Kind::HardYakkaFor, 1),
            Token::new(Kind::Ident("dummyFunction".into()), 1),
            Token::new(Kind::LeftParen, 1),
            Token::new(Kind::Ident("x".into()), 1),
            Token::new(Kind::RightParen, 1),
            Token::new(Kind::LeftBoomerang, 1),
            Token::new(Kind::Bail, 2),
            Token::new(Kind::String("dinkum".into()), 2),
            Token::new(Kind::RightBoomerang, 3),
            Token::new(Kind::EOF, 3),
        ],
        false,
    );
}

#[test]
pub fn test_lex_keywords() {
    test_lexing(
        "BLIMEY MATE
        I RECON x = 5
        I RECON y = 10
        CHOOK BICKEY
        I RECON z = 12
        ",
        vec![
            Token::new(Kind::BlimeyMate, 1),
            Token::new(Kind::IRecon, 2),
            Token::new(Kind::Ident("x".into()), 2),
            Token::new(Kind::Assign, 2),
            Token::new(Kind::Number(5f64), 2),
            Token::new(Kind::IRecon, 3),
            Token::new(Kind::Ident("y".into()), 3),
            Token::new(Kind::Assign, 3),
            Token::new(Kind::Number(10f64), 3),
            Token::new(Kind::ChookBickey, 4),
        ],
        false,
    );
}

#[test]
fn test_lex_ya_recon() {
    test_lexing(
        "Ya recon x == 5 <
            bail Nah, yeah
        >
        bail Yeah, Nah",
        vec![
            Token::new(Kind::YaRecon, 1),
            Token::new(Kind::Ident("x".into()), 1),
            Token::new(Kind::Equals, 1),
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::LeftBoomerang, 1),
            Token::new(Kind::Bail, 2),
            Token::new(Kind::NahYeah, 2),
            Token::new(Kind::RightBoomerang, 3),
            Token::new(Kind::Bail, 4),
            Token::new(Kind::YeahNah, 4),
            Token::new(Kind::EOF, 4),
        ],
        false,
    )
}

#[test]
fn test_lex_walkabout() {
    test_lexing(
        "WALKABOUT (I recon x = 0; x < 5; x = x + 1) <
            x = x + 1
        >",
        vec![
            Token::new(Kind::Walkabout, 1),
            Token::new(Kind::LeftParen, 1),
            Token::new(Kind::IRecon, 1),
            Token::new(Kind::Ident("x".into()), 1),
            Token::new(Kind::Assign, 1),
            Token::new(Kind::Number(0f64), 1),
            Token::new(Kind::Semicolon, 1),
            Token::new(Kind::Ident("x".into()), 1),
            Token::new(Kind::LeftBoomerang, 1),
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::Semicolon, 1),
            Token::new(Kind::Ident("x".into()), 1),
            Token::new(Kind::Assign, 1),
            Token::new(Kind::Ident("x".into()), 1),
            Token::new(Kind::Plus, 1),
            Token::new(Kind::Number(1f64), 1),
            Token::new(Kind::RightParen, 1),
            Token::new(Kind::LeftBoomerang, 1),
            Token::new(Kind::Ident("x".into()), 2),
            Token::new(Kind::Assign, 2),
            Token::new(Kind::Ident("x".into()), 2),
            Token::new(Kind::Plus, 2),
            Token::new(Kind::Number(1f64), 2),
            Token::new(Kind::RightBoomerang, 3),
            Token::new(Kind::EOF, 3),
        ],
        false,
    );
}
