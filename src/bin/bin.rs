use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};


fn main() -> Result<(), Error> {
    // Arg Parse
    let fastq_path = std::env::args().nth(1).expect("no Input file given");
    let re_site = std::env::args()
                                            .nth(2)
                                            .expect("no RE site given to split reads");

    // hard coded shit
    let min_seq_length = 20;

    // Open FASTQ
    let file = File::open(fastq_path).unwrap();
    let reader = BufReader::new(file);
    
    // Open outfile
    let mut out_file = File::create("foo.txt")?;

    // Alternative implementation using while
    let mut lines_iter = reader.lines();
    while let (Some(header), 
               Some(seq),
               _,
               Some(qual),
               ) = (lines_iter.next(), 
                    lines_iter.next(),
                    lines_iter.next(),
                    lines_iter.next()) {
        let seq = seq.unwrap();
        if let Some(j) = seq.find(&re_site){
            let header = header.unwrap();
            let qual = qual.unwrap();
            let subseq = &seq.as_str()[..j];
            if subseq.chars().count() > min_seq_length {
                writeln!(out_file, "{}", header)?;
                writeln!(out_file, "{}", subseq)?;
                writeln!(out_file, "+")?;
                writeln!(out_file, "{}", &qual.as_str()[..j])?;
            }
            let subseq = &seq.as_str()[j..];
            if subseq.chars().count() > min_seq_length {
                writeln!(out_file, "{}", header)?;
                writeln!(out_file, "{}", subseq)?;
                writeln!(out_file, "+")?;
                writeln!(out_file, "{}", &qual.as_str()[j..])?;
            }
        }
    }

    // Alternative implementation using for
    // let lines_iter = reader.lines();

    // let mut header = String::from("");
    // let mut seq = String::from("");
    // let mut qual:String;

    // for (i, line) in lines_iter.enumerate() {
    //     if 0 == i % 4{
    //         header = line.unwrap();
    //     } else if 1 == i % 4{
    //         seq = line.unwrap();
    //     } else if 3 == i % 4{
    //         qual = line.unwrap();
    //         if let Some(j) = seq.find(&re_site){
    //             let subseq = &seq.as_str()[..j];
    //             if subseq.chars().count() > min_seq_length {
    //                 writeln!(out_file, "{}", header)?;
    //                 writeln!(out_file, "{}", subseq)?;
    //                 writeln!(out_file, "+")?;
    //                 writeln!(out_file, "{}", &qual.as_str()[..j])?;
    //             }
    //             let subseq = &seq.as_str()[j..];
    //             if subseq.chars().count() > min_seq_length {
    //                 writeln!(out_file, "{}", header)?;
    //                 writeln!(out_file, "{}", subseq)?;
    //                 writeln!(out_file, "+")?;
    //                 writeln!(out_file, "{}", &qual.as_str()[j..])?;
    //             }
    //         }
    //     }
    // }

    Ok(())
}

