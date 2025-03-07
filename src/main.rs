extern crate tokio;
use libsql::Builder;
use libsql::Database;
use std::path::{Path};
use std::io::{Write, BufRead, BufReader};
use std::fs::File;
use std::env;
use std::process::Command as stdCommand;
use std::time::Instant as timeInstant;
use chrono::Local;
#[tokio::main]
async fn main() {
    let mut bolok = true;
    let parm1dir: String;
    let mut parm2dir = String::new();
    let mut outseq: u32 = 1;

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!(" no input parameters; need bkdatabase, windirparse");
    } else {
        println!("The first argument is {}", args[1]);
        if args.len() < 3 {
            println!("The Only first argument and no winddirparse");
        } else {
            println!("The second argument is {}", args[2]);
            if !Path::new(&args[1]).exists() {
                println!("The first argument {} does not exist", args[1]);
            } else {
                println!("The first argument {} exists", args[1]);
                parm1dir = args[1].to_string();
                let db: Database;
                match Builder::new_local(parm1dir.clone()).build().await {
                   Err(e) => {
                       println!("data base build error: {}", e);
                   }
                   Ok(dbv) => {
                       db = dbv;
                       let conn1 = db.connect().unwrap();

                       let mut rows = conn1.query("SELECT name FROM sqlite_master WHERE type = \"table\" ", ()).await.unwrap();

                       let row = rows.next().await.unwrap().unwrap();

                       let tablena = row.get_value(0).unwrap();
                       if !(tablena == "blubackup".into()) {
                           println!("invalid table of {:?}", tablena);
                           bolok = false;
                       } else {
                           let mut rows1 = conn1.query("SELECT GROUP_CONCAT(NAME,',') FROM PRAGMA_TABLE_INFO('blubackup')", ()).await.unwrap();
                           let mut numlist = 0;
                           let mut collist: String = "---".to_string();
                           loop {
                                 match rows1.next().await {
                                     Ok(None) => {
                                         println!("none value after line {}", numlist);
                                         break;
                                     }
                                     Ok(rowx) => {
                                         numlist = numlist + 1;
                                         match rowx.unwrap().get_str(0) {
                                             Ok(strvalx) => {
                                                 collist = strvalx.to_string();
                                                 println!("line {} column listing output {}", numlist, collist);
                                             }
                                             Err(e) => {
                                                 println!("line {} column listing output is not text {:?}", numlist, e);
                                                 bolok = false;
                                                 break;
                                             }
                                         }
                                     }
                                     Err(e) => {
                                         println!("error after line {} of {}", numlist, e);
                                         bolok = false;
                                         break;
                                     }
                                 }
                           }
                           if numlist == 0 {
                               println!("no columns for table blubackup in database");
                               bolok = false;
                           } else if !(numlist == 1) {
                               println!("{} column list in database, last column list is: {}", numlist, collist);
                               bolok = false;
                           } else {
                              if !(collist == "refname,filename,dirname,filesize,filedate,md5sum,locations,notes") {
                                   println!("column list of {} instead of refname,filename,dirname,filesize,filedate,md5sum,locations,notes", collist);
                                   bolok = false;
                              }
                           }
                       }
                       if bolok {
                           if !Path::new(&args[2]).exists() {
                               println!("The second argument {} does not exist", args[2]);
                               bolok = false;
                           } else {
                               println!("The second argument {} exists", args[2]);
                               parm2dir = args[2].to_string();
                               let outputx = stdCommand::new("wc")
                                              .arg("-l")
                                              .arg(&parm2dir)
                                              .output()
                                              .expect("failed to execute process");
                               let stroutx = String::from_utf8_lossy(&outputx.stdout);
                               let vecout: Vec<&str> = stroutx.split(" ").collect();
                               let numlinesx: i64 = vecout[0].parse().unwrap_or(-9999);
                               if numlinesx == -9999 {
                                   println!("size of {} is invalid for wc -l command call", vecout[0]);
                                   bolok = false;
                               } else {
                                   let rows_num = numlinesx as u64;
                                   if rows_num < 2 {
                                       println!("size of {} is less than 2 for {}", rows_num, parm2dir);
                                       bolok = false;
                                   } else {
                                       let file = File::open(parm2dir.clone()).unwrap();
                                       let mut reader = BufReader::new(file);
                                       let mut linehd = String::new();
                                       bolok = false;
                                       loop {
                                             match reader.read_line(&mut linehd) {
                                                  Ok(bytes_read) => {
                                                     if bytes_read == 0 {
                                                         println!("error bytes_read == 0 for {}", parm2dir);
                                                         break;
                                                     }
                                                     let cnt = linehd.matches("|").count();
                                                     if cnt != 4 {
                                                         println!("first line of windirparse file is not valid: {}", linehd);
                                                     } else {
                                                         println!("windirparse file is ok with size of {} rows", rows_num);
                                                         bolok = true;
                                                     }
                                                     break;
                                                  }
                                                  Err(err) => {  
                                                     println!("error of {} reading {}", err, parm2dir);
                                                     break;
                                                  }
                                             };
                                       }
                                   }
                               }
                           }
                       }
                       if bolok {
                           let mut more1out: String = format!("./more1{:02}.excout", outseq);
                           let mut just1out: String = format!("./just1{:02}.neout", outseq);
                           let mut diffdateout: String = format!("./diffdate{:02}.excout", outseq);
                           let mut nobkupout: String = format!("./nobkup{:02}.neout", outseq);
                           let mut errout: String = format!("./generrors{:02}.errout", outseq);
                           loop {
                                 if !Path::new(&errout).exists() && !Path::new(&more1out).exists() && !Path::new(&just1out).exists()
                                    && !Path::new(&diffdateout).exists() && !Path::new(&nobkupout).exists() {
                                     break;
                                 } else {
                                     outseq = outseq + 1;
                                     more1out = format!("./more1{:02}.excout", outseq);
                                     just1out = format!("./just1{:02}.neout", outseq);
                                     diffdateout = format!("./diffdate{:02}.excout", outseq);
                                     nobkupout = format!("./nobkup{:02}.neout", outseq);
                                     errout = format!("./generrors{:02}.errout", outseq);
                                 }
                           }          
                           let mut diffdatefile = File::create(diffdateout).unwrap();
                           let mut nobkupfile = File::create(nobkupout).unwrap();
                           let mut more1file = File::create(more1out).unwrap();
                           let mut just1file = File::create(just1out).unwrap();
                           let mut errfile = File::create(errout).unwrap();
                           let filex = File::open(parm2dir.clone()).unwrap();
                           let mut readerx = BufReader::new(filex);
                           let mut linex = String::new();
                           let mut line1000: u64 = 0;
                           let mut linenumx: u64 = 0;
                           let start_time = timeInstant::now();

                           loop {
                                 match readerx.read_line(&mut linex) {
                                    Err(err) => {
                                        println!("read error {:?}", err);
                                        break;
                                    }
                                    Ok(bytes_read) => {
                                        if bytes_read == 0 {
                                            break;
                                        }
                                        line1000 = line1000 + 1;
                                        linenumx = linenumx + 1;
                                        if line1000 > 20 {
                                            let diffy = start_time.elapsed();
                                            let minsy: f64 = diffy.as_secs() as f64/60 as f64;
                                            let dateyy = Local::now();
                                            println!("line number {} records elapsed time {:.1} mins at {}", linenumx, minsy, dateyy.format("%H:%M:%S"));
                                            line1000 = 0;
                                        }
                                        let vecline: Vec<&str> = linex.split("|").collect();
                                        let inptdir = vecline[1].to_string();
                                        let inptsize: String = vecline[3].to_string();
                                        let inptdate: String = format!("{}.000", vecline[2]);
                                        let mut inptfilenm: String = vecline[0].to_string();
                                        if inptfilenm[..1].to_string() == '"'.to_string() {
                                            inptfilenm = inptfilenm[1..(inptfilenm.len()-1)].to_string();
                                        }
                                        let mut stmt = conn1
                                                        .prepare("SELECT  rowid, refname, filename, dirname, filesize, filedate, md5sum, locations, notes
                                                                  FROM blubackup
                                                                  WHERE filename = ?1")
                                                        .await
                                                        .unwrap();
                                        let mut rowsy = stmt.query([inptfilenm.clone()]).await.unwrap();
                                        let mut numentries = 0;
                                        let mut numdate = 0;
                                        let mut numsize = 0;
                                        loop {
                                              match rowsy.next().await {
                                                 Err(e) => {
                                                     println!("error  line {} after entries {} of {}", linenumx, numentries, e);
                                                     break;
                                                 }
                                                 Ok(None) => {
                                                     break;
                                                 }
                                                 Ok(rowz) => {
                                                     numentries = numentries + 1;
                                                     let bksizeint = rowz.as_ref().unwrap().get::<u64>(4).unwrap();
                                                     let bksize = format!("{}", bksizeint);
                                                     let bkdir = rowz.as_ref().unwrap().get::<String>(3).unwrap();
                                                     let bkdate = rowz.as_ref().unwrap().get::<String>(5).unwrap();
                                                     let bkref = rowz.as_ref().unwrap().get::<String>(1).unwrap();
                                                     let stroutput = format!("{}|{}|{}|{}|{}|{}|{}|{}|", 
                                                              bkref, bkdir, inptfilenm, bksize, inptsize, bkdate, inptdate, inptdir);
                                                     if bksize == inptsize {
                                                         numsize = numsize + 1;
                                                         if bkdate == inptdate {
                                                             numdate = numdate + 1;
                                                             if numdate > 1 {
                                                                 writeln!(&mut more1file, "{}", stroutput).unwrap();
                                                             } else {
                                                                 writeln!(&mut just1file, "{}", stroutput).unwrap();
                                                             }
                                                         } else {
                                                            writeln!(&mut diffdatefile, "{}", stroutput).unwrap();
                                                         }
                                                     }

//                                                     println!("line {} has {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", linenumx, rowz.as_ref().unwrap().column_name(0), rowz.as_ref().unwrap().column_name(1), rowz.as_ref().unwrap().column_name(2),
//                                                        rowz.as_ref().unwrap().column_name(3), rowz.as_ref().unwrap().column_name(4), rowz.as_ref().unwrap().column_name(5),
//                                                        rowz.as_ref().unwrap().column_name(6), rowz.as_ref().unwrap().column_name(7), rowz.as_ref().unwrap().column_name(8));
//                                                     println!("line {} has {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", linenumx, rowz.as_ref().unwrap().column_type(0), rowz.as_ref().unwrap().column_type(1), rowz.as_ref().unwrap().column_type(2),
//                                                        rowz.as_ref().unwrap().column_type(3), rowz.as_ref().unwrap().column_type(4), rowz.as_ref().unwrap().column_type(5),
//                                                        rowz.as_ref().unwrap().column_type(6), rowz.as_ref().unwrap().column_type(7), rowz.unwrap().column_type(8));
                                                 }
                                              }
                                        }
                                        if numentries < 1 {
                                            let stroutput: String = format!("{} -{}- -{}-", linenumx, linex, inptfilenm);
                                            writeln!(&mut nobkupfile, "{}", stroutput).unwrap();
                                        } else {
                                            if numsize < 1 {
                                                let stroutput: String = format!("{} NO MATCHING SIZE -{}- -{}-", linenumx, linex, inptfilenm);
                                                writeln!(&mut errfile, "{}", stroutput).unwrap();
                                            } else {
                                                if numdate < 1 {
                                                    let stroutput: String = format!("{} NO MATCHING DATE -{}- -{}-", linenumx, linex, inptfilenm);
                                                    writeln!(&mut errfile, "{}", stroutput).unwrap();
                                                }
                                            }
                                        }
                                        linex.clear();
                                    }
                                 }
                           }
                       }
                   }
                }
            }
        }
    }
}
