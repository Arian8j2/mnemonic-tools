use anyhow::ensure;
use clap::Parser;
use rand::RngCore;

#[derive(clap::Parser)]
struct Args {
    #[clap(subcommand)]
    subcommand: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Generates random valid mnemonic
    GenerateRandom,

    /// Replaces last word of mnemonic words with random word that makes the mnemonic valid
    ///
    /// take in consideration that the input mnemonic words must contain valid words from mnemonic word list
    FixLastWord { mnemonic: String },

    /// Shows all possible valid last words for your mnemonic
    ///
    /// in mnemonic words some bits of last word is used as checksum of all the entropy
    /// this command shows all possible last words that makes that checksum valid
    PossibleLastWords { mnemonic: String },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.subcommand {
        Command::GenerateRandom => generate_random_mnemonic(),
        Command::FixLastWord { mnemonic } => fix_mnemonic_last_word(mnemonic)?,
        Command::PossibleLastWords { mnemonic } => possible_last_words(mnemonic)?,
    }
    Ok(())
}

fn generate_random_mnemonic() {
    let mut entropy = [0u8; mnemonic_tools::ENTROPY_BYTES];
    rand::thread_rng().fill_bytes(&mut entropy);
    let indices = mnemonic_tools::entropy_to_indices(&entropy);
    let words = mnemonic_tools::indices_to_mnemonic_words(&indices);
    println!("{}", words.join(" "));
}

fn fix_mnemonic_last_word(mnemonic: String) -> anyhow::Result<()> {
    let words = mnemonic.split(" ").collect::<Vec<_>>();
    ensure!(
        words.len() == mnemonic_tools::WORD_LEN,
        "currently mnemonic-tools only supports {} words mnemonic",
        mnemonic_tools::WORD_LEN
    );

    let (mut entropy, _) = mnemonic_tools::mnemonic_words_to_entropy(&words)?;
    // last word is 7 bit entropy and 4 bit checksum
    let seven_bit_random_mask = rand::random::<u8>() | 0b1000_0000;
    *entropy.last_mut().unwrap() &= 0b1000_0000;
    *entropy.last_mut().unwrap() |= seven_bit_random_mask;

    let indices = mnemonic_tools::entropy_to_indices(&entropy);
    let words = mnemonic_tools::indices_to_mnemonic_words(&indices);
    println!("{}", words.join(" "));
    Ok(())
}

fn possible_last_words(mnemonic: String) -> anyhow::Result<()> {
    let words = mnemonic.split(" ").collect::<Vec<_>>();
    ensure!(
        words.len() == mnemonic_tools::WORD_LEN,
        "currently mnemonic-tools only supports {} words mnemonic",
        mnemonic_tools::WORD_LEN
    );

    let (mut entropy, _) = mnemonic_tools::mnemonic_words_to_entropy(&words)?;
    // last word is 7 bit entropy and 4 bit checksum, so we start from
    // 0b1000_0000 to not overwrite other 1 bit that is for another word
    let words = (0..=0b0111_1111)
        .map(|seven_bit| {
            let last_entropy = entropy.last_mut().unwrap();
            *last_entropy &= 0b1000_0000;
            *last_entropy |= seven_bit;
            let indices = mnemonic_tools::entropy_to_indices(&entropy);
            let last_index: usize = (*indices.last().unwrap()).into();
            mnemonic_tools::ENGLISH_WORD_LIST[last_index]
        })
        .collect::<Vec<_>>();

    println!("{}", words.join(" "));
    Ok(())
}
