use csv;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Debug)]
pub struct Matrix {
    ncols: usize,
    elems: Vec<Row>,
    allsum: Vec<Row>,
}

// Row of a matrix containing all the columns within that row.
// Deserialized from a CSV record.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
struct Row {
    cols: Vec<i64>,
}

impl Matrix {
    pub fn new(filepath: &str) -> Result<Self, Box<dyn Error>> {
        // Read the CSV file record-by-record
        let file = File::open(filepath)?;

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file);

        // Initialize the main matrix and the pre-computed sum matrix
        let mut elems: Vec<Row> = Vec::new();
        let mut allsum: Vec<Row> = Vec::new();

        let mut ncols = 0;

        // Read and deserialize each row from the CSV
        for result in rdr.deserialize() {
            let row: Row = result?;
            let rl = row.cols.len();

            // Push the read row (it's clone actually due to Rust semantics)
            // into the main matrix
            elems.push(row.clone());

            // Cross validate that each read row from CSV has the same number
            // of columns
            if ncols == 0 {
                ncols = rl
            } else if ncols != rl {
                return Err(format!("each row is expected to have same number of columns, previous rows had {ncols}, this row has {rl}"))?;
            }

            // Precompute the sums for each row and append it to the presum
            // matrix
            let asrow = precomp_allsum(row, &allsum);
            allsum.push(asrow)
        }

        // Return the initialized matrix with its pre-computed sums
        Ok(Matrix {
            ncols: ncols,
            elems: elems,
            allsum: allsum,
        })
    }

    // sum returns the sum of all the elements of the matrix between the given
    // coordinates.
    pub fn sum(
        &self,
        startx: usize,
        starty: usize,
        endx: usize,
        endy: usize,
    ) -> Result<i64, String> {
        // Validations for input coordinates.
        //
        // Because usize means unsigned startx and starty have to be greater
        // than 0 and hence need not be checked.
        if endx >= self.ncols {
            return Err(
                "endx should be lesser than number of columns {self.ncols-1}, got {endx}"
                    .to_string(),
            );
        }
        if endy >= self.elems.len() {
            return Err(format!(
                "endy should be lesser than number of rows {}, got {endy}",
                self.elems.len() - 1
            ));
        }

        let mut sum = self.allsum[endy].cols[endx];
        if startx > 0 {
            sum -= self.allsum[endy].cols[startx - 1]
        }
        if starty > 0 {
            sum -= self.allsum[starty - 1].cols[endx]
        }
        if startx > 0 && starty > 0 {
            sum += self.allsum[starty - 1].cols[startx - 1]
        }

        Ok(sum)
    }
}

// precomp_allsum computes cumulative sums of all the rows and columns
// upto the given cell and stores it in the corresponding cell.
fn precomp_allsum(row: Row, allsum: &Vec<Row>) -> Row {
    let ncols = row.cols.len();
    let mut sumrow: Row = Row {
        cols: Vec::with_capacity(ncols),
    };

    let defrow = Row {
        cols: vec![0; ncols],
    };
    let psr = allsum.last().unwrap_or_else(|| &defrow);

    let mut sum: i64 = 0;

    for ci in 0..ncols {
        sum += row.cols[ci];

        sumrow.cols.push(sum + psr.cols[ci])
    }
    sumrow
}

// 1, 2, 5, 11
// 5, 9, 11, 15
// 2, 17, 8, -10

// 1  3   8   19
// 6  17  33  59
// 8  36  60  76
