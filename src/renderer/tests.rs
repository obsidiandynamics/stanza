#[test]
fn wrap() {
    #[derive(Debug)]
    struct Case {
        input: (&'static str, usize),
        expect: Vec<&'static str>
    }

    for case in vec![
        Case {
            input: ("", 20),
            expect: vec![""]
        },
        Case {
            input: (" ", 20),
            expect: vec![""]
        },
        Case {
            input: ("small", 20),
            expect: vec!["small"]
        },
        Case {
            input: ("small ", 20),
            expect: vec!["small"]
        },
        Case {
            input: ("large pair", 10),
            expect: vec!["large pair"]
        },
        Case {
            input: ("large pair", 20),
            expect: vec!["large pair"]
        },
        Case {
            input: ("the quick brown fox jumped over the lazy", 20),
            expect: vec!["the quick brown fox", "jumped over the lazy"]
        },
        Case {
            input: ("the quick brown fox jumped over the lazy dog", 20),
            expect: vec!["the quick brown fox", "jumped over the lazy", "dog"]
        },
        Case {
            input: ("Supercalifragilisticexpialidocious", 10),
            expect: vec!["Supercalif", "ragilistic", "expialidoc", "ious"]
        },
        Case {
            input: ("abcd Supercalifragilisticexpialidocious", 10),
            expect: vec!["abcd Super", "califragil", "isticexpia", "lidocious"]
        },
        Case {
            input: ("abcdefgh abcdefghijklmno", 10),
            expect: vec!["abcdefgh a", "bcdefghijk", "lmno"]
        },
        Case {
            input: ("abcdefghi abcdefghijklmno", 10),
            expect: vec!["abcdefghi", "abcdefghij", "klmno"]
        },
        Case {
            input: ("abcdefghij abcdefghijklmno", 10),
            expect: vec!["abcdefghij", "abcdefghij", "klmno"]
        },
        Case {
            input: ("abcde edcba abcde", 10),
            expect: vec!["abcde", "edcba", "abcde"]
        },
        Case {
            input: ("abcd edcba abcde", 10),
            expect: vec!["abcd edcba", "abcde"]
        },
        Case {
            input: ("abcde abcdefghij large pair", 10),
            expect: vec!["abcde", "abcdefghij", "large pair"]
        },
        Case {
            input: ("abc abcdefghijk tiny pair", 10),
            expect: vec!["abc abcdef", "ghijk tiny", "pair"]
        },
        Case {
            input: ("abc\ndef", 10),
            expect: vec!["abc", "def"]
        },
        Case {
            input: ("abc\n\ndef", 10),
            expect: vec!["abc", "", "def"]
        },
        Case {
            input: ("abc\n\ndef\n", 10), // single trailing newline is ignored
            expect: vec!["abc", "", "def"]
        },
        Case {
            input: ("abc\n\ndef\n\n", 10),
            expect: vec!["abc", "", "def", ""]
        },
        Case {
            input: ("abc\n\ndef ghi jkl", 10),
            expect: vec!["abc", "", "def ghi", "jkl"]
        },
    ] {
        let actual = super::wrap(case.input.0, case.input.1);
        assert_eq!(case.expect, actual, "for {:?}", &case);
    }
}