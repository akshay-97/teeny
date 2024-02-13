mod utils;
use std::fmt;
use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// #[repr(u8)]
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum Cell{
//     Dead=0,
//     Alive=1
// }

#[wasm_bindgen]
pub struct Universe{
    width : u32,
    height : u32,
    cells : Vec<Cell>
}

/*
pub struct Universe{
    width : u32,
    height : u32,
    cells : Vec<Cell>
}

let bytes_n = (width *height) / 8 

bytes_n >> index //8 + (index % 8)
alive => u8 || 1 live bit
dead => u8 && 1 0 bit
*/

impl Universe{
    fn get_index(&self, x:u32, y:u32) -> usize{
        ((x*self.width) + y) as usize
    }
//  0 1 2 3  | 4  5 6 7 | 8 9 10 11| 12 13 14 15|
    fn live_neighbour_count(&self, x:u32, y:u32) -> u8{
        let mut count = 0;
        let _n1 = count += self.cells[self.get_index((x + 1)  % self.width, y)] as u8;
        let _n2 = count += self.cells[self.get_index((x-1 + self.width) % self.width, y)] as u8;
        let _n3 = count += self.cells[self.get_index(x, (y+1) % self.height)] as u8;
        let _n4 = count += self.cells[self.get_index(x, (y-1 + self.height) % self.height)] as u8;
        let _n5 = count += self.cells[self.get_index((x-1 + self.width) % self.width, (y-1 + self.height) % self.height)] as u8;
        let _n6 = count += self.cells[self.get_index((x+1) % self.width, (y+1) % self.height)] as u8;
        let _n7 = count += self.cells[self.get_index((x-1 + self.width) % self.width, (y+1) % self.height)] as u8;
        let _n8 = count += self.cells[self.get_index((x+1) % self.width, (y-1 + self.height) % self.height)] as u8;
        count as u8

    }
}

#[wasm_bindgen]
impl Universe{
    pub fn tick(&mut self){
        let mut next = self.cells.clone();
        for row in 0..self.height{
            for col in 0..self.width{
                let index = self.get_index(row, col);
                let live_n = self.live_neighbour_count(row, col);

                let n_cell = match (self.cells[index], live_n){
                    (Cell::Alive, a) => if a <2 || a > 3 { Cell::Dead} else { Cell::Alive},
                    (Cell::Dead, a) => if a ==3 { Cell::Alive} else {Cell::Dead},
                };
                next[index] = n_cell
            }
        }
        self.cells = next;
    }

    pub fn new() -> Universe{
        let height = 32;
        let width = 32;

        let cells = (0..height*width).map(|i|
                                            {if i %2 == 0 { Cell::Alive} else if i%5 ==0 {Cell::Alive} else {Cell::Dead}}).collect();
        Universe { width, height, cells }
    }

    pub fn render(&self) -> String{
        self.to_string()
    }

    pub fn width(&self) -> u32{
        self.width
    }

    pub fn height(&self) -> u32{
        self.height
    }

    pub fn cells(&self) -> *const Cell{
        self.cells.as_ptr()
    }
}

impl fmt::Display for Universe{

    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize){
            for c in line{
                let symbol = if c == &Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}


type Cell = u8;