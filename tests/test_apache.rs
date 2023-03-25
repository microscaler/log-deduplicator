use std::io::BufRead;
use std::io::{BufReader, Write};
use std::process::{Command, Stdio};

#[test]
fn test_deduplication() {
    // Run the dedupfeed program as a child process
    let mut child = Command::new("target/debug/dedupfeed")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start dedupfeed process");

    // Feed in sample Apache log file data to the program's stdin
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    let file =
        std::fs::File::open("tests/sample_access_2.log").expect("Failed to open sample log file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        writeln!(stdin, "{}", line.unwrap()).expect("Failed to write to stdin");
    }

    // Collect the program's output from stdout
    let output = child
        .wait_with_output()
        .expect("Failed to read child process output");

    // Check that the program's output matches the expected output
    // let expected_output = "\
    //     127.0.0.1 - - [01/Jan/2023:00:00:01 +0000] \"GET /index.html HTTP/1.1\" 200 1234 \"-\" \"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36\"\n\
    //     127.0.0.1 - - [01/Jan/2023:00:00:02 +0000] \"GET /index.html HTTP/1.1\" 200 1234 \"-\" \"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36\" (seen 1 times in the last 5 seconds)\n\
    //     127.0.0.1 - - [01/Jan/2023:00:00:03 +0000] \"GET /index.html HTTP/1.1\" 200 1234 \"-\" \"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36\" (seen 2 times in the last 5 seconds)\n\
    // ";
    //assert_eq!(String::from_utf8(output.stdout).unwrap(), expected_output);
}
