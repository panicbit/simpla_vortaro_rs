extern crate simpla_vortaro as sv;
extern crate clap;
extern crate colored;
extern crate itertools;
extern crate textwrap;

use clap::{App, Arg};
use colored::Colorize;
use itertools::Itertools;
use textwrap::{termwidth, wrap};
use std::borrow::Cow;
use std::iter;

// TODO: Make textwrap aware of terminal escapes

#[tokio::main]
async fn main() {
    let cli = App::new("sv")
        .version("0.1.0")
        .author("panicbit <panicbit.dev@gmail.com>")
        .arg(Arg::with_name("VORTO")
            .required(true)
            .index(1)
            .help("La vorton kion vi volas serĉi")
        )
        .arg(Arg::with_name("difinu")
            .short("d")
            .long("difinu")
            .help("Akiras la difino de la VORTO anstataŭ serĉi ĝin")
        )
        .get_matches();
    
    let vorto = cli.value_of("VORTO").unwrap();
    let mut width = termwidth();

    if width >= 2 {
        width -= 2;
    }

    let res = match cli.is_present("difinu") {
        false => sercxi(vorto, width).await,
        true => difinu(vorto, width).await,
    };

    if let Err(e) = res {
        println!("{}", e);
    }
}

fn print_header(text: &str) {
    println!("\n{}", text.bold());
}

async fn sercxi(vorto: &str, width: usize) -> sv::Result<()> {
    let trovo = sv::trovi(vorto).await?;

    if !trovo.preciza.is_empty() { 
        print_header("  Preciza serĉo");

        println!();

        for preciza in &trovo.preciza {
            println!("    {}", preciza.bold().bright_green());
        }
    }

    if !trovo.malpreciza.is_empty() {
        let initial_indent = create_indent(4);
        let subsequent_indent = create_indent(4);
        let options = textwrap::Options::new(width)
            .initial_indent(&initial_indent)
            .subsequent_indent(&subsequent_indent);

        print_header("  Malpreciza serĉo");

        println!();

        let mut vortoj = trovo.malpreciza.iter().map(|v| v.bold().bright_green());
        let text = format!("{}", vortoj.join(", "));

        for text in wrap(&text, options) {
            println!("{}", text);
        }
    }

    if !trovo.vortfarado.is_empty() {
        print_header("  Vortfarada serĉo");

        println!();

        for vortfarado in &trovo.vortfarado {
            let rezulto = vortfarado.rezulto.green();
            let mut partoj = vortfarado.partoj.iter().flat_map(|p| &p.vorto).map(|v| v.bold().bright_green());

            println!("    {} ({})", rezulto, partoj.join(", "));
        }
    }
    
    if !trovo.tradukoj.is_empty() {
        print_header("  Alialingva serĉo");
    }
    print_translations(&trovo.tradukoj, 4, width);

    println!();

    Ok(())
}

fn create_indent(len: usize) -> Cow<'static, str> {
    const INDENT: &str = "          ";

    if len <= INDENT.len() {
        return INDENT[..len].into()
    }

    iter::repeat(' ').take(len).collect()
}

async fn difinu(vorto: &str, width: usize) -> sv::Result<()> {
    let mut width_l1 = width;
    let mut width_l2 = width;
    if width_l1 > 2 { width_l1 -= 2 }
    if width_l2 > 4 { width_l2 -= 4 }

    let vorto = match sv::vorto(vorto).await? {
        None => return Ok(()),
        Some(vorto) => vorto,
    };

    println!();

    for (i, difino) in vorto.difinoj.iter().enumerate() {
        let i = i + 1;
        let index = format!("{}. ", i);
        let text = format!("{}{}", index, difino.difino.as_ref().map(|s| &**s).unwrap_or(""));
        let subsequent_indent = create_indent(2 + index.chars().count());
        let options = textwrap::Options::new(width_l1)
            .initial_indent("  ")
            .subsequent_indent(&subsequent_indent);

        for (i, text) in wrap(&text, &options).iter().enumerate() {
            let mut text = text.as_ref();

            if i == 0 {
                let pivot = options.initial_indent.len() + index.len();
                let (index, rest) = text.split_at(pivot);
                print!("{}", index.bold().bright_green());
                text = rest;
            }

            println!("{}", text);
        }

        print_examples(&difino.ekzemploj, index.chars().count() + 2, width);

        print_translations(&difino.tradukoj, index.chars().count() + 2, width);

        for (j, pludifino) in difino.pludifinoj.iter().enumerate() {
            let j = j + 1;
            let initial_indent = create_indent(2 + index.chars().count());
            let index = format!("{}.{}. ", i, j);
            let text = format!("{}{}", index, pludifino.difino.as_ref().map(|s| &**s).unwrap_or(""));
            let subsequent_indent = create_indent(initial_indent.chars().count() + index.chars().count());
            let options = textwrap::Options::new(width_l2)
                .initial_indent(&initial_indent)
                .subsequent_indent(&subsequent_indent);

            println!();

            for (i, text) in wrap(&text, &options).iter().enumerate() {
                let mut text = text.as_ref();

                if i == 0 {
                    let pivot = options.initial_indent.len() + index.len();
                    let (index, rest) = text.split_at(pivot);
                    print!("{}", index.bold().magenta());
                    text = rest;
                }

                println!("{}", text);
            }

            print_examples(&pludifino.ekzemploj, index.chars().count() + 2, width);
        }

        println!();
    }

    Ok(())
}

fn print_translations(tradukoj: &[sv::Traduko], indent: usize, width: usize) {
    if !tradukoj.is_empty() {
        let tradukoj = tradukoj.iter().sorted_by_key(|tra| tra.lingvo.clone());
        let tradukoj = tradukoj.into_iter().group_by(|tra| &tra.lingvo);
        let indent = create_indent(indent);

        println!();

        for (lingvo, traduko) in &tradukoj {
            let mut vortoj = traduko.map(|tra| if let Some(ref vorto) = tra.vorto {
                format!("{} ({})", vorto.bold().bright_green(), tra.traduko.bold().magenta())
            } else {
                format!("{}", tra.traduko.bold().magenta())
            });
            
            println!("{}{}: {}", indent, lingvo, vortoj.join(", "));
        }
    }
}

fn print_examples(ekzemploj: &[sv::Ekzemplo], indent: usize, width: usize) {
    if !ekzemploj.is_empty() {
        let initial_indent = create_indent(indent);
        let subsequent_indent = create_indent(indent + 2);
        let options = textwrap::Options::new(width)
            .initial_indent(&initial_indent)
            .subsequent_indent(&subsequent_indent);

        println!();

        for ekzemplo in ekzemploj {
            use std::fmt::Write;

            let mut text = format!("- {}", ekzemplo.ekzemplo);

            if let Some(ref fonto) = ekzemplo.fonto {
                write!(text, " [{}]", fonto).unwrap();
            }

            for text in wrap(&text, &options) {
                println!("{}", text.blue());
            }
        }
    }
}
