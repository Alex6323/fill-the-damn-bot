//! `fill-the-damn-bot` is a terminal tool, that helps you keep track of what you did during your working day.
//! You know how it goes in (open source) software development...there are so many different tasks: writing your own
//! perfect code, fixing other's bugs, reviewing PRs, writing proposals, reading up on new trendy libraries you could
//! use at some point, having meetings, answering questions from users, and so and so forth. At the end of each day you
//! are required to write a short summary about the shit you got done, but you're already pretty tired, and the only
//! thing your memory is still useful for is remembering your wife's and kid's names to say "good night" to them, and
//! finding the bed.
//! What if you had a tool next to your development environment, that you could just tell what you did right after you
//! finished a small task. After 16 hours minimum you would tell it that you're finished working for today, and it
//! would print out the summary, that is, everything you entered as a list of bullet points (even with the same
//! typos!!!!), and you just needed to c&p that into your company's bot, hell I know I am going crazy with this, but
//! what if it even c&p'ed it into memory for you so you'ld just have to paste it? Wouldn't that be handy?
//! I know what you're thinking now: "Damn, that's ingenious. This should have been MY idea! If it would have been me,
//! I'ld be rich soon and be buying my own island using all my power/money to drive out anybody who's already living
//! there. Now it's him!!! That's so unfair.". Yea, sorry guys, I already packed my snorchel and my army of lawyers.
//! I am off, soon. In the meantime, enjoy this tool.

use clipboard::{ClipboardContext, ClipboardProvider};
use dialoguer::{console, theme::ColorfulTheme, Confirm, Input};
use rand::{thread_rng, Rng};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    clear_screen();

    let theme = ColorfulTheme::default();

    let acks = get_acknowledgements();
    let mut accomplished_tasks = vec![];

    loop {
        let accomplished: String = Input::with_theme(&theme)
            .with_prompt("What did you accomplish?")
            .interact_text()?;

        accomplished_tasks.push(accomplished);

        println!("{}", acks[thread_rng().gen_range(0..acks.len())]);

        let finished: String = Input::with_theme(&theme)
            .with_prompt("Are you finished for today?")
            .default("No".into())
            .interact_text()?;

        if finished.to_lowercase() == "yes" {
            break;
        }
    }

    copy_to_clipboard(&accomplished_tasks);

    while !Confirm::with_theme(&theme)
        .with_prompt("Filled the damn bot?")
        .default(true)
        .interact()?
    {
        copy_to_clipboard(&accomplished_tasks);
    }

    print_bot();

    Ok(())
}

fn get_acknowledgements() -> Vec<&'static str> {
    let mut vec = vec![];

    vec.push("That's genious! You sure, that was you who did that?");
    vec.push("Awesome. You should get a raise.");
    vec.push("One step closer to becoming a billionair.");
    vec.push("Elon would be proud of ya.");
    // TODO: Wanna extend this list? PRs are welcomed with open (and deadly robotic) arms.

    vec
}

fn clear_screen() {
    console::Term::stdout().clear_screen().expect("error clearing screen");
}

fn copy_to_clipboard(accomplished_tasks: &Vec<String>) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut s = String::new();
    for accomplished in accomplished_tasks {
        s.push_str("* ");
        s.push_str(accomplished);
        s.push_str("\n");
    }
    ctx.set_contents(s).unwrap();
}

fn print_bot() {
    println!("");
    println!("______♥╣[-_-] ╠♥______");
    println!("");
}
