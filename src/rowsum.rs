// This approach precomputes the cumulative sum of each column within
// a row and stores it alongside the matrix.
//
// And then when the sum is requested between a pair of coordinates,
// it does the following:
//    - For each row, find the difference of the presums for the end column and
//      the one before start column (0 if start column is 0 because there is
//      nothing before).
//    - Add all these differences to find the final sum

use csv;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Debug)]
pub struct Matrix {
    ncols: usize,
    elems: Vec<Row>,
    presum: Vec<Row>,
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
        let mut presum: Vec<Row> = Vec::new();

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
            let rowsum = precomp_rowsum(row);
            presum.push(rowsum)
        }

        // Return the initialized matrix with its pre-computed sums
        Ok(Matrix {
            ncols: ncols,
            elems: elems,
            presum: presum,
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

        let mut sum: i64 = 0;
        // For each row, read the presum for end column and the one before
        // start column (0 if start column is 0 because there is nothing
        // before) and add them up to find the final sum
        for j in starty..(endy + 1) {
            let mut start = 0;
            if startx != 0 {
                start = self.presum[j].cols[startx - 1]
            }
            sum += self.presum[j].cols[endx] - start
        }

        Ok(sum)
    }
}

// precomp_rowsum computes cumulative sum for each column in the row and
// stores it in the corresponding column. For example, the value stored
// in column 3 is the sum of all the values from columns 0, 1, 2, and 3
// I.e. for the row [1, 2, 5, 11], column 3 has 19 in the rowsum
// returned.
fn precomp_rowsum(row: Row) -> Row {
    let ncols = row.cols.len();
    let mut rowsum = Row {
        cols: Vec::with_capacity(ncols),
    };
    let mut sum: i64 = 0;

    for ci in 0..ncols {
        sum += row.cols[ci];
        rowsum.cols.push(sum)
    }
    rowsum
}

// 1, 2, 5, 11
// 5, 9, 11, 15
// 2, 17, 8, -10

// 1  3   8   19
// 5  14  25  40
// 2  19  27  17
