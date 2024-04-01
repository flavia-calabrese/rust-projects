use anyhow::Context;
use clap;
use clap::{Parser, Subcommand};
use std::fmt::format;
use std::fs;
use std::path::PathBuf;

const BSIZE: usize = 20;

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug, Clone)]

pub enum Commands {
    /// Add new boat
    Add {
        file: PathBuf,
        orientation: String,
        #[arg(value_delimiter = ',')]
        start_position: Vec<u8>,
    },

    /// Create new board
    New {
        file: PathBuf,
        #[arg(value_delimiter = ',')]
        boats: Vec<u8>,
    },
}
pub enum Error {
    Overlap,
    OutOfBounds,
    BoatCount,
}
impl Error {
    pub fn to_string(self) -> String {
        return match self {
            Error::Overlap => "Overlap Error".to_string(),
            Error::OutOfBounds => "OutOfBound Error".to_string(),
            Error::BoatCount => "BoatCount Error".to_string(),
        };
    }
}
pub enum Boat {
    Vertical(usize),
    Horizontal(usize),
}
pub struct Board {
    boats: [u8; 4],
    data: [[u8; BSIZE]; BSIZE],
}
impl Board {
    /** crea una board vuota con una disponibilità di navi */
    pub fn new(boats: &[u8]) -> Board {
        return Board {
            boats: <[u8; 4]>::try_from(boats).unwrap(),
            data: [[0; BSIZE]; BSIZE],
        };
    }

    /* crea una board a partire da una stringa che rappresenta tutto il contenuto del file board.txt */
    pub fn from(s: String) -> Board {
        let mut iter = s.split("\n");
        let boats = iter
            .next()
            .unwrap()
            .split(" ")
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<_>>();
        let mut data = [[0; BSIZE]; BSIZE];
        for (index_row, row) in iter.enumerate() {
            for (index_boat, boat) in row.chars().enumerate() {
                data[index_row][index_boat] = if boat == ' ' { 0 } else { 1 };
            }
        }
        return Board {
            boats: <[u8; 4]>::try_from(&boats as &[u8]).unwrap(),
            data,
        };
    }

    /* aggiunge la nave alla board, restituendo la nuova board se possibile */
    /* bonus: provare a *non copiare* data quando si crea e restituisce
    una nuova board con la barca, come si può fare? */
    pub fn add_boat(self, boat: Boat, pos: (usize, usize)) -> Result<Board, Error> {
        let mut board = self;
        match boat {
            Boat::Horizontal(h) => {
                if pos.0 >= BSIZE || pos.1 >= BSIZE || h + pos.1 >= BSIZE {
                    return Err(Error::OutOfBounds);
                }
                if board.boats[h - 1] == 0 {
                    return Err(Error::BoatCount);
                }
                for boat in &board.data[pos.0][pos.1..(pos.1 + h)] {
                    if *boat == 1 {
                        return Err(Error::Overlap);
                    }
                }
                for j in pos.1..(pos.1 + h) {
                    board.data[pos.0][j] = 1;
                }
                board.boats[h - 1] -= 1;
            }
            Boat::Vertical(v) => {
                if pos.0 >= BSIZE || pos.1 >= BSIZE || v + pos.0 >= BSIZE {
                    return Err(Error::OutOfBounds);
                }
                if board.boats[v - 1] == 0 {
                    return Err(Error::BoatCount);
                }
                for boat in &board.data[pos.0..(pos.0 + v)] {
                    if boat[pos.1] == 1 {
                        return Err(Error::Overlap);
                    }
                }
                for j in pos.0..(pos.0 + v) {
                    board.data[j][pos.1] = 1;
                }
                board.boats[v - 1] -= 1;
            }
        }
        return Ok(board);
    }
    /* converte la board in una stringa salvabile su file */
    pub fn to_string(&self) -> String {
        let mut board: String = "".to_string();
        board = format!(
            "{} {} {} {}\n",
            self.boats[0], self.boats[1], self.boats[2], self.boats[3]
        );
        for row in self.data {
            let mut row_str = String::new();
            for boat in row {
                if boat == 0 {
                    row_str.push(' ');
                } else {
                    row_str.push('B');
                }
            }
            row_str.push('\n');
            board.push_str(&row_str);
        }
        return board;
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.command {
        Commands::Add {
            file,
            orientation,
            start_position,
        } => add(file, orientation, start_position)?,
        Commands::New { file, boats } => new(file, boats)?,
    }
    Ok(())
}

pub fn new(file: PathBuf, boats: Vec<u8>) -> anyhow::Result<()> {
    if boats.len() != 4 {
        return Err(anyhow::Error::msg("boats must be 4"));
    }
    let board = Board::new(&boats);

    fs::write(&file, board.to_string())
        .with_context(|| format!("could not open {}", file.to_string_lossy()))?;
    return Ok(());
}

pub fn add(file: PathBuf, orientation: String, start_position: Vec<u8>) -> anyhow::Result<()> {
    if start_position.len() != 2 {
        return Err(anyhow::Error::msg("position needs 2 values"));
    }
    let orientation = orientation.chars().collect::<Vec<_>>();
    if orientation.len() != 2 {
        return Err(anyhow::Error::msg("position needs 2 values"));
    }
    /*let type_boat = orientation[0];
    let direction = orientation[1];*/
    let boat = if orientation[1] == 'V' {
        Boat::Vertical(
            orientation[0]
                .to_digit(10)
                .ok_or(anyhow::Error::msg("first char is not a number"))? as usize,
        )
    } else if orientation[1] == 'H' {
        Boat::Horizontal(
            orientation[0]
                .to_digit(10)
                .ok_or(anyhow::Error::msg("first char is not a number"))? as usize,
        )
    } else {
        return Err(anyhow::Error::msg("second char is different from V or H"));
    };
    let my_file = fs::read_to_string(&file)?;
    let my_board = Board::from(my_file);
    let result = my_board.add_boat(
        boat,
        (start_position[0] as usize, start_position[1] as usize),
    );

    match result {
        Ok(board) => {
            fs::write(&file, board.to_string())
                .with_context(|| format!("Could not open file {}", file.to_string_lossy()))?;
            return Ok(());
        }
        Err(e) => return Err(anyhow::Error::msg(e.to_string())),
    }
}
