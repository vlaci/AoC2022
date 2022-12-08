use std::{collections::VecDeque, mem, slice};

use color_eyre::{
    eyre::{Context, ContextCompat},
    Report, Result,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, not_line_ending, space1},
    combinator::{all_consuming, eof, map, map_res},
    multi::{many0, separated_list0},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

fn main() -> Result<()> {
    let input = libaoc::init()?;
    let commands = terminal::parse(&input)?;
    let fs: DirectoryTree = commands.try_into()?;

    let size = part1(&fs);
    println!("The total size of directories is {size}");
    let size = part2(&fs);
    println!("The total size of directories is {size}");
    Ok(())
}

fn part1(dt: &DirectoryTree) -> usize {
    dt.iter_dirs()
        .map(|d| d.size())
        .filter(|size| *size <= 100_000)
        .sum()
}

fn part2(dt: &DirectoryTree) -> usize {
    let total = 70_000_000;
    let required = 30_000_000;
    let used = dt.size();
    let unused = total - used;
    let needed = required - unused;

    let mut matching: Vec<_> = dt
        .iter_dirs()
        .map(DirectoryTree::size)
        .filter(|s| *s >= needed)
        .collect();

    matching.sort();
    matching[0]
}

#[derive(Debug)]
enum DirectoryTree {
    File(String, usize),
    Directory(String, Vec<Self>),
}

impl DirectoryTree {
    fn size(&self) -> usize {
        match self {
            DirectoryTree::File(_, s) => *s,
            DirectoryTree::Directory(_, contents) => contents.iter().map(|e| e.size()).sum(),
        }
    }

    fn name(&self) -> &str {
        match self {
            DirectoryTree::File(n, _) => n,
            DirectoryTree::Directory(n, _) => n,
        }
    }

    fn iter_dirs(&self) -> impl Iterator<Item = &'_ Self> {
        DirectoryTreeIter {
            children: slice::from_ref(self),
            parent: None,
        }
    }
}

#[derive(Default, Debug)]
struct DirectoryTreeIter<'a> {
    children: &'a [DirectoryTree],
    parent: Option<Box<Self>>,
}

impl<'a> Iterator for DirectoryTreeIter<'a> {
    type Item = &'a DirectoryTree;

    fn next(&mut self) -> Option<Self::Item> {
        match self.children.get(0) {
            None => match self.parent.take() {
                Some(parent) => {
                    *self = *parent;
                    self.next()
                }
                None => None,
            },
            Some(DirectoryTree::File(_, _)) => {
                self.children = &self.children[1..];
                self.next()
            }
            Some(d @ DirectoryTree::Directory(_, subdirs)) => {
                self.children = &self.children[1..];
                *self = Self {
                    children: subdirs.as_slice(),
                    parent: Some(Box::new(mem::take(self))),
                };
                Some(d)
            }
        }
    }
}

fn traverse(tree: &mut DirectoryTree, commands: &mut VecDeque<terminal::Command>) -> Result<()> {
    match tree {
        DirectoryTree::File(_, _) => Ok(()),
        DirectoryTree::Directory(_, contents) => {
            while let Some(command) = commands.pop_front() {
                match command {
                    terminal::Command::Cd(dir) => match dir {
                        terminal::Path::Root => Ok(()),
                        terminal::Path::Parent => break,
                        terminal::Path::Subdir(dir) => traverse(
                            contents
                                .iter_mut()
                                .find(|c| c.name() == dir)
                                .wrap_err_with(|| format!("No such directory '{dir}'"))?,
                            commands,
                        ),
                    },
                    terminal::Command::Ls(listing) => {
                        for entry in listing {
                            match entry {
                                terminal::DirectoryEntry::Dir(ref d) => {
                                    contents.push(DirectoryTree::Directory(d.into(), vec![]));
                                }
                                terminal::DirectoryEntry::File(ref f, s) => {
                                    contents.push(DirectoryTree::File(f.into(), s));
                                }
                            }
                        }
                        Ok(())
                    }
                }?
            }
            Ok(())
        }
    }
}

impl TryFrom<Vec<terminal::Command>> for DirectoryTree {
    type Error = Report;

    fn try_from(commands: Vec<terminal::Command>) -> Result<Self, Self::Error> {
        let mut root = Self::Directory("/".into(), vec![]);

        traverse(&mut root, &mut commands.into())?;
        Ok(root)
    }
}

mod terminal {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    pub enum Path {
        Root,
        Parent,
        Subdir(String),
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Command {
        Cd(Path),
        Ls(Vec<DirectoryEntry>),
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum DirectoryEntry {
        Dir(String),
        File(String, usize),
    }

    fn path(input: &str) -> IResult<&str, Path> {
        alt((
            map(tag("/"), |_| Path::Root),
            map(tag(".."), |_| Path::Parent),
            map(not_line_ending, |d: &str| Path::Subdir(d.into())),
        ))(input)
    }

    fn listing(input: &str) -> IResult<&str, Vec<DirectoryEntry>> {
        separated_list0(
            line_ending,
            alt((
                preceded(
                    tag("dir "),
                    map(not_line_ending, |d: &str| DirectoryEntry::Dir(d.into())),
                ),
                map_res(
                    separated_pair(digit1, space1, not_line_ending),
                    |(size, name): (&str, &str)| -> Result<DirectoryEntry, Report> {
                        Ok(DirectoryEntry::File(
                            name.into(),
                            size.parse().wrap_err("Invalid size")?,
                        ))
                    },
                ),
            )),
        )(input)
    }

    fn cd(input: &str) -> IResult<&str, Command> {
        map(
            delimited(terminated(tag("cd"), space1), path, line_ending),
            Command::Cd,
        )(input)
    }

    fn ls(input: &str) -> IResult<&str, Command> {
        map(
            delimited(
                terminated(tag("ls"), line_ending),
                listing,
                alt((line_ending, eof)),
            ),
            Command::Ls,
        )(input)
    }

    pub fn parse(i: &str) -> Result<Vec<Command>> {
        all_consuming(many0(preceded(tag("$ "), alt((cd, ls)))))(i)
            .map_err(|err| Report::msg(format!("Could not parse input, {}", err)))
            .map(|(_, result)| result)
    }

    #[cfg(test)]
    mod tests {
        use pretty_assertions::assert_eq;
        use rstest::*;

        use super::*;

        #[rstest]
        #[case("cd /\n", Command::Cd(Path::Root))]
        #[case("cd ..\n", Command::Cd(Path::Parent))]
        #[case("cd abcd\n", Command::Cd(Path::Subdir("abcd".into())))]
        fn test_cd(#[case] input: &str, #[case] expected: Command) {
            assert_eq!(all_consuming(cd)(input).unwrap().1, expected);
        }

        #[rstest]
        #[case("dir abcd")]
        #[case("123 abcd")]
        #[case("123 abcd\ndir efg")]
        fn test_listing(#[case] input: &str) {
            all_consuming(listing)(input).unwrap();
        }
        #[rstest]
        #[case("ls\n")]
        #[case("ls\ndir abcd\n")]
        #[case("ls\n123 abcd\n")]
        #[case("ls\n123 abc.d\ndir efg\n")]
        fn test_ls(#[case] input: &str) {
            all_consuming(ls)(input).unwrap();
        }
    }
}
#[cfg(test)]
mod tests {
    use indoc::indoc;

    use pretty_assertions::assert_eq;
    use rstest::*;

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        indoc! {"
            $ cd /
            $ ls
            dir a
            14848514 b.txt
            8504156 c.dat
            dir d
            $ cd a
            $ ls
            dir e
            29116 f
            2557 g
            62596 h.lst
            $ cd e
            $ ls
            584 i
            $ cd ..
            $ cd ..
            $ cd d
            $ ls
            4060174 j
            8033020 d.log
            5626152 d.ext
            7214296 k
        "}
    }

    #[rstest]
    fn test_terminal(input: &str) {
        let commands = terminal::parse(input).unwrap();
        let fs: DirectoryTree = commands.try_into().unwrap();

        assert_eq!(fs.size(), 48381165);

        assert_eq!(part1(&fs), 95437);
        assert_eq!(part2(&fs), 24933642);
    }
}
