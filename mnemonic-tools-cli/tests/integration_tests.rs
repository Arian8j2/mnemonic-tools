use assert_cmd::Command;

fn mnemonic_tools_cli() -> Command {
    Command::cargo_bin("mnemonic-tools-cli").unwrap()
}

#[test]
fn test_cli_generate_random() {
    mnemonic_tools_cli()
        .arg("generate-random")
        .assert()
        .success();
}

#[test]
fn test_cli_fix_last_word() {
    let input_words = [
        "satisfy", "spend", "denial", "mammal", "salon", "trade", "monster", "echo", "until",
        "stand", "say", "sting",
    ];
    let assert = mnemonic_tools_cli()
        .arg("fix-last-word")
        .arg(input_words.join(" "))
        .assert();
    let output = assert.get_output().clone();
    assert.success();

    let mut stdout = output.stdout;
    stdout.pop(); // newline
    let stdout = String::from_utf8(stdout).unwrap();
    let output_words: Vec<&str> = stdout.split(" ").collect();
    assert_eq!(input_words[..=10], output_words[..=10]);
    assert_ne!(input_words[11], output_words[11]);
}

#[test]
fn test_cli_make_valid_should_fail_on_notexist_word() {
    mnemonic_tools_cli()
        .arg("make-valid")
        .arg("satisfy spend denial mammal salon trade monster echo until stand say haha")
        .assert()
        .failure();
}

#[test]
fn test_cli_possible_last_words() {
    let input_words = [
        "satisfy", "spend", "denial", "mammal", "salon", "trade", "monster", "echo", "until",
        "stand", "say", "aunt",
    ];
    let assert = mnemonic_tools_cli()
        .arg("possible-last-words")
        .arg(input_words.join(" "))
        .assert();
    let output = assert.get_output().clone();
    assert.success();

    let mut stdout = output.stdout;
    stdout.pop(); // newline
    let stdout = String::from_utf8(stdout).unwrap();
    let output_words: Vec<&str> = stdout.split(" ").collect();
    let possible_last_words = [
        "absurd", "admit", "agree", "always", "amused", "apple", "aspect", "aunt", "aware",
        "barrel", "behind", "bind", "bonus", "bracket", "bronze", "cable", "cannon", "cash",
        "ceiling", "chief", "cinnamon", "clean", "clump", "comfort", "copy", "creek", "crucial",
        "dawn", "deal", "deliver", "dial", "dilemma", "donkey", "draw", "dutch", "edit", "embody",
        "envelope", "erode", "example", "extra", "fall", "february", "fine", "flavor", "fog",
        "foster", "garden", "gaze", "glow", "grain", "group", "hawk", "hint", "horse", "hurt",
        "impose", "inherit", "invest", "invite", "keen", "kind", "laptop", "legend", "letter",
        "long", "lunch", "manual", "meadow", "message", "midnight", "morning", "music", "neck",
        "never", "obey", "oil", "onion", "over", "panic", "parrot", "pear", "pitch", "play",
        "position", "price", "pulse", "puzzle", "raccoon", "rather", "remain", "require", "reward",
        "riot", "rotate", "sand", "scale", "senior", "shaft", "shuffle", "six", "slice", "social",
        "sound", "south", "spoil", "stem", "strong", "supply", "swear", "symptom", "teach",
        "three", "tiny", "torch", "trade", "trust", "turn", "unique", "urge", "various", "venue",
        "virus", "water", "weekend", "width", "worth", "wrong",
    ];
    assert_eq!(output_words, possible_last_words);

    let mut words = input_words.clone();
    for word in possible_last_words {
        *words.last_mut().unwrap() = word;
        assert!(
            mnemonic_tools::is_mnemonic_valid(&input_words),
            "mnemonic with last word '{word}' is not valid"
        );
    }
}
